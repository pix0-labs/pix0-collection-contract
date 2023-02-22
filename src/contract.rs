#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::ins::{create_collection, create_user, create_item};
use crate::query::{get_all_collections, get_collections, get_collection, user_exists, get_user, get_users};
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

        ExecuteMsg::CreateUser { user_name, first_name, last_name, email, mobile}
        => create_user(deps, _env, info,user_name, first_name, last_name,email, mobile),

        ExecuteMsg::CreateItem { 
            collection_owner,  
            collection_name, 
            collection_symbol, 
            name,
            description,
            links,
            attributes,
            background_color}
        => create_item(deps, _env, info, crate::state::Item {
            collection_owner : collection_owner,
            collection_name : collection_name,
            collection_symbol : collection_symbol,
            name : name, 
            description : description,
            links: links, 
            attributes : attributes,
            background_color : background_color,
            date_created : None,
            date_updated : None, 
        }),
        /* 
        ExecuteMsg::MintItem { index, owner, collection_name, collection_symbol }
        => mint_item(deps, _env, info, index, owner, collection_name, collection_symbol),
        */
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

        QueryMsg::GetUsers { start_after, limit } =>
        to_binary(&get_users(deps, start_after, limit)?),

        QueryMsg::GetUser {  wallet_address} =>
        to_binary(&get_user(deps, wallet_address)?),

        QueryMsg::UserExists {wallet_address } =>
        to_binary(&user_exists(deps, wallet_address)?),

    }
}

