use axum::{Router, routing::get};
use crate::handlers::scheduler::*;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/email-jobs", get(trigger_email_test))
}