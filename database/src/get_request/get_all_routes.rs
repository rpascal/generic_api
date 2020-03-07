use crate::{Pool, db_connection};
use crate::errors::DatabaseResult;
use super::model::GetRequest;

use diesel::{RunQueryDsl, ExpressionMethods};
use uuid::Uuid;
use diesel::query_dsl::filter_dsl::FilterDsl;

pub fn execute(header_api_key: Uuid, pool: &Pool) -> DatabaseResult<Vec<GetRequest>> {
    use crate::schema::get_requests::{table, api_key};

    let conn = &db_connection(pool)?;

    let results = table
        .filter(api_key.eq(header_api_key))
        .load::<GetRequest>(conn)?;

    Ok(results)
}