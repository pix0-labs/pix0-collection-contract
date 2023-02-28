use cosmwasm_std::{DepsMut, Env, Response, MessageInfo, Addr, Order};
use crate::state::{Collection, Treasury, Attribute, PriceType, Item, COLLECTION_STATUS_DRAFT,
COLLECTION_STATUS_ACTIVATED, COLLECTION_STATUS_DEACTIVATED, PRICE_TYPE_STANDARD};
use crate::indexes::{collections_store,ITEMS_STORE };
use crate::error::ContractError;
use crate::query::{internal_get_collection, internal_get_all_items, internal_get_item};
use crate::nft_ins::init_and_mint_nft;
use std::cell::RefCell;

pub fn collection_id ( name : String, symbol : String ) -> String {
    format!("{}-{}", name, symbol)
}

  
pub fn collection_exists( info: MessageInfo, name : String, symbol : String, deps: &DepsMut ) -> bool {

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

pub fn create_collection (deps: DepsMut, 
    _env : Env, info: MessageInfo,
    name : String, symbol : String, 
    description : Option<String> ,
    treasuries : Option<Vec<Treasury>>,
    attributes : Option<Vec<Attribute>>, 
    prices : Option<Vec<PriceType>>,
    _status : Option<u8>) -> Result<Response, ContractError> {
        
    internal_create_collection(deps, _env, info, name, symbol, description, treasuries, attributes, prices, _status )
   
}


pub fn update_collection(deps: DepsMut, 
    _env : Env, info: MessageInfo,
    name : String, symbol : String, 
    description : Option<String> ,
    treasuries : Option<Vec<Treasury>>,
    attributes : Option<Vec<Attribute>>, 
    prices : Option<Vec<PriceType>>,
    _status : Option<u8>, 
    ) -> Result<Response, ContractError> {
  
    let owner = info.clone().sender;

    if treasuries.is_some() {
        let _ = are_treasuries_valid(&treasuries)?;
    }
  
    let _key = (owner.clone(), collection_id(name.clone(), symbol.clone()) );
  
    let mut status = COLLECTION_STATUS_DRAFT;

    if _status.is_some() {
        let stat = _status.unwrap();
        if !is_status_valid(stat) {
            return Err(ContractError::InvalidCollectionStatus { text: 
            format!("Invalid status :{}!", stat ).to_string() } );
        }
        status = stat; 
    }
    
    let mut collection_to_update = internal_get_collection(deps.as_ref(), owner, name, symbol);

    let mut to_update : bool = false;

    if description.is_some() {
        collection_to_update.description = description;
        to_update = true; 
    }
    if treasuries.is_some() {
        collection_to_update.treasuries = treasuries;
        to_update = true; 
    }

    if prices.is_some() {
        collection_to_update.prices = prices;
        to_update = true; 
    }

    if attributes.is_some() {
        collection_to_update.attributes = attributes;
        to_update = true; 
    }

    if _status.is_some() {
        collection_to_update.status = status; 
        to_update = true; 
    }

    if to_update {
        collection_to_update.date_updated = _env.block.time;
    }

    if to_update {

        collections_store().save(deps.storage, _key.clone(), &collection_to_update)?;
        common_response(format!("{}-{}",_key.0, _key.1).as_str(), "update_collection", STATUS_OK, None)
    }
    else {
        common_response(format!("{}-{}",_key.0, _key.1).as_str(), "update_collection", 
        STATUS_ERROR, Some("Nothing updated!".to_string()))
    }
 
}




fn is_status_valid ( status : u8) -> bool {

    status == COLLECTION_STATUS_DRAFT ||
    status == COLLECTION_STATUS_ACTIVATED ||
    status == COLLECTION_STATUS_DEACTIVATED

}

fn are_treasuries_valid (treasuries : &Option<Vec<Treasury>>)  -> Result<bool, ContractError> {

    if treasuries.is_some () {

        let mut total_percentage = 0;

        let ts = treasuries.clone().unwrap();

        ts.iter().for_each(|t| total_percentage += t.percentage);

        if total_percentage > 100 || total_percentage < 100 {

            return Err(ContractError::CustomErrorMesg { message : 
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

pub (crate) fn internal_create_collection(deps: DepsMut, 
    _env : Env, info: MessageInfo,
    name : String, symbol : String, 
    description : Option<String> ,
    treasuries : Option<Vec<Treasury>>,
    attributes : Option<Vec<Attribute>>, 
    prices : Option<Vec<PriceType>>,
    _status : Option<u8>, 
    ) -> Result<Response, ContractError> {
  
    let owner = info.clone().sender;

    if collection_exists(info.clone(), name.clone(), symbol.clone(), &deps) {
        return Err(ContractError::CustomErrorMesg { message: format!("Collection {}-{} already exists!", name, symbol).to_string() } );
    }  
  
    let _ = are_treasuries_valid(&treasuries)?;

    let _key = (owner.clone(), collection_id(name.clone(), symbol.clone()) );

    let date_created = _env.block.time;
    
    let mut status = COLLECTION_STATUS_DRAFT;

    if _status.is_some() {
        let stat = _status.unwrap();
        if !is_status_valid(stat) {
            return Err(ContractError::InvalidCollectionStatus { text: 
                format!("Invalid status :{}!", stat ).to_string() } );
        }

        status = stat; 
    }
    
    let new_collection = Collection {
        name : name.clone(), 
        symbol : symbol.clone(),
        owner : owner, 
        treasuries : treasuries,
        attributes : attributes,
        prices : prices, 
        description : description,
        status : status,
        date_created : date_created,
        date_updated : date_created,
    };

    collections_store().save(deps.storage, _key.clone(), &new_collection)?;

    common_response(format!("{}-{}",_key.0, _key.1).as_str(), "create_collection", STATUS_OK, None)
}


pub fn item_exists( info: MessageInfo, 
    collection_name : String,
    collection_symbol : String,
    name : String, 
    deps: &DepsMut ) -> bool {

    let owner = info.clone().sender;
    
    let _key = (owner, collection_id(collection_name
        , collection_symbol), name );

    let stored_item = ITEMS_STORE.load(deps.storage,_key);
    
    stored_item.is_ok()
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

    let stored_item = ITEMS_STORE.key(_key.clone());

    let item_result = stored_item.may_load(deps.storage);
    
    match item_result {

        Ok(_)=> {ITEMS_STORE.remove(deps.storage, _key.clone()); true},

        Err(_)=> false , 
    }    

}


pub fn create_item(deps: DepsMut, 
    _env : Env, info: MessageInfo,item : Item 
) -> Result<Response, ContractError> {
  
    let mut item = item; 

    let owner = info.clone().sender;

    if !collection_exists(info.clone(), item.collection_name.clone() ,item.collection_symbol.clone(),&deps) {
        return Err(ContractError::CustomErrorMesg { message: format!("The collection '{}' does NOT exist!", 
        item.collection_name.clone()).to_string() } );
    }  


    if item_exists(info.clone(), item.collection_name.clone() ,
    item.collection_symbol.clone(),
    item.name.clone(), &deps) {
        return Err(ContractError::CustomErrorMesg { message: format!("The item {} in collection '{}' already exists!", 
        item.name.clone(),
        item.collection_name.clone()).to_string() } );
    }  
  
    let _key = (item.collection_owner.clone(), 
    collection_id(item.collection_name.clone(), item.collection_symbol.clone()), 
    item.name.clone() );

    let date_created = _env.block.time;
    
    item.collection_owner = owner;

    item.date_created = Some(date_created);

    item.date_updated = item.date_created;

    ITEMS_STORE.save(deps.storage, _key.clone(), &item)?;
    
    common_response( format!("{}-{}={}",_key.0, _key.1,
    _key.2).as_str(), "create_item", STATUS_OK, None)
}


pub fn mint_item (mut deps : DepsMut , 
    _env : Env, info: MessageInfo, index : i32,
    owner : Addr,collection_name : String,  
    collection_symbol : String , 
    price_type : Option<u8>, 
    token_uri : Option<String>)-> Result<Response, ContractError> {

    if index < 0 {
        return Err(ContractError::InvalidIndexOfNft { text : format!("Invalid index :{}", index)});
    }

    let collection = internal_get_collection(deps.as_ref(), owner.clone(), 
    collection_name.clone(), collection_symbol.clone());

    if collection.status != COLLECTION_STATUS_ACTIVATED{
        return Err(ContractError::NftStatusIsNotReadyForMinting { text: "Collection is NOT ready for minting!".to_string()});
    }

    let items = internal_get_all_items(deps.as_ref(), owner.clone(), collection_name.clone(), 
    collection_symbol.clone());

    let index = index as usize;

    if index < items.len() {

        let itm = items.get(index);
        if itm.is_some() {

            let i = itm.unwrap();

            let price = collection.price_by_type(price_type.unwrap_or(PRICE_TYPE_STANDARD));

            let res = init_and_mint_nft(deps.branch(), _env, info, i.clone(), collection.treasuries(), price, token_uri);

            if res.is_ok() {
                internal_remove_item(owner, collection_name, collection_symbol, i.name.clone(), deps);
            }
           
            res 
        }
        else {
            Err(ContractError::FailedToFindNft { text : format!("Failed to find item at index :{}", index)})
        }
    }
    else {
        Err(ContractError::NftIndexOutOfBound { text: format!("Item at index :{} out of bound", index)})
    }
       
}

pub fn mint_item_by_name (mut deps : DepsMut , 
    _env : Env, info: MessageInfo, item_name : String ,
    owner : Addr,collection_name : String,  
    collection_symbol : String , 
    price_type : Option<u8>, 
    token_uri : Option<String>)-> Result<Response, ContractError> {

    let collection = internal_get_collection(deps.as_ref(), owner.clone(), 
    collection_name.clone(), collection_symbol.clone());
    
    if collection.status != COLLECTION_STATUS_ACTIVATED{
        return Err(ContractError::CustomErrorMesg{message : "Collection is NOT ready for minting!".to_string()});
    }

    let item = internal_get_item(deps.as_ref(), owner.clone(), collection_name.clone(), 
    collection_symbol.clone(), item_name.clone());

    let price = collection.price_by_type(price_type.unwrap_or(PRICE_TYPE_STANDARD));

    if item.is_some() {
        let itm = item.unwrap();
        let res = init_and_mint_nft(deps.branch(), _env, info, itm.clone(), collection.treasuries(),price,token_uri);

        if res.is_ok() {
            internal_remove_item(owner, collection_name, collection_symbol, itm.name.clone(), deps);
        }

        res 
       
    }
    else {
        Err(ContractError::CustomErrorMesg{message : format!("Item named {} not found", item_name )})
    }
       
}



pub fn remove_collection (
    name : String,
    symbol : String,
    mut deps: DepsMut ,  
    info: MessageInfo) -> bool {
    
    let owner = info.clone().sender;

   
    let _key = (owner.clone(), collection_id(name.clone(), symbol.clone()) );

    let removed_res = collections_store().remove(deps.branch().storage, _key);
    
    match removed_res {

        Ok(_)=> {
            remove_all_items(owner, name, symbol, deps);
            true
        },

        Err(_)=> false , 
    }    

}

pub (crate) fn remove_all_items( 
    owner : Addr, 
    collection_name : String,
    collection_symbol : String, 
    deps: DepsMut) {

    let _prefix = (owner.clone(), collection_id(collection_name, collection_symbol));

    let deps2 = RefCell::new(deps);

    let mut keys : Vec<(Addr,String,String)> = Vec::new();

    {

        let borrowed_deps = deps2.borrow();

        let mut iter = ITEMS_STORE
            .prefix(_prefix)
            .range(borrowed_deps.storage, None, None, Order::Ascending)
            .into_iter();
    
      
        while let Some(itm_res) = iter.next() {
            if let Ok(itm) = itm_res {
    
                let _key = (owner.clone(), collection_id(itm.1.collection_name, 
                    itm.1.collection_symbol), itm.1.name);
                //println!("to.remove::Key::{:?}", _key);

                keys.push(_key);
                //ITEMS_STORE.remove(borrowed_mut_deps.storage, _key);
            }
        }
    
    }

    let mut  _borrowed_mut_deps = deps2.borrow_mut();
    
    for _key in keys.iter() {
        ITEMS_STORE.remove(_borrowed_mut_deps.storage, _key.clone());    
    }
    
}



#[allow(dead_code)]
const STATUS_ERROR : i8 = -1;

const STATUS_OK : i8 = 1;


pub (crate) fn common_response (key : &str , method : &str, status : i8,
message : Option<String>) -> Result<Response, ContractError> {

    if message.is_some() {

        let mesg = message.unwrap();
 
        Ok(Response::new()
        .add_attribute("key",key)
        .add_attribute("method", method)
        .add_attribute("status", format!("{}", status))
        .add_attribute("message", mesg))

    }
    else {

        Ok(Response::new()
        .add_attribute("key",key)
        .add_attribute("method", method)
        .add_attribute("status", format!("{}", status)))
    }
   
}