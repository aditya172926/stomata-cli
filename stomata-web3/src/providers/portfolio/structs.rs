use rust_decimal::Decimal;

pub struct Inputs {
    pub rpc_url: String,
    pub user_address: String,
}

#[derive(Debug, Default)]
pub enum AccountType {
    #[default]
    EOA,
    CONTRACT,
}

#[derive(Default, Debug)]
pub struct Portfolio {
    pub account_type: AccountType,
    pub native_balance: Decimal,
    pub transaction_count: u64,
}
