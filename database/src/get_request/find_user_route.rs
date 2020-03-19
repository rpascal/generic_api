use crate::{Pool, db_connection, system_db};
use crate::errors::{DatabaseResult, DatabaseError};

use uuid::Uuid;
use arangors::AqlQuery;
use models::get_request::GetRequest;
use std::collections::HashMap;

pub fn execute(path: &str, query_params: HashMap<String, String>, header_api_key: Uuid, pool: &Pool) -> DatabaseResult<serde_json::Value> {
    let pooled_connection = &db_connection(pool)?;
    let db = &system_db(pooled_connection)?;
    let query_params_as_str = serde_json::to_value(&query_params)?;

    let aql = AqlQuery::new("FOR u IN get_requests FILTER u.api_key==@api_key FILTER u.route==@route FILTER u.query_params==@query_params RETURN u")
        .bind_var("api_key", header_api_key.to_string())
        .bind_var("route", path)
        .bind_var("query_params", query_params_as_str);

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