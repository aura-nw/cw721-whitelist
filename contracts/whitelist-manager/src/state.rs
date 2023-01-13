use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct WhitelistStatus {
    pub status: bool,
    pub uri: String,
}

pub const ADMIN: Item<Addr> = Item::new("admin");
pub const TOKEN_CONTRACT: Item<Addr> = Item::new("token_contract");
pub const WHITELIST: Map<Addr, WhitelistStatus> = Map::new("whilelist");
