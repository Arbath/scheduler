use axum::{Router, routing::{get, post, patch, delete}};

use crate::handlers::user::{get_profile, update_profile, get_all_users, create_user, delete_user};
use crate::state::AppState;


pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user/me", get(get_profile))
        .route("/user/me", patch(update_profile))
        .route("/user", get(get_all_users))
        .route("/user", post(create_user))
        .route("/user/{id}", delete(delete_user))
}