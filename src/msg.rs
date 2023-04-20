use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::{Collection,Item};
use cosmwasm_std::{Addr, Coin};
use pix0_contract_common::state::{Fee, Contract};
use pix0_market_handlers::state::SellOffer;
use cw721::Cw721ReceiveMsg;

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

        contracts : Option<Vec<Contract>>, 

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

        token_id : Option<String>,

    },

    MintItemByName {
        
        name : String,

        owner : Addr, 

        collection_name : String, 

        collection_symbol : String, 

        price_type : Option<u8>, 

        token_uri : Option<String>, 

        token_id : Option<String>,

    },

    SimpleMint {

        item : Item , 

        token_uri : Option<String>, 

        token_id : Option<String>,

    },
   
    TransferNft {

        recipient : String ,

        token_id : String, 
    }, 

    BurnNft { token_id: String },

    SendNft {
        token_id : String, 
   
        contract_addr : String,

        action : String , 
    },

    ReceiveNft(Cw721ReceiveMsg),

    CreateSellOffer {
        
        offer : SellOffer,

        create_sell_offer_fee : Option<Coin>,
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
        
        start_after : Option<String>,
        
        limit : Option<u32>,
    },

    GetActiveCollections { 
        
        keyword : Option<String>,
        
        category : Option<String>,
        
        start : Option<u32>,
        
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


    NumTokens {},
    
    OwnerOf {
        token_id: String,
        include_expired: Option<bool>,
    },

    NftInfo {
        token_id: String,
    },

    AllNftInfo {
        token_id: String,
        include_expired: Option<bool>,
    },
    
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
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
pub struct CollectionsWithParamsResponse {

    pub collections : Vec<Collection>,

    pub total : Option<u32>,

    pub start : Option<u32>,

    pub limit : Option<u32>,
}

impl CollectionsWithParamsResponse {

    pub fn empty_response() -> Self {

        CollectionsWithParamsResponse {
            collections: vec![],
            total : None,
            start : None,
            limit : None, 
        }
    }
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
