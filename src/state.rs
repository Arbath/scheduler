use apalis_sql::postgres::PostgresStorage;
use sqlx::PgPool;
use std::sync::Arc;
use crate::models::fetch::Api;

#[derive(Clone)]
pub struct AppConfig {
    pub secret: String,
    pub access_ttl: i64,
    pub refresh_ttl: i64,
    pub concurrency: u32,
}

#[derive(Clone)]
pub struct AppState {
    pub app_config: Arc<AppConfig>,
    pub database: PgPool,
    pub http_client: reqwest::Client,
    pub job_queue: PostgresStorage<Api>,
}