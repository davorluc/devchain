use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    sender: String,
    receiver: String,
    amount: i64, // TODO: turn into floating point value
    pub birth: usize,
    // TODO: add tip
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: i64, birth: usize) -> Self {
        Self {
            sender,
            receiver,
            amount,
            birth,
        }
    }
}
