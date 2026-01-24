use anyhow::{Error, Result, anyhow, bail};
use reqwest::Client;
use rust_decimal::Decimal;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use crate::providers::{
    portfolio::structs::{AccountType, ChainInfo},
    rpc::{helper::parse_hex_u128, structs::EVMProvider, traits::ChainProvider},
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

    async fn native_balance(&self) -> Option<Decimal> {
        let hex_balance: String = rpc_call(
            &self.rpc_url,
            "eth_getBalance",
            json!([self.address, "latest"]),
        )
        .await
        .unwrap();
        println!("Hex balance {:?}", hex_balance);
        match parse_hex_u128(&hex_balance) {
            Ok(val) => Some(val.into()),
            Err(err) => None,
        }
    }

    async fn account_type(&self) -> Option<AccountType> {
        let code: String = rpc_call(
            &self.rpc_url,
            "eth_getCode",
            json!([self.address, "latest"]),
        )
        .await
        .unwrap();

        if code.len() == 0 {
            Some(AccountType::EOA)
        } else {
            Some(AccountType::CONTRACT)
        }
    }

    async fn transaction_count(&self) -> u64 {
        let transaction_count: String = rpc_call(
            &self.rpc_url,
            "eth_getTransactionCount",
            json!([self.address, "latest"]),
        )
        .await
        .unwrap();

        u64::from_str_radix(transaction_count.trim_start_matches("0x"), 16).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    fn init_evm_provider() -> EVMProvider {
        dotenv().ok();
        let rpc_url = std::env::var("ETHEREUM_MAINNET_RPC_URL")
            .expect("Ethereum mainnet rpc not found in env");
        let user_address =
            std::env::var("TEST_EVM_ADDRESS").expect("Test evm address not found in env");
        EVMProvider::new(user_address, rpc_url)
    }

    #[tokio::test]
    async fn test_chain_id_fetch() {
        let evm_provider = init_evm_provider();
        let chain_info = evm_provider.chain_info().await.unwrap();
        assert!(chain_info.chain_id == 1)
    }

    #[tokio::test]
    async fn test_native_balance_fetch() {
        let evm_provider = init_evm_provider();
        let native_balance = evm_provider.native_balance().await;
        assert!(native_balance.is_some(), "Failed to fetch native balance");
    }

    #[tokio::test]
    async fn test_account_type() {
        let evm_provider = init_evm_provider();
        let account_type = evm_provider.account_type().await;
        assert!(account_type.is_some(), "Failed to fetch account_type");
    }

    #[tokio::test]
    async fn test_transaction_count() {
        let evm_provider = init_evm_provider();
        let nonce: u64 = evm_provider.transaction_count().await;
        assert!(nonce > 0, "Failed to fetch transaction count");
    }
}
