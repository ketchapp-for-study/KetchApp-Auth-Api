use actix_web::web::Data;
use crate::errors::ServiceError;
pub use crate::models::user::{NewUser, User};
use crate::repositories::{establish_connection, PgPool};
use crate::schema::users;
use crate::schema::{groups, user_groups};
use diesel::prelude::*;
use diesel::row::NamedRow;
use uuid::Uuid;
use crate::schema::users::dsl::*;

pub fn get_users(pool: &PgPool) -> Result<Vec<User>, diesel::result::Error> {
    let mut conn = pool.get().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;
    users::table
        .select((
            users::id,
            users::username,
            users::email,
            users::password,
            users::created_at,
            users::updated_at,
        ))
        .load::<User>(&mut conn)
}

pub fn new_user(pool: &PgPool, new_user: NewUser) -> Result<User, diesel::result::Error> {
    let mut conn = establish_connection(pool)?;
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&mut conn)
}

pub fn get_user_roles(user_id: Uuid, pool: &PgPool) -> Result<Vec<String>, ServiceError> {

    let mut conn = establish_connection(pool)?;
    let roles = user_groups::table
        .inner_join(groups::table.on(user_groups::group_id.eq(groups::id)))
        .filter(user_groups::user_id.eq(user_id))
        .select(groups::name)
        .load::<String>(&mut conn)
        .map_err(ServiceError::DatabaseError)?;
    Ok(roles)
}

pub fn user_exists_by_username_or_email(pool: &PgPool, other_username: &str, other_email: &str) -> Result<bool, diesel::result::Error> {

    let mut conn = establish_connection(pool)?;
    let count = users
        .filter(username.eq(other_username).or(email.eq(other_email)))
        .count()
        .get_result::<i64>(&mut conn)?;
    Ok(count > 0)
}

pub fn find_user_by_username_or_email(pool: &PgPool, username_or_email: &str) -> Result<User, diesel::result::Error> {
    let mut conn = establish_connection(pool)?;
    users
        .filter(username.eq(username_or_email).or(email.eq(username_or_email)))
        .first::<User>(&mut conn)
}

pub fn get_user_by_username(pool: &PgPool, other_username: &str) -> Result<User, diesel::result::Error> {
    let mut conn = establish_connection(pool)?;
    users
        .filter(username.eq(other_username))
        .first::<User>(&mut conn)
}