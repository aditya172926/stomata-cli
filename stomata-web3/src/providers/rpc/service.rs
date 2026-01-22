use anyhow::{Error, Result, anyhow, bail};
use reqwest::Client;
use rust_decimal::Decimal;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use crate::providers::{
    portfolio::structs::ChainInfo,
    rpc::{structs::EVMProvider, traits::ChainProvider},
};

async fn rpc_call<T: DeserializeOwned>(rpc_url: &str, method: &str, params: Value) -> Result<T> {
    let request_client = Client::new();

    let payload = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": method,
        "params": params,
    });

    let resp = request_client
        .post(rpc_url)
        .json(&payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    if let Some(err) = resp.get("error") {
        let code = err.get("code").unwrap_or(&Value::Null);
        let msg = err.get("message").unwrap_or(&Value::Null);
        return Err(anyhow!("RPC error {}: {}", code, msg));
    }

    let result = resp
        .get("result")
        .ok_or_else(|| anyhow!("Missing result field in RPC response"))?;

    Ok(serde_json::from_value(result.clone())?)
}

impl ChainProvider for EVMProvider {
    async fn chain_info(&self) -> anyhow::Result<crate::providers::portfolio::structs::ChainInfo> {
        let hex_id: String = rpc_call(&self.rpc_url, "eth_chainId", json!([])).await?;

        // remove 0x and parse hex
        let id = u64::from_str_radix(hex_id.trim_start_matches("0x"), 16)?;

        Ok(ChainInfo { chain_id: id })
    }

    async fn native_balance(&self) -> anyhow::Result<rust_decimal::Decimal> {
        let hex_balance: String =
            rpc_call(&self.rpc_url, "eth_getBalance", json!([self.address, "latest"])).await?;

        Ok(Decimal::from_str_exact(&hex_balance)?)
    }
}
