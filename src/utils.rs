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


pub struct RandomNumGen {
    seed: u64,
}

impl RandomNumGen {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    pub fn generate_range(&mut self, min: u64, max: u64) -> u64 {
        assert!(max > min);
        let range = max - min + 1;
        let random = self.generate() % range;
        random + min
    }

    fn generate(&mut self) -> u64 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        self.seed
    }
}