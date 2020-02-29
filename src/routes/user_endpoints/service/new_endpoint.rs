use actix_web::web::{Data, Json};
use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::routes::user_endpoints::model::{UserEndpoint, InsertableUserEndpoint};

use diesel::RunQueryDsl;

pub fn new_endpoint(pool: Data<Pool>, body: Json<InsertableUserEndpoint>) -> ServiceResult<UserEndpoint> {
    use crate::database::schema::user_endpoints::table;

    let conn = &db_connection(&pool)?;

    let result: UserEndpoint = diesel::insert_into(table)
        .values(body.into_inner())
        .get_result(conn)?;

    Ok(result)
}