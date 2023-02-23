use cosmwasm_std::{DepsMut, Env, Response, MessageInfo, Addr};
use crate::state::{Collection, Treasury, Attribute, PriceType, Item, COLLECTION_STATUS_DRAFT,
COLLECTION_STATUS_ACTIVATED, COLLECTION_STATUS_DEACTIVATED, PRICE_TYPE_STANDARD};
use crate::indexes::{collections_store,ITEMS_STORE };
use crate::error::ContractError;
use crate::query::{internal_get_collection, internal_get_items, internal_get_item};
use crate::nft_ins::init_and_mint_nft;

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



fn is_status_valid ( status : u8) -> bool {

    status == COLLECTION_STATUS_DRAFT ||
    status == COLLECTION_STATUS_ACTIVATED ||
    status == COLLECTION_STATUS_DEACTIVATED

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
  
    let _key = (owner.clone(), collection_id(name.clone(), symbol.clone()) );

    let date_created = _env.block.time;
    
    let mut status = COLLECTION_STATUS_DRAFT;

    if _status.is_some() {
        let stat = _status.unwrap();
        if !is_status_valid(stat) {
            return Err(ContractError::CustomErrorMesg { message: 
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


pub fn mint_item (deps : DepsMut , 
    _env : Env, info: MessageInfo, index : i32,
    owner : Addr,collection_name : String,  
    collection_symbol : String , 
    price_type : Option<u8>, 
    token_uri : Option<String>)-> Result<Response, ContractError> {

    if index < 0 {
        return Err(ContractError::CustomErrorMesg{message : format!("Invalid index :{}", index)});
    }

    let collection = internal_get_collection(deps.as_ref(), owner.clone(), 
    collection_name.clone(), collection_symbol.clone());

    if collection.status != COLLECTION_STATUS_ACTIVATED{
        return Err(ContractError::CustomErrorMesg{message : "Collection is NOT ready for minting!".to_string()});
    }

    let items = internal_get_items(deps.as_ref(), owner, collection_name, 
    collection_symbol, None, None);

    let index = index as usize;

    if index < items.len() {

        let itm = items.get(index);
        if itm.is_some() {

            let i = itm.unwrap();

            let price = collection.price_by_type(price_type.unwrap_or(PRICE_TYPE_STANDARD));

            init_and_mint_nft(deps, _env, info, i.clone(), collection.treasuries(), price, token_uri)
        }
        else {
            Err(ContractError::CustomErrorMesg{message : format!("Failed to find item at index :{}", index)})
        }
    }
    else {
        Err(ContractError::CustomErrorMesg{message : format!("Item at index :{} out of bound", index)})
    }
       
}

pub fn mint_item_by_name (deps : DepsMut , 
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

    let item = internal_get_item(deps.as_ref(), owner, collection_name, 
    collection_symbol, item_name.clone());

    let price = collection.price_by_type(price_type.unwrap_or(PRICE_TYPE_STANDARD));

    if item.is_some() {
        let itm = item.unwrap();
        init_and_mint_nft(deps, _env, info, itm.clone(), collection.treasuries(),price,token_uri)
    }
    else {
        Err(ContractError::CustomErrorMesg{message : format!("Item named {} not found", item_name )})
    }
       
}


#[allow(dead_code)]
const STATUS_ERROR : i8 = -1;

const STATUS_OK : i8 = 1;


pub (crate) fn common_response (key : &str , method : &str, status : i8,
message : Option<String>) -> Result<Response, ContractError> {

    if message.is_some() {

        let mesg = message.unwrap();

        println!("at@common_response:{}",mesg);
        
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