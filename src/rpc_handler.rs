
use anyhow::{anyhow, Result};
use serde_json::{Value, to_value};

use crate::ledger::Ledger;

/// A struct that implements all the request handling methods.
pub struct RpcRequestHandler;

impl RpcRequestHandler {
    pub fn new() -> Self {
        Self
    }

    /// Dispatch by method name.
    pub async fn handle_request(
        &self,
        method: &str,
        params: &Value,
        ledger: &Ledger
    ) -> Result<Value> {
        match method {
            "getBlockEntries" => self.handle_get_block_entries(params, ledger).await,
            _ => Err(anyhow!("Method '{}' not found", method)),
        }
    }

    /// Handles `getBlockEntries` requests.
    async fn handle_get_block_entries(
        &self,
        params: &Value,
        ledger: &Ledger
    ) -> Result<Value> {
        let slot = parse_slot_param(params)?;
        // 1) Read the slot entries from the ledger
        let entries = ledger.read_slot_entries(slot)?;

        // 2) Convert to JSON
        Ok(to_value(entries)?)
    }
}

/// Extract a slot from JSON params.
fn parse_slot_param(params: &Value) -> Result<u64> {
    match params {
        Value::Array(arr) if !arr.is_empty() => {
            arr[0]
                .as_u64()
                .ok_or_else(|| anyhow!("First array element is not a valid u64"))
        }
        Value::Number(num) => {
            num.as_u64().ok_or_else(|| anyhow!("Param is not a valid u64"))
        }
        _ => Err(anyhow!("Invalid slot param format")),
    }
}