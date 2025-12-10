use axum::{extract::FromRequestParts, http::{request::Parts, header}};
use crate::state::AppState;
use crate::utils::{response::AppError, auth::verify_access_token};
use crate::models::user::User; 

pub struct AuthUser(pub User);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
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
            .map_err(|_| AppError::AuthError("Invalid ID format in token".to_string()))?;
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
            .fetch_optional(&state.database)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?
            .ok_or(AppError::AuthError("User not found".to_string()))?;

        Ok(AuthUser(user))
    }
}