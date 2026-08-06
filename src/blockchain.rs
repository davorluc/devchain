use crate::block::Block;
use crate::transaction::{Transaction, TxOut};
use crate::wallet::Wallet;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: Vec<Transaction>,
    pub utxo_set: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_wallet = Wallet::new();
        let genesis_output = TxOut::new(100, genesis_wallet.get_address().to_string());
        let genesis_transaction = vec![Transaction::coinbase(vec![genesis_output]).unwrap()];

        let genesis_block = Block::new(
            0,
            genesis_transaction,
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        );
        Self::store_block(&genesis_block);

        Self {
            chain: vec![genesis_block],
            mempool: Vec::new(),
            utxo_set: Vec::new(),
        }
    }

    pub fn add_block(&mut self) {
        self.sort_mempool();
        let prev_block = self.chain.last().unwrap();

        let tx_count: usize = self.mempool.len().min(10);
        let txs: Vec<Transaction> = self.mempool.drain(..tx_count).collect();

        let new_block: Block = Block::new(prev_block.index + 1, txs, prev_block.hash.clone());

        Self::store_block(&new_block);
        self.chain.push(new_block);
        self.clean_mempool();
    }

    fn store_block(block: &Block) {
        let path_string: String = "./data/".to_owned() + &block.index.to_string() + ".block";
        let path = Path::new(&path_string);
        let display = path.display();
        println!("{}", display);

        let block_data = format!(
            "{};{};{:?};{};{};{}",
            block.index,
            block.timestamp,
            serde_json::to_string(&block.data),
            block.hash,
            block.previous_hash,
            block.nonce,
        );
        println!("{}", block_data);

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(block_data.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }

    fn _is_chain_valid(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate().skip(1) {
            if block.previous_hash != self.chain[i - 1].hash {
                return false;
            }
        }
        true
    }

    // TODO: this may become redundant if we only drain valid txs into blocks.
    // Keep it around if we later add tx expiration, invalidation, or orphan handling.
    fn clean_mempool(&mut self) {}

    // TODO: re-implement once Transaction carries tip/fee metadata again.
    // Then sort by fee priority before draining into a block.
    fn sort_mempool(&mut self) {}
}
