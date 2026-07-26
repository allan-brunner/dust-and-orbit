use axum::{extract::State, response::Json};
use mongodb::bson::doc;
use serde::Serialize;
use std::{sync::Arc, time::Instant};

use crate::state::AppState;

#[derive(Serialize)]
pub struct PublicHealthResponse {
    pub status: String,
}

pub async fn public_health_handler() -> Json<PublicHealthResponse> {
    Json(PublicHealthResponse {
        status: "ok".to_string(),
    })
}

#[derive(Serialize)]
pub struct AdminStatusResponse {
    pub status: String,
    pub backend_version: String,
    pub db_postgres: String,
    pub db_mongo: String,
}

pub async fn admin_status_handler(State(state): State<Arc<AppState>>) -> Json<AdminStatusResponse> {
    let pg_start = Instant::now();
    let db_postgres = match sqlx::query("SELECT 1").execute(&state.pg_pool).await {
        Ok(_) => format!("ok ({}ms)", pg_start.elapsed().as_micros() as f64 / 1000.0),
        Err(_) => "error".to_string(),
    };

    let mongo_start = Instant::now();
    let db_mongo = match state
        .mongo_client
        .database("admin")
        .run_command(doc! { "ping": 1 })
        .await
    {
        Ok(_) => format!("ok ({}ms)", mongo_start.elapsed().as_micros() as f64 / 1000.0),
        Err(_) => "error".to_string(),
    };

    Json(AdminStatusResponse {
        status: "ok".to_string(),
        backend_version: env!("CARGO_PKG_VERSION").to_string(),
        db_postgres,
        db_mongo,
    })
}