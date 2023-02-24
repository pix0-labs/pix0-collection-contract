#[cfg(test)]
mod tests {
  
    use crate::nft_ins::pay_collection_treasuries;
    // use rand::Rng;
    // use crate::users::user_resp::*;
    // use std::mem::size_of;
    use crate::state::*;
    use cosmwasm_std::testing::{mock_env, mock_info, mock_dependencies_with_balance};
    use cosmwasm_std::{coins, Addr};
    use crate::msg::*;
    use crate::nft_ins::DEFAULT_PRICE_DENOM;
    use crate::contract::*;
    use crate::ins::mint_item;

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


    // cargo test test_create_collection_mint_item -- --show-output
    #[test]
    fn test_create_collection_mint_item(){

        let owner : &str = "archway14l92fdhae4htjtkyla73f262c39cngf2wc65ky";

        let mut deps = mock_dependencies_with_balance(&coins(2, DEFAULT_PRICE_DENOM));
        let info = mock_info(owner, &coins(2, DEFAULT_PRICE_DENOM));
       
        let collection_name =  "Test Collection 111111".to_string();

        let collection_symb = "Coll.x.111".to_string();

        let prices = vec![PriceType {

            price_type : PRICE_TYPE_STANDARD,
            value : 123900,
            denom : Some(DEFAULT_PRICE_DENOM.to_string()),
            date_start : None, date_end : None, 
        }];

        let treasuries : Vec<Treasury> = vec![Treasury {
            wallet : Addr::unchecked("archway1nxqd7h869sj9pn0xyq0lqqqxjqx6vt550z4aj7".to_string()),
            percentage : 70,
            name : None,
        }, Treasury {
            wallet : Addr::unchecked("archway122w9rr76aac9pmke9qq6ya5l8245qr44h8jvtm".to_string()),
            percentage : 30,
            name : None,
        }];

        let create_collection = ExecuteMsg::CreateCollection {
            name : collection_name.clone(),
            symbol : collection_symb.clone(),
            description : Some("Test collection 1111111".to_string()),
            treasuries : Some(treasuries),
            attributes : None, 
            status : Some(COLLECTION_STATUS_ACTIVATED),
            prices : Some(prices),
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), 
        create_collection.clone());
       
        println!("1.res.create.collection::{:?}\n", res);

        let price_type = PRICE_TYPE_STANDARD;

        // loop create items
        for x in 0..30 {
     
           let links = vec![Link{link_type: LINK_TYPE_IMAGE_URL, 
               value:format!("https://rm.img/img_000{}.png",x) }];

           let itm = Item {
               collection_owner :  Addr::unchecked(owner.to_string()),
               collection_name : collection_name.clone(),
               collection_symbol : collection_symb.clone(),
               name : format!("Item #00{}",(x+1)),
               traits : Vec::new(),
               links : links,
               description : None,
               background_color : None,
               date_created : None,
               date_updated : None, 
           };

           let create_item = ExecuteMsg::CreateItem {
              item : itm.clone()
           };

           let _res = execute(deps.as_mut(), mock_env(), info.clone(), 
           create_item.clone());

           if _res.is_err() {

               println!("Error.creating item:{}, error:is::{:?}", itm.name, _res);
           }
          
        }
       
        let mut idx = 2; 
    
        let r = mint_item(deps.as_mut(), mock_env(), info.clone(), idx, Addr::unchecked(owner), 
        collection_name.clone(), collection_symb.clone(), Some(price_type), None);

        println!("Minted.item:{}::res:{:?}", idx,  r);

        idx = 6;

        let r = mint_item(deps.as_mut(), mock_env(), info, idx, 
        Addr::unchecked(owner), collection_name, 
        collection_symb, Some(price_type), None);

        println!("Minted.item:{}::res:{:?}", idx,  r);



    }
}