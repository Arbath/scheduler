use axum::{extract::FromRequestParts, http::{request::Parts, header}};
use crate::state::AppState;
use crate::utils::{response::*, auth::verify_access_token};
use crate::models::user::User; 

pub struct AuthUser(pub User);
pub struct AuthAdmin(pub User);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let user = fetch_user_from_request(parts, state).await?;
        Ok(AuthUser(user))
    }
}

impl FromRequestParts<AppState> for AuthAdmin {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let uri = parts.uri.clone();
        let user = fetch_user_from_request(parts, state).await.map_err(|e|e.with_path(&uri))?;

        if !user.is_superuser {
            return Err(AppError::Forbidden("Insufficient permissions: Superuser required".to_string()).with_path(&uri));
        }

        Ok(AuthAdmin(user))
    }
}

async fn fetch_user_from_request(parts: &Parts, state: &AppState) -> Result<User, AppError> {
    let auth_header = parts
        .headers
        .get(header::AUTHORIZATION)
        .ok_or(AppError::AuthError("Missing Authorization header".to_string()))?;

    let auth_value = auth_header
        .to_str()
        .map_err(|_| AppError::AuthError("Invalid header format".to_string()))?;

    if !auth_value.starts_with("Bearer ") {
        return Err(AppError::AuthError("Invalid token type".to_string()));
    }

    let token = &auth_value[7..];

    let claims = verify_access_token(&state.jwt_config.secret, token)
        .map_err(|_| AppError::AuthError("Invalid or expired token".to_string()))?;

    let user_id = claims.sub.parse::<i32>()
        .map_err(|_| AppError::AuthError("Invalid ID format".to_string()))?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1").bind(user_id)
        .fetch_optional(&state.database)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?
        .ok_or(AppError::AuthError("User not found".to_string()))?;

    Ok(user)
}