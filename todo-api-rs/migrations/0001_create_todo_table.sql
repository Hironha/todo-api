CREATE TABLE IF NOT EXISTS Todo (
  id uuid UNIQUE NOT NULL,
  title varchar NOT NULL,
  description varchar,
  todo_at date,
  created_at timestamptz NOT NULL,
  updated_at timestamptz NOT NULL,
  CONSTRAINT PK_Todo PRIMARY KEY (id)
);