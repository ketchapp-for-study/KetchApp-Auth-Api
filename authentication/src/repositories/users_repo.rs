pub use crate::models::user::{NewUser, User};
use crate::repositories::{establish_connection, PgPool};
use crate::schema::users;
use crate::schema::users::dsl::*;
use diesel::prelude::*;

// * Inserisce un nuovo utente nel database e restituisce l'utente creato
pub fn new_user(pool: &PgPool, new_user: NewUser) -> Result<User, diesel::result::Error> {
    let mut conn = establish_connection(pool)?;
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&mut conn)
}

// * Verifica se esiste già un utente con lo stesso username o email
pub fn user_exists_by_username_or_email(
    pool: &PgPool,
    other_username: &str,
    other_email: &str,
) -> Result<bool, diesel::result::Error> {
    let mut conn = establish_connection(pool)?;
    let count = users
        .filter(username.eq(other_username).or(email.eq(other_email)))
        .count()
        .get_result::<i64>(&mut conn)?;
    Ok(count > 0)
}

// * Recupera un utente tramite username
pub fn get_user_by_username(
    pool: &PgPool,
    other_username: &str,
) -> Result<User, diesel::result::Error> {
    let mut conn = establish_connection(pool)?;
    users
        .filter(username.eq(other_username))
        .first::<User>(&mut conn)
}
