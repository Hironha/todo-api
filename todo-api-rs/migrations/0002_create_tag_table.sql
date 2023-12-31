CREATE TABLE IF NOT EXISTS tag (
    id uuid UNIQUE NOT NULL,
    name  varchar(64) NOT NULL,
    description varchar(128),
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    CONSTRAINT tag_pk PRIMARY KEY (id),
    CONSTRAINT tag_ak_name UNIQUE (name)
);