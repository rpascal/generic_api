use actix_web::web::{Data};
use crate::database::{db_connection, Pool};
use crate::errors::{ServiceResult};
use crate::routes::user_endpoints::model::{GetRequest, BasicGetRequest};

use diesel::RunQueryDsl;
use uuid::Uuid;

pub fn new_endpoint(header_api_key: Uuid, pool: Data<Pool>, body: BasicGetRequest) -> ServiceResult<GetRequest> {
    use crate::database::schema::get_requests::table;

    let conn = &db_connection(&pool)?;

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