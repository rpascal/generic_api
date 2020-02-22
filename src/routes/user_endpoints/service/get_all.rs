use actix_web::web::Data;
use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::routes::user_endpoints::model::UserEndpoint;

use diesel::RunQueryDsl;

pub fn get_all(pool: Data<Pool>) -> ServiceResult<Vec<UserEndpoint>> {
    use crate::database::schema::user_endpoints::table;

    let conn = &db_connection(&pool)?;

    let results = table
        .load::<UserEndpoint>(conn)
        .expect("Error loading posts");

    Ok(results)
}