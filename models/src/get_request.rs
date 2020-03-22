use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub api_key:  uuid::Uuid,
    pub request: Request,
    pub response: serde_json::Value
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicGetRequest {
    pub request: Request,
    pub response: serde_json::Value
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub route: String,
    pub query_params: HashMap<String, String>
}