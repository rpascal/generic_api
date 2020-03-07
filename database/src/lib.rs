// use crate::errors::ServiceError;
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

pub mod pool;
pub mod schema;
pub mod api_key;
pub mod get_request;
pub mod errors;
use diesel::r2d2::PoolError;
use crate::errors::{DatabaseError, DatabaseResult};

type ConnectionManager = diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>;
pub type Pool = diesel::r2d2::Pool<ConnectionManager>;
pub type PooledConnection = diesel::r2d2::PooledConnection<ConnectionManager>;

pub fn db_connection(pool: &Pool) -> DatabaseResult<PooledConnection> {
    Ok(pool.get().map_err(|_| DatabaseError::UnableToConnectToDb)?)
}
