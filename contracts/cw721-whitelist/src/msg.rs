use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub minter: String,
    pub whitelist: Vec<String>,
    pub number_nft_each_address: u8,
}
