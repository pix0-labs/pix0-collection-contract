use cosmwasm_std::{Deps, QueryRequest, Binary, WasmQuery};
use serde::{Deserialize, Serialize};
use crate::error::ContractError;

const USER_CONTRACT_ADDR : &str = "archway12sa9fttr0kqv2fev03kuqvm00dnrhxav5pzu322vadm7x92ga20qnz8nx2";

// Define a query message to send to another contract
#[derive(Serialize, Deserialize)]
struct UserExistsQuery {
    pub wallet_address: String,
}

// Define a response from the other contract
#[derive(Serialize, Deserialize, Debug)]
struct UserExistsResponse {
    pub exists: bool,
}



pub (crate) fn check_remote_user_exists( wallet_address : String , deps: Deps  ) ->Result<bool, ContractError> {


    let query = UserExistsQuery {
        wallet_address: wallet_address,
    };

    let binary_query = Binary::from_base64(serde_json::to_string(&query).unwrap().as_str());


    match binary_query {
        
        Ok(b) =>{

            let _response: UserExistsResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: USER_CONTRACT_ADDR.to_string(),
                msg: b,
            }))?;

            return Ok(_response.exists);
        },

        Err(_e) =>  {return Err(ContractError::CustomErrorMesg { message: 
            format!("Check remote user error :{:?}!", _e).to_string() } ); }

    }
}
    

