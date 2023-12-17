DO $$ BEGIN
    CREATE TYPE todo_status AS ENUM ('todo', 'in_progress', 'done');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS todo (
    id uuid UNIQUE NOT NULL,
    title varchar(64) NOT NULL,
    description varchar(256),
    todo_at date,
    status todo_status,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    CONSTRAINT todo_pk PRIMARY KEY (id),
    CONSTRAINT todo_ak_title UNIQUE (title)
);

CREATE INDEX IF NOT EXISTS todo_created_at_idx ON todo(created_at);
