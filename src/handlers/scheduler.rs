use crate::jobs::email_jobs::EmailJob;
// use apalis_sql::postgres::PostgresStorage;
// use axum::{Json as AxumJson};
// use axum::Extension;
use apalis::prelude::Storage;
use crate::state::AppState;
use axum::{extract::State, http::Uri};
use crate::utils::{response::*};
use axum::{response::IntoResponse};


pub async fn trigger_email_test(
    uri: Uri,
    State(mut state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let job = EmailJob {
        to: "user@example.com".to_string(),
        subject: "Halo dari Rust!".to_string(),
        body: "Ini tes worker.".to_string(),
    };
    let run_at = chrono::Utc::now().timestamp() + 30;

    state
        .email_job_queue
        // .push(job.clone())
        .schedule(job.clone(), run_at)
        .await
        .map_err(|e| {
            tracing::error!("Failed to enqueue email job: {e}");
            AppError::InternalError("Gagal memicu email job".to_string())
        })?;

    Ok(WebResponse::ok(
        &uri,
        "Berhasil memicu email job",
        job,
    ))
}
