use rust_decimal::Decimal;

pub fn parse_hex_u128(s: &str) -> Result<u128, Box<dyn std::error::Error>> {
    let s = s.trim_start_matches("0x");
    Ok(u128::from_str_radix(s, 16)?)
}

pub fn wei_to_eth(wei: u128) -> Decimal {
    let wei_dec = Decimal::from(wei);
    let scale = Decimal::from(10u128.pow(18));
    wei_dec / scale
}
