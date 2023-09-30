CREATE TABLE IF NOT EXISTS todo (
    id uuid UNIQUE NOT NULL,
    title varchar(64) NOT NULL,
    description varchar(256),
    todo_at date,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    CONSTRAINT pk_todo PRIMARY KEY (id)
);