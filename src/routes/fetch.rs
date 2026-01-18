use axum::routing::{get, post, delete, patch};
use axum::Router;
use crate::handlers::fetch::*;
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

        .route("/fetch/{fetch_id}/member", get(get_all_member))
        .route("/fetch/{fetch_id}/member", post(create_fetch_member))
        .route("/fetch/{fetch_id}/member/{id}", get(get_fetch_member))
        .route("/fetch/{fetch_id}/member/{id}", patch(update_fetch_member))
        .route("/fetch/{fetch_id}/member/{id}", delete(delete_fetch_member))

        .route("/fetch/execute", get(get_all_execute))
        .route("/fetch/execute", post(create_fetch_execute))
        .route("/fetch/execute/{id}", get(get_fetch_execute))
        .route("/fetch/execute/{id}", patch(update_fetch_execute))
        .route("/fetch/execute/{id}", delete(delete_fetch_execute))
}