use crate::{Pool, db_connection, system_db};
use crate::errors::{DatabaseResult, DatabaseError};

use uuid::Uuid;
use arangors::AqlQuery;
use models::get_request::{GetRequest, Request};

pub fn execute(request: &Request, header_api_key: Uuid, pool: &Pool) -> DatabaseResult<serde_json::Value> {
    let pooled_connection = &db_connection(pool)?;
    let db = &system_db(pooled_connection)?;
    let serialized_request = serde_json::to_value(request)?;

    let aql = AqlQuery::new("FOR u IN get_requests FILTER u.api_key==@api_key FILTER u.request==@request RETURN u")
        .bind_var("api_key", header_api_key.to_string())
        .bind_var("request", serialized_request);

    match db.aql_query::<GetRequest>(aql) {
        Ok(mut resp) => {
            if resp.len() == 1 {
                if let Some(val) = resp.pop() {
                    return Ok(val.response);
                }
            }
        },
        Err(e) => {
            let error_msg: String = format!("{:?}", e);
            return Err(DatabaseError::BadRequest(error_msg));
        }
    }
    Err(DatabaseError::InternalServerError)

}