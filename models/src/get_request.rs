#[derive(Debug, Serialize, Deserialize)]
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
