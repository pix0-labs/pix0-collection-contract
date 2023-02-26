use crate::msg::{CollectionResponse, CollectionsResponse, ItemCountResponse, ItemsResponse, ItemResponse};
use cosmwasm_std::{Deps, StdResult, Order, Addr };
use crate::state::{Collection, Item};
use crate::indexes::{collections_store, ITEMS_STORE};
use cw_storage_plus::Bound;
use crate::ins::collection_id;

pub const DEFAULT_LIMIT : u32 = 10;

pub const MAX_LIMIT : u32 = 20;

pub fn get_collection(deps: Deps, owner : Addr, name : String, symbol : String  ) -> StdResult<CollectionResponse>{

    Ok (CollectionResponse { collection : internal_get_collection(deps, owner, name, symbol) })
}


pub (crate) fn internal_get_collection(deps: Deps, owner : Addr, name : String, symbol : String  ) -> Collection{

    let _key = (owner, collection_id(name, symbol) );

    let stored_user = collections_store().key(_key.clone());
    
    stored_user.may_load(deps.storage).expect("Failed to find the collection").expect(
        format!("Failed to unwrap, key not found :\"{:?}\"", _key).as_str())
        
}


pub fn get_collections(deps : Deps , 
owner : Addr,     
start_after: Option<String>, limit: Option<u32>) 
->StdResult<CollectionsResponse> {

    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    
    //let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

    let start = start_after.map(Bound::exclusive);
    
    let colls : StdResult <Vec<Collection>> = 
    collections_store()
    .idx.collections
    .prefix(owner)
    .range(deps.storage, start, None, Order::Ascending)
    .take(limit)
    .map(|col| {
        
        let (_k, c) = col?;

        Ok(
            Collection { owner : c.owner, name : c.name, 
            treasuries : c.treasuries,
            attributes : c.attributes,
            prices : c.prices, status: c.status, 
            symbol : c.symbol, description: c.description, 
            date_created: c.date_created, date_updated: c.date_updated }
        )
    }).collect();

    Ok(CollectionsResponse {
        collections: colls?,
    })
}


pub fn get_all_collections(deps : Deps, limit: Option<u32>) 
    ->StdResult<CollectionsResponse> {
        
    
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    let colls : StdResult <Vec<Collection>> = 
   
    collections_store().idx.collections
    .range(deps.storage, None, None, Order::Ascending)
    .take(limit)
    .map(|col| {
        
        let (_k, c) = col?;

        Ok(
            Collection { owner : c.owner, name : c.name, 
                treasuries : c.treasuries,
                attributes : c.attributes,
                prices : c.prices, status: c.status, 
                symbol : c.symbol, description: c.description, 
                date_created: c.date_created, date_updated: c.date_updated }
        )
    }).collect();

    
    Ok(CollectionsResponse {
        collections: colls?,
    })
}




pub (crate) fn internal_get_item(deps : Deps , 
    owner : Addr,collection_name : String,  
    collection_symbol : String, item_name : String) ->Option<Item> {

    let _key = (owner, 
    collection_id(collection_name, collection_symbol), 
    item_name.clone());

    let stored_item = ITEMS_STORE.key(_key.clone());
    
    let item_result = stored_item.may_load(deps.storage);
    
    match item_result {

        Ok(i)=> i,

        Err(_)=> None, 
    }
}

pub (crate) fn internal_get_items(deps : Deps , 
owner : Addr,collection_name : String,  
collection_symbol : String,    
start_after: Option<String>, limit: Option<u32>) 
->Vec<Item> {

    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    
    let start = start_after.map(Bound::exclusive);
    
    let _prefix = (owner, collection_id(collection_name
        , collection_symbol) );
    
    let items : StdResult <Vec<Item>> = 
    ITEMS_STORE
    .prefix(_prefix)
    .range(deps.storage, start, None, Order::Ascending)
    .take(limit)
    .map(|itm| {
        
        let (_k, i) = itm?;

        Ok(
            Item { collection_owner : i.collection_owner, name : i.name, 
            collection_name : i.collection_name, 
            collection_symbol : i.collection_symbol, 
            description: i.description, 
            traits : i.traits, links : i.links, background_color: i.background_color, 
            date_created: i.date_created, date_updated: i.date_updated }
        )
    }).collect();

    match items {

        Ok(itms)=> itms,

        Err(_) => Vec::new(),
    }
    
}
    

pub (crate) fn internal_get_all_items(deps : Deps , 
    owner : Addr,collection_name : String,  
    collection_symbol : String) 
    ->Vec<Item> {
    
        let _prefix = (owner, collection_id(collection_name
            , collection_symbol) );
        
        let items : StdResult <Vec<Item>> = 
        ITEMS_STORE
        .prefix(_prefix)
        .range(deps.storage, None, None, Order::Ascending)
        .map(|itm| {
            
            let (_k, i) = itm?;
    
            Ok(
                Item { collection_owner : i.collection_owner, name : i.name, 
                collection_name : i.collection_name, 
                collection_symbol : i.collection_symbol, 
                description: i.description, 
                traits : i.traits, links : i.links, background_color: i.background_color, 
                date_created: i.date_created, date_updated: i.date_updated }
            )
        }).collect();
    
        match items {
    
            Ok(itms)=> itms,
    
            Err(_) => Vec::new(),
        }
        
}
        
    

pub (crate) fn internal_get_items_count(deps : Deps , owner : Addr,
    collection_name : String,collection_symbol : String)
    ->usize {
    
    let _prefix = (owner, collection_id(collection_name, collection_symbol));
    
    ITEMS_STORE
    .prefix(_prefix)
    .range(deps.storage, None, None, Order::Ascending)
    .count()
          
}


pub fn get_items_count(deps : Deps , owner : Addr,
    collection_name : String,collection_symbol : String)
    ->StdResult<ItemCountResponse>{

    let items_count = internal_get_items_count(deps, owner, collection_name, collection_symbol);
    
    Ok(ItemCountResponse {
        count : items_count ,
    })
}


pub fn get_items(deps : Deps , 
    owner : Addr,collection_name : String,  
    collection_symbol : String,    
    start_after: Option<String>, limit: Option<u32>) -> StdResult<ItemsResponse>  {

    Ok ( ItemsResponse {
        items : internal_get_items(deps, owner, collection_name, collection_symbol, start_after, limit)
    })
}



pub fn get_item(deps : Deps , 
    owner : Addr,collection_name : String,  
    collection_symbol : String, item_name : String) -> StdResult<ItemResponse>{
    
    Ok ( ItemResponse {
        item : internal_get_item(deps, owner, collection_name, collection_symbol, item_name)
    })

}




