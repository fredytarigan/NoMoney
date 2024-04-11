-- Your SQL goes here
CREATE TABLE users (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    username VARCHAR(64) NOT NULL UNIQUE,
    first_name VARCHAR(64),
    last_name VARCHAR(64),
    password VARCHAR(128) NOT NULL,
    active BOOLEAN NOT NULL DEFAULT FALSE,
    family_id uuid NOT NULL REFERENCES families(id),
    role_id uuid NOT NULL REFERENCES roles(id),
    email VARCHAR(64) NOT NULL,
    email_validated BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL,
    PRIMARY KEY (id)
);