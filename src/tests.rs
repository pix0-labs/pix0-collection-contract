#[cfg(test)]
mod tests {
  
    use crate::nft_ins::pay_collection_treasuries;
    // use rand::Rng;
    // use crate::users::user_resp::*;
    // use std::mem::size_of;
    use crate::state::Treasury;
    use cosmwasm_std::Addr;



    // cargo test test_pay_treasuries -- --show-output
    #[test]
    fn test_pay_treasuries(){

        let ts = vec![Treasury{
            wallet :  Addr::unchecked("Alex".to_string()),
            percentage : 9,
            name : None, 
        },
        Treasury{
            wallet : Addr::unchecked("Carmen".to_string()),
            percentage : 10,
            name : None, 
        },
        ];

        let _r = pay_collection_treasuries(ts, 1250500, Some("uconst".to_string()));
        println!("Res::{:?}",_r);
    }
}