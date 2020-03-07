use actix_web::web::Data;
use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use super::model::GetRequest;

use diesel::{RunQueryDsl, ExpressionMethods};
use uuid::Uuid;
use diesel::query_dsl::filter_dsl::FilterDsl;

pub fn execute(header_api_key: Uuid, pool: Data<Pool>) -> ServiceResult<Vec<GetRequest>> {
    use crate::database::schema::get_requests::{table, api_key};

    let conn = &db_connection(&pool)?;

    let results = table
        .filter(api_key.eq(header_api_key))
        .load::<GetRequest>(conn)?;

    Ok(results)
}