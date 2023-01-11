use cosmwasm_std::{DepsMut, Empty, Env, MessageInfo, Response};
use cw721_base::msg::InstantiateMsg as Cw721BaseInstantiateMsg;
use cw721_base::state::TokenInfo;
use cw721_base::{ContractError, Cw721Contract, Extension, MintMsg};

pub use crate::msg::InstantiateMsg;

pub mod msg;
pub mod state;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw721-whitelist";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Cw721Whitelist<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    use super::*;

    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        // set contract's information
        let cw721_base_instantiate_msg = Cw721BaseInstantiateMsg {
            name: msg.name,
            symbol: msg.symbol,
            minter: msg.minter.clone(),
        };

        Cw721Whitelist::default().instantiate(
            deps.branch(),
            env,
            info,
            cw721_base_instantiate_msg,
        )?;

        // set whitelist
        for address_str in msg.whitelist {
            state::WHILELIST.save(
                deps.storage,
                deps.api.addr_validate(&address_str).unwrap(),
                &msg.number_nft_each_address,
            )?;
        }

        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        Ok(Response::default()
            .add_attribute("contract_name", CONTRACT_NAME)
            .add_attribute("contract_version", CONTRACT_VERSION))
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::Mint(msg) => _mint(deps, env, info, msg),
            _ => Cw721Whitelist::default().execute(deps, env, info, msg),
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Cw721Whitelist::default().query(deps, env, msg)
    }
}

// rewrite mint function of cw721 base to ignore minter checking
fn _mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: MintMsg<Extension>,
) -> Result<Response, ContractError> {
    // check if sender is in whitelist
    let whitelist_remained = state::WHILELIST
        .load(deps.storage, info.sender.clone())
        .unwrap();

    // if whitelist_remained is less than or equal 0, it means that sender cannot mint anymore
    if whitelist_remained == 0 {
        return Err(ContractError::Unauthorized {});
    }

    // decrease whitelist remained
    state::WHILELIST.save(deps.storage, info.sender.clone(), &(whitelist_remained - 1))?;

    // create the token
    let token = TokenInfo {
        owner: deps.api.addr_validate(&msg.owner)?,
        approvals: vec![],
        token_uri: msg.token_uri,
        extension: msg.extension,
    };

    Cw721Whitelist::default()
        .tokens
        .update(deps.storage, &msg.token_id, |old| match old {
            Some(_) => Err(ContractError::Claimed {}),
            None => Ok(token),
        })?;

    Cw721Whitelist::default().increment_tokens(deps.storage)?;

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("minter", info.sender)
        .add_attribute("owner", msg.owner)
        .add_attribute("token_id", msg.token_id))
}
