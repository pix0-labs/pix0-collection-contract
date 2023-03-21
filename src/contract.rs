#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::ins::{create_collection, update_collection, create_item, mint_item_by_name, mint_item, 
    remove_collection,update_contract_info};
use crate::nft_ins::{transfer_nft, burn_nft, send_nft};
use crate::query::{get_all_collections, get_collections, get_collection, get_items_count, get_items, get_item};
use crate::nft_query::*;
use crate::msg::{ExecuteMsg,QueryMsg, MigrateMsg};
use crate::utils::str_to_u64;
use pix0_contract_common::funcs::{create_contract_info, get_contract_info, get_log_info};
use pix0_contract_common::msg::InstantiateMsg;

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
   
    create_contract_info(deps, _env, info.clone() ,_msg.allowed_admins,
    _msg.treasuries, _msg.fees, _msg.contracts,_msg.log_last_payment)?;
  

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
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
        ExecuteMsg::CreateCollection {collection }
        => create_collection(deps, _env, info, collection ),

        ExecuteMsg::UpdateCollection { collection }
        => update_collection(deps, _env, info, collection ),

        ExecuteMsg::RemoveCollection { name, symbol}
        => remove_collection(name,symbol, deps,info ),

         ExecuteMsg::CreateItem { item }
        => create_item(deps, _env, info, item ),
        
        ExecuteMsg::MintItemByName { name , owner, collection_name, collection_symbol, 
            price_type, token_uri, token_id }
        => mint_item_by_name(deps, _env, info, name , owner, 
            collection_name, collection_symbol, price_type,token_uri, token_id),

        ExecuteMsg::MintItem { seed , owner, collection_name, 
            collection_symbol, price_type, token_uri, token_id }
        => mint_item(deps, _env, info, str_to_u64(seed, 20502) , owner, 
        collection_name, collection_symbol,price_type, token_uri, token_id),

        ExecuteMsg::UpdateContractInfo { fees, treasuries , contracts,  log_last_payment} =>
        update_contract_info(deps, _env, info, fees, treasuries, contracts,log_last_payment),

        ExecuteMsg::TransferNft { recipient, token_id} => 
        transfer_nft(deps, _env, info, recipient, token_id),

        ExecuteMsg::BurnNft { token_id} => 
        burn_nft(deps, _env, info, token_id),

        ExecuteMsg::SendNft { token_id, contract_addr, action} => 
        send_nft(deps, _env, info, token_id, contract_addr, action),

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

        QueryMsg::MintedTokensByOwner { owner, start_after, limit } =>
        get_minted_tokens_by_owner(deps, _env, owner, start_after, limit),

        QueryMsg::NftTokenInfo { token_id} =>
        get_token_info(deps, _env, token_id),

        QueryMsg::GetItemsCount { owner, collection_name, collection_symbol } =>
        to_binary(&get_items_count(deps, owner, collection_name, collection_symbol)?),
        
        QueryMsg::GetItems { owner, collection_name, collection_symbol, start_after, limit } =>
        to_binary(&get_items(deps, owner, collection_name, collection_symbol, start_after, limit )?),
        
        QueryMsg::GetItem { owner, collection_name, collection_symbol, item_name } =>
        to_binary(&get_item(deps, owner, collection_name, collection_symbol, item_name )?),
        
        QueryMsg::GetContractInfo {} =>
        to_binary(&get_contract_info(deps)?),

        QueryMsg::GetLogInfo {} =>
        to_binary(&get_log_info(deps)?),
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new()
    .add_attribute("method", "migrate")
    .add_attribute("message", _msg.message))
}
