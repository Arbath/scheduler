use apalis::prelude::*;
use crate::state::AppState;
use crate::jobs::rest::execute_job;

pub async fn setup_background_workers(state: AppState,) {
    let concurrency = state.app_config.concurrency.clone();
    tokio::spawn(async move {
        Monitor::new()
            .register(
                WorkerBuilder::new("teknohole-scheduler")
                    .concurrency(concurrency as usize)
                    .data(state.clone())
                    .backend(state.job_queue.clone())
                    .build_fn(execute_job),
            )
            .run()
            .await
            .expect("Scheduler worker crashed");
    });
}
