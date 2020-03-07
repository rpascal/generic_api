use crate::database::schema::api_keys;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
pub struct ApiKey {
    pub api_key:  uuid::Uuid
}