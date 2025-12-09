use axum::{Json, extract::State, http::Uri, response::IntoResponse};
use crate::response::{WebResponse, AppError};
use crate::models::user::User;
use crate::utils::auth::{gen_access_token, gen_refresh_token};
use crate::state::AppState;
use crate::models::auth::{LoginReq, LoginRes, RefreshToken, RefreshTokenReq};
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

pub async fn refresh_hand(
    State(state): State<AppState>,
    uri: Uri,
    Json(json): Json<RefreshTokenReq>
    ) -> Result<impl IntoResponse, AppError> {

    let claims= RefreshToken::verify_refresh_token(&state.jwt_secret, &json.refresh_token)?;
    let user_id = claims.sub.parse::<i32>()
        .map_err(|_| AppError::AuthError("Invalid ID format".to_string()))?;
    RefreshToken::token_exists(&state.database, &json.refresh_token).await?;
    let user = User::user_exists(&state.database, &user_id).await?;
    let access_token = gen_access_token(&user, &state).await?;
    let refresh_token = gen_refresh_token(&user, &state).await?;
    let response_data = LoginRes { access_token, refresh_token };

    Ok(WebResponse::ok(&uri, "Refresh success!", response_data))
}

pub async fn logout_hand(
    State(state): State<AppState>,
    uri: Uri,
    Json(json): Json<RefreshTokenReq>
    ) -> Result<impl IntoResponse, AppError> {
    let claims= RefreshToken::verify_refresh_token(&state.jwt_secret, &json.refresh_token)?;
    let user_id = claims.sub.parse::<i32>()
        .map_err(|_| AppError::AuthError("Invalid ID format".to_string()))?;
    User::user_exists(&state.database, &user_id).await?;
    RefreshToken::revoke_token(&state.database, &json.refresh_token).await?;
    Ok(WebResponse::ok_empty(&uri, "Logout successful!"))
}