-- Your SQL goes here
CREATE TABLE group_permissions
(
    group_id      UUID NOT NULL REFERENCES groups (id) ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES permissions (id) ON DELETE CASCADE,
    PRIMARY KEY (group_id, permission_id)
);