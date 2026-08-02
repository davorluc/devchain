mod block;
mod blockchain;
mod transaction;
mod wallet;

use crate::blockchain::Blockchain;
// use crate::transaction::Transaction;
use crate::wallet::Wallet;

fn main() {
    let mut blockchain = Blockchain::new();

    let _moff_gideon = Wallet::new();
    let _din_djarin = Wallet::new();
    let _din_grogu = Wallet::new();
    let _hotdog_man = Wallet::new();
/*
    let transaction1 = Transaction::new(
        "Moff Gideon".to_string(),
        "Din Djarin".to_string(),
        1000,
        (blockchain.chain.len() - 1).try_into().unwrap(),
        2.0,
    );
    let transaction2 = Transaction::new(
        "Din Djarin".to_string(),
        "Din Grogu".to_string(),
        10,
        (blockchain.chain.len() - 1).try_into().unwrap(),
        1.6,
    );
    let transaction3 = Transaction::new(
        "Din Grogu".to_string(),
        "Hot Dog Man".to_string(),
        10,
        (blockchain.chain.len() - 1).try_into().unwrap(),
        3.2,
    );

    blockchain.mempool.push(transaction1);
    blockchain.mempool.push(transaction2);
    blockchain.mempool.push(transaction3);

    blockchain.add_block();*/
}
