use axum::{Router, routing::post};
use crate::handlers::auth::{login_hand, refresh_hand};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_hand))
        .route("/refresh/token", post(refresh_hand))
}