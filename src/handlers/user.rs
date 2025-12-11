use axum::{http::Uri,response::IntoResponse};
use crate::middleware::auth::AuthUser;
use crate::utils::{response::AppError,response::WebResponse, requests::ValidatedJson};
use crate::models::user::UpdateUserReq;
use crate::services::user::UserService;

pub async fn get_profile(
    uri: Uri,
    AuthUser(user): AuthUser,
    service: UserService,
)-> Result<impl IntoResponse, AppError>{
    let response_data = service.get_profile(&user).await?;

    Ok(WebResponse::ok(&uri, "Selamat datang", response_data))
}

pub async fn update_profile(
    uri: Uri,
    AuthUser(user): AuthUser,
    service: UserService,
    ValidatedJson(data): ValidatedJson<UpdateUserReq>
) -> Result<impl IntoResponse, AppError> {
    
    let response_data = service.update_profile(&user, &data).await?;

    Ok(WebResponse::ok(&uri, "Berhasil update profile", response_data))
}