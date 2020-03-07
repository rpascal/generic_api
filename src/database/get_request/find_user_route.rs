use actix_web::web::Data;
use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use super::model::GetRequest;

use diesel::{RunQueryDsl, ExpressionMethods, QueryDsl};

use uuid::Uuid;

pub fn execute(path: &str, header_api_key: Uuid, pool: Data<Pool>) -> ServiceResult<serde_json::Value> {
    use crate::database::schema::get_requests::{table, api_key, route};

    let conn = &db_connection(&pool)?;

    let results: GetRequest = table
        .filter(api_key.eq(header_api_key))
        .filter(route.eq(path))
        .first::<GetRequest>(conn)?;

    Ok(results.response)
}