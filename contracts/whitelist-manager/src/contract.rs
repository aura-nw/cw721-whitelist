#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, WasmMsg,
};
use cw2::set_contract_version;
use cw721_base::{Extension, MintMsg};

pub type Cw721ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;

use sha2::{Digest, Sha256};

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryMsg, TokenContractResponse, WhitelistInfo,
    WhitelistStatusResponse,
};
use crate::state::{WhitelistStatus, ADMIN, TOKEN_CONTRACT, WHITELIST};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:whitelist-manager";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // set admin
    ADMIN.save(deps.storage, &info.sender)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetTokenContract { token_contract } => {
            set_token_contract(deps, info, token_contract)
        }
        ExecuteMsg::AddToWhitelist { whitelist_infos } => {
            add_to_whitelist(deps, info, whitelist_infos)
        }
        ExecuteMsg::RemoveFromWhitelist { addresses } => {
            remove_from_whitelist(deps, info, addresses)
        }
        ExecuteMsg::Mint {} => mint(deps, info),
    }
}

// let user in whitelist mint NFT
pub fn mint(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    // check if the info.sender in the whitelist
    let whitelist_status = WHITELIST.load(deps.storage, info.sender.clone());
    match whitelist_status {
        Ok(status) => {
            if status.status {
                // set status to minted
                let minted_status = WhitelistStatus {
                    status: false,
                    uri: status.uri.clone(),
                };
                WHITELIST.save(deps.storage, info.sender.clone(), &minted_status)?;

                // token_id is the SHA256 hash of the info.sender
                let hash = Sha256::digest(info.sender.to_string().as_bytes());
                let token_id = hex::encode(hash.as_slice());

                // prepare MintMsg
                let msg = MintMsg {
                    token_id,
                    owner: info.sender.to_string(),
                    token_uri: Some(status.uri),
                    extension: Some(Empty {}),
                };

                // send MintMsg to NFT contract
                let token_contract = TOKEN_CONTRACT.load(deps.storage)?;

                // prepare mint nft msg
                let mint_nft_msg = WasmMsg::Execute {
                    contract_addr: token_contract.to_string(),
                    msg: to_binary(&Cw721ExecuteMsg::Mint(msg))?,
                    funds: vec![],
                };

                Ok(Response::new()
                    .add_message(mint_nft_msg)
                    .add_attribute("method", "mint")
                    .add_attribute("owner", info.sender))
            } else {
                Err(ContractError::Unauthorized {})
            }
        }
        Err(_) => Err(ContractError::Unauthorized {}),
    }
}

// let admin set the contract address of NFT
pub fn set_token_contract(
    deps: DepsMut,
    info: MessageInfo,
    token_contract: String,
) -> Result<Response, ContractError> {
    let owner = ADMIN.load(deps.storage)?;
    if owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let token_contract = deps.api.addr_validate(&token_contract)?;

    TOKEN_CONTRACT.save(deps.storage, &token_contract)?;

    Ok(Response::new()
        .add_attribute("method", "set_token_contract")
        .add_attribute("owner", owner)
        .add_attribute("token_contract", token_contract))
}

// let admin add addresses to the whitelist
pub fn add_to_whitelist(
    deps: DepsMut,
    info: MessageInfo,
    whitelist_infos: Vec<WhitelistInfo>,
) -> Result<Response, ContractError> {
    let owner = ADMIN.load(deps.storage)?;
    if owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    for whitelist_info in whitelist_infos.clone() {
        let address = deps.api.addr_validate(&whitelist_info.address)?;

        let whitelist_status = WhitelistStatus {
            status: true,
            uri: whitelist_info.uri,
        };

        WHITELIST.save(deps.storage, address, &whitelist_status)?;
    }

    Ok(Response::new()
        .add_attribute("method", "add_to_whitelist")
        .add_attribute("owner", owner)
        .add_attribute("number_added", whitelist_infos.len().to_string()))
}

// let admin remove addresses from the whitelist
pub fn remove_from_whitelist(
    deps: DepsMut,
    info: MessageInfo,
    addresses: Vec<String>,
) -> Result<Response, ContractError> {
    let owner = ADMIN.load(deps.storage)?;
    if owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    for address in addresses.clone() {
        let address = deps.api.addr_validate(&address)?;

        WHITELIST.remove(deps.storage, address);
    }

    Ok(Response::new()
        .add_attribute("method", "remove_from_whitelist")
        .add_attribute("owner", owner)
        .add_attribute("addresses", addresses.join(",")))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TokenContract {} => to_binary(&query_token_contract(deps)?),
        QueryMsg::WhitelistStatus { address } => to_binary(&query_whitelist_status(deps, address)?),
    }
}

// query the contract address of NFT
pub fn query_token_contract(deps: Deps) -> StdResult<TokenContractResponse> {
    let token_contract = TOKEN_CONTRACT.load(deps.storage).unwrap();

    Ok(TokenContractResponse {
        token_contract: token_contract.to_string(),
    })
}

// query the status of an address in the whitelist
pub fn query_whitelist_status(deps: Deps, address: String) -> StdResult<WhitelistStatusResponse> {
    let address = deps.api.addr_validate(&address).unwrap();
    let status = WHITELIST.load(deps.storage, address);

    match status {
        Ok(status) => {
            if status.status {
                Ok(WhitelistStatusResponse::AllowMint)
            } else {
                Ok(WhitelistStatusResponse::Minted)
            }
        }
        Err(_) => Ok(WhitelistStatusResponse::DenyMint),
    }
}
