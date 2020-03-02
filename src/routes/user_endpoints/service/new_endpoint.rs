use actix_web::web::{Data, Json};
use crate::database::{db_connection, Pool};
use crate::errors::{ServiceResult};
use crate::routes::user_endpoints::model::{GetRequest, BasicGetRequest};

use diesel::RunQueryDsl;
use actix_web::HttpRequest;

use crate::routes::user_endpoints::validate_api_key::get_api_key_from_header_map;

pub fn new_endpoint(_req: HttpRequest, pool: Data<Pool>, body: Json<BasicGetRequest>) -> ServiceResult<GetRequest> {
    use crate::database::schema::get_requests::table;

    let conn = &db_connection(&pool)?;
    let request_body = body.into_inner();

    let header_api_key = get_api_key_from_header_map(_req.headers())?;

    let new_get_request : GetRequest = GetRequest {
        api_key: header_api_key,
        route: request_body.route,
        response: request_body.response
    };

    let result: GetRequest = diesel::insert_into(table)
        .values(new_get_request)
        .get_result(conn)?;

    Ok(result)
}