use cosmwasm_schema::{cw_serde, QueryResponses};

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    // Set the contract address of NFT
    SetTokenContract { token_contract: String },
    // Add whitelist_infos to the whitelist
    AddToWhitelist { whitelist_infos: Vec<WhitelistInfo> },
    // Remove addresses from the whitelist
    RemoveFromWhitelist { addresses: Vec<String> },
    // Mint NFT to the address in the whitelist
    Mint {},
}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // Query the contract address of NFT
    #[returns(TokenContractResponse)]
    TokenContract {},
    // Query the status of an address in the whitelist
    #[returns(WhitelistStatusResponse)]
    WhitelistStatus { address: String },
}

#[cw_serde]
pub struct TokenContractResponse {
    pub token_contract: String,
}

#[cw_serde]
pub enum WhitelistStatusResponse {
    AllowMint,
    DenyMint,
    Minted,
}

#[cw_serde]
pub struct WhitelistInfo {
    pub address: String,
    pub uri: String,
}
