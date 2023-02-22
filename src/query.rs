use crate::msg::{CollectionResponse, CollectionsResponse, UserExistsResponse, UsersResponse, 
    UserResponse};
use cosmwasm_std::{Deps, StdResult, Order, Addr };
use crate::state::{Collection, User, Item};
use crate::indexes::{collections_store, users_store, ITEMS_STORE};
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


fn user_exists_by( wallet_address : String , deps: Deps ) -> bool {

    let addr = deps.api.addr_validate(&wallet_address);

    match addr {

        Ok(owner) =>{

            let loaded_user = users_store().idx.owners.item(deps.storage, owner);

            let mut exists = false; 
        
            match loaded_user {
        
                Ok (u) => {
                    if u.is_some() {
                        exists = true
                    }
                },
        
                Err(_)=> exists = false, 
            }
        
            return exists;

        },

        Err(_) => false, 
    }
    
  
}




pub fn user_exists(  deps: Deps, wallet_address : String  ) -> StdResult<UserExistsResponse> {

    let exists = user_exists_by(wallet_address, deps);
    Ok(UserExistsResponse { exists: exists })
}

pub fn get_users(deps : Deps , start_after: Option<String>, limit: Option<u32>) 
->StdResult<UsersResponse> {

    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    
    let start = start_after.map(Bound::exclusive);
    
    let _users : StdResult <Vec<User>> = 
    users_store()
    .range(deps.storage, start, None, Order::Ascending)
    .take(limit)
    .map(|usr| {
        
        let (_k, u) = usr?;

        Ok(User {
            owner : u.owner,
            user_name : u.user_name,
            first_name : u.first_name,
            last_name : u.last_name, 
            email : u.email,
            mobile : u.mobile,
            date_created : u.date_created,
            date_updated : u.date_updated, 
        })
    }).collect();

    Ok(UsersResponse {
        users: _users?,
    })
}



pub fn get_user(deps : Deps, wallet_address : String  ) 
->StdResult<UserResponse> {

    let _key = wallet_address;

    let stored_user = users_store().key(_key.clone());
    
    let  user = stored_user.may_load(deps.storage).expect("Failed to find the user").expect(
        format!("Failed to unwrap user: key not found :\"{}\"", _key).as_str());
    
    Ok (UserResponse { user : user })

}



pub (crate) fn internal_get_items(deps : Deps , 
owner : Addr,collection_name : String,  
collection_symbol : String,    
start_after: Option<String>, limit: Option<u32>) 
->Vec<Item> {

    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    
    //let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

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
            attributes : i.attributes, links : i.links, background_color: i.background_color, 
            date_created: i.date_created, date_updated: i.date_updated }
        )
    }).collect();

    match items {

        Ok(itms)=> itms,

        Err(_) => Vec::new(),
    }
    
}
    
