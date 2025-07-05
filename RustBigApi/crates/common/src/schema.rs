// @generated automatically by Diesel CLI.

diesel::table! {
    group_permissions (group_id, permission_id) {
        group_id -> Uuid,
        permission_id -> Uuid,
    }
}

diesel::table! {
    groups (id) {
        id -> Uuid,
        name -> Text,
    }
}

diesel::table! {
    permissions (id) {
        id -> Uuid,
        #[max_length = 64]
        name -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    user_groups (user_id, group_id) {
        user_id -> Uuid,
        group_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 16]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(group_permissions -> groups (group_id));
diesel::joinable!(group_permissions -> permissions (permission_id));
diesel::joinable!(user_groups -> groups (group_id));
diesel::joinable!(user_groups -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    group_permissions,
    groups,
    permissions,
    posts,
    user_groups,
    users,
);
