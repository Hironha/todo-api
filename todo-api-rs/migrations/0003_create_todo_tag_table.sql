CREATE TABLE IF NOT EXISTS todo_tag (
    todo_id uuid NOT NULL,
    tag_id uuid NOT NULl,
    CONSTRAINT pk_todo_tag PRIMARY KEY(todo_id, tag_id),
    CONSTRAINT fk_todo_tag_todo_id FOREIGN KEY(todo_id) REFERENCES todo(id) ON DELETE CASCADE,
    CONSTRAINT fk_todo_tag_tag_id FOREIGN KEY(tag_id) REFERENCES tag(id) ON DELETE CASCADE
);