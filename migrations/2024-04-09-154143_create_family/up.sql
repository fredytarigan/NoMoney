-- Your SQL goes here
CREATE TABLE family (
    id uuid DEFAULT gen_random_uuid(),
    name VARCHAR(64) NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL,
    PRIMARY KEY (id)
)