use rust_decimal::Decimal;

pub struct Inputs {
    pub rpc_url: String,
    pub user_address: String,
}

#[derive(Debug)]
pub struct ChainInfo {
    pub chain_id: u64,
}

#[derive(Debug)]
pub enum AccountType {
    EOA,
    CONTRACT
}

pub struct Portfolio {
    pub account_type: AccountType,
    pub native_balance: Decimal,
    pub transaction_count: u64
}
