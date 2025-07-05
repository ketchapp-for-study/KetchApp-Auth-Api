use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use r2d2::{Pool, PooledConnection};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(pool: &PgPool) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
    pool.get().map_err(|e| {
        Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })
}
pub mod users_repo;
