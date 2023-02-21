#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::ins::create_collection;
use crate::query::{get_all_collections, get_collections, get_collection};
use crate::msg::{ExecuteMsg,InstantiateMsg, QueryMsg};
use crate::state::ContractInfo;
use crate::indexes::CONTRACT_INFO;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:pix0-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let contract_info = ContractInfo {

        allowed_admins : _msg.allowed_admins,

        date_instantiated :_env.block.time,

        name : _msg.name, 
    };
   
    CONTRACT_INFO.save(deps.storage, &contract_info)?;


    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("contract info", contract_info)
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateCollection { name, symbol, description, treasuries, attributes, prices, status }
        => create_collection(deps, _env, info, name,symbol, description, treasuries, attributes, prices, status ),
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCollection { owner, name, symbol } =>
        to_binary(&get_collection(deps, owner, name, symbol)?),
        
        QueryMsg::GetCollections { owner, start_after, limit } =>
        to_binary(&get_collections(deps,owner, start_after, limit)?),

        QueryMsg::GetAllCollections { limit } =>
        to_binary(&get_all_collections(deps, limit)?),
    }
}
