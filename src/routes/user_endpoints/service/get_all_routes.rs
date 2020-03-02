use actix_web::web::Data;
use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::routes::user_endpoints::model::GetRequest;

use diesel::{RunQueryDsl, ExpressionMethods, QueryDsl};
use actix_web::HttpRequest;
use crate::routes::user_endpoints::validate_api_key::get_api_key_from_header_map;

pub fn get_all_routes(_req: HttpRequest, pool: Data<Pool>) -> ServiceResult<Vec<GetRequest>> {
    use crate::database::schema::get_requests::{table, api_key};

    let conn = &db_connection(&pool)?;

    let header_api_key = get_api_key_from_header_map(_req.headers())?;

    let results = table.filter(api_key.eq(header_api_key))
        .load::<GetRequest>(conn)
        .expect("Error loading posts");

    Ok(results)
}