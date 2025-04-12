
use std::sync::Arc;
use anyhow::Result;
use serde_json::Value;

use crate::{
    rpc_server::ApplicationState,
    rpc_handler::RpcRequestHandler,
    rpc_request::JsonRpcRequest,
    rpc_response::{JsonRpcResponse, json_rpc_ok, json_rpc_error},
};

#[derive(Clone)]
pub struct RpcRouter {
    handler: Arc<RpcRequestHandler>,
}

impl RpcRouter {
    pub fn new(handler: Arc<RpcRequestHandler>) -> Self {
        Self { handler }
    }

    /// Handle a JSON-RPC request.
    pub async fn handle(
        &self,
        state: &ApplicationState,
        req: &JsonRpcRequest
    ) -> JsonRpcResponse<Value> {
        // Must be JSON-RPC 2.0
        if req.jsonrpc != "2.0" {
            return json_rpc_error(
                Some(req.id.clone()),
                -32600,
                "Invalid JSON-RPC version (expected '2.0')".to_string(),
            );
        }

        match self.dispatch(req.method.as_str(), &req.params, state).await {
            Ok(val) => json_rpc_ok(Some(req.id.clone()), val),
            Err(e) => json_rpc_error(
                Some(req.id.clone()),
                -32603,
                format!("Method '{}' failed: {}", req.method, e),
            ),
        }
    }

    async fn dispatch(
        &self,
        method: &str,
        params: &Value,
        state: &ApplicationState
    ) -> Result<Value> {
        self.handler
            .handle_request(method, params, &state.ledger)
            .await
    }
}