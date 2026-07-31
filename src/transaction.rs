use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    sender: String,
    receiver: String,
    amount: i64, // TODO: use fixed-point or atomic units if fractional amounts are needed
    pub birth: usize,
    pub tip: f32,// TODO: add an optional tip/fee field for mempool prioritization
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: i64, birth: usize, tip: f32) -> Self {
        Self {
            sender,
            receiver,
            amount,
            birth,
            tip,
        }
    }
}
