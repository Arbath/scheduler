use axum::routing::{get, post, delete, patch};
use axum::Router;
use crate::handlers::fetch::{create_fetch_api, delete_fetch_api, fetch_external_api, get_all, get_fetch_api, get_fetch_job, update_fetch_api};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/fetch/test", get(fetch_external_api))
        .route("/fetch", get(get_all))
        .route("/fetch", post(create_fetch_api))
        .route("/fetch/{id}", get(get_fetch_api))
        .route("/fetch/{id}", patch(update_fetch_api))
        .route("/fetch/{id}", delete(delete_fetch_api))
        .route("/fetch/{job_id}/job", get(get_fetch_job))
}