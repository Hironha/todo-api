CREATE TABLE tag IF NOT EXISTS (
    id uuid UNIQUE NOT NULL,
    name  varchar(64) NOT NULL,
    description varchar(128),
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    CONSTRAINT pk_tag PRIMARY KEY (id)
)