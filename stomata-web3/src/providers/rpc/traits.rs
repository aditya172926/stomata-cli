use anyhow::Result;
use rust_decimal::Decimal;

use crate::providers::portfolio::structs::{AccountType, ChainInfo};

pub trait ChainProvider {
    async fn chain_info(&self) -> Result<ChainInfo>;
    async fn native_balance(&self) -> Option<Decimal>;
    async fn account_type(&self) -> Option<AccountType>;
}
