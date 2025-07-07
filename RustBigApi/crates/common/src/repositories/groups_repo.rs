use diesel::prelude::*;
use uuid::Uuid;

/// Returns all group IDs for a given user UUID.
pub fn get_group_ids_for_user(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<Vec<Uuid>> {
    use crate::schema::user_groups::dsl::*;
    user_groups
        .filter(user_id.eq(user_id))
        .select(group_id)
        .load::<Uuid>(conn)
}

/// Returns all group names for a given user UUID.
pub fn get_group_names_for_user(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<Vec<String>> {
    use crate::schema::user_groups;
    use crate::schema::groups;
    groups::table
        .inner_join(user_groups::table.on(groups::id.eq(user_groups::group_id)))
        .filter(user_groups::user_id.eq(user_id))
        .select(groups::name)
        .distinct()
        .load::<String>(conn)
}
