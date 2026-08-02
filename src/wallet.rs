use secp256k1::rand;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use secp256k1::hashes::{sha256, Hash};


pub struct Wallet {
    _secret_key: SecretKey,
    _public_key: PublicKey,
    _address: sha256::Hash,
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (_secret_key, _public_key) = secp.generate_keypair(&mut rand::rng());
        let _address = sha256::Hash::hash(_public_key.to_string().as_bytes());

        Self {
            _secret_key,
            _public_key,
            _address,
        }
    }


    pub fn _get_public_key(&self) -> &PublicKey {
        &self._public_key
    }

    pub fn _get_address(&self) -> &sha256::Hash {
        &self._address
    }

    // TODO: implement transaction signature
    // INFO: transaction functinality probably has to be adjusted
    /*pub fn sign(&self, message: &[u8]) -> _PrivateKey;*/
}
