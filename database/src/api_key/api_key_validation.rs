use uuid::Uuid;
use super::model::ApiKey;
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};
use crate::{Pool, db_connection};
use crate::errors::{DatabaseResult, DatabaseError};

pub fn execute(test_api_key: Uuid, pool: &Pool) -> DatabaseResult<()> {
    let conn = &db_connection(pool)?;
    use crate::schema::api_keys::{table, api_key};

    let results = table.filter(api_key.eq(test_api_key))
        .limit(1)
        .load::<ApiKey>(conn)
        .expect("Error loading posts");

    if results.len() > 0 {
        return Ok(());
    }

    return Err(DatabaseError::Unauthorized(format!("Bad api_key: {0}", test_api_key.to_string())));
}
