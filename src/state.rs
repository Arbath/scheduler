use sqlx::PgPool;
use std::sync::Arc;

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
}