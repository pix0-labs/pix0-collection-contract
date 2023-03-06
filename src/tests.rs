#[cfg(test)]
mod tests {
  
    use crate::nft_ins::pay_collection_treasuries;
    // use rand::Rng;
    // use crate::users::user_resp::*;
    // use std::mem::size_of;
    use crate::state::*;
    use cosmwasm_std::testing::{mock_env, mock_info, mock_dependencies_with_balance};
    use cosmwasm_std::{coins, Addr, Deps, from_binary};
    use crate::msg::*;
    use crate::nft_ins::{DEFAULT_PRICE_DENOM, Extension};
    use crate::contract::*;
    use crate::ins::*;
    use crate::utils::RandomNumGen;

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

        
        let create_collection = ExecuteMsg::CreateCollection { collection:
            Collection {
                name : collection_name.clone(),
                symbol : collection_symb.clone(),
                description : Some("Test collection 1111111".to_string()),
                treasuries : Some(treasuries),
                attributes : None, 
                status : Some(COLLECTION_STATUS_ACTIVATED),
                prices : Some(prices),
                royalties : None, 
                date_created : None,
                date_updated : None, 
                owner : Some(Addr::unchecked(owner)), 
            }
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

        let r = mint_item(deps.as_mut(), mock_env(), info.clone(),
         idx, Addr::unchecked(owner), 
        collection_name.clone(), collection_symb.clone(), Some(price_type), 
        Some("https://some.metadata/x199x.json".to_string()));

        println!("Minted.item:{}::res:{:?}",  idx,  r);

        print_items_count(&deps.as_ref(), Addr::unchecked(owner.clone()), 
        collection_name.clone(), collection_symb.clone());

        idx = 19;

        let r = mint_item(deps.as_mut(), mock_env(), info.clone(),  
        idx, Addr::unchecked(owner.clone()), collection_name.clone(), 
        collection_symb.clone(), Some(price_type), 
        Some("https://some.metadata/x208y.json".to_string()));

        println!("Minted.item:{}:res:{:?}",idx, r);

        print_items_count(&deps.as_ref(), Addr::unchecked(owner), 
        collection_name.clone(), collection_symb.clone());
       
        print_nfts_by_owner(&deps.as_ref(), owner);
       
        let rs = remove_collection(collection_name.clone(), collection_symb.clone(), deps.as_mut(), info);
        println!("\n\nremoved.collection.result::{:?}",rs);

        print_items_count(&deps.as_ref(), Addr::unchecked(owner), 
        collection_name.clone(), collection_symb.clone());
       
    }


    fn print_items_count(deps : &Deps, owner : Addr,  collection_name : String, collection_symbol : String ) {

       
        let msg = QueryMsg::GetItemsCount { owner:
            owner.clone(), collection_name: collection_name.clone(), 
            collection_symbol: collection_symbol.clone() };

        let res = query(*deps, mock_env(), msg).expect("failed to unwrap!!");

        let result : ItemCountResponse = from_binary(&res).unwrap();

        println!("\nNumber of items in {}-{}-{}\n{:?}\n", owner, collection_name, collection_symbol, result);
     }


    fn print_nfts_by_owner(deps : &Deps, owner : &str) {

        print!("\n======================================\nNfts By {}", owner);

        let msg = QueryMsg::MintedTokensByOwner { owner:
            owner.to_string(), start_after: None, limit: None };

        let res = query(*deps, mock_env(), msg).expect("failed to unwrap!!");

        let result : cw721::TokensResponse = from_binary(&res).unwrap();

        //println!("Nfts::{:?}",result);

        print_tokens_with_info(&result, &deps);
     }


     fn print_tokens_with_info (res : &cw721::TokensResponse, deps : &Deps) {

        for (i, x) in res.tokens.iter().enumerate() {

            let tid = x.clone();

            print!("\nNFT :{}: ID:{}",(i+1), x.clone());

            let msg = QueryMsg::NftTokenInfo { token_id: tid.to_string() };
    
            let res = query(*deps, mock_env(), msg).expect("failed to unwrap!!");
    
            let result : cw721::NftInfoResponse<Extension> = from_binary(&res).unwrap();
            
            println!("\nInfo:{:?}",result);

        }

     }

    // cargo test test_rand_gen -- --show-output
    #[test]
    fn test_rand_gen(){

        let mut rng = RandomNumGen::new(432);


        for i in 0..10 {
            let rnd = rng.generate_range(1, 12);

            println!("{}.rnd.num.is::{}", i, rnd);
    
        }
       
    }

}