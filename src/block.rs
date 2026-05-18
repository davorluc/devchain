pub struct Block {
    index: i32,
    timestamp: i32,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: i32,
}

impl Block {
    fn new(index: i32, timestamp: i32, data: String, previous_hash: String, nonce: i32) -> Self {
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

    fn calculate_hash(
        index: i32,
        timestamp: i32,
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

