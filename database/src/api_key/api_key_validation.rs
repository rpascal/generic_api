use uuid::Uuid;
use crate::{Pool, db_connection, system_db};
use crate::errors::{DatabaseResult, DatabaseError};
use arangors::AqlQuery;
use models::api_key::ApiKey;

pub fn execute(test_api_key: Uuid, pool: &Pool) -> DatabaseResult<()> {
    let pooled_connection = &db_connection(pool)?;
    let db = &system_db(pooled_connection)?;

    let aql = AqlQuery::new("FOR u IN api_keys FILTER u.api_key==@api_key LIMIT 1 RETURN u")
        .bind_var("api_key", test_api_key.to_string());

    if let Ok(resp) = db.aql_query::<ApiKey>(aql) {
        if !resp.is_empty() {
            return Ok(());
        }
    }

    return Err(DatabaseError::Unauthorized(format!("Bad api_key: {0}", test_api_key.to_string())));
}
