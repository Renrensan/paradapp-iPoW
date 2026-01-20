use ethers::{types::U256, utils::parse_ether};

pub fn parse_human_native_token(x: impl ToString) -> anyhow::Result<U256> {
    let s = x.to_string();
    let v = parse_ether(s)?;
    Ok(v)
}
