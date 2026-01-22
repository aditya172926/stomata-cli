pub struct EVMProvider {
    pub address: String,
    pub rpc_url: String,
}

impl EVMProvider {
    fn new(address: String, rpc_url: String) -> Self {
        Self { address, rpc_url }
    }
}