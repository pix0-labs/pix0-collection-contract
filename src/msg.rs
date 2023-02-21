use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::{Collection,Item, Treasury, Attribute, PriceType, User};
use cosmwasm_std::Addr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {

    pub name : String, 

    pub allowed_admins : Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {

    CreateUser {

        user_name : String, 

        first_name : Option<String>,

        last_name : Option<String>,
        
        email : Option<String>,

        mobile : Option<String>,
    },


 
    CreateCollection {
        name : String, 

        symbol : String, 

        description : Option<String>,

        treasuries : Option<Vec<Treasury>>,

        attributes : Option<Vec<Attribute>>,
    
        prices : Option<Vec<PriceType>>,

        status : Option<u8>, 

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


    GetUsers { 
        start_after : Option<String>,
        
        limit : Option<u32>,
    },

    UserExists {

        wallet_address : String, 
    },

    GetUser { },

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


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UsersResponse {

    pub users : Vec<User>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserExistsResponse {
    
    pub exists : bool ,
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserResponse {

    pub user : User,
}
