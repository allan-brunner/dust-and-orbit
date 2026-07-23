use axum::http::Method;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::{routing::{get, post}, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::state::AppState;
use crate::handlers::status::status_handler;
use crate::handlers::auth::{refresh_token, request_otp, verify_otp, logout};

pub fn create_router(shared_state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_origin([
            "http://localhost:5173".parse().unwrap(),
            "https://dustandorbit.allanbrunner.dev".parse().unwrap()
        ])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS]);

    Router::new()
        .route("/api/status", get(status_handler))

        // Authentication routes
        .route("/auth/request-otp", post(request_otp))
        .route("/auth/verify", post(verify_otp))
        .route("/auth/refresh", post(refresh_token))
        .route("/auth/logout", post(logout))

        // State and CORS layer
        .with_state(shared_state)
        .layer(cors)
}