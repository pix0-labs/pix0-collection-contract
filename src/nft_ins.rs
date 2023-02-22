use cosmwasm_std::{Empty, DepsMut, MessageInfo, Env, Response, BankMsg, coins};
use crate::state::{Item, Treasury};
use crate::error::ContractError;
//use std::convert::{TryFrom};
use crate::utils::nft_token_id;

// refer to https://docs.opensea.io/docs/metadata-standards
pub type Metadata = crate::state::Metadata;

pub type Extension = Option<Metadata>;

pub type NftContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;


pub fn init_contract(deps: DepsMut,  _env : Env, 
info: MessageInfo, name : String ,
symbol : String) -> NftContract {
    
    let msg =  cw721_base::InstantiateMsg {
        name: name,
        symbol: symbol,
        minter: String::from(info.sender.clone()),
    };

    let contract = NftContract::default();

    let _ = contract.instantiate(deps, _env.clone(), info.clone(),msg);

    
    contract
}


pub fn mint_nft(deps: DepsMut,  _env : Env, 
    info: MessageInfo, 
    contract :  NftContract,
    item : Item, _treasuries : Option<Vec<Treasury>>)-> Result<Response, ContractError>  {

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
        token_uri: ext_url,
        extension: ext ,
    };

    let mint_msg = cw721_base::msg::ExecuteMsg::Mint(msg);

    let res = contract.execute(deps, _env.clone(), info, mint_msg);

    match res {

        Ok(_res) =>  {

            if _treasuries.is_some()  {

                let _ts = _treasuries.unwrap();

                let res = pay_collection_treasuries(_ts,
                    245, None);
    
                if res.is_err() {
    
                    return Err(ContractError::CustomErrorMesg{message : 
                        "some error while paying treasuries".to_string()});
                }
    
                let resp = res.expect("Failed to unwrap pay treasuries' response");
                
                Ok(resp.add_attribute("method", "nft-minted"))
            }
            else {

                Ok(_res.add_attribute("method", "nft-minted"))
            }
           
        },

        Err(e) => Err(ContractError::CustomErrorMesg{message : e.to_string()}), 

    }

}



pub fn init_and_mint_nft(mut deps: DepsMut,  _env : Env, 
    info: MessageInfo, 
    item : Item, _treasuries : Vec<Treasury>) -> Result<Response, ContractError>{

    let msg =  cw721_base::InstantiateMsg {
        name: item.collection_name.clone(),
        symbol: item.collection_symbol.clone(),
        minter: String::from(info.sender.clone()),
    };

    let contract = NftContract::default();

    let _res = contract.instantiate(deps.branch(), _env.clone(), info.clone(),msg);
    
    mint_nft(deps, _env, info, contract, item, None)
    
}

pub fn pay_collection_treasuries (
treasuries : Vec<Treasury>,    
total_amount : u64, _denom : Option<String>) -> 
Result<Response, ContractError>{

    let mut error : Option<ContractError> = None ;

    treasuries.iter().for_each( |t| {

        let fraction: u64 = (t.percentage as u64) * 100 / 100;

        let amount = total_amount * fraction / 100;
      
        let res =  pay_treasury(t.wallet.as_str(), amount as u128, _denom.clone());

        match res {

            Ok(_) => error = None  ,

            Err(e) => error = Some(ContractError::CustomErrorMesg{ message : e.to_string()} ),

        }

    });

    if error.is_none() {

        Ok(Response::new().add_attribute("action", "paid-all-teasuries"))
    }
    else {

        let err = error.unwrap_or(ContractError::CustomErrorMesg{message: String::from(
            "Some Error When Paying All Treasuries!")});

        Err(err) 
    }
}

pub const DEFAULT_PRICE_DENOM : &str = "uconst";

fn pay_treasury (wallet_address : &str, amount : u128 , _denom : Option <String>)
-> Result<Response, ContractError>{

    /* 
    if amount == 0 {
        return Err(ContractError::CustomErrorMesg {message : "Invalid Amount".to_string()});
    }*/

    let mut denom = String::from(DEFAULT_PRICE_DENOM);

    if _denom != None {
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
