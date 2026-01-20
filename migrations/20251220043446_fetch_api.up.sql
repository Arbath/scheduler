-- Add up migration script here
-- CREATE ENUM TYPE
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_type WHERE typname = 'fetch_api_type'
    ) THEN
        CREATE TYPE fetch_api_type AS ENUM (
            'rest',
            'websocket',
            'mqtt',
            'graphql'
        );
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_type WHERE typname = 'fetch_api_method'
    ) THEN
        CREATE TYPE fetch_api_method AS ENUM (
            'get',
            'post',
            'put',
            'patch',
            'delete'
        );
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_type WHERE typname = 'execute_type'
    ) THEN
        CREATE TYPE execute_type AS ENUM (
            'seconds',
            'minutes',
            'hours',
            'days'
        );
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_type WHERE typname = 'fetch_member_role'
    ) THEN
        CREATE TYPE fetch_member_role AS ENUM (
            'owner',
            'editor',
            'viewer'
        );
    END IF;
END $$;

-- CREATE TABLE fetch_api_execute
CREATE TABLE IF NOT EXISTS fetch_api_execute (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    is_repeat BOOLEAN NOT NULL DEFAULT false,
    type execute_type NOT NULL DEFAULT 'minutes',
    value BIGINT NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_fetch_execute_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);
DROP TRIGGER IF EXISTS set_timestamp ON fetch_api_execute;
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON fetch_api_execute
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();

-- CREATE TABLE fetch_api_header
CREATE TABLE IF NOT EXISTS fetch_api_header (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    headers JSONB DEFAULT '{}'::jsonb,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_fetch_header_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);
DROP TRIGGER IF EXISTS set_timestamp ON fetch_api_header;
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON fetch_api_header
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();

-- CREATE TABLE fetch_api
CREATE TABLE IF NOT EXISTS fetch_api (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    type fetch_api_type NOT NULL DEFAULT 'rest',
    method fetch_api_method NOT NULL DEFAULT 'get',
    topic JSONB DEFAULT '{}'::jsonb,
    job_id TEXT NOT NULL UNIQUE,
    description TEXT,
    payload TEXT,

    -- CONFIG RELATIONSHIPS
    execute_id INTEGER NOT NULL,
    header_id INTEGER,
    
    is_active BOOLEAN NOT NULL DEFAULT false,
    secure BOOLEAN NOT NULL DEFAULT false,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_fetch_execute
        FOREIGN KEY (execute_id)
        REFERENCES fetch_api_execute(id)
        ON DELETE RESTRICT,
    
    CONSTRAINT fk_fetch_header
        FOREIGN KEY (header_id)
        REFERENCES fetch_api_header(id)
        ON DELETE SET NULL
);

-- INDEX
CREATE INDEX IF NOT EXISTS idx_fetch_api_execute_id ON fetch_api(execute_id);
CREATE INDEX IF NOT EXISTS idx_fetch_api_header_id ON fetch_api(header_id);

DROP TRIGGER IF EXISTS set_timestamp ON fetch_api;
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON fetch_api
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();

-- CREATE TABLE fetch_api_data
CREATE TABLE IF NOT EXISTS fetch_api_data (
    id SERIAL PRIMARY KEY,
    fetch_id INTEGER NOT NULL,

    name TEXT NOT NULL,
    
    -- RESPONSE
    status_code SMALLINT,
    response TEXT,
    response_headers JSONB DEFAULT '{}'::jsonb,

    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_fetch_api
        FOREIGN KEY (fetch_id)
        REFERENCES fetch_api(id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_fetch_api_data_fetch_id ON fetch_api_data(fetch_id);

DROP TRIGGER IF EXISTS set_timestamp ON fetch_api_data;
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON fetch_api_data
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();


-- SHARED JOBS
CREATE TABLE fetch_api_members (
    fetch_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    role fetch_member_role NOT NULL DEFAULT 'owner',
    created_at TIMESTAMPTZ DEFAULT NOW(),

    -- Composite Primary Key: Mencegah duplikasi (User A tidak bisa join Job 1 dua kali)
    PRIMARY KEY (fetch_id, user_id),

    CONSTRAINT fk_member_fetch
        FOREIGN KEY (fetch_id)
        REFERENCES fetch_api(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_member_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE 
);

CREATE INDEX idx_fetch_api_members_user_id ON fetch_api_members(user_id);