use crate::block::Block;
use bytes::Bytes;
use serde::{ser::SerializeStruct, Serialize};

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
    height: usize,
}

impl Serialize for Blockchain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Blockchain", 2)?;
        s.serialize_field("blocks", &self.blocks)?;
        s.serialize_field("height", &self.height)?;
        s.end()
    }
}

impl Blockchain {
    pub fn geneses_block() -> Self {
        let b = Block::new(String::from("Genesis Block"), Bytes::from("0".repeat(64)));
        Self {
            blocks: vec![b],
            height: 0,
        }
    }
    pub fn new() -> Self {
        Self::geneses_block()
    }

    pub fn add_block(&mut self, data: String) {
        let prev_block_hash = self.blocks.last().unwrap().get_hash();
        let new_block = Block::new(data, prev_block_hash);
        self.blocks.push(new_block);
        self.height += 1;
    }

    pub fn to_string_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
