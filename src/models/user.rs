use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use chrono::{DateTime, Utc};
use crate::response::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32, 
    pub username: String,
    pub email: String,
    #[serde(skip)] 
    pub password: String,
    pub is_superuser: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub async fn user_exists(
        pool: &PgPool,
        user_id: &i32,
    ) -> Result<User, AppError> {
    let user =sqlx ::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::InternalError(e.to_string()))?
    .ok_or(AppError::NotFound("User not found".to_string()))?;
    Ok(user)
    }
}