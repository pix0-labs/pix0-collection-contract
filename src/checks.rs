use cosmwasm_std::{DepsMut, MessageInfo, Addr, Coin, Uint128};
use crate::indexes::{collections_store, COLLECTION_ITEMS_STORE};
use crate::ins::collection_id;
use crate::query::internal_get_collection;
use crate::error::ContractError;
use crate::state::{COLLECTION_STATUS_ACTIVATED, COLLECTION_STATUS_DEACTIVATED, COLLECTION_STATUS_DRAFT, Treasury};
use pix0_market_handlers::state::Royalty;

fn collection_exists( deps: &DepsMut, info: MessageInfo, name : String, symbol : String ) -> 
bool {

    let owner = info.clone().sender;
    
    let _key = (owner, collection_id(name, symbol));

    let loaded_collection = collections_store()
    .idx.collections.item(deps.storage, _key);
    
    let mut exists = false; 

    match loaded_collection {

        Ok (c) => {
            if c.is_some() {
                exists = true
            }
        },

        Err(_)=> exists = false, 
    }

    return exists;
}


pub (crate) fn check_if_collection_exists(
    deps: &DepsMut,info: MessageInfo, name : String, symbol : String, 
    error_on_exists : bool, 
) -> Result<(), ContractError>{

    let exists = collection_exists(deps, info, name.clone(), symbol.clone());
    if error_on_exists {

        if exists {
            return Err(ContractError::CollectionAlreadyExists { 
                text: format!("Collection {}-{} already exists!", name, symbol).to_string() } );
          
        }
       
    }
    else {

        if !exists {
            return Err(ContractError::CollectionNotFound { 
                text: format!("Collection {}-{} NOT found!", name, symbol).to_string() } );
          
        }
       
    }
    
    Ok(())

}


fn is_status_valid ( status : u8) -> bool {

    status == COLLECTION_STATUS_DRAFT ||
    status == COLLECTION_STATUS_ACTIVATED ||
    status == COLLECTION_STATUS_DEACTIVATED

}

pub (crate) fn check_if_collection_status_valid(_status : Option<u8>)-> 
Result<u8,ContractError>{

    if _status.is_some() {
        let stat = _status.unwrap();
        
        if !is_status_valid(stat) {
            return Err(ContractError::InvalidCollectionStatus { text: 
                format!("Invalid status :{}!", stat ).to_string() } );
        }

        return Ok(stat); 
    }
   
    Ok(COLLECTION_STATUS_DRAFT)
}


pub (crate) fn are_treasuries_valid (treasuries : &Option<Vec<Treasury>>)  -> Result<bool, ContractError> {

    if treasuries.is_some () {

        let mut total_percentage = 0;

        let ts = treasuries.clone().unwrap();

        ts.iter().for_each(|t| total_percentage += t.percentage);

        if total_percentage > 100 || total_percentage < 100 {

            return Err(ContractError::InvalidAllocationsForTreasuries { message:  
                format!("Invalid percentage {} for treasuries amount, the total must be 100", total_percentage) } );
        }
        else {
            Ok(true)
        }
    }
    else {

        Ok(false)
    }
}



pub (crate) fn are_royalties_valid (royalties : &Option<Vec<Royalty>>)  -> Result<bool, ContractError> {

    if royalties.is_some () {

        let mut total_percentage = 0;

        let rs =  royalties.clone().unwrap();

        rs.iter().for_each(|t| total_percentage += t.percentage);

        // allow max total royalty of 15 percent only
        if total_percentage > 15 {

            return Err(ContractError::InvalidAllocationsForRoyalties { message : 
                format!("Invalid percentage {} for treasuries amount, the total must be 15% only", total_percentage) } );
        }
        else {
            Ok(true)
        }
    }
    else {

        Ok(false)
    }
}


fn item_exists( info: MessageInfo, 
    collection_name : String,
    collection_symbol : String,
    name : String, 
    deps: &DepsMut ) -> bool {

    let owner = info.clone().sender;
    
    let _key = (owner, collection_id(collection_name
        , collection_symbol), name );

    let stored_item = COLLECTION_ITEMS_STORE.load(deps.storage,_key);
    
    stored_item.is_ok()
}


pub (crate) fn check_if_item_exists(deps: &DepsMut,info: MessageInfo, 
    collection_name : String,
    collection_symbol : String,
    name : String) -> Result<(), ContractError> {

    if item_exists(info.clone(), collection_name.clone() ,
    collection_symbol,
    name.clone(), &deps) {
        return Err(ContractError::CustomErrorMesg { message: format!("The item {} in collection '{}' already exists!", 
        name.clone(),
        collection_name.clone()).to_string() } );
    }  

    Ok(())
}


pub (crate) fn collectionn_allowed_for_removal(owner: Addr, name : String,
    symbol : String, deps: &DepsMut) -> Result<bool,ContractError> {

    let collection = internal_get_collection(deps.as_ref(), owner.clone(), name.clone(), symbol.clone());

    if collection.is_none() {
        return Err(ContractError::CollectionNotFound { text: "Collection is NOT found!".to_string()});
    }
    else {
        let coll = collection.unwrap();
        if coll.status.is_none() || coll.status.unwrap() == COLLECTION_STATUS_ACTIVATED {
            return Err(ContractError::InvalidCollectionStatus { text: "Active collection cannot be removed!".to_string()});
        }
        else {
            Ok(true)
        }
    }
}


fn is_fund_sufficient (info : MessageInfo, required_fund : Coin) -> (bool, Coin) {

    let sent_funds: Vec<Coin> = info.funds.clone();

    if sent_funds.len() == 0 {
        return (false, Coin { amount :Uint128::default(), denom :"uconst".to_string()});
    }

    let first_fund = sent_funds.get(0).unwrap();

    if first_fund.amount < Uint128::from(required_fund.amount) ||
    first_fund.denom != required_fund.denom {
        (false,first_fund.clone()) 
    }
    else {
        (true,first_fund.clone())
    }
}


pub (crate) fn check_if_fund_sufficient(info : MessageInfo, required_fund : Coin) 
-> Result<(),ContractError> {

    let fund_checked = is_fund_sufficient(info.clone(), required_fund.clone());
    if !fund_checked.0 {
        return Err(ContractError::InsufficientFund {
            text: format!("Insufficient fund: sent:{}, required: {}!", 
            fund_checked.1, required_fund)});

    }

    Ok(())
}