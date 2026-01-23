use anyhow::Result;
use rust_decimal::Decimal;

use crate::providers::portfolio::structs::ChainInfo;

pub trait ChainProvider {
    async fn chain_info(&self) -> Result<ChainInfo>;
    async fn native_balance(&self) -> Option<Decimal>;
}
