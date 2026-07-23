use std::{env, sync::Arc};
use axum::{Json, extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse};
use axum_extra::extract::{CookieJar, cookie::{Cookie, SameSite}};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use rand::RngExt;
use time;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct OtpRequest {
    pub email: String,
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub email: String,
    pub code: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
struct JwtClaims {
    pub sub: String, // User ID
    pub exp: usize,  // Expiration
}

fn hash_secret(secret: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(secret.as_bytes());
    hex::encode(hasher.finalize())
}

pub async fn request_otp(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<OtpRequest>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let email = payload.email.trim().to_lowercase();

    let plain_code = format!("{:06}", rand::rng().random_range(0..1_000_000));
    let code_hash = hash_secret(&plain_code);

    let expires_at = Utc::now() + Duration::minutes(15);

    let mut tx = state.pg_pool.begin().await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    sqlx::query!("DELETE FROM auth_otps WHERE email = $1", email)
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if rand::rng().random_bool(0.05) {
        let pool_clone = state.pg_pool.clone();

        tokio::spawn(async move {
            let _ = sqlx::query!("DELETE FROM auth_otps WHERE expires_at < NOW()")
                .execute(&pool_clone)
                .await;
        });
    }

    sqlx::query!(
        "INSERT INTO auth_otps (email, code_hash, expires_at) VALUES ($1, $2, $3)",
        email, 
        code_hash,
        expires_at
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit().await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    println!("OTP for {} is {}", email, plain_code);

    Ok((StatusCode::OK, "Otp sent successfully"))
}

pub async fn verify_otp(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    jar: CookieJar,
    Json(payload): Json<VerifyRequest>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let email = payload.email.trim().to_lowercase();
    let code_hash = hash_secret(&payload.code.trim());

    let otp_record = sqlx::query!(
        "SELECT id, expires_at FROM auth_otps WHERE email = $1 AND code_hash = $2",
        email,
        code_hash
    )
    .fetch_optional(&state.pg_pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let otp = match otp_record {
        Some(record) => record,
        None => return Err((StatusCode::UNAUTHORIZED, "Invalid OTP".to_string()))
    };

    if otp.expires_at < Utc::now() {
        return Err((StatusCode::UNAUTHORIZED, "OTP has expired".to_string()));
    }

    sqlx::query!("DELETE FROM auth_otps WHERE id = $1", otp.id)
        .execute(&state.pg_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user_record = sqlx::query!("SELECT id FROM users WHERE email = $1", email)
        .fetch_optional(&state.pg_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user_id = match user_record {
        Some(u) => u.id,
        None => {
            let new_id = Uuid::now_v7();
            sqlx::query!("INSERT INTO users (id, email) VALUES ($1, $2) RETURNING id", new_id, email)
                .fetch_one(&state.pg_pool)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
                .id
        }
    };

    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "super_secret_fallback".into());
    let expiration = (Utc::now() + Duration::minutes(15)).timestamp() as usize;

    let claims = JwtClaims {
        sub: user_id.to_string(),
        exp: expiration
    };

    let access_token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes())
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("JWT Error: {}", e)))?;

    let refresh_id = Uuid::now_v7();

    let refresh_uuid = Uuid::now_v7();
    let refresh_hash = hash_secret(&refresh_uuid.to_string());
    let refresh_expires = Utc::now() + Duration::days(30);

    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    sqlx::query!(
        "INSERT INTO refresh_tokens (id, user_id, token_hash, expires_at, user_agent) VALUES ($1, $2, $3, $4, $5)",
        refresh_id,
        user_id,
        refresh_hash,
        refresh_expires,
        user_agent
    )
    .execute(&state.pg_pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let cookie = Cookie::build(("refresh_token", refresh_uuid.to_string()))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/auth")
        .max_age(time::Duration::days(30))
        .build();

    let response_body = AuthResponse {
        access_token,
        message: "Authentication successful".to_string()
    };

    Ok((jar.add(cookie), Json(response_body)))
}

pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    jar: CookieJar
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let cookie = jar.get("refresh_token")
        .ok_or((StatusCode::UNAUTHORIZED, "No refresh token found".to_string()))?;

    let refresh_uuid_str = cookie.value();
    let token_hash = hash_secret(refresh_uuid_str);


    let session = sqlx::query!(
        r#"
        SELECT r.user_id
        FROM refresh_tokens r
        WHERE r.token_hash = $1 AND r.expires_at > NOW()
        "#,
        token_hash
    )
    .fetch_optional(&state.pg_pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let record = session.ok_or((StatusCode::UNAUTHORIZED, "Invalid or expired refresh token".to_string()))?;

    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "super_secret_fallback".into());
    let expiration = (Utc::now() + Duration::minutes(15)).timestamp() as usize;

    let claims = JwtClaims {
        sub: record.user_id.to_string(),
        exp: expiration
    };

    let access_token = encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(jwt_secret.as_bytes())
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("JWT Error: {}", e)))?;

    Ok(Json(AuthResponse {
        access_token,
        message: "Token refreshed successfully".to_string()
    }))
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
    jar: CookieJar
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Some(cookie) = jar.get("refresh_token") {
        let token_hash = hash_secret(cookie.value());

        let _ = sqlx::query!("DELETE FROM refresh_tokens WHERE token_hash = $1", token_hash)
            .execute(&state.pg_pool)
            .await;
    }

    let mut removal_cookie = Cookie::from("refresh_token");
    removal_cookie.set_path("/auth");

    let updated_jar = jar.remove(removal_cookie);

    Ok((
        updated_jar,
        Json(serde_json::json!({ "message": "Logged out successfully" }))
    ))
}