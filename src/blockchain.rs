use crate::block::Block;
use crate::transaction::Transaction;

#[allow(dead_code)]
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

        Self {
            chain: vec![genesis_block],
            mempool: Vec::new(),
        }
    }

    pub fn add_block(&mut self) -> () {
        let prev_block = self.chain.last().unwrap();

        let new_block: Block = Block::new(
            prev_block.index + 1,
            self.mempool.clone(),
            prev_block.hash.clone(),
        );
        self.chain.push(new_block);
        self.mempool.clear();
    }
}
