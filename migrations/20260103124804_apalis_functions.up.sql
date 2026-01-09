-- Add up migration script here
--get_jobs
CREATE OR REPLACE FUNCTION apalis.get_jobs(
    worker_id TEXT,
    v_job_type TEXT,
    v_job_count INTEGER DEFAULT 5
)
RETURNS SETOF apalis.jobs
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    UPDATE apalis.jobs
    SET
        status = 'Running',
        lock_by = worker_id,
        lock_at = now()
    WHERE id IN (
        SELECT id
        FROM apalis.jobs
        WHERE
            (status = 'Pending'
             OR (status = 'Failed' AND attempts < max_attempts))
            AND run_at < now()
            AND job_type = v_job_type
        ORDER BY priority DESC, run_at ASC
        LIMIT v_job_count
        FOR UPDATE SKIP LOCKED
    )
    RETURNING *;
END;
$$;

--push_jobs
CREATE OR REPLACE FUNCTION apalis.push_job(
    job_type TEXT,
    job JSONB DEFAULT NULL,
    status TEXT DEFAULT 'Pending',
    run_at TIMESTAMPTZ DEFAULT now(),
    max_attempts INTEGER DEFAULT 25,
    priority INTEGER DEFAULT 0
)
RETURNS apalis.jobs
LANGUAGE plpgsql
AS $$
DECLARE
    v_job_row apalis.jobs;
BEGIN
    IF job_type IS NOT NULL AND length(job_type) > 512 THEN
        RAISE EXCEPTION 'Job_type is too long (max length: 512)';
    END IF;

    IF max_attempts < 1 THEN
        RAISE EXCEPTION 'Job maximum attempts must be at least 1';
    END IF;

    INSERT INTO apalis.jobs (
        id,
        job,
        job_type,
        status,
        attempts,
        max_attempts,
        run_at,
        priority
    )
    VALUES (
        gen_random_uuid()::TEXT,
        job,
        job_type,
        status,
        0,
        max_attempts,
        run_at,
        priority
    )
    RETURNING * INTO v_job_row;

    RETURN v_job_row;
END;
$$;

--notify_new_jobs
CREATE OR REPLACE FUNCTION apalis.notify_new_jobs()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    PERFORM pg_notify('apalis::job', 'insert');
    RETURN NEW;
END;
$$;
