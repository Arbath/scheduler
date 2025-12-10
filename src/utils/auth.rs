use jsonwebtoken::{encode, EncodingKey, Header, decode, DecodingKey, Validation, Algorithm};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::{Duration, Utc};
use crate::models::auth::Claims;
use crate::models::user::User;
use crate::utils::response::AppError;
use crate::state::AppState;

pub async fn gen_access_token(user: &User, state: &AppState) -> Result<String, AppError> {
    let now = Utc::now();
    let access_duration = Duration::seconds(state.jwt_config.access_ttl);
    let access_expires_at = now + access_duration;
    let claims = Claims {
        sub: user.id.to_string(),
        username: user.username.clone(),
        exp: access_expires_at.timestamp() as usize, 
        iat: now.timestamp() as usize,
        token_type: "access".to_string(),
    };

    let access_token = encode(
        &Header::default(),&claims,
        &EncodingKey::from_secret(state.jwt_config.secret.as_bytes())
    ).map_err(|e| AppError::InternalError(e.to_string()))?;

    Ok(access_token)
}
pub async fn gen_refresh_token(user: &User, state: &AppState) -> Result<String, AppError> {
    let now = Utc::now();
    let refresh_duration = Duration::seconds(state.jwt_config.refresh_ttl);
    let refresh_expires_at = now + refresh_duration;
    let claims = Claims {
        sub: user.id.to_string(),
        username: user.username.clone(),
        exp: refresh_expires_at.timestamp() as usize, 
        iat: now.timestamp() as usize,
        token_type: "refresh".to_string(),
    };
    let refresh_token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_config.secret.as_bytes())
    ).map_err(|e| AppError::InternalError(e.to_string()))?;

    Ok(refresh_token)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|_| "Invalid hash format in database".to_string())?;

    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub fn verify_access_token(jwt_secret: &str, token: &str) -> Result<Claims, AppError> {
    let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token, 
        &decoding_key, 
        &validation,
    ).map_err(|_err| {
        AppError::AuthError("Invalid or expired access token".to_string())
    })?;

    Ok(token_data.claims)
}

pub fn verify_refresh_token(jwt_secret: &str, token: &str) -> Result<Claims, AppError> {
    let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    
    let token_data = decode::<Claims>(
        token, 
        &decoding_key, 
        &Validation::default(),
    ).map_err(|_| AppError::AuthError("Invalid or expired refresh token".to_string()))?;

    Ok(token_data.claims)
}