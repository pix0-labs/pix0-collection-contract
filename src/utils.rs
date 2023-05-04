use std::hash::Hash;
use pix0_market_handlers::utils::hash_to_hex;


pub fn nft_token_id<T:Hash>(t: &T) -> String {
    format!("Nft{}", hash_to_hex(t))
}


pub fn str_to_num(num_str : String) -> i32 {
    num_str.parse().unwrap_or(-1)
}


pub fn str_to_u64(num_str : String, default : u64) -> u64 {
    num_str.parse().unwrap_or(default)
}


pub fn str_to_u128(num_str : String, null_replace_by : u128) -> u128 {
    num_str.parse().unwrap_or(null_replace_by)
}

