use crate::transaction::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
pub struct Block {
    pub index: i32,
    timestamp: u64,
    data: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    nonce: i32,
}

impl Block {
    pub fn new(index: i32, data: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Self::current_timestamp();
        let nonce: i32 = 0;
        let (hash, nonce) =
            Self::calculate_hash(index, timestamp, data.clone(), &previous_hash, nonce);
        println!("======== Block {} ========", index);
        println!("");
        println!("nonce: {}", nonce);
        println!("hash: {}", hash);
        println!("previous hash: {}", previous_hash);
        println!("index: {}", index);
        println!("timestamp: {}", timestamp);
        println!("");
        println!("==========================");

        Self {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            nonce,
        }
    }

    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    fn calculate_hash(
        index: i32,
        timestamp: u64,
        data: Vec<Transaction>,
        previous_hash: &str,
        mut nonce: i32,
    ) -> (String, i32) {
        use hex;
        use sha2::{Digest, Sha256};

        let tx_json = serde_json::to_string(&data).unwrap();

        let hash_string: String = loop {
            let input = format!(
                "{}{}{}{}{}",
                index, timestamp, tx_json, previous_hash, nonce
            );
            let mut hasher = Sha256::new();
            hasher.update(input);
            let hash = hasher.finalize();
            let current_hash = hex::encode(hash);

            if current_hash.starts_with("0000") {
                break current_hash;
            }
            nonce += 1;
        };

        (hash_string, nonce)
    }
}
