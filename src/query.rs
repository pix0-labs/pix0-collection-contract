use std::convert::TryInto;

use crate::msg::{CollectionResponse, CollectionsResponse, ItemCountResponse, ItemsResponse, ItemResponse, 
    CollectionsWithParamsResponse};
use cosmwasm_std::{Deps, StdResult, Order, Addr};
use crate::state::{Collection, Item, COLLECTION_STATUS_ACTIVATED};
use crate::indexes::{collections_store, COLLECTION_ITEMS_STORE};
use cw_storage_plus::Bound;
use crate::ins::collection_id;

pub const DEFAULT_LIMIT : u32 = 10;

pub const MAX_LIMIT : u32 = 20;

pub fn get_collection(deps: Deps, owner : Addr, name : String, symbol : String  ) -> StdResult<CollectionResponse>{

    Ok (CollectionResponse { collection : internal_get_collection(deps, owner, name, symbol) })
}


pub (crate) fn internal_get_collection(deps: Deps, owner : Addr, name : String, symbol : String  ) -> Option<Collection>{

    let _key = (owner, collection_id(name, symbol) );

    let stored_collection = collections_store().key(_key.clone());
    
    let res = stored_collection.may_load(deps.storage);

    if res.is_ok() {

        let value = res.unwrap_or_else(|_| {
            None
        });

        value 
    }
    else {

        None
    }
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
            prices : c.prices, status: c.status, royalties: c.royalties,
            symbol : c.symbol, description: c.description, 
            date_created: c.date_created, date_updated: c.date_updated }
        )
    }).collect();

    let mut collections : Vec<Collection> = vec![];

    if colls.is_ok() {
        collections = colls.ok().unwrap();
        collections.sort_by(|a, b| b.date_updated.cmp(&a.date_updated));
    }

    Ok(CollectionsResponse {
        collections: collections,
    })
}



pub fn get_all_collections(deps : Deps, start_after: Option<String>, limit: Option<u32>) 
    ->StdResult<CollectionsResponse> {
        
    
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
  
    let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));
  
    let colls : StdResult <Vec<Collection>> = 
   
   
    collections_store().idx.collections
    .range(deps.storage, start, None, Order::Ascending)
    .take(limit)
    .map(|col| {
        
        let (_k, c) = col?;

        Ok(
            Collection { owner : c.owner, name : c.name, 
                treasuries : c.treasuries,
                attributes : c.attributes, royalties: c.royalties,
                prices : c.prices, status: c.status, 
                symbol : c.symbol, description: c.description, 
                date_created: c.date_created, date_updated: c.date_updated }
        )
    }).collect();

    let mut collections : Vec<Collection> = vec![];

    if colls.is_ok() {
        collections = colls.ok().unwrap();
        collections.sort_by(|a, b| b.date_updated.cmp(&a.date_updated));
    }
    
    Ok(CollectionsResponse {
        collections: collections
    })
}


pub fn get_active_collections(deps : Deps,
    keyword : Option<String>,  
    category : Option<String>, 
    start: Option<u32>, limit: Option<u32>) 
    ->StdResult<CollectionsWithParamsResponse> {    
   
    let all_colls : StdResult<Vec<Collection>> = 
    
    collections_store().idx.collections
    .range(deps.storage, None, None, Order::Ascending)
    .map(|col| {
        
        let (_k, c) = col?;
        Ok (
            Collection { owner : c.owner, name : c.name, 
            treasuries : c.treasuries,
            attributes : c.attributes, royalties: c.royalties,
            prices : c.prices, status: c.status, 
            symbol : c.symbol, description: c.description, 
            date_created: c.date_created, date_updated: c.date_updated }
        )
    }).collect();


    if all_colls.is_err() {

        return Ok(CollectionsWithParamsResponse::empty_response())
    
    }

    let mut all_colls = all_colls.unwrap();

    all_colls.sort_by(|a, b| b.date_created.cmp(&a.date_created));
    
    
    let res : (Vec<Collection>,usize) = filter_collection_result(all_colls, keyword, category, start, limit);

    Ok(CollectionsWithParamsResponse {
        collections: res.0,
        total : Some(res.1.try_into().unwrap_or(0)),
        start : start,
        limit : limit
    })
    
}


pub (crate) fn collection_category(collection : Collection) -> String {

    collection.category().unwrap_or(String::from(""))
}

fn is_category_of(collection : Collection, category : String) -> bool {

    let cat = collection_category(collection);

    return cat == category;
}


fn contains_keyword(collection : Collection, keyword : String) -> bool {

    return collection.name.contains(&keyword) ||
    collection.description.clone().unwrap_or("".to_string()).contains(&keyword);

}


fn filter_collection_result(all_colls : Vec<Collection>, 
    keyword : Option<String>, 
    category : Option<String>, 
    start : Option<u32>,
    limit: Option<u32>) -> (Vec<Collection>,usize){

    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    let skip = start.unwrap_or(0) as usize ;
    
    let res = filter_collection_result_all(all_colls, keyword, category);

    (res.clone()
    .into_iter()
    .skip(skip)
    .take(limit)
    .collect::<Vec<Collection>>(), res.len())
}

fn filter_collection_result_all(all_colls : Vec<Collection>, 
    keyword : Option<String>, 
    category : Option<String>) -> Vec<Collection>{

   
    if keyword.is_some() && category.is_some(){

        let kw = keyword.unwrap();

        all_colls.into_iter().filter(|c| 
            c.status == Some(COLLECTION_STATUS_ACTIVATED) 
            && contains_keyword(c.clone(), kw.clone()) 
            && is_category_of(c.clone(), category.clone().unwrap())
            )
            .collect::<Vec<Collection>>()
    }

    else if keyword.is_some(){

        let kw = keyword.unwrap();

        all_colls.into_iter().filter(|c| 
            c.status == Some(COLLECTION_STATUS_ACTIVATED) 
            && contains_keyword(c.clone(), kw.clone()) )
            .collect::<Vec<Collection>>()
    }
    else {

        all_colls.into_iter().filter(|c| c.status 
            == Some(COLLECTION_STATUS_ACTIVATED))
          .collect::<Vec<Collection>>()
            
    }


}


pub (crate) fn internal_get_item(deps : Deps , 
    owner : Addr,collection_name : String,  
    collection_symbol : String, item_name : String) ->Option<Item> {

    let _key = (owner, 
    collection_id(collection_name, collection_symbol), 
    item_name.clone());

    let stored_item = COLLECTION_ITEMS_STORE.key(_key.clone());
    
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
    COLLECTION_ITEMS_STORE
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
        COLLECTION_ITEMS_STORE
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
    
    COLLECTION_ITEMS_STORE
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


