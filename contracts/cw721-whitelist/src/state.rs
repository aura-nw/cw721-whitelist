use cosmwasm_std::Addr;
use cw_storage_plus::Map;

pub const WHILELIST: Map<Addr, u8> = Map::new("whilelist");
