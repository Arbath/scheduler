use apalis_sql::postgres::PostgresStorage;
use sqlx::PgPool;
use std::sync::Arc;
use crate::models::fetch::Api;

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub access_ttl: i64,
    pub refresh_ttl: i64,
}

#[derive(Clone)]
pub struct AppState {
    pub jwt_config: Arc<JwtConfig>, 
    pub database: PgPool,
    pub http_client: reqwest::Client,
    pub job_queue: PostgresStorage<Api>,
}