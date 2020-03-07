use crate::{Pool, db_connection};
use crate::errors::DatabaseResult;
use super::model::GetRequest;

use diesel::{RunQueryDsl, ExpressionMethods, QueryDsl};

use uuid::Uuid;

pub fn execute(path: &str, header_api_key: Uuid, pool: &Pool) -> DatabaseResult<serde_json::Value> {
    use crate::schema::get_requests::{table, api_key, route};

    let conn = &db_connection(pool)?;

    let results: GetRequest = table
        .filter(api_key.eq(header_api_key))
        .filter(route.eq(path))
        .first::<GetRequest>(conn)?;

    Ok(results.response)
}