use sqlx::PgPool;
use chrono::{DateTime, Utc};

pub struct TokenRepository {
    pool: PgPool,
}

impl TokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save_token(
        &self, 
        token: &str, 
        user_id: i32, 
        expires_at: DateTime<Utc>
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO refresh_tokens (token, user_id, expires_at) VALUES ($1, $2, $3)",
            token,
            user_id,
            expires_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    
    pub async fn exists(&self, token: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "SELECT 1 as one FROM refresh_tokens WHERE token = $1",
            token
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.is_some())
    }

    pub async fn revoke(&self, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM refresh_tokens WHERE token = $1", token)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}