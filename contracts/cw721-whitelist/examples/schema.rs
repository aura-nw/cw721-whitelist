use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;

use cw721_base::{ExecuteMsg, QueryMsg};
use cw721_whitelist::msg::InstantiateMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg<Empty, Empty>,
        query: QueryMsg<Empty>,
    }
}
