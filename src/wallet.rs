use secp256k1::ecdsa::Signature;
use secp256k1::hashes::{sha256, Hash};
use secp256k1::rand;
use secp256k1::{Message, Secp256k1, SecretKey, PublicKey};

pub struct Wallet {
    secret_key: SecretKey,
    public_key: PublicKey,
    address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::rng());
        let address = sha256::Hash::hash(public_key.to_string().as_bytes()).to_string();

        Self {
            secret_key,
            public_key,
            address,
        }
    }

    pub fn get_public_key(&self) -> &PublicKey {
        &self.public_key
    }

    pub fn get_public_key_string(&self) -> String {
        self.public_key.to_string()
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn sign(&self, message: &[u8]) -> String {
        let secp = Secp256k1::new();
        let digest = sha256::Hash::hash(message);
        let msg = Message::from_digest(digest.to_byte_array());
        let sig: Signature = secp.sign_ecdsa(msg, &self.secret_key);
        hex::encode(sig.serialize_compact())
    }
}
