use actix_web::web::Data;
use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::routes::user_endpoints::model::{UserEndpoint, InsertableUserEndpoint};

use diesel::RunQueryDsl;

pub fn request_new_key(pool: Data<Pool>) -> ServiceResult<UserEndpoint> {
    use crate::database::schema::user_endpoints::table;

    let conn = &db_connection(&pool)?;

    let json: serde_json::Value = serde_json::from_str(r#"{
        "Example": "Article Circle Expressway 1",
        "Hello": "North Pole"
    }"#).unwrap();

    let new_endpoint : InsertableUserEndpoint = InsertableUserEndpoint {
        key : uuid::Uuid::new_v4(),
        endpoint: String::from("/"),
        response: json,
    };

    let result: UserEndpoint = diesel::insert_into(table)
        .values(&new_endpoint)
        .get_result(conn)?;

    Ok(result)
}