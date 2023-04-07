use cosmwasm_std::{DepsMut, Env, Response, MessageInfo, Addr, Order, Coin, Uint128, BankMsg};
use crate::state::{Collection, Treasury, Attribute, PriceType, Item, Royalty, 
COLLECTION_STATUS_ACTIVATED, PRICE_TYPE_STANDARD};
use crate::indexes::{collections_store,COLLECTION_ITEMS_STORE };
use crate::error::ContractError;
use crate::query::{internal_get_collection, internal_get_all_items, internal_get_item};
use crate::nft_ins::init_and_mint_nft;
use pix0_contract_common::funcs::{try_paying_contract_treasuries};
use pix0_contract_common::state::{Fee, Contract};
use crate::checks::*;

pub fn collection_id ( name : String, symbol : String ) -> String {
    format!("{}-{}", name, symbol)
}

/*
Wrapper function
 */
pub fn update_contract_info (deps: DepsMut, 
    _env : Env, info: MessageInfo,
    _fees : Option<Vec<Fee>>, treasuries : Option<Vec<Addr>>, 
    contracts : Option<Vec<Contract>>, 
    _log_last_payment : Option<bool>, 
 ) -> Result<Response, ContractError> {

    let res =  pix0_contract_common::funcs::update_contract_info(
        deps, _env, info, _fees, treasuries, contracts, _log_last_payment);
           
    match res {

        Ok(r)=> Ok(r),

        Err(e)=> Err(ContractError::from(e)),
    }
}

  

pub fn create_collection (deps: DepsMut, 
    _env : Env, info: MessageInfo,
    collection : Collection) -> Result<Response, ContractError> {
        
    internal_create_collection(deps, _env, info, collection.name, 
        collection.symbol, collection.description, collection.treasuries, 
        collection.attributes, collection.prices, collection.royalties, collection.status)
   
}


pub fn update_collection(deps: DepsMut, 
    _env : Env, info: MessageInfo,
    collection : Collection) -> Result<Response, ContractError> {
  
    let owner = info.clone().sender;

    if collection.treasuries.is_some() {
        let _ = are_treasuries_valid(&collection.treasuries)?;
    }
  
    let _key = (owner.clone(), collection_id(collection.name.clone(), collection.symbol.clone()) );
  
    check_if_collection_status_valid(collection.status)?;
    
    let collection_to_update = internal_get_collection(deps.as_ref(), owner, collection.name, 
    collection.symbol);

    if collection_to_update.is_none() {
        return Err(ContractError::CollectionNotFound { text: "Collection is NOT found!".to_string()});
    }

    let mut collection_to_update = collection_to_update.unwrap();

    let mut to_update : bool = false;

    if collection.description.is_some() {
        collection_to_update.description = collection.description;
        to_update = true; 
    }
    if collection.treasuries.is_some() {
        collection_to_update.treasuries = collection.treasuries;
        to_update = true; 
    }

    if collection.prices.is_some() {
        collection_to_update.prices = collection.prices;
        to_update = true; 
    }

    if collection.attributes.is_some() {
        collection_to_update.attributes = collection.attributes;
        to_update = true; 
    }

    if collection.status.is_some() {
        collection_to_update.status = collection.status; 
        to_update = true; 
    }

    if to_update {
        collection_to_update.date_updated = Some(_env.block.time);
    }

    if to_update {

        collections_store().save(deps.storage, _key.clone(), &collection_to_update)?;
        common_response(format!("{}-{}",_key.0, _key.1).as_str(), "update_collection", STATUS_OK, None, None)
    }
    else {
        common_response(format!("{}-{}",_key.0, _key.1).as_str(), "update_collection", 
        STATUS_ERROR, Some("Nothing updated!".to_string()), None)
    }
 
}





pub (crate) fn internal_create_collection(mut deps: DepsMut, 
    _env : Env, info: MessageInfo,
    name : String, symbol : String, 
    description : Option<String> ,
    treasuries : Option<Vec<Treasury>>,
    attributes : Option<Vec<Attribute>>, 
    prices : Option<Vec<PriceType>>,
    royalties : Option<Vec<Royalty>>,
    _status : Option<u8>, 
    ) -> Result<Response, ContractError> {
  
    let owner = info.clone().sender;

    check_if_collection_exists(&deps, info.clone(), name.clone(), symbol.clone())?;

    let _msgs = try_paying_contract_treasuries(deps.branch(), _env.clone(), 
    info, "CREATE_COLLECTION_FEE")?;
 
    are_treasuries_valid(&treasuries)?;

    let _key = (owner.clone(), collection_id(name.clone(), symbol.clone()) );

    let date_created = _env.block.time;
    
    let status = check_if_collection_status_valid(_status)?;
    
    let new_collection = Collection {
        name : name.clone(), 
        symbol : symbol.clone(),
        owner : Some(owner), 
        treasuries : treasuries,
        attributes : attributes,
        prices : prices, 
        description : description,
        status : Some(status),
        royalties : royalties,
        date_created : Some(date_created),
        date_updated : Some(date_created),
    };

    collections_store().save(deps.storage, _key.clone(), &new_collection)?;

    common_response(format!("{}-{}",_key.0, _key.1).as_str(), "create_collection", STATUS_OK, 
    None, Some(_msgs))

    
}



#[allow(dead_code)]
pub (crate) fn internal_remove_item (
    owner : Addr, 
    collection_name : String,
    collection_symbol : String,
    name : String, 
    deps: DepsMut ) -> bool {
    
    let _key = (owner, collection_id(collection_name
        , collection_symbol), name );

    let stored_item = COLLECTION_ITEMS_STORE.key(_key.clone());

    let item_result = stored_item.load(deps.storage);
    
    if item_result.is_ok() {

        let loaded_item : Item = item_result.ok().unwrap();
        
        let akey = (loaded_item.collection_owner, collection_id(
            loaded_item.collection_name,loaded_item.collection_symbol),
            loaded_item.name);

        assert_eq!(akey, _key);  
        //println!("akey:{:?}::key:{:?}", akey, _key);

        COLLECTION_ITEMS_STORE.remove(deps.storage, _key.clone());

        true 
    }
    else {

        false 
    }

}


pub fn create_item(mut deps: DepsMut, 
    _env : Env, info: MessageInfo,item : Item 
) -> Result<Response, ContractError> {
  
    let mut item = item; 

    let owner = info.clone().sender;

    check_if_collection_exists(&deps, info.clone(), item.collection_name.clone(), 
    item.collection_symbol.clone())?;

    check_if_item_exists(&deps, info.clone(), item.collection_name.clone(), 
    item.collection_symbol.clone(), item.name.clone())?;

    let _msgs = try_paying_contract_treasuries(deps.branch(), _env.clone(), 
    info, "CREATE_ITEM_FEE")?;
 
    let _key = (item.collection_owner.clone(), 
    collection_id(item.collection_name.clone(), item.collection_symbol.clone()), 
    item.name.clone() );

    let date_created = _env.block.time;
    
    item.collection_owner = owner;

    item.date_created = Some(date_created);

    item.date_updated = item.date_created;

    COLLECTION_ITEMS_STORE.save(deps.storage, _key.clone(), &item)?;
    
    common_response( format!("{}-{}={}",_key.0, _key.1,
    _key.2).as_str(), "create_item", STATUS_OK, None, Some(_msgs))
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

pub fn mint_item (mut deps : DepsMut , 
    _env : Env, info: MessageInfo, seed : u64,
    owner : Addr,collection_name : String,  
    collection_symbol : String , 
    price_type : Option<u8>, 
    token_uri : Option<String>,
    token_id : Option<String> )-> Result<Response, ContractError> {

    let collection = internal_get_collection(deps.as_ref(), owner.clone(), 
    collection_name.clone(), collection_symbol.clone());

    if collection.is_none() {
        return Err(ContractError::CollectionNotFound { text: "Collection is NOT found!".to_string()});
    }

    let collection = collection.unwrap();

    if collection.status.is_none() || collection.status.unwrap() != COLLECTION_STATUS_ACTIVATED{
        return Err(ContractError::NftStatusIsNotReadyForMinting { text: "Collection is NOT ready for minting!".to_string()});
    }


    let items = internal_get_all_items(deps.as_ref(), owner.clone(), collection_name.clone(), 
    collection_symbol.clone());

    let mut rng = crate::utils::RandomNumGen::new(seed);
    let index = rng.generate_range(0, items.len() as u64) as usize;
   
   // println!("minted.at.index::{}", index);

    let itm = items.get(index);
    if itm.is_some() {

        let i = itm.unwrap();

        let price = collection.price_by_type(price_type.unwrap_or(PRICE_TYPE_STANDARD));

        let fund_checked = is_fund_sufficient(info.clone(), price.clone().unwrap());
        if !fund_checked.0 {
            return Err(ContractError::InsufficientFund {
                text: format!("Insufficient fund: sent:{}, required: {}!", 
                fund_checked.1, price.unwrap())});
 
        }

        let res = init_and_mint_nft(deps.branch(), _env, info, 
        i.clone(), collection, price_type, token_uri, Some("random-mint".to_string()), token_id);

        if res.is_ok() {
            internal_remove_item(owner, collection_name, collection_symbol, i.name.clone(), deps);
        }
        
        res 
    }
    else {
        Err(ContractError::FailedToFindNft { text : format!("Failed to find item at index :{}", index)})
    }
    
}

pub fn mint_item_by_name (mut deps : DepsMut , 
    _env : Env, info: MessageInfo, item_name : String ,
    owner : Addr,collection_name : String,  
    collection_symbol : String , 
    price_type : Option<u8>, 
    token_uri : Option<String>,
    token_id : Option<String>)-> Result<Response, ContractError> {

    let collection = internal_get_collection(deps.as_ref(), owner.clone(), 
    collection_name.clone(), collection_symbol.clone());

    if collection.is_none() {
        return Err(ContractError::CollectionNotFound { text: "Collection is NOT found!".to_string()});
    }

    let collection = collection.unwrap();
    
    if collection.status.is_none() || collection.status.unwrap() != COLLECTION_STATUS_ACTIVATED {
        return Err(ContractError::NftStatusIsNotReadyForMinting { text: "Collection is NOT ready for minting!".to_string()});
    }

    if !collection.is_mint_by_name_allowed() {

        return Err(ContractError::MintByNameIsNotAllowed { text: 
            "Collection does NOT allow minting by name!".to_string()});
  
    }

    let item = internal_get_item(deps.as_ref(), owner.clone(), collection_name.clone(), 
    collection_symbol.clone(), item_name.clone());

    let price = collection.price_by_type(price_type.unwrap_or(PRICE_TYPE_STANDARD));

    let fund_checked = is_fund_sufficient(info.clone(), price.clone().unwrap());
    if !fund_checked.0 {
        return Err(ContractError::InsufficientFund {
            text: format!("Insufficient fund: sent:{:?}, required: {:?}!", 
            fund_checked.1, price.unwrap())});

    }


    if item.is_some() {
        let itm = item.unwrap();
        let res = init_and_mint_nft(deps.branch(), 
        _env, info, itm.clone(), collection,price_type,token_uri,
        Some("mint-by-name".to_string()),token_id);

        if res.is_ok() {
            internal_remove_item(owner, collection_name, collection_symbol, itm.name.clone(), deps);
        }

        res 
       
    }
    else {
        Err(ContractError::CustomErrorMesg{message : format!("Item named {} not found", item_name )})
    }
       
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


pub fn remove_collection (
    name : String,
    symbol : String,
    mut deps: DepsMut ,  
    info: MessageInfo) -> Result<Response, ContractError> {
    
    let owner = info.clone().sender;

    let _ = collectionn_allowed_for_removal(owner.clone(), name.clone(), symbol.clone(), &deps)?;

    let _key = (owner.clone(), collection_id(name.clone(), symbol.clone()) );

    let removed_res = collections_store().remove(deps.branch().storage, _key.clone());
    
    match removed_res {

        Ok(_)=> {
            remove_all_items(owner, name, symbol, deps);
            common_response(format!("{}-{}",_key.0, _key.1).as_str(), "remove_collection", STATUS_OK, None, None)
        },
        Err(e)=> {common_response(format!("{}-{}",_key.0, _key.1).as_str(), "remove_collection", 
        STATUS_ERROR, Some(e.to_string()), None ) }
         , 
    }    

}

pub (crate) fn remove_all_items( 
    owner : Addr, 
    collection_name : String,
    collection_symbol : String, 
    deps: DepsMut) {

    let _prefix = (owner.clone(), collection_id(collection_name, collection_symbol));

    
    let mut keys : Vec<(Addr,String,String)> = Vec::new();

    {
        let mut iter = COLLECTION_ITEMS_STORE
            .prefix(_prefix)
            .range(deps.storage, None, None, Order::Ascending)
            .into_iter();
    
      
        while let Some(itm_res) = iter.next() {
            if let Ok(itm) = itm_res {
    
                let _key = (owner.clone(), collection_id(itm.1.collection_name, 
                    itm.1.collection_symbol), itm.1.name);
              
                keys.push(_key);
            }
        }
    
    }

    for _key in keys.iter() {
        COLLECTION_ITEMS_STORE.remove(deps.storage, _key.clone());    
    }
    
}



#[allow(dead_code)]
const STATUS_ERROR : i8 = -1;

const STATUS_OK : i8 = 1;


pub (crate) fn common_response (key : &str , method : &str, status : i8,
message : Option<String>, bank_messages : Option<Vec<BankMsg>>) -> Result<Response, ContractError> {

    let mut bmsgs : Vec<BankMsg> = Vec::new();

    if bank_messages.is_some() {

        bmsgs = bank_messages.unwrap();
    }

    if message.is_some() {

        let mesg = message.unwrap();
        
      
        Ok(Response::new()
        .add_messages(bmsgs)
        .add_attribute("key",key)
        .add_attribute("method", method)
        .add_attribute("status", format!("{}", status))
        .add_attribute("message", mesg))

    }
    else {

        Ok(Response::new()
        .add_messages(bmsgs)
        .add_attribute("key",key)
        .add_attribute("method", method)
        .add_attribute("status", format!("{}", status)))
    }
   
}