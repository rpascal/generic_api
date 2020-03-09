#[macro_use]
extern crate serde_derive;

pub mod pool;
pub mod api_key;
pub mod get_request;
pub mod errors;

use crate::errors::{DatabaseError, DatabaseResult};
use arangors::Database;

type ConnectionManager = r2d2_arangors::pool::ArangoDBConnectionManager;
pub type Pool = r2d2::Pool<ConnectionManager>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager>;

static DEFAULT_DB_NAME: &str = "_system";

pub fn db_connection(pool: &Pool) -> DatabaseResult<PooledConnection> {
    if let Ok(p) = pool.get() {
        return Ok(p);
    }
    Err(DatabaseError::UnableToConnectToDb)
}

pub fn system_db(pooled_connection: &PooledConnection) -> DatabaseResult<Database> {
    if let Ok(db) = pooled_connection.db(DEFAULT_DB_NAME) {
        return Ok(db);
    }
    Err(DatabaseError::UnableToConnectToDb)
}
