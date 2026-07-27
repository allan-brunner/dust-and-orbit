use lettre::{AsyncSmtpTransport, Tokio1Executor};
use mongodb::Client as MongoClient;
use redis::aio::ConnectionManager;
use sqlx::PgPool;

pub struct AppState {
    pub pg_pool: PgPool,
    pub mongo_client: MongoClient,
    pub redis_manager: ConnectionManager,
    pub mailer: AsyncSmtpTransport<Tokio1Executor>,
}