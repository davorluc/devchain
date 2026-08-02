use serde::Serialize;
use secp256k1::hashes::{sha256, Hash};
use secp256k1::PublicKey;
use secp256k1::ecdsa::Signature;


#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    // sender: String,
    // receiver: String,
    // amount: i64, // TODO: use fixed-point or atomic units if fractional amounts are needed
    // pub birth: usize,
    // pub tip: f32,
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
    txid: sha256::Hash,
}    

#[derive(Debug, Clone, Serialize)]
struct TxIn {
    prev_txid: sha256::Hash,
    vout: u64,
    signature: Signature,
    pub_key: PublicKey,
}

#[derive(Debug, Clone, Serialize)]
struct UnsignedTxIn {
    prev_txid: sha256::Hash,
    vout: u64,
}

#[derive(Debug, Clone, Serialize)]
struct TxOut {
    amount: u64,
    recipient: sha256::Hash,
}

/*impl Transaction {
    pub fn new(sender: String, receiver: String, amount: i64, birth: usize, tip: f32) -> Self {
        Self {
            sender,
            receiver,
            amount,
            birth,
            tip,
        }
    }
}*/

impl Transaction {
    pub fn new(inputs: Vec<TxIn>, outputs: Vec<TxOut>) -> Result<Self, serde_json::Error> {
        let unsigned_inputs: Vec<UnsignedTxIn> = inputs.iter().map(|i| i.unsigned()).collect();
        let bytes: Vec<u8> = serde_json::to_vec(&(&unsigned_inputs, &outputs))?;
        let txid = sha256::Hash::hash(&bytes);
        Ok(Self {
            inputs,
            outputs,
            txid,
        })
    }
}   

impl TxIn {
    fn new(prev_txid: sha256::Hash, vout: u64, signature: Signature, pub_key: PublicKey) -> Self {
        Self {
            prev_txid,
            vout,
            signature,
            pub_key
        }
    }

    fn unsigned(&self) -> UnsignedTxIn {
        UnsignedTxIn {
            prev_txid: self.prev_txid,
            vout: self.vout,
        }
    }
}

impl TxOut {
    fn new(amount: u64, recipient: sha256::Hash) -> Self {
        Self {
            amount,
            recipient
        }
    }
}
