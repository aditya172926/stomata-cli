use anyhow::Result;

use crate::providers::{
    portfolio::structs::Portfolio,
    rpc::{structs::EVMProvider, traits::ChainProvider},
};

pub async fn get_portfolio(provider: EVMProvider) -> Result<Portfolio> {
    let chain_info = provider.chain_info().await?;
    let native_balance = provider.native_balance().await.unwrap();
    let account_type = provider.account_type().await.unwrap();
    Ok(Portfolio {
        native_balance,
        account_type,
        transaction_count: 0,
    })
}
