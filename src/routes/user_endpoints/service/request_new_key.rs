use actix_web::web::Data;
use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::routes::user_endpoints::model::ApiKey;

use diesel::RunQueryDsl;

pub fn request_new_key(pool: Data<Pool>) -> ServiceResult<ApiKey> {
    use crate::database::schema::api_keys::table;

    let conn = &db_connection(&pool)?;

    let new_key : ApiKey = ApiKey {
        api_key : uuid::Uuid::new_v4()
    };

    let result: ApiKey = diesel::insert_into(table)
        .values(&new_key)
        .get_result(conn)?;

    Ok(result)
}