#[cfg(test)]
mod tests {
  
    // use rand::Rng;
    // use crate::users::user_resp::*;
    // use std::mem::size_of;
    use crate::state::*;
    use cosmwasm_std::testing::{mock_env, mock_info, mock_dependencies_with_balance};
    use cosmwasm_std::{coins, Addr, Deps, from_binary, Coin, Uint128};
    use crate::msg::*;
    use crate::nft_ins::Extension;
    use crate::contract::*;
    use crate::ins::*;
    use pix0_contract_common::state::{Fee, ContractInfoResponse};
    use pix0_contract_common::msg::InstantiateMsg;


    const DEFAULT_PRICE_DENOM : &str = "uconst";
   
    // cargo test test_create_collection_mint_item -- --show-output
    #[test]
    fn test_create_collection_mint_item(){

        let owner : &str = "archway14l92fdhae4htjtkyla73f262c39cngf2wc65ky";

        let mut deps = mock_dependencies_with_balance(&coins(2, DEFAULT_PRICE_DENOM));
        let info = mock_info(owner, &coins(134000, DEFAULT_PRICE_DENOM));

        let admin =  Addr::unchecked(owner.to_string());

        let ins = InstantiateMsg {

            allowed_admins : Some(vec![admin.clone()]),
            treasuries : Some(vec![admin]),
            contracts : None, 
            fees : Some(vec![ 
                Fee {name : "CREATE_COLLECTION_FEE".to_string(),
                value : Coin { amount : Uint128::from(1280u64), denom : "uconst".to_string()}},
                Fee {name : "CREATE_ITEM_FEE".to_string(),
                value : Coin { amount : Uint128::from(1320u64), denom : "uconst".to_string()}},
            ]) ,
            log_last_payment : Some(true)

        };

        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), ins.clone());
       
        println!("Instantiated::{:?}\n", res);
       
        print_contract_info(&deps.as_ref());


        let collection_name =  "Test Collection 111111".to_string();

        let collection_symb = "Coll.x.111".to_string();

        let prices = vec![PriceType {

            price_type : PRICE_TYPE_STANDARD,
            value : Coin {amount :Uint128::from(123900u64),
            denom : DEFAULT_PRICE_DENOM.to_string()},
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


        let attbs = vec![Attribute{
            name : ALLOWED_MINT_ITEM_BY_NAME.to_string(),
            value : "true".to_string()
        }];
        
        let create_collection = ExecuteMsg::CreateCollection { collection:
            Collection {
                name : collection_name.clone(),
                symbol : collection_symb.clone(),
                description : Some("Test collection 1111111".to_string()),
                treasuries : Some(treasuries),
                attributes : Some(attbs), 
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
       
        let seed = 42;

        let r = mint_item(deps.as_mut(), mock_env(), info.clone(),
         seed, Addr::unchecked(owner), 
        collection_name.clone(), collection_symb.clone(), Some(price_type), 
        Some("https://some.metadata/x199x.json".to_string()), None );

        println!("Minted.item:seed::{}::res:{:?}",  seed,  r);

        print_items_count(&deps.as_ref(), Addr::unchecked(owner.clone()), 
        collection_name.clone(), collection_symb.clone());

        //seed = 1892;

        let r =  mint_item_by_name(deps.as_mut(), mock_env(), info.clone(),  
        format!("Item #00{}",2), Addr::unchecked(owner.clone()), collection_name.clone(), 
        collection_symb.clone(), Some(price_type), 
        Some("https://some.metadata/x208y.json".to_string()), None );
       

        println!("Minted.item:seed::{}:res:{:?}", seed, r);

        print_items_count(&deps.as_ref(), Addr::unchecked(owner), 
        collection_name.clone(), collection_symb.clone());
       
        let toks = print_nfts_by_owner(&deps.as_ref(), owner);
       
        let rs = remove_collection(collection_name.clone(), collection_symb.clone(), deps.as_mut(), info.clone());
        println!("\n\nremoved.collection.result::{:?}",rs);

        print_items_count(&deps.as_ref(), Addr::unchecked(owner), 
        collection_name.clone(), collection_symb.clone());
       
        let tx_to : &str = "archway1nxqd7h869sj9pn0xyq0lqqqxjqx6vt550z4aj7";
        
        let first_tokid = toks[0].clone();

        let tmsg = ExecuteMsg::TransferNft {

            recipient : tx_to.to_string(),

            token_id : first_tokid.clone(),
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), tmsg);
       
        println!("Tx.nft:{}.to::{:?}\nRes::\n{:?}", first_tokid.clone(), tx_to,res );

        let _ = print_nfts_by_owner(&deps.as_ref(), tx_to);


        let sec_tokid = toks[1].clone();

        let to_addr : &str = "archway122w9rr76aac9pmke9qq6ya5l8245qr44h8jvtm";

        let smsg = ExecuteMsg::SendNft {

            token_id : sec_tokid.clone(),
            contract_addr : String::from(to_addr),
            action : String::from("{\"execute\":{\"action\":\"burn\"}}")
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), smsg);
               
        println!("Sending.nft:{}.in::{:?}\nRes::\n{:?}", sec_tokid, to_addr ,res );



        /* 
        let bmsg = ExecuteMsg::BurnNft {

            token_id : sec_tokid.clone(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, bmsg);
               
        println!("Burn.nft:{}.in::{:?}\nRes::\n{:?}", sec_tokid, owner ,res );

        */

        let _ = print_nfts_by_owner(&deps.as_ref(), to_addr);
       
        let res2 = instantiate(deps.as_mut(), mock_env(), info, ins);
       
        println!("Instantiated.2nd.time::{:?}\n", res2);

    }


    fn print_contract_info(deps : &Deps ) {

       
        let msg = QueryMsg::GetContractInfo {  };

        let res = query(*deps, mock_env(), msg).expect("failed to unwrap!!");

        let result : ContractInfoResponse = from_binary(&res).unwrap();

        println!("\nContract.info::{:?}\n", result);
     }

    fn print_items_count(deps : &Deps, owner : Addr,  collection_name : String, collection_symbol : String ) {

       
        let msg = QueryMsg::GetItemsCount { owner:
            owner.clone(), collection_name: collection_name.clone(), 
            collection_symbol: collection_symbol.clone() };

        let res = query(*deps, mock_env(), msg).expect("failed to unwrap!!");

        let result : ItemCountResponse = from_binary(&res).unwrap();

        println!("\nNumber of items in {}-{}-{}\n{:?}\n", owner, collection_name, collection_symbol, result);
     }


    fn print_nfts_by_owner(deps : &Deps, owner : &str) -> Vec<String>{

        print!("\n\n******\n======================================\nNfts By {}", owner);

        let msg = QueryMsg::MintedTokensByOwner { owner:
            owner.to_string(), start_after: None, limit: None };

        let res = query(*deps, mock_env(), msg).expect("failed to unwrap!!");

        let result : cw721::TokensResponse = from_binary(&res).unwrap();

        //println!("Nfts::{:?}",result);

        print_tokens_with_info(&result, &deps)
     }


     fn print_tokens_with_info (res : &cw721::TokensResponse, deps : &Deps) -> Vec<String> {

        let mut toks : Vec<String> = Vec::new();


        for (i, x) in res.tokens.iter().enumerate() {

            let tid = x.clone();

            toks.push(tid.clone());

            print!("\nNFT :{}: ID:{}",(i+1), x.clone());

            let msg = QueryMsg::NftTokenInfo { token_id: tid.to_string() };
    
            let res = query(*deps, mock_env(), msg).expect("failed to unwrap!!");
    
            let result : cw721::NftInfoResponse<Extension> = from_binary(&res).unwrap();
            
            println!("\nInfo:{:?}",result);

        }

        if res.tokens.len() == 0 {

            println!("\nZero(0) NFT found!");
        }

        println!("\n\n");

        toks 
     }

    // cargo test test_rand_gen -- --show-output
    #[test]
    fn test_rand_gen(){

        let mut rng = crate::utils::RandomNumGen::new(2390);
       
        for i in 0..10 {
           
            let rnd = rng.generate_range(0, 1234) ;

            println!("{}.rnd.num.is::{}", i, rnd);
    
        }
       
    }

}