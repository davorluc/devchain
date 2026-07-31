mod block;
mod blockchain;
mod transaction;

use crate::blockchain::Blockchain;
use crate::transaction::Transaction;

fn main() {
    let mut blockchain = Blockchain::new();

    let transaction1 = Transaction::new(
        "Moff Gideon".to_string(),
        "Din Djarin".to_string(),
        1000,
        (blockchain.chain.len() - 1).try_into().unwrap(),
        1.0,
    );
    let transaction2 = Transaction::new(
        "Din Djarin".to_string(),
        "Din Grogu".to_string(),
        10,
        (blockchain.chain.len() - 1).try_into().unwrap(),
        1.0,
    );
    let transaction3 = Transaction::new(
        "Din Grogu".to_string(),
        "Hot Dog Man".to_string(),
        10,
        (blockchain.chain.len() - 1).try_into().unwrap(),
        1.0,
    );

    blockchain.mempool.push(transaction1);
    blockchain.mempool.push(transaction2);
    blockchain.mempool.push(transaction3);

    blockchain.add_block();
}
