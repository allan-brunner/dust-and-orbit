use mongodb::Client as MongoClient;
use sqlx::PgPool;

pub struct AppState {
    pub pg_pool: PgPool,
    pub mongo_client: MongoClient,
}