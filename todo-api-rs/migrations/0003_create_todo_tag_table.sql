CREATE TABLE IF NOT EXISTS todo_tag (
    todo_id uuid NOT NULL,
    tag_id uuid NOT NULl,
    created_at timestamptz NOT NULL,
    CONSTRAINT todo_tag_pk PRIMARY KEY(todo_id, tag_id),
    CONSTRAINT todo_tag_fk_todo_id FOREIGN KEY(todo_id) REFERENCES todo(id) ON DELETE CASCADE,
    CONSTRAINT todo_tag_fk_tag_id FOREIGN KEY(tag_id) REFERENCES tag(id) ON DELETE CASCADE
);