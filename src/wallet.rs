use secp256k1::rand;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use secp256k1::hashes::{sha256, Hash};


struct _Wallet {
    secret_key: SecretKey,
    public_key: PublicKey,
    address: sha256::Hash,
}

impl _Wallet {
    pub fn _new() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::rng());
        let address = sha256::Hash::hash(public_key.to_string().as_bytes());

        Self {
            secret_key,
            public_key,
            address,
        }
    }


    pub fn _get_public_key(&self) -> &PublicKey {
        &self.public_key
    }

    pub fn _get_address(&self) -> &sha256::Hash {
        &self.address
    }

    // TODO: implement transaction signature
    // INFO: transaction functinality probably has to be adjusted
    /*pub fn sign(&self, message: &[u8]) -> _PrivateKey;*/
}
