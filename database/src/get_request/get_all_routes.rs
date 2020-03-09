use crate::{Pool, db_connection, system_db};
use crate::errors::{DatabaseResult, DatabaseError};

use uuid::Uuid;
use models::get_request::GetRequest;
use arangors::AqlQuery;

pub fn execute(header_api_key: Uuid, pool: &Pool) -> DatabaseResult<Vec<GetRequest>> {
    let pooled_connection = &db_connection(pool)?;
    let db = &system_db(pooled_connection)?;

    let aql = AqlQuery::new("FOR u IN get_requests FILTER u.api_key==@api_key RETURN u")
        .bind_var("api_key", header_api_key.to_string());

    if let Ok(resp) = db.aql_query::<GetRequest>(aql) {
        return Ok(resp);
    }

    return Err(DatabaseError::InternalServerError);
}