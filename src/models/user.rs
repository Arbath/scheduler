use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

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
