use crate::block::Block;
use crate::transaction::Transaction;
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
        let genesis_transactions = vec![Transaction::new(
            "Alice".to_string(),
            "Bob".to_string(),
            100,
            0,
            0.0,
        )];
        let genesis_block = Block::new(
            0,
            genesis_transactions,
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        );
        Self::store_block(&genesis_block);

        Self {
            chain: vec![genesis_block],
            mempool: Vec::new(),
        }
    }

    pub fn add_block(&mut self) -> () {
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

        // TODO: extract block persistence into a dedicated function or module
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(block_data.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }

    // TODO: make genesis validation deterministic; recreating the genesis block here uses a new
    // timestamp, so its hash will never match the stored one
    fn _is_chain_valid(&self) -> bool {
        let mut result = true;
        for (i, block) in self.chain.iter().enumerate() {
            if block.index == 0 {
                let genesis_transactions = vec![Transaction::new(
                    "Alice".to_string(),
                    "Bob".to_string(),
                    100,
                    block.index.try_into().unwrap(),
                    0.0,
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

    // TODO: add tests for mempool pruning once the expiration policy is finalized
    // IDEA: when implementing tips, test the prioritization by mixing high- and low-tip TXs
    fn clean_mempool(&mut self) {
        self.mempool.retain(|tx| tx.birth + 5 > self.chain.len());
    }

    // TODO: sort the mempool by fee/tip once transactions carry prioritization data
    fn sort_mempool(&mut self) {
        self.mempool.sort_by(|a, b| b.tip.total_cmp(&a.tip))
    }
}
