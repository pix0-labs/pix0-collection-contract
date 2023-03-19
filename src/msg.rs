use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::{Collection,Item};
use cosmwasm_std::Addr;
use pix0_contract_common::state::Fee;


pub struct InstantiateMsg {

    pub allowed_admins : Option<Vec<Addr>>,
    
    pub treasuries : Option<Vec<Addr>>,

    pub fees : Option<Vec<Fee>>, 

    pub log_last_payment : Option<bool>, 

}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {
    pub message : String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {

    UpdateContractInfo {

        fees : Option<Vec<Fee>>, 

        treasuries : Option<Vec<Addr>>,

        log_last_payment : Option<bool>, 
    },

    CreateCollection {
       
       collection : Collection, 

    },

    UpdateCollection {
       
       collection : Collection, 

    },


    RemoveCollection {
        name : String, 

        symbol : String, 
    },

    CreateItem {

       item : Item, 
    },


    MintItem {

        seed : String,

        owner : Addr, 

        collection_name : String, 

        collection_symbol : String, 

        price_type : Option<u8>, 
        
        token_uri : Option<String>, 

    },

    MintItemByName {
        
        name : String,

        owner : Addr, 

        collection_name : String, 

        collection_symbol : String, 

        price_type : Option<u8>, 

        token_uri : Option<String>, 

    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    
    GetCollection { 
        owner : Addr,

        name : String,

        symbol : String, 
    },

    GetCollections { 

        owner : Addr, 
        
        start_after : Option<String>,
        
        limit : Option<u32>,
    },

    GetAllCollections { 
        
        limit : Option<u32>,
    },


    GetItem { 

        owner : Addr, 
        
        collection_name : String, 

        collection_symbol : String, 

        item_name : String, 
      
    },
    
    GetItems { 

        owner : Addr, 
        
        collection_name : String, 

        collection_symbol : String, 
      
        start_after : Option<String>,
        
        limit : Option<u32>,
  
    },

    GetItemsCount { 

        owner : Addr, 
        
        collection_name : String, 

        collection_symbol : String, 
    },



    MintedTokensByOwner {

        owner : String,

        start_after : Option<String>,
        
        limit : Option<u32>,
    },

    NftTokenInfo {

        token_id : String, 
    },

    GetContractInfo{},
     
    GetLogInfo{},
    
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionResponse {
    
    pub collection : Option<Collection>,
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionsResponse {

    pub collections : Vec<Collection>,
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ItemResponse {
    
    pub item : Option<Item>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ItemsResponse {
    
    pub items : Vec<Item>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ItemCountResponse {
    pub count : usize,
}
