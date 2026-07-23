use axum::{extract::State, response::Json};
use std::sync::Arc;
use shared_logic::calculate_level_from_xp;

use crate::state::AppState;

pub async fn status_handler(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let pg_row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&state.pg_pool)
        .await
        .unwrap_or((0,));

    let pg_market_listings: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM market_listings")
        .fetch_one(&state.pg_pool)
        .await
        .unwrap_or(Some(0))
        .unwrap_or(0);

    let mongo_db = state.mongo_client.database("dust_and_orbit");
    let mongo_collections = mongo_db.list_collection_names().await.unwrap_or_default();

    let player_xp = 4500;
    let server_calculated_level = calculate_level_from_xp(player_xp);

    Json(serde_json::json!({
        "status": "online",
        "postgres_connected": pg_row.0 == 1,
        "postgres_market_listings": pg_market_listings,
        "mongo_collections_found": mongo_collections.len(),
        "message": "Backend is locked and loaded!",
        "player_stats": {
            "xp": player_xp,
            "verified_level": server_calculated_level
        }
    }))
}