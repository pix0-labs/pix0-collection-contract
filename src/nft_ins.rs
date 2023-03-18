use cosmwasm_std::{Empty, DepsMut, MessageInfo, Env, Response, BankMsg, coins};
use crate::state::{Item, Collection, PRICE_TYPE_STANDARD};
use crate::error::ContractError;
//use std::convert::{TryFrom};
use crate::utils::nft_token_id;
use pix0_contract_common::funcs::{pay_by_percentage, to_bank_messages};

// refer to https://docs.opensea.io/docs/metadata-standards
pub type Metadata = crate::state::Metadata;

pub type Extension = Option<Metadata>;

pub type NftContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;



pub fn mint_nft(deps: DepsMut,  
    _env : Env, 
    info: MessageInfo, 
    contract :  NftContract,
    item : Item, 
    collection : Collection,
    price_type : Option<u8>,
    token_uri : Option<String>,
    method : Option<String>)-> Result<Response, ContractError>  {

    let new_owner = info.clone().sender;
   
    let ext_url = item.external_link();

    let token_id = nft_token_id(&(
    item.name.clone(), 
    item.collection_owner.to_string(),
    item.collection_name.clone(), 
    item.collection_symbol.clone()));

    let c_item = item.clone();

    let ext = Some(Metadata {
        description: item.description,
        name: Some(item.name) ,
        image : c_item.image_link(), 
        youtube_url : c_item.video_link(),
        animation_url : c_item.animation_link(), 
        external_url : ext_url.clone() ,
        attributes : Some(item.traits), 
        ..Metadata::default()
    });

    
    let msg = cw721_base::msg::MintMsg {
        token_id: token_id ,
        owner: new_owner.to_string(),
        token_uri: token_uri,
        extension: ext ,
    };

    let mint_msg = cw721_base::msg::ExecuteMsg::Mint(msg);

    let res = contract.execute(deps, _env.clone(), info, mint_msg);

    match res {

        Ok(_res) =>  {

           let mut prc_typ = PRICE_TYPE_STANDARD;

           if price_type.is_some() {
                prc_typ = price_type.unwrap();
           }

           let bank_msgs = pay_collection_treasuries(deps, _env, 
            info, collection, prc_typ);

            if bank_msgs.is_some() {

                Ok(Response::new().add_attribute("method", "mint-nft")
                .add_messages(bank_msgs.unwrap()))
    
            }
            else {
                Err(ContractError::FailedToMakePayment { text: "Failed to make payment when minting NFT".to_string()})
            }
           
        },

        Err(e) => Err(ContractError::CustomErrorMesg{message : e.to_string()}), 

    }

}



pub fn init_and_mint_nft(mut deps: DepsMut,  _env : Env, 
    info: MessageInfo, 
    item : Item, 
    collection : Collection, 
    price_type : Option<u8>,
    token_uri : Option<String>,
    method : Option<String>) -> Result<Response, ContractError>{

    let msg =  cw721_base::InstantiateMsg {
        name: item.collection_name.clone(),
        symbol: item.collection_symbol.clone(),
        minter: String::from(info.sender.clone()),
    };

    let contract = NftContract::default();

    let _res = contract.instantiate(deps.branch(), _env.clone(), info.clone(),msg);
    
    mint_nft(deps, _env, info, contract, item,collection, price_type, token_uri, method)
    
}

pub fn pay_collection_treasuries (
deps: DepsMut,  _env : Env, 
info: MessageInfo,     
collection : Collection, price_type : u8 ) -> Option<Vec<BankMsg>>{

    let payments = collection.treasuries_to_payments();
    let price = collection.price_by_type( price_type);

    if price.is_some() {

        let amts = 
        pay_by_percentage(deps, info, _env.block.time, payments, price.unwrap());

        let bank_msgs = to_bank_messages(amts);

        bank_msgs
    }
    else {

        None
    }

}

pub const DEFAULT_PRICE_DENOM : &str = "uconst";

fn pay_treasury (wallet_address : &str, amount : u128 , _denom : Option <String>)
-> Result<Response, ContractError>{

    let mut denom = String::from(DEFAULT_PRICE_DENOM);

    if _denom.is_some() {
        denom = _denom.unwrap_or( String::from( DEFAULT_PRICE_DENOM) );
    }

    println!("Going to pay :{},:{}", wallet_address, amount);

    let coin = coins(amount, denom);
    let bank_mesg = BankMsg::Send {
        to_address: String::from(wallet_address),
        amount: coin, 
    };

   
    Ok(Response::new().add_attribute("action", "approve").add_message(bank_mesg))

}
