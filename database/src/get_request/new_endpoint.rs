use crate::{Pool, db_connection, system_db};
use crate::errors::{DatabaseResult, DatabaseError};

use uuid::Uuid;
use models::get_request::{BasicGetRequest, GetRequest};
use arangors::AqlQuery;

pub fn execute(header_api_key: Uuid, pool: &Pool, body: BasicGetRequest) -> DatabaseResult<GetRequest> {
    let pooled_connection = &db_connection(pool)?;
    let db = &system_db(pooled_connection)?;

    let new_get_request : GetRequest = GetRequest {
        api_key: header_api_key,
        request: body.request,
        response: body.response,
    };

    let serialized_get_request = serde_json::value::to_value(&new_get_request)?;

    let aql = AqlQuery::new("INSERT @get_request INTO get_requests RETURN NEW")
        .bind_var("get_request", serialized_get_request);

    let mut resp = db.aql_query::<GetRequest>(aql)?;

    if let Some(val) = resp.pop() {
        return Ok(val);
    }

    return Err(DatabaseError::InternalServerError);

}