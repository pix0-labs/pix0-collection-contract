use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}


pub fn hash_to_hex<T:Hash>(t: &T) -> String {

    let h = calculate_hash(&t);
    format!("{:x}", h)
}


pub fn nft_token_id<T:Hash>(t: &T) -> String {
    format!("Nft{}", hash_to_hex(t))
}


pub fn str_to_num(num_str : String) -> i32 {
    num_str.parse().unwrap_or(-1)
}


pub fn str_to_u128(num_str : String, null_replace_by : u128) -> u128 {
    num_str.parse().unwrap_or(null_replace_by)
}
