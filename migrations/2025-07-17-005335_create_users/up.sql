-- Enable the "pgcrypto" extension if it's not already enabled.
-- This extension provides functions for cryptographic operations, including UUID generation.
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Create the "users" table to store user information.
CREATE TABLE users
(
    -- Unique identifier for each user, automatically generated using a UUID.
    id         UUID PRIMARY KEY      DEFAULT gen_random_uuid(),
    -- Username for the user, must be unique, between 6 and 16 characters long, and contain only letters.
    username   VARCHAR(16)  NOT NULL UNIQUE CHECK (char_length(username) BETWEEN 6 AND 16 AND username ~ '^[a-zA-Z]+$'),
    -- Email address for the user, must be unique and in a valid email format.
    email      VARCHAR(255) NOT NULL UNIQUE CHECK (email ~* '^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'),
    -- Password for the user, stored as a VARCHAR(255).  Should be hashed in a real application.
    password   VARCHAR(255) NOT NULL,
    -- Timestamp indicating when the user was created, defaults to the current time.
    created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    -- Timestamp indicating when the user was last updated, defaults to the current time.
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

-- Creates or replaces a function to update the updated_at column.
CREATE OR REPLACE FUNCTION update_updated_at_column()
    RETURNS TRIGGER AS
$$
BEGIN
    -- Set the updated_at column to the current timestamp.
    NEW.updated_at = NOW();
    -- Return the updated row.
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to automatically update the updated_at column before each update operation on the users table.
CREATE TRIGGER set_updated_at
    BEFORE UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Enable Row Level Security (RLS) on the users table.
-- RLS allows defining policies to control access to rows based on certain conditions.
ALTER TABLE users
    ENABLE ROW LEVEL SECURITY;

-- Create a policy to allow users to select their own user data.
-- This policy checks if the user's ID matches the ID stored in the 'app.current_user_id' setting.
CREATE POLICY select_own_user ON users
    FOR SELECT
    USING (id::text = current_setting('app.current_user_id'));

-- Create a policy to allow users to update their own user data.
-- This policy checks if the user's ID matches the ID stored in the 'app.current_user_id' setting.
CREATE POLICY update_own_user ON users
    FOR UPDATE
    USING (id::text = current_setting('app.current_user_id'));

-- Create a policy to allow users to delete their own user data.
-- This policy checks if the user's ID matches the ID stored in the 'app.current_user_id' setting.
CREATE POLICY delete_own_user ON users
    FOR DELETE
    USING (id::text = current_setting('app.current_user_id'));

-- Revoke all default privileges on the users table from the PUBLIC role.
-- This ensures that only users with explicit grants can access the table.
REVOKE ALL ON users FROM PUBLIC;