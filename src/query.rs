use crate::msg::{CollectionResponse, CollectionsResponse, ItemCountResponse, ItemsResponse, ItemResponse};
use cosmwasm_std::{Deps, StdResult, Order, Addr };
use crate::state::{Collection, Item, COLLECTION_STATUS_ACTIVATED, ATTRB_CATEGORY};
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

    Ok(CollectionsResponse {
        collections: colls?,
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

    
    Ok(CollectionsResponse {
        collections: colls?,
    })
}


pub fn get_active_collections(deps : Deps,
    keyword : Option<String>,  
    category : Option<String>, 
    start: Option<u32>, limit: Option<u32>) 
    ->StdResult<CollectionsResponse> {
        
   // let start = start_after.map(Bound::exclusive);
   
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

        return Ok(CollectionsResponse {
            collections: vec![],
        })
    
    }

    let all_colls = all_colls.unwrap();

    let colls : Vec<Collection> = filter_collection_result(all_colls, keyword, category, start, limit);

    Ok(CollectionsResponse {
        collections: colls,
    })
    
}


pub (crate) fn collection_category(collection : Collection) -> String {

    let a = collection.attributes.unwrap_or(vec![])
    .into_iter()
    .find(|a|a .name == ATTRB_CATEGORY);

    if a.is_some(){
        a.unwrap().value
    }
    else {

        "".to_string()
    }
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
    limit: Option<u32> ) -> Vec<Collection>{

    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    let skip = start.unwrap_or(0) as usize ;

    if keyword.is_some() && category.is_some(){

        let kw = keyword.unwrap();

        all_colls.into_iter().filter(|c| 
            c.status == Some(COLLECTION_STATUS_ACTIVATED) 
            && contains_keyword(c.clone(), kw.clone()) 
            && is_category_of(c.clone(), category.clone().unwrap())
            )
            .skip(skip)
            .take(limit)
            .collect::<Vec<Collection>>()
    }

    else if keyword.is_some(){

        let kw = keyword.unwrap();

        all_colls.into_iter().filter(|c| 
            c.status == Some(COLLECTION_STATUS_ACTIVATED) 
            && contains_keyword(c.clone(), kw.clone()) )
            .skip(skip)
            .take(limit)
            .collect::<Vec<Collection>>()
    }
    else {

        all_colls.into_iter().filter(|c| c.status 
            == Some(COLLECTION_STATUS_ACTIVATED))
            .skip(skip)
            .take(limit)
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




