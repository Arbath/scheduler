use serde::{Deserialize, Serialize};
use apalis::prelude::*;
use sqlx::PgPool;

#[derive(Debug,Clone, Deserialize, Serialize)]
pub struct EmailJob {
    pub to: String,
    pub subject: String,
    pub body: String,
}

// Task Workers
use apalis_sql::context::SqlContext;

pub async fn execute_email_job(
    job: EmailJob,
    _ctx: SqlContext,
    _pool: Data<PgPool>, // Untuk inject data dengan pool sesuai di monitor
) -> Result<(), anyhow::Error> {
    tracing::info!("Processing email to: {}", job.to);

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    tracing::info!("Email sent to: {}", job.to);
    Ok(())
}
