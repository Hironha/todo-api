CREATE TABLE IF NOT EXISTS todos (
  id UUID PRIMARY KEY,
  title VARCHAR NOT NULL,
  description VARCHAR,
  todo_at DATE,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL
);