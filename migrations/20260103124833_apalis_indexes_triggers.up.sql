-- Add up migration script here
CREATE INDEX IF NOT EXISTS idx_workers_last_seen
    ON apalis.workers(last_seen);

CREATE INDEX IF NOT EXISTS idx_workers_worker_type
    ON apalis.workers(worker_type);

CREATE INDEX IF NOT EXISTS idx_jobs_status
    ON apalis.jobs(status);

CREATE INDEX IF NOT EXISTS idx_jobs_job_type
    ON apalis.jobs(job_type);

CREATE INDEX IF NOT EXISTS idx_jobs_lock_by
    ON apalis.jobs(lock_by);

CREATE INDEX IF NOT EXISTS idx_jobs_priority_run_at
    ON apalis.jobs(priority DESC, run_at ASC);

DROP TRIGGER IF EXISTS notify_workers ON apalis.jobs;

CREATE TRIGGER notify_workers
AFTER INSERT ON apalis.jobs
FOR EACH STATEMENT
EXECUTE FUNCTION apalis.notify_new_jobs();
