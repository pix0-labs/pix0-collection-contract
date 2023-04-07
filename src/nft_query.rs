use cosmwasm_std::{Deps, Env, StdResult, Binary};
use pix0_market_handlers::nft_ins::NftContract;


pub fn get_minted_tokens_by_owner( deps: Deps, _env : Env, owner : String ,
    start_after: Option<String>, limit: Option<u32>) -> StdResult<Binary> {

    let msg = cw721_base::msg::QueryMsg::Tokens {
        owner : owner ,
        start_after : start_after,
        limit : limit,
    };

    let contract = NftContract::default();

    contract.query(deps, _env, msg)
}


pub fn get_token_info( deps: Deps, _env : Env, token_id : String ) 
-> StdResult<Binary> {

    let msg = cw721_base::msg::QueryMsg::NftInfo { token_id: token_id };

    let contract = NftContract::default();
    
    contract.query(deps, _env, msg)
}