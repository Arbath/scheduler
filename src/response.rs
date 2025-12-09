use axum::{
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use chrono::Utc;

#[derive(Serialize)]
pub struct WebResponse<T> {
    pub success: bool,
    pub status: u16,
    pub message: String,
    pub path: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> WebResponse<T> {
    // Helper SUCCESS
    pub fn ok(uri: &Uri, message: &str, data: T) -> (StatusCode, Json<Self>) {
        let status = StatusCode::OK;
        (
            status,
            Json(Self {
                success: true,
                status: status.as_u16(),
                message: message.to_string(),
                path: uri.path().to_string(),
                timestamp: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(), // Format Z
                data: Some(data),
            }),
        )
    }

    // Helper CREATED
    pub fn created(uri: &Uri, message: &str, data: T) -> (StatusCode, Json<Self>) {
        let status = StatusCode::CREATED;
        (
            status,
            Json(Self {
                success: true,
                status: status.as_u16(),
                message: message.to_string(),
                path: uri.path().to_string(),
                timestamp: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                data: Some(data),
            }),
        )
    }
}

// Custom ERROR
pub enum AppError {
    AuthError(String),
    InternalError(String),
    NotFound(String),
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::AuthError(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(WebResponse {
            success: false,
            status: status.as_u16(),
            message,
            path: "".to_string(),
            timestamp: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            data: None::<()>,
        });

        (status, body).into_response()
    }
}

// Helper (?)
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::InternalError(format!("Database Error: {}", err))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}