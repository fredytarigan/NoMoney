-- Your SQL goes here
CREATE TABLE roles (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    name VARCHAR(64) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL,
    PRIMARY KEY (id)
);

--- seed data for admin roles ---
INSERT INTO roles (name, description)
VALUES ('admin', 'Administrator Role');

--- seed data for editor roles ---
INSERT INTO roles (name, description)
VALUES ('editor', 'Editor Role');

--- seed data for viewer roles ---
INSERT INTO roles (name, description)
VALUES ('viewer', 'Viewer Role');