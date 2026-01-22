use anyhow::Result;

use crate::providers::{portfolio::structs::Portfolio, rpc::{structs::EVMProvider, traits::ChainProvider}};

pub async fn get_portfolio(provider: EVMProvider) -> Result<Portfolio> {
    let chain_info = provider.chain_info().await?;
    let native_balance = provider.native_balance().await?;

    Ok(
        Portfolio { native_balance }
    )
}