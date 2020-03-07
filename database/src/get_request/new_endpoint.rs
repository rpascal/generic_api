use crate::{Pool, db_connection};
use crate::errors::DatabaseResult;
use super::model::{GetRequest, BasicGetRequest};

use diesel::RunQueryDsl;
use uuid::Uuid;

pub fn execute(header_api_key: Uuid, pool: &Pool, body: BasicGetRequest) -> DatabaseResult<GetRequest> {
    use crate::schema::get_requests::table;

    let conn = &db_connection(pool)?;

    let new_get_request : GetRequest = GetRequest {
        api_key: header_api_key,
        route: body.route,
        response: body.response
    };

    let result: GetRequest = diesel::insert_into(table)
        .values(new_get_request)
        .get_result(conn)?;

    Ok(result)
}