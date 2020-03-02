
use crate::database::schema::api_keys;
use crate::database::schema::get_requests;

/**
It is VERY important that the order of the columns match the order of the field
or else diesel will scream
*/

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
pub struct ApiKey {
    pub api_key:  uuid::Uuid
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
pub struct GetRequest {
    pub api_key:  uuid::Uuid,
    pub route: String,
    pub response: serde_json::Value
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicGetRequest {
    pub route: String,
    pub response: serde_json::Value
}
