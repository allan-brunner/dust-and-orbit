mod handlers;
mod router;
mod state;

use dotenvy::dotenv;
use lettre::{AsyncSmtpTransport, Tokio1Executor, transport::smtp::authentication::Credentials};
use mongodb::{options::ClientOptions, Client as MongoClient};
use sqlx::{postgres::PgPoolOptions, migrate};
use std::{env, net::SocketAddr, sync::Arc};
use redis::Client as RedisClient;

use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // PostgreSQL Setup
    let pg_login = env::var("PG_LOGIN").expect("PG_LOGIN must be set");
    let pg_pass = env::var("PG_PASS").expect("PG_PASS must be set");
    let pg_host = env::var("PG_HOST").expect("PG_HOST must be set");
    let pg_port = env::var("PG_PORT").expect("PG_PORT must be set");
    let pg_db = env::var("PG_DB").expect("PG_DB must be set");

    let pg_url = format!("postgres://{pg_login}:{pg_pass}@{pg_host}:{pg_port}/{pg_db}");
    
    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_url)
        .await?;
    
    migrate!("../migrations").run(&pg_pool).await?;
    println!("Connected to PostgreSQL and migrated!");

    // MongoDB Setup
    let mongo_login = env::var("MONGO_LOGIN").expect("MONGO_LOGIN must be set");
    let mongo_pass = env::var("MONGO_PASS").expect("MONGO_PASS must be set");
    let mongo_host = env::var("MONGO_HOST").expect("MONGO_HOST must be set");
    let mongo_port = env::var("MONGO_PORT").expect("MONGO_PORT must be set");

    let mongo_url = format!("mongodb://{mongo_login}:{mongo_pass}@{mongo_host}:{mongo_port}");
    
    let client_options = ClientOptions::parse(mongo_url).await?;
    let mongo_client = MongoClient::with_options(client_options)?;
    println!("Connected to MongoDB");

    // Redis Setup
    let redis_host = env::var("REDIS_HOST").expect("REDIS_HOST must be set");
    let redis_port = env::var("REDIS_PORT").unwrap_or_else(|_| "6379".into());
    let redis_pass = env::var("REDIS_PASS").expect("REDIS_PASS must be set");

    let redis_url = format!("redis://:{redis_pass}@{redis_host}:{redis_port}");
    let redis_client = RedisClient::open(redis_url)?;
    
    let redis_manager = redis_client.get_connection_manager().await?;
    println!("Connected to Redis!");

    let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "mail.infomaniak.com".into());
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let smtp_pass = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

    let creds = Credentials::new(smtp_username, smtp_pass);

    let mailer: AsyncSmtpTransport<Tokio1Executor> = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)
        .expect("Failed to build SMTP relay")
        .credentials(creds)
        .build();

    // App State & Router
    let shared_state = Arc::new(AppState {
        pg_pool,
        mongo_client,
        redis_manager,
        mailer,
    });

    let app = router::create_router(shared_state);

    // Server Boot
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    
    println!("Server running on http://0.0.0.0:{0}", port);
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    Ok(())
}