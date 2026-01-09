
use apalis::prelude::*;
use apalis_sql::{postgres::PostgresStorage};
use sqlx::PgPool;
use crate::jobs::email_jobs::{EmailJob, execute_email_job};

pub async fn setup_background_workers(
    pool: PgPool,
    email_storage: PostgresStorage<EmailJob>,
) {
    tokio::spawn(async move {
        Monitor::new()
            .register(
                WorkerBuilder::new("email-scheduler")
                    .concurrency(2)
                    .data(pool)
                    .backend(email_storage)
                    .build_fn(execute_email_job),
            )
            .run()
            .await
            .expect("Email worker crashed");
    });
}
