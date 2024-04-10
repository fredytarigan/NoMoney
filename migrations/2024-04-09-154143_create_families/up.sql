-- Your SQL goes here
CREATE TABLE families (
    id uuid NOT NULL,
    name VARCHAR(64) NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL,
    PRIMARY KEY (id)
)