pub mod extract_jwt_claims;

use diesel::prelude::*;
use uuid::Uuid;
use crate::repositories::groups_repo::get_group_ids_for_user;
use crate::repositories::permissions_repo::get_permission_names_for_user;

/// Returns all permission names for a user by UUID, automatically checking all their groups.
pub fn get_user_permissions(conn: &mut PgConnection, user_uuid: Uuid) -> QueryResult<Vec<String>> {
    // This uses the permissions_repo, which already does the join logic.
    get_permission_names_for_user(conn, user_uuid)
}
