use super::{ConnectionManager, Pool, PoolError};

fn init_pool(database_url: &str) -> Result<Pool, PoolError> {
    let manager = ConnectionManager::new(database_url);
    Pool::builder().build(manager)
}

pub(crate) fn establish_connection(args: crate::CliAndEnvArgs) -> Pool {
    init_pool(&args.database_url).expect("Failed to create pool")
}
