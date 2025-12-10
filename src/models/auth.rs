use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, 
    pub username: String,
    pub exp: usize,
    pub iat: usize,   
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct RefreshToken {
    pub token: String,
    pub user_id: i32,
    pub expires_at: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct LoginReq {
    pub identifier: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginRes {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct RefreshTokenReq {
    pub refresh_token: String,
}