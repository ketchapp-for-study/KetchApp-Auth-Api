-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE users
(
    id         UUID PRIMARY KEY      DEFAULT gen_random_uuid(),
    username   VARCHAR(16)  NOT NULL UNIQUE CHECK (char_length(username) BETWEEN 6 AND 16 AND username ~ '^[a-zA-Z]+$'),
    email      VARCHAR(255) NOT NULL UNIQUE CHECK (email ~* '^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'),
    password   VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

-- Trigger function to update updated_at on row update
CREATE OR REPLACE FUNCTION update_updated_at_column()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_updated_at
    BEFORE UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Enable Row Level Security
ALTER TABLE users
    ENABLE ROW LEVEL SECURITY;

-- Example policy: allow users to select only their own row (assuming you use id as user identifier)
CREATE POLICY select_own_user ON users
    FOR SELECT
    USING (id::text = current_setting('app.current_user_id'));

CREATE POLICY update_own_user ON users
    FOR UPDATE
    USING (id::text = current_setting('app.current_user_id'));

CREATE POLICY delete_own_user ON users
    FOR DELETE
    USING (id::text = current_setting('app.current_user_id'));

-- Prevent superuser/other users from seeing all users by default
REVOKE ALL ON users FROM PUBLIC;

-- Optionally, restrict INSERT/UPDATE/DELETE to only the owner (add more policies as needed)
-- CREATE POLICY modify_own_user ON users
--     FOR ALL
--     USING (id::text = current_setting('app.current_user_id'));

-- To use this policy, set the session variable 'app.current_user_id' at connection time.
-- Example: SET app.current_user_id = '123e4567-e89b-12d3-a456-426614174000';
-- You can add more policies for INSERT, UPDATE, DELETE as needed.
