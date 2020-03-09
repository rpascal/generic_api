use crate::{Pool, db_connection, system_db};
use crate::errors::{DatabaseResult, DatabaseError};

use uuid::Uuid;
use arangors::AqlQuery;
use models::get_request::GetRequest;

pub fn execute(path: &str, header_api_key: Uuid, pool: &Pool) -> DatabaseResult<serde_json::Value> {
    let pooled_connection = &db_connection(pool)?;
    let db = &system_db(pooled_connection)?;

    let aql = AqlQuery::new("FOR u IN get_requests FILTER u.api_key==@api_key FILTER u.route==@route RETURN u")
        .bind_var("api_key", header_api_key.to_string())
        .bind_var("route", path);

    if let Ok(mut resp) = db.aql_query::<GetRequest>(aql) {
        if resp.len() == 1 {
            if let Some(val) = resp.pop() {
                return Ok(val.response);
            }
        }
    }

    return Err(DatabaseError::InternalServerError);

}