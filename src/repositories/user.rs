use sqlx::PgPool;
use crate::models::user::User;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: &i32) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await
    }
    
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(&self.pool)
        .await
    }
    
    pub async fn find_by_username_or_email(&self, identifier: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1 OR email = $1")
            .bind(identifier)
            .fetch_optional(&self.pool)
            .await
    }
}