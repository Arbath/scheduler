use axum::routing::{get, post, delete, patch};
use axum::Router;
use crate::handlers::fetch::{
    fetch_external_api, get_all, get_fetch_api, get_fetch_job, get_all_member, get_fetch_member,
    create_fetch_api, create_fetch_member,
    update_fetch_api, update_fetch_member,
    delete_fetch_api, delete_fetch_member, 
};
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

        .route("/fetch/{id}/member", get(get_all_member))
        .route("/fetch/{id}/member", post(create_fetch_member))
        .route("/fetch/member/{id}", get(get_fetch_member))
        .route("/fetch/member/{id}", patch(update_fetch_member))
        .route("/fetch/member/{id}", delete(delete_fetch_member))
}