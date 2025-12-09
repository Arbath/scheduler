use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use crate::response::AppError;
use jsonwebtoken::{decode, DecodingKey, Validation};

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

    pub async fn token_exists(
        pool: &PgPool,
        token: &str,
    ) -> Result<(), AppError> {
        let token_exists = sqlx::query!(
            "SELECT user_id FROM refresh_tokens WHERE token = $1",
            token
        )
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()));

        if token_exists?.is_none() {
            return Err(AppError::AuthError("Refresh token has been revoked/used".to_string()));
        }
        Ok(())
    }

    pub async fn revoke_token(
        pool: &PgPool,
        token: &str
    ) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM refresh_tokens WHERE token = $1", token)
            .execute(pool)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;
        Ok(())
    }

    pub fn verify_refresh_token(jwt_secret: &str, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(
            token, 
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        ).map_err(|_| AppError::AuthError("Invalid token".to_string()))?;

        Ok(token_data.claims)
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