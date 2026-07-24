use axum::{extract::State, Json};
use mongodb::bson::doc;
use serde::Serialize;
use std::{sync::Arc, time::Instant};

use crate::state::AppState;

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub backend_version: String,
    pub db_postgres: String,
    pub db_mongo: String,
    pub active_sessions: i64,
}

pub async fn status_handler(State(state): State<Arc<AppState>>) -> Json<StatusResponse> {
    let pg_start = Instant::now();
    let db_postgres = match sqlx::query("SELECT 1").execute(&state.pg_pool).await {
        Ok(_) => format!("ok ({:.2}ms)", pg_start.elapsed().as_micros() as f64 / 1000.0),
        Err(e) => format!("error: {}", e),
    };

    let mongo_start = Instant::now();
    let db_mongo = match state
        .mongo_client
        .database("admin")
        .run_command(doc! { "ping": 1 })
        .await
    {
        Ok(_) => format!("ok ({:.2}ms)", mongo_start.elapsed().as_micros() as f64 / 1000.0),
        Err(e) => format!("error: {}", e),
    };

    let active_sessions: i64 = sqlx::query_scalar!(
        "SELECT COUNT(DISTINCT user_id) FROM refresh_tokens WHERE expires_at > NOW()"
    )
    .fetch_one(&state.pg_pool)
    .await
    .unwrap_or(Some(0))
    .unwrap_or(0);

    let backend_version = env!("CARGO_PKG_VERSION").to_string();

    Json(StatusResponse {
        status: "ok".to_string(),
        backend_version,
        db_postgres,
        db_mongo,
        active_sessions,
    })
}