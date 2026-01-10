use axum::serve;
use dotenvy::dotenv;
use scheduler::{
    config::*,
    create_app,
    state::{AppState, JwtConfig},
    db::{postgres, postgres::{migrate_app, create_root_user}},
    jobs::{email_jobs::EmailJob, workers::setup_background_workers}
};
use std::sync::Arc;
use apalis_sql::postgres::PostgresStorage;

use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::init(); 
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(config.log_level)
        .compact()
        .init();
    
    let port = config.port; 
    let pool = postgres::create_pool(config.db_url).await;
    if config.migrate {
        migrate_app(&pool).await;
        let _ = create_root_user(&pool, config.root_username, config.root_email, config.root_password).await;
    }

    let jwt_config = JwtConfig {
        secret: config.jwt_secret,
        access_ttl: config.access_ttl as i64,
        refresh_ttl: config.refresh_ttl as i64,
    };
    let apalis_config = apalis_sql::Config::default()
        .set_poll_interval(std::time::Duration::from_secs(config.min_job_interval));

    let email_storage =
        PostgresStorage::<EmailJob>::new_with_config(pool.clone(), apalis_config);

    setup_background_workers(pool.clone(),email_storage.clone()).await;

    let state = AppState {
        jwt_config: Arc::new(jwt_config),
        database: pool,
        email_job_queue: email_storage.clone(),
    };
    
    let addr = format!("0.0.0.0:{}", port);
    let app = create_app(state);
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            error!("Failed bind to {}: {:?}", port, e);
            return;
        }
    };
    
    info!("Log level : {}", &config.log_level);
    info!("Server started at : {}", port);
    
    if let Err(e) = serve(listener, app).await {
        error!("Server Error: {:?}", e);
    }
}
