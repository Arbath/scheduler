use std::env;
use tracing::Level;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub jwt_secret: String,
    pub access_ttl:i32,
    pub refresh_ttl:i32,
    pub db_url: String,
    pub migrate: bool,
    pub log_level: Level,
}

impl Config {
    pub fn init() -> Config {
        let port = env::var("APP_PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse::<u16>()
            .expect("Invalid APP_PORT");

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET required");
        let access_ttl = env::var("ACCESS_TTL_IN_MINUTES").ok().and_then(|v| v.parse::<i32>().ok()).unwrap_or(15) * 60;
        let refresh_ttl = env::var("REFRESH_TTL_IN_DAYS").ok().and_then(|v| v.parse::<i32>().ok()).unwrap_or(7) * 24 * 60 * 60;
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL required");
        let migrate = env::var("MIGRATIONS").unwrap_or("false".to_string()).to_lowercase().parse::<bool>().unwrap_or(false);
        let log_level_str = env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".to_string()).to_uppercase();
        
        let log_level = match log_level_str.as_str() {
            "TRACE" => Level::TRACE,
            "DEBUG" => Level::DEBUG,
            "WARN"  => Level::WARN,
            "ERROR" => Level::ERROR,
            _ => Level::INFO,
        };

        Config {
            port,
            jwt_secret,
            access_ttl,
            refresh_ttl,
            db_url,
            migrate,
            log_level,
        }
    }
}