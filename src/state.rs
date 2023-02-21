use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Timestamp};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractInfo {

    pub name : String, 

    pub allowed_admins : Vec<String>,

    pub date_instantiated : Timestamp,

}

// Implement the `Display` trait for the `Person` struct
impl std::fmt::Display for ContractInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}\nAllowed Admins: {:?}\nDate Instantiated: {}",
            self.name, self.allowed_admins, self.date_instantiated
        )
    }
}

impl Into<String> for ContractInfo {
    fn into(self) -> String {
        format!("Name: {}\nAllowed Admins: {:?}\nDate Instantiated: {}", self.name, self.allowed_admins, 
        self.date_instantiated)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct User {
   
    pub owner: Addr, // the user's wallet address

    pub user_name : String,

    pub first_name : Option<String>,

    pub last_name : Option<String>,

    pub email : Option<String>,

    pub mobile : Option<String>,

    pub date_created : Option<Timestamp>,

    pub date_updated : Option<Timestamp>,

}

impl User {

    pub fn new (owner : Addr, user_name : String, 
    first_name : Option<String>, last_name : Option<String>,     
    email : Option<String>, mobile : Option<String>,
    date_created : Timestamp) -> User {

        User {
            owner : owner,
            user_name : user_name,
            first_name : first_name,
            last_name : last_name, 
            email : email,
            mobile : mobile,
            date_created : Some(date_created),
            date_updated : Some(date_created),
        }
    }
}


pub const PRICE_TYPE_STANDARD : u8 = 1;

pub const PRICE_TYPE_WL : u8 = 2;

pub const PRICE_TYPE_OG : u8 = 3;




#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceType {

    pub price_type : u8, 

    pub value : u32, 

    pub denom : Option<String>, 

    pub date_start : Timestamp,

    pub date_end : Timestamp, 
}

pub const COLLECTION_STATUS_NEW : u8 = 0;

pub const COLLECTION_STATUS_ACTIVE : u8 = 1;

pub const COLLECTION_STATUS_DEACTIVATED : u8 = 2;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Attribute {

    pub name : String, 

    pub value : String, 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Collection {
   
    pub owner : Addr,

    pub name : String,

    pub symbol : String, 

    pub description : Option<String>,

    pub treasuries: Option<Vec<Treasury>>,

    pub attributes : Option<Vec<Attribute>>,

    pub prices : Option<Vec<PriceType>>,

    pub status : u8, 

    pub date_created : Timestamp,

    pub date_updated : Timestamp,

}


impl Collection {

    pub fn treasuries(&self) -> Vec<Treasury> {

        if self.treasuries.is_some() {

            let t = self.treasuries.clone().unwrap();
            return t;
        }

        return vec![Treasury{wallet: self.owner.clone(), percentage :100, name : None}];
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Treasury {

    pub wallet : Addr, 

    pub percentage : u8, 

    pub name : Option<String>,
}


pub const LINK_TYPE_IMAGE_URL : u8 = 1;
 
pub const LINK_TYPE_EXTERNAL_LINK : u8 = 2;

pub const LINK_TYPE_VIDEO_URL : u8 = 3;

pub const LINK_TYPE_ANIMATION_URL : u8 = 4;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Link {

    pub link_type : u8,

    pub value : String, 
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Item {
   
    pub collection_owner : Addr,

    pub collection_name : String, 

    pub collection_symbol : String, 
    
    pub name : String,

    pub description : Option<String>,

    pub links : Vec<Link>,

    pub attributes: Vec<Trait>,

    pub background_color : Option<String>,

    pub date_created : Option<Timestamp>,

    pub date_updated : Option<Timestamp>,

}


impl Item {

    pub fn link_by_type (&self, link_type : u8) -> Vec<Link> {

        let links = 
        self.links
        .clone()
        .into_iter()
        .filter(|l| l.link_type == link_type)
        .collect();

        return links;
    }


    fn link_value (&self, link_type : u8) ->Option<String>{

        let links = self.link_by_type(link_type);
        
        if links.first().is_some() {
            let l = links.first().unwrap();
            return Some(l.value.clone());
        }
        return None;
    }

    pub fn image_link(&self) -> Option<String> {

        self.link_value(LINK_TYPE_IMAGE_URL)
    }

    pub fn video_link(&self) -> Option<String> {

        self.link_value(LINK_TYPE_VIDEO_URL)
    }


    pub fn animation_link(&self) -> Option<String> {

        self.link_value(LINK_TYPE_ANIMATION_URL)
    }

    pub fn external_link(&self) -> Option<String> {

        self.link_value(LINK_TYPE_EXTERNAL_LINK)
    }


}


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}


// see: https://docs.opensea.io/docs/metadata-standards
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Metadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}