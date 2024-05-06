-- Add down migration script here
DROP COLLATION IF EXISTS case_insensitive;
DROP FUNCTION IF EXISTS trigger_updated_at();
DROP FUNCTION IF EXISTS set_updated_at();
DROP EXTENSION IF EXISTS "uuid-ossp";