use rust_decimal::Decimal;

pub struct Inputs {
    pub rpc_url: String,
    pub user_address: String,
}

#[derive(Debug)]
pub struct ChainInfo {
    pub chain_id: u64,
}

pub struct Portfolio {
    pub native_balance: Decimal,
}
