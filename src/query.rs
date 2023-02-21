use crate::msg::{CollectionResponse, CollectionsResponse};
use cosmwasm_std::{Deps, StdResult, Order, Addr};
use crate::state::Collection;
use crate::indexes::collections_store;
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
