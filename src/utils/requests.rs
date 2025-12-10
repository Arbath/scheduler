use axum::{
    extract::{FromRequest, Request},
    extract::rejection::JsonRejection,
    Json,
};
use serde::de::DeserializeOwned;
use crate::utils::response::AppError;

// Digunakan untuk format response jika request tidak sesuai
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let error_message = rejection.to_string(); 
                Err(AppError::BadRequest(format!("Input Validation Error: {}", error_message)))
            }
        }
    }
}
