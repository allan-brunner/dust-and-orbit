mod handlers;
mod router;
mod state;

use dotenvy::dotenv;
use mongodb::{options::ClientOptions, Client as MongoClient};
use sqlx::{postgres::PgPoolOptions, migrate};
use std::{env, sync::Arc};

use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // PostgreSQL Setup
    let pg_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env::var("PG_LOGIN").expect("PG_LOGIN must be set"),
        env::var("PG_PASS").expect("PG_PASS must be set"),
        env::var("PG_HOST").expect("PG_HOST must be set"),
        env::var("PG_PORT").expect("PG_PORT must be set"),
        env::var("PG_DB").expect("PG_DB must be set")
    );
    
    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_url)
        .await?;
    
    migrate!("../migrations").run(&pg_pool).await?;
    println!("Connected to PostgreSQL and migrated!");

    // MongoDB Setup
    let mongo_url = format!(
        "mongodb://{}:{}@{}:{}",
        env::var("MONGO_LOGIN").expect("MONGO_LOGIN must be set"),
        env::var("MONGO_PASS").expect("MONGO_PASS must be set"),
        env::var("MONGO_HOST").expect("MONGO_HOST must be set"),
        env::var("MONGO_PORT").expect("MONGO_PORT must be set")
    );
    
    let client_options = ClientOptions::parse(mongo_url).await?;
    let mongo_client = MongoClient::with_options(client_options)?;
    println!("Connected to MongoDB");

    // App State & Router
    let shared_state = Arc::new(AppState {
        pg_pool,
        mongo_client,
    });

    let app = router::create_router(shared_state);

    // Server Boot
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    
    println!("Server running on http://0.0.0.0:{0}", port);
    axum::serve(listener, app).await?;

    Ok(())
}