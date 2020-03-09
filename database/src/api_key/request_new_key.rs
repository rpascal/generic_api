use crate::{Pool, db_connection, system_db};
use crate::errors::{DatabaseResult, DatabaseError};

use arangors::AqlQuery;
use models::api_key::ApiKey;

pub fn execute(pool: &Pool) -> DatabaseResult<Vec<ApiKey>> {
    let pooled_connection = &db_connection(pool)?;
    let db = &system_db(pooled_connection)?;

    let new_key: ApiKey = ApiKey {
        api_key: uuid::Uuid::new_v4()
    };

    let aql = AqlQuery::new("INSERT @api_key INTO api_keys RETURN NEW")
        .bind_var("api_key", serde_json::value::to_value(&new_key)?);

    if let Ok(resp) = db.aql_query(aql) {
        return Ok(resp);
    }

    return Err(DatabaseError::InternalServerError);
}