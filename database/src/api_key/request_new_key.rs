use crate::{Pool, db_connection};
use crate::errors::DatabaseResult;
use super::model::ApiKey;

use diesel::RunQueryDsl;

pub fn execute(pool: &Pool) -> DatabaseResult<ApiKey> {
    use crate::schema::api_keys::table;

    let conn = &db_connection(pool)?;

    let new_key : ApiKey = ApiKey {
        api_key : uuid::Uuid::new_v4()
    };

    let result: ApiKey = diesel::insert_into(table)
        .values(&new_key)
        .get_result(conn)?;

    Ok(result)
}