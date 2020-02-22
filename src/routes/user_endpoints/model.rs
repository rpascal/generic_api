
use crate::database::schema::user_endpoints;

/**
It is VERY important that the order of the columns match the order of the field
or else diesel will scream
*/

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct UserEndpoint {
    pub key:  uuid::Uuid,
    pub endpoint: String,
    pub response: serde_json::Value,
    pub id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "user_endpoints"]
pub struct InsertableUserEndpoint  {
    pub key:  uuid::Uuid,
    pub endpoint: String,
    pub response: serde_json::Value,
}
