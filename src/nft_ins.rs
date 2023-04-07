use cosmwasm_std::{DepsMut, MessageInfo, Env, Response, BankMsg, Binary };
use crate::state::{Item, Collection, PRICE_TYPE_STANDARD};
use crate::error::ContractError;
use crate::utils::nft_token_id;
use pix0_contract_common::funcs::{pay_by_percentage_checked, to_bank_messages, try_paying_contract_treasuries};
use pix0_market_handlers::nft_ins::NftContract;
use pix0_market_handlers::state::Metadata;

pub fn mint_nft(mut deps: DepsMut,  
    _env : Env, 
    info: MessageInfo, 
    contract :  NftContract,
    item : Item, 
    collection : Collection,
    price_type : Option<u8>,
    token_uri : Option<String>,
    method : Option<String>,
    _token_id : Option<String>)-> Result<Response, ContractError>  {

    let new_owner = info.clone().sender;
   
    let ext_url = item.external_link();

    let mut token_id = _token_id;

    if token_id.is_none () {

        token_id = Some( nft_token_id(&(
            item.name.clone(), 
            item.collection_owner.to_string(),
            item.collection_name.clone(), 
            item.collection_symbol.clone())));
    }
    
    
    
   
    let c_item = item.clone();

    let ext = Some(Metadata {
        description: item.clone().description,
        name: Some(item.clone().name) ,
        image : c_item.image_link(), 
        youtube_url : c_item.video_link(),
        animation_url : c_item.animation_link(), 
        external_url : ext_url.clone() ,
        attributes : Some(item.add_simple_collection_info_to_traits()), 
        ..Metadata::default()
    });

    
    let msg = cw721_base::msg::MintMsg {
        token_id: token_id.unwrap() ,
        owner: new_owner.to_string(),
        token_uri: token_uri,
        extension: ext ,
    };

    let mint_msg = cw721_base::msg::ExecuteMsg::Mint(msg);

    let res = contract.execute(deps.branch(), _env.clone(), info.clone(), mint_msg);

    match res {

        Ok(_res) =>  {

           let mut prc_typ = PRICE_TYPE_STANDARD;

           if price_type.is_some() {
                prc_typ = price_type.unwrap();
           }

           // let bank_msgs = pay_collection_treasuries(deps, _env, 
           // info, collection, prc_typ);

           let bank_msgs = pay_all_treasuries(deps, _env, 
            info, collection, prc_typ);

            if bank_msgs.is_some() {

                let mut mthd = "mint-nft".to_string();
                if method.is_some() {
                    mthd = method.unwrap();
                }

                Ok(Response::new().add_attribute("method", mthd)
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
    method : Option<String>,
    _token_id : Option<String>) -> Result<Response, ContractError>{

    let msg =  cw721_base::InstantiateMsg {
        name: item.collection_name.clone(),
        symbol: item.collection_symbol.clone(),
        minter: String::from(info.sender.clone()),
    };

    let contract = NftContract::default();

    let _res = contract.instantiate(deps.branch(), _env.clone(), info.clone(),msg);
    
    mint_nft(deps, _env, info, contract, item,collection, price_type, token_uri, method, _token_id)
    
}



pub fn init_and_simple_mint(mut deps: DepsMut,  _env : Env, 
    info: MessageInfo, 
    item : Item, 
    token_uri : Option<String>,
    _token_id : Option<String>) -> Result<Response, ContractError>{

    let msg =  cw721_base::InstantiateMsg {
        name: item.collection_name.clone(),
        symbol: item.collection_symbol.clone(),
        minter: String::from(info.sender.clone()),
    };

    let contract = NftContract::default();

    let _res = contract.instantiate(deps.branch(), _env.clone(), info.clone(),msg);
    
    simple_mint(deps, _env, info, contract, item,token_uri, None, _token_id)
    
}


fn simple_mint(mut deps: DepsMut,  
    _env : Env, 
    info: MessageInfo, 
    contract :  NftContract,
    item : Item, 
    token_uri : Option<String>,
    method : Option<String>,
    _token_id : Option<String>)-> Result<Response, ContractError>  {

    let new_owner = info.clone().sender;
   
    let ext_url = item.external_link();

    let mut token_id = _token_id;

    if token_id.is_none () {

        token_id = Some( nft_token_id(&(
            item.name.clone(), 
            item.collection_owner.to_string(),
            item.collection_name.clone(), 
            item.collection_symbol.clone())));
    }
    
    
    let c_item = item.clone();

    let ext = Some(Metadata {
        description: item.clone().description,
        name: Some(item.clone().name) ,
        image : c_item.image_link(), 
        youtube_url : c_item.video_link(),
        animation_url : c_item.animation_link(), 
        external_url : ext_url.clone() ,
        attributes : Some(item.add_simple_collection_info_to_traits()), 
        ..Metadata::default()
    });

    
    let msg = cw721_base::msg::MintMsg {
        token_id: token_id.unwrap() ,
        owner: new_owner.to_string(),
        token_uri: token_uri,
        extension: ext ,
    };

    let mint_msg = cw721_base::msg::ExecuteMsg::Mint(msg);

    let res = contract.execute(deps.branch(), _env.clone(), info.clone(), mint_msg);

    match res {

        Ok(_res) =>  {

        
           let bank_msgs = pay_simple_mint_fee(deps, _env, info);

            if bank_msgs.is_some() {

                let mut mthd = "simple-mint-nft".to_string();
                if method.is_some() {
                    mthd = method.unwrap();
                }

                Ok(Response::new().add_attribute("method", mthd)
                .add_messages(bank_msgs.unwrap()))
    
            }
            else {
                Err(ContractError::FailedToMakePayment { text: "Failed to make payment when minting NFT".to_string()})
            }
           
        },

        Err(e) => Err(ContractError::CustomErrorMesg{message : e.to_string()}), 

    }

}



pub fn transfer_nft ( deps: DepsMut,  _env : Env, 
    info: MessageInfo,  recipient : String, token_id : String ) -> Result<Response, ContractError> {

    let res = pix0_market_handlers::nft_ins::transfer_nft( deps, _env, info, recipient, token_id);

    match res {

        Ok(_res) =>  {

            Ok(_res)
        }
        ,
        Err(e)=>{
            Err(ContractError::FailedToTransferNft{text : e.to_string()})
        },
    }

}

pub fn burn_nft ( deps: DepsMut,  _env : Env, 
    info: MessageInfo,  token_id : String ) -> Result<Response, ContractError>  {

    let msg = cw721_base::msg::ExecuteMsg::Burn{
        token_id : token_id,
    };

    let contract = NftContract::default();

    let res = contract.execute(deps, _env, info, msg);

    match res {

        Ok(_res) =>  {

            Ok(_res)
        }
        ,
        Err(e)=>{
            Err(ContractError::FailedToBurnNft{text : e.to_string()})

        },
    }

}



pub fn send_nft ( deps: DepsMut,  _env : Env, 
    info: MessageInfo,  token_id : String, contract_addr : String,
    action : String ) -> Result<Response, ContractError>  {


    let binary_action = Binary::from(serde_json::to_vec(&action).unwrap());

    let msg = cw721_base::msg::ExecuteMsg::SendNft{
        token_id : token_id,
        contract : contract_addr.clone(),
        msg : binary_action, 
    };

    let contract = NftContract::default();

    let res = contract.execute(deps, _env, info, msg);

    match res {

        Ok(_res) =>  {

            Ok(_res)
        }
        ,
        Err(e)=>{
            Err(ContractError::FailedToSendNft{text :format!("Failed to send NFT to :{}\nError:{:?}", contract_addr,
            e)})
        },
    }

}



pub fn pay_all_treasuries (mut deps : DepsMut, _env: Env, info : MessageInfo, collection : Collection, price_type : u8) -> 
Option<Vec<BankMsg>>{

    let bank_msgs = pay_collection_treasuries(deps.branch(), _env.clone(), info.clone(), collection, price_type);

    let mut new_bmsgs : Vec<BankMsg> = Vec::new();

    if bank_msgs.is_some() {
        new_bmsgs.extend(bank_msgs.unwrap());
    }

    let _msgs = try_paying_contract_treasuries(deps, _env, 
    info, "NFT_MINTING_FEE");

    if _msgs.is_ok() {

        let bmsgs = _msgs.ok();
        if bmsgs.is_some() {
            new_bmsgs.extend(bmsgs.unwrap());
        }
    }

    if new_bmsgs.len() > 0 {

        Some(new_bmsgs)
    }
    else {
        None 
    } 
}


pub fn pay_simple_mint_fee (deps : DepsMut, _env: Env, info : MessageInfo) -> 
Option<Vec<BankMsg>>{

    let _msgs = try_paying_contract_treasuries(deps, _env, 
    info, "SIMPLE_NFT_MINTING_FEE");

    if _msgs.is_ok() {

        _msgs.ok()
    }
    else {
        None 
    } 
}

pub fn pay_collection_treasuries (
deps: DepsMut,  _env : Env, 
info: MessageInfo,     
collection : Collection, price_type : u8 ) -> Option<Vec<BankMsg>>{

    let payments = collection.treasuries_to_payments();
    let price = collection.price_by_type( price_type);

    if price.is_some() {

        let amts = 
        pay_by_percentage_checked(deps, info, _env.block.time, payments, price.unwrap());

        let bank_msgs = to_bank_messages(amts);

        bank_msgs
    }
    else {

        None
    }

}
