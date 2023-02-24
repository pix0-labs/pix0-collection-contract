use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::{Collection,Item, Treasury, Attribute, PriceType};
use cosmwasm_std::Addr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {

    pub name : String, 

    pub allowed_admins : Option<Vec<String>>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {
    pub message : String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {

    CreateCollection {
        name : String, 

        symbol : String, 

        description : Option<String>,

        treasuries : Option<Vec<Treasury>>,

        attributes : Option<Vec<Attribute>>,
    
        prices : Option<Vec<PriceType>>,

        status : Option<u8>, 

    },

    CreateItem {

       item : Item, 
    
    },
   
   
    RandomMintItem {

        owner : Addr, 

        collection_name : String, 

        collection_symbol : String, 

        price_type : Option<u8>, 
        
        token_uri : Option<String>, 

    },

    
    MintItem {

        index : String,

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

    MintedTokensByOwner {

        owner : String,

        start_after : Option<String>,
        
        limit : Option<u32>,
    },

    NftTokenInfo {

        token_id : String, 
    }
    
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionResponse {
    
    pub collection : Collection,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionsResponse {

    pub collections : Vec<Collection>,
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ItemResponse {
    
    pub item : Item,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ItemsResponse {
    
    pub items : Vec<Item>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ItemCountResponse {
    pub count : usize,
}
