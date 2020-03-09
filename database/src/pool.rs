use super::{ConnectionManager, Pool};
use r2d2::Error;

fn init_pool(url: &str, username: &str, password: &str) -> Result<Pool, Error> {
    let manager = ConnectionManager::new(url, username, password, false);
    Pool::builder().build(manager)
}

pub fn establish_connection(url: &str, username: &str, password: &str) -> Pool {
    init_pool(url, username, password).expect("Failed to create pool")
}
