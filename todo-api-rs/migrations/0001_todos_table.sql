CREATE TABLE IF NOT EXISTS todos (
  id uuid PRIMARY KEY,
  title varchar NOT NULL,
  description varchar,
  todo_at date,
  created_at timestamptz NOT NULL,
  updated_at timestamptz NOT NULL
);