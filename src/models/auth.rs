use serde::{Deserialize, Serialize};
use sqlx::PgPool;
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

impl RefreshToken {
    pub async fn save_refresh_token(
        pool: &PgPool, 
        token: &str, 
        user_id: &i32,
        expires_at: DateTime<Utc>
    ) -> Result<(), sqlx::Error> {
        
        _ = sqlx::query!(
            "INSERT INTO refresh_tokens (token, user_id, expires_at) VALUES ($1, $2, $3)",
            token,
            user_id,
            expires_at
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

#[derive(Deserialize)]
pub struct LoginReq {
    pub username: String,
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