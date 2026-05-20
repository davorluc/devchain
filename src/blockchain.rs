use crate::block::Block;

#[allow(dead_code)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(
            0,
            "I will do what I must.".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        );

        let mut chain: Vec<Block> = Vec::new();
        chain.push(genesis_block);

        Self { chain }
    }

    fn _add_block() {}
}
