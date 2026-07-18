use axum::{
    routing::get,
    Router,
    response::Json,
    extract::State,
};
use mongodb::{Client as MongoClient, options::ClientOptions};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use dotenvy::dotenv;
use std::env;
use shared_logic::calculate_level_from_xp;

struct AppState {
    pg_pool: PgPool,
    mongo_client: MongoClient,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pg_login = env::var("PG_LOGIN").expect("PG_LOGIN must be set in .env");
    let pg_pass = env::var("PG_PASS").expect("PG_PASS must be set in .env");
    let pg_host = env::var("PG_HOST").expect("PG_HOST must be set in .env");
    let pg_port = env::var("PG_PORT").expect("PG_PORT must be set in .env");
    let pg_db = env::var("PG_DB").expect("PG_DB must be set in .env");

    let mongo_login = env::var("MONGO_LOGIN").expect("MONGO_LOGIN must be set in .env");
    let mongo_pass = env::var("MONGO_PASS").expect("MONGO_PASS must be set in .env");
    let mongo_host = env::var("MONGO_HOST").expect("MONGO_HOST must be set in .env");
    let mongo_port = env::var("MONGO_PORT").expect("MONGO_PORT must be set in .env");

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let pg_url = format!("postgres://{pg_login}:{pg_pass}@{pg_host}:{pg_port}/{pg_db}");
    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_url)
        .await?;
    println!("Connected to PostgreSQL");

    let mongo_url = format!("mongodb://{mongo_login}:{mongo_pass}@{mongo_host}:{mongo_port}");
    let client_options = ClientOptions::parse(mongo_url).await?;
    let mongo_client = MongoClient::with_options(client_options)?;
    println!("Connected to MongoDB");

    let shared_state = Arc::new(AppState {
        pg_pool,
        mongo_client,
    });

    let app = Router::new()
        .route("/api/status", get(status_handler))
        .with_state(shared_state)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    println!("Server running on http://0.0.0.0:{0}", port);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn status_handler(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let pg_row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&state.pg_pool)
        .await
        .unwrap_or((0,));

    let mongo_db = state.mongo_client.database("dust_and_orbit");
    let mongo_collections = mongo_db.list_collection_names().await.unwrap_or_default();

    let player_xp = 4500;
    let server_calculated_level = calculate_level_from_xp(player_xp);

    Json(serde_json::json!({
        "status": "online",
        "postgres_connected": pg_row.0 == 1,
        "mongo_collections_found": mongo_collections.len(),
        "message": "Backend is locked and loaded!",

        "player_stats": {
            "xp": player_xp,
            "verified_level": server_calculated_level
        }
    }))
}