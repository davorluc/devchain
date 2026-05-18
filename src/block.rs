use std::time::{SystemTime, UNIX_EPOCH};

pub struct Block {
    index: i32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: i32,
}

impl Block {
    pub fn new(index: i32, data: String, previous_hash: String, nonce: i32) -> Self {
        let timestamp = Self::current_timestamp();
        let hash = Self::calculate_hash(index, timestamp, &data, &previous_hash, nonce);

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
        data: &str,
        previous_hash: &str,
        nonce: i32,
    ) -> String {
        use hex;
        use sha2::{Digest, Sha256};

        let input = format!("{}{}{}{}{}", index, timestamp, data, previous_hash, nonce);

        let mut hasher = Sha256::new();
        hasher.update(input);
        let hash = hasher.finalize();
        let hash_string = hex::encode(hash);
        hash_string
    }
}
