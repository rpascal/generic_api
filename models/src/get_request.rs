use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub api_key:  uuid::Uuid,
    pub route: String,
    pub response: serde_json::Value,
    pub query_params: HashMap<String, String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicGetRequest {
    pub route: String,
    pub response: serde_json::Value
}
