use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sqlx::PgPool;
use crate::models::user::User; 

pub async fn authenticate(pool: &PgPool, username: &str, password: &str) -> Result<User, &'static str> {

    let user = sqlx::query_as!(
        User, 
        "SELECT * FROM users WHERE username = $1",
        username
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| "Database error")?;

    let user = match user {
        Some(u) => u,
        None => return Err("Account not found"),
    };

    let parsed_hash = PasswordHash::new(&user.password)
        .map_err(|_| "Invalid hash format in database")?;

    if Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_err() {
        return Err("Invalid password");
    }

    Ok(user)
}