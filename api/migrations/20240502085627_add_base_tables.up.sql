-- Add up migration script here

---------------------------------
--- create table for families ---
CREATE TABLE families (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    name VARCHAR(64) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL,

    PRIMARY KEY (id)
);

--- seed data for default families ---
INSERT INTO FAMILIES(name, description)
VALUES('Default', 'Default Generated Family');

------------------------------
--- create table for roles ---
CREATE TABLE roles (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    name VARCHAR(64) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL,

    PRIMARY KEY (id)
);

--- seed data for super admin roles ---
INSERT INTO ROLES(name, description)
VALUES('superadmin', 'Super Administrator Role');

--- seed data for admin roles ---
INSERT INTO ROLES(name, description)
VALUES('admin', 'Administrator Role');

--- seed data for editor roles ---
INSERT INTO ROLES(name, description)
VALUES('editor', 'Editor Role');

--- seed data for viewer roles ---
INSERT INTO ROLES(name, description)
VALUES('viewer', 'Viewer Role');

------------------------------
--- create table for users ---
CREATE TABLE users (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    username VARCHAR(64) NOT NULL UNIQUE,
    first_name VARCHAR(64),
    last_name VARCHAR(64),
    password VARCHAR(128) NOT NULL,
    active BOOLEAN NOT NULL DEFAULT FALSE,
    family_id uuid NOT NULL REFERENCES families(id),
    role_id uuid NOT NULL REFERENCES roles(id),
    avatar_path VARCHAR(64),
    email VARCHAR(64) NOT NULL,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL,

    PRIMARY KEY (id)
);

--- seed data for the very first user ---
--- this user will be a super admin ---
INSERT INTO USERS (username, password, active, family_id, role_id, email, email_verified)
VALUES(
    'admin',
    '$argon2id$v=19$m=19456,t=2,p=1$rzXUR01cSMTEvf6id9jpBA$vciMYiDyPNXwy92jLKaepyz8PxZIM67gCCbcTK5uZkU',
    TRUE,
    (SELECT id FROM families ORDER BY created_at ASC LIMIT 1),
    (SELECT id FROM roles WHERE name = 'superadmin' LIMIT 1),
    'admin@example.com',
    TRUE
)