use crate::block::Block;
use crate::transaction::Transaction;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_transactions = vec![Transaction::new(
            "Alice".to_string(),
            "Bob".to_string(),
            100,
        )];
        let genesis_block = Block::new(
            0,
            genesis_transactions,
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        );
        Self::_store_block(&genesis_block);

        Self {
            chain: vec![genesis_block],
            mempool: Vec::new(),
        }
    }

    pub fn add_block(&mut self) -> () {
        let prev_block = self.chain.last().unwrap();

        let txs: Vec<Transaction> = self.mempool.clone();

        if txs.len() < 10 {
            let new_block: Block = Block::new(prev_block.index + 1, txs, prev_block.hash.clone());
            Self::_store_block(&new_block);
            self.chain.push(new_block);
            self.mempool.clear();
        } else {
            let new_block: Block = Block::new(
                prev_block.index + 1,
                txs[..=9].to_vec(),
                prev_block.hash.clone(),
            );
            Self::_store_block(&new_block);
            self.chain.push(new_block);
            self.mempool.drain(..=9);
        }
    }

    fn _store_block(block: &Block) {
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

    // TODO: Test this function, as it *might* always return false, as timestamp is dynamically
    // set, thus block.hash != genesis_block.hash, as current time is included in hashing
    fn _is_chain_valid(&self) -> bool {
        let mut result = true;
        for (i, block) in self.chain.iter().enumerate() {
            if block.index == 0 {
                let genesis_transactions = vec![Transaction::new(
                    "Alice".to_string(),
                    "Bob".to_string(),
                    100,
                )];
                let genesis_block = Block::new(
                    0,
                    genesis_transactions,
                    "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
                );
                if block.hash != genesis_block.hash {
                    result = false;
                    break;
                } else {
                    continue;
                }
            } else {
                if block.previous_hash != self.chain[i - 1].hash {
                    result = false;
                    break;
                }
            }
        }
        result
    }
}
