use crate::{Pool, db_connection, system_db};
use crate::errors::{DatabaseResult, DatabaseError};

use uuid::Uuid;
use models::get_request::{BasicGetRequest, GetRequest};
use arangors::AqlQuery;
use std::collections::HashMap;

pub fn execute(header_api_key: Uuid, query_params: HashMap<String, String>, pool: &Pool, body: BasicGetRequest) -> DatabaseResult<GetRequest> {
    let pooled_connection = &db_connection(pool)?;
    let db = &system_db(pooled_connection)?;

    let new_get_request : GetRequest = GetRequest {
        api_key: header_api_key,
        route: body.route,
        response: body.response,
        query_params
    };

    let aql = AqlQuery::new("INSERT @get_request INTO get_requests RETURN NEW")
        .bind_var("get_request", serde_json::value::to_value(&new_get_request)?);

    let mut resp = db.aql_query::<GetRequest>(aql)?;

    // if let Ok(mut resp) = db.aql_query::<GetRequest>(aql) {
        if let Some(val) = resp.pop() {
            return Ok(val);
        }
    // }

    return Err(DatabaseError::InternalServerError);

}