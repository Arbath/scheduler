use axum::{Router, routing::*};

use crate::handlers::user::*;
use crate::state::AppState;


pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user/me", get(get_profile))
        .route("/user/me", patch(update_profile))
        .route("/user", get(get_all_users))
        .route("/user", post(create_user))
        .route("/user/{id}", delete(delete_user))
}