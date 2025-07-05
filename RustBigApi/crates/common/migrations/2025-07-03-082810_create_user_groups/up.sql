-- Your SQL goes here
CREATE TABLE user_groups
(
    user_id  UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    group_id UUID NOT NULL REFERENCES groups (id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, group_id)
);