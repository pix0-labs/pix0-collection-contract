use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Timestamp, Coin};
use pix0_contract_common::state::PaymentByPercentage;
use pix0_market_handlers::state::Trait;
use pix0_market_handlers::state::Royalty;
use pix0_market_handlers::state::SimpleCollectionInfo;

pub const PRICE_TYPE_STANDARD : u8 = 1;

pub const PRICE_TYPE_WL : u8 = 2;

pub const PRICE_TYPE_OG : u8 = 3;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceType {

    pub price_type : u8, 

    pub value : Coin, 

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


    pub fn treasuries_to_payments(&self) -> Vec<PaymentByPercentage>{

        let treas = self.treasuries();
        let mut payments : Vec<PaymentByPercentage> = Vec::new();

        treas.iter()
        .for_each(|t| {
            payments.push( PaymentByPercentage { wallet: t.clone().wallet, 
                percentage: t.percentage });    
        });

        payments
    }
}

pub const ATTRB_ALLOWED_MINT_ITEM_BY_NAME : &str = "ALLOWED_MINT_ITEM_BY_NAME";

pub const ATTRB_CATEGORY : &str = "CATEGORY";

pub const ATTRB_MINT_CAP : &str = "MINT_CAP";

impl Collection {

    pub fn price_by_type (&self,  _type : u8) -> Option<Coin> {

        if self.prices.is_some() {

            let prc = self.prices.clone().unwrap();

            let prcs : Vec<PriceType> = prc.into_iter()
            .filter(|p| p.price_type  ==  _type)
            .collect();

            let prc_type = prcs.first().unwrap();

            Some(prc_type.clone().value)


        }   
        else {
            None 
        }
    }


    pub fn is_mint_by_name_allowed(&self) -> bool {

        if self.attributes.is_some() {

            let attbs = self.attributes.clone().unwrap();

            let m : Vec<Attribute>= 
            attbs.into_iter()
            .filter(|i| i.name == ATTRB_ALLOWED_MINT_ITEM_BY_NAME.to_string())
            .collect();

            if m.len() > 0 {
                m.first().unwrap().value == "true"
            }
            else {

                false
            }
        }
        else {

            false 
        }
    }



    pub fn category(&self) -> Option<String> {

        let a = self.attributes.clone().unwrap_or(vec![])
        .into_iter()
        .find(|a|a .name == ATTRB_CATEGORY);

        if a.is_some(){
            Some(a.unwrap().value)

        }
        else {

            None 
        }
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

impl Item {


    fn add_to_trait_if_not_exist ( traits : &mut Vec<Trait>, 
    trait_type : String,
    display_type : String,
    value : String ) {

        if traits 
        .iter()
        .find(|t| t.trait_type == trait_type)
        .is_none() {

            traits.push( Trait {
                trait_type : trait_type.clone(),
                display_type : Some(display_type),
                value : value 
            })
        }
    }


    pub fn add_simple_collection_info_to_traits(&self, category : Option<String>,
    royalties : Option<Vec<Royalty>>) -> Vec<Trait> {

        let mut trs = self.traits.clone();

        let sinfo = SimpleCollectionInfo {

            owner : self.collection_owner.clone(),
            collection_name : self.collection_name.clone(),
            collection_symbol : self.collection_symbol.clone(),
            category : category,
            royalties : royalties, 

        };

        Self::add_to_trait_if_not_exist(&mut trs, String::from("collection-info"), 
        String::from("Collection Info"), serde_json::to_string(&sinfo).unwrap_or("".to_string()));

        return trs;

    }
}



