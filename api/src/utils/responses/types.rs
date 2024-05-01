use serde_json::Value;

pub struct AppResponse {
    pub code: u16,
    pub status: String,
    pub message: String,
    pub data: Option<Value>,
    pub errors: Option<Value>,
}
