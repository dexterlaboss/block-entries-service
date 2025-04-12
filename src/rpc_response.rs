
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug, Serialize)]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// Create a successful JSON-RPC response
pub fn json_rpc_ok<T: Serialize>(id: Option<Value>, result: T) -> JsonRpcResponse<Value> {
    let val = serde_json::to_value(result).unwrap_or_else(|_| json!({}));
    JsonRpcResponse {
        jsonrpc: "2.0",
        id,
        result: Some(val),
        error: None,
    }
}

/// Create an error JSON-RPC response
pub fn json_rpc_error(id: Option<Value>, code: i64, message: String) -> JsonRpcResponse<Value> {
    JsonRpcResponse {
        jsonrpc: "2.0",
        id,
        result: None,
        error: Some(JsonRpcError {
            code,
            message,
            data: None,
        }),
    }
}