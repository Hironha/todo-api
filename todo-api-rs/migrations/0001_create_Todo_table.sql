CREATE TABLE IF NOT EXISTS "Todo" (
    id uuid UNIQUE NOT NULL,
    title varchar(64) NOT NULL,
    description varchar(256),
    todo_at date,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    CONSTRAINT "PK_Todo" PRIMARY KEY (id)
);