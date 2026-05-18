use crate::block::Block;

pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(
            0,
            "I will do what I must.".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            0,
        );

        let mut chain: Vec<Block> = Vec::new();
        chain.push(genesis_block);

        Self { chain }
    }

    fn add_block() {}
}
