use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: String,
    pub database: PgPool,
}