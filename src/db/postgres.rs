use std::time::Duration;

use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::info;
use sqlx::migrate::Migrator;

pub async fn create_pool(database_url :String) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Failed connect to Posgtresql");
    
    info!("Posgtresql connected");

    pool
}


static APP_MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn migrate_app(pool: &PgPool) {
    APP_MIGRATOR
        .run(pool)
        .await
        .expect("App migration failed");
}