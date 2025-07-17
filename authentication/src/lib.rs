pub mod config;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod schema;

pub use diesel::r2d2::{ConnectionManager, Pool};
pub use diesel::PgConnection;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
