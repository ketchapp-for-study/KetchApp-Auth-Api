use diesel::prelude::*;
use uuid::Uuid;

/// Returns all permission names for a given user UUID (via their groups).
pub fn get_permission_names_for_user(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<Vec<String>> {
    use crate::schema::user_groups;
    use crate::schema::group_permissions;
    use crate::schema::permissions;

    permissions::table
        .inner_join(group_permissions::table.on(permissions::id.eq(group_permissions::permission_id)))
        .inner_join(user_groups::table.on(group_permissions::group_id.eq(user_groups::group_id)))
        .filter(user_groups::user_id.eq(user_id))
        .select(permissions::name)
        .distinct()
        .load::<String>(conn)
}

