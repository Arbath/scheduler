use axum::{Router, routing::post};
// crate::handlers::user;

pub fn routes() -> Router {
    Router::new()
        .route("/", post(user::me))
}