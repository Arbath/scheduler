use axum::serve;
use dotenvy::dotenv;
use scheduler::{config::*, create_app, db, state::AppState};

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
    let pool = db::postgres::create_pool(config.db_url, config.migrate).await;
    let state = AppState {
        jwt_secret: config.jwt_secret,
        database: pool,
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
