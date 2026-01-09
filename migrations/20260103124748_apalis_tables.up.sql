-- Add up migration script here
CREATE TABLE IF NOT EXISTS apalis.workers (
    id TEXT PRIMARY KEY,
    worker_type TEXT NOT NULL,
    storage_name TEXT NOT NULL,
    layers TEXT NOT NULL DEFAULT '',
    last_seen TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS apalis.jobs (
    job JSONB NOT NULL,
    id TEXT PRIMARY KEY,
    job_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 25,
    run_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_error TEXT,
    lock_at TIMESTAMPTZ,
    lock_by TEXT,
    done_at TIMESTAMPTZ,
    priority INTEGER DEFAULT 0,

    CONSTRAINT fk_worker_lock_by
        FOREIGN KEY (lock_by)
        REFERENCES apalis.workers(id)
        ON DELETE SET NULL
);
