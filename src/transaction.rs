use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    sender: String,
    receiver: String,
    amount: i64,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: i64) -> Self {
        Self {
            sender,
            receiver,
            amount,
        }
    }
}
