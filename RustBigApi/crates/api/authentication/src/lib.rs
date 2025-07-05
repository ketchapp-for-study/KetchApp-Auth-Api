pub use crate::models::register::RegisterUser;
pub use crate::models::login::LoginUser;
pub mod handlers;
pub mod models;
mod config;

pub use diesel::r2d2::{ConnectionManager, Pool};
pub use diesel::PgConnection;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
