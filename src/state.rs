use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Timestamp};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractInfo {

    pub name : String, 

    pub allowed_admins : Option<Vec<String>>,

    pub date_instantiated : Timestamp,

}


impl Into<String> for ContractInfo {
    fn into(self) -> String {
        format!("Name: {}\nAllowed Admins: {:?}\nDate Instantiated: {}", self.name, self.allowed_admins, 
        self.date_instantiated)
    }
}


pub const PRICE_TYPE_STANDARD : u8 = 1;

pub const PRICE_TYPE_WL : u8 = 2;

pub const PRICE_TYPE_OG : u8 = 3;




#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceType {

    pub price_type : u8, 

    pub value : u64, 

    pub denom : Option<String>, 

    pub date_start : Option<Timestamp>,

    pub date_end : Option<Timestamp>, 
}

pub const COLLECTION_STATUS_DRAFT : u8 = 0;

pub const COLLECTION_STATUS_ACTIVATED : u8 = 1;

pub const COLLECTION_STATUS_DEACTIVATED : u8 = 2;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Attribute {

    pub name : String, 

    pub value : String, 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Collection {
   
    pub owner : Option<Addr>,

    pub name : String,

    pub symbol : String, 

    pub description : Option<String>,

    pub treasuries: Option<Vec<Treasury>>,

    pub attributes : Option<Vec<Attribute>>,

    pub prices : Option<Vec<PriceType>>,

    pub royalties : Option<Vec<Royalty>>,

    pub status : Option<u8>, 

    pub date_created : Option<Timestamp>,

    pub date_updated : Option<Timestamp>,

}


impl Collection {

    pub fn treasuries(&self) -> Vec<Treasury> {

        if self.treasuries.is_some() {

            let t = self.treasuries.clone().unwrap();
            return t;
        }
        if self.owner.is_some() {
            return vec![Treasury{wallet: self.owner.clone().unwrap().clone(), percentage :100, name : None}];
        }
        else {

            return vec![Treasury{wallet: Addr::unchecked("unknown"), percentage :0, name : None}];
            
        }
    }
}


impl Collection {

    pub fn price_by_type (&self,  _type : u8) -> Option<u64> {

        if self.prices.is_some() {

            let prc = self.prices.clone().unwrap();

            let prcs : Vec<PriceType> = prc.into_iter()
            .filter(|p| p.price_type  ==  _type)
            .collect();

            Some(prcs.first().unwrap().value)


        }   
        else {
            None 
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Royalty{

    pub wallet : Addr, 

    pub percentage : u8, 

    pub name : Option<String>,
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

    pub traits : Vec<Trait>,

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