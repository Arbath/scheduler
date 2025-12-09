use std::time::Duration;

use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::info;

pub async fn create_pool(database_url :String, auto_migrate: bool) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Failed connect to Posgtresql");
    
    info!("Posgtresql connected");

    if auto_migrate {
        info!("Migrate database....");
        migrations(&pool).await;
    }

    pool
}

async fn migrations(pool: &PgPool){
    sqlx::migrate!("./migrations") // Lokasi folder migrations
        .run(pool)
        .await
        .expect("Gagal menjalankan database migration");

    info!("Migrations success");
}