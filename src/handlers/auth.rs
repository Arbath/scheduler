use axum::{Json, extract::State, http::Uri, response::IntoResponse};
use crate::response::{WebResponse, AppError};
use crate::models::user::User;
use crate::utils::auth::{gen_access_token, gen_refresh_token};
use crate::state::AppState;
use crate::models::auth::{LoginReq, LoginRes, RefreshTokenReq};
use crate::services::auth::authenticate;

pub async fn login_hand(
    State(state): State<AppState>,
    uri: Uri,
    Json(data): Json<LoginReq>
) -> Result<impl IntoResponse, AppError> {  
    // Authenticate User
    let user = authenticate(&state.database, &data.username, &data.password) // Asumsi butuh DB connection
        .await
        .map_err(|_| AppError::AuthError("Invalid username or password".to_string()))?;
    
    let access_token = gen_access_token(&user, &state).await?;
    let refresh_token = gen_refresh_token(&user, &state).await?;
    let response_data = LoginRes { access_token, refresh_token };

    Ok(WebResponse::ok(&uri, "Login success!", response_data))
}

use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::models::auth::Claims;

pub async fn refresh_hand(
    State(state): State<AppState>,
    uri: Uri,
    Json(json): Json<RefreshTokenReq>
) -> Result<impl IntoResponse, AppError> {
    // Decode jwt
    let token_data = decode::<Claims>(
        &json.refresh_token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::AuthError("Invalid token".to_string()))?;

    let claims = token_data.claims;
    let user_id = claims.sub.parse::<i32>()
        .map_err(|_| AppError::AuthError("Invalid ID format".to_string()))?;

    // Check to database
    let token_exists = sqlx::query!(
        "SELECT user_id FROM refresh_tokens WHERE token = $1",
        json.refresh_token
    )
    .fetch_optional(&state.database)
    .await
    .map_err(|e| AppError::InternalError(e.to_string()))?;

    if token_exists.is_none() {
        return Err(AppError::AuthError("Refresh token has been revoked/used".to_string()));
    }

    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(&state.database)
    .await
    .map_err(|e| AppError::InternalError(e.to_string()))?
    .ok_or(AppError::NotFound("User not found".to_string()))?;

    // Revoke refresh token
    sqlx::query!("DELETE FROM refresh_tokens WHERE token = $1", json.refresh_token)
        .execute(&state.database)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    // Generate new token
    let access_token = gen_access_token(&user, &state).await?;
    let refresh_token = gen_refresh_token(&user, &state).await?;
    let response_data = LoginRes { access_token, refresh_token };

    Ok(WebResponse::ok(&uri, "Refresh success!", response_data))
}