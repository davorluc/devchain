use serde::Serialize;
use secp256k1::hashes::{sha256, Hash};

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
    // TODO: add fee/tip metadata back once mempool prioritization is implemented.
    txid: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TxIn {
    prev_txid: String,
    vout: u64,
    signature: String,
    pub_key: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnsignedTxIn {
    prev_txid: String,
    vout: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TxOut {
    amount: u64,
    recipient: String,
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
        let txid = sha256::Hash::hash(&bytes).to_string();
        Ok(Self {
            inputs,
            outputs,
            txid,
        })
    }

    pub fn coinbase(outputs: Vec<TxOut>) -> Result<Self, serde_json::Error> {
        Self::new(Vec::new(), outputs)
    }

    pub fn signing_message(inputs: &[UnsignedTxIn], outputs: &[TxOut]) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&(inputs, outputs))
    }
}

impl TxIn {
    pub fn new(prev_txid: String, vout: u64, signature: String, pub_key: String) -> Self {
        Self {
            prev_txid,
            vout,
            signature,
            pub_key,
        }
    }

    fn unsigned(&self) -> UnsignedTxIn {
        UnsignedTxIn {
            prev_txid: self.prev_txid.clone(),
            vout: self.vout,
        }
    }
}

impl TxOut {
    pub fn new(amount: u64, recipient: String) -> Self {
        Self { amount, recipient }
    }
}
