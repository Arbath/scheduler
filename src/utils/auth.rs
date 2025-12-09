use crate::response::AppError;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::models::auth::{Claims, RefreshToken};
use crate::state::AppState;
use crate::models::user::User;

pub async fn gen_access_token(user: &User, state: &AppState) -> Result<String, AppError> {
    let now = Utc::now();
    let access_duration = Duration::days(7);
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
        &EncodingKey::from_secret(state.jwt_secret.as_bytes())
    ).map_err(|e| AppError::InternalError(e.to_string()))?;

    Ok(access_token)
}
pub async fn gen_refresh_token(user: &User, state: &AppState) -> Result<String, AppError> {
    let now = Utc::now();
    let refresh_duration = Duration::days(7);
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
        &EncodingKey::from_secret(state.jwt_secret.as_bytes())
    ).map_err(|e| AppError::InternalError(e.to_string()))?;

    // Save to database
    RefreshToken::save_refresh_token(
        &state.database,
        &refresh_token, 
        &user.id,
        refresh_expires_at
    )
    .await
    .map_err(|e| AppError::InternalError(e.to_string()))?;

    Ok(refresh_token)
}