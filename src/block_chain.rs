use crate::block::Block;
use crate::storage::Storage;
use crate::utils::vec_bytes_serde_format;
use bytes::Bytes;
use serde::Serialize;

#[derive(Serialize)]
pub struct Blockchain {
    #[serde(with = "vec_bytes_serde_format")]
    hashs: Vec<Bytes>,
    height: usize,
    #[serde(skip)]
    storage: Storage,
}

impl Blockchain {
    pub fn geneses_block() -> Self {
        let b = Block::new(String::from("Genesis Block"), Bytes::from("0".repeat(64)));
        let hash = b.get_hash();
        let storage = Storage::new("blockchain").unwrap();
        storage
            .insert(hash.clone(), Bytes::from(b.to_string()))
            .unwrap();
        Self {
            hashs: vec![hash],
            height: 0,
            storage,
        }
    }
    pub fn new() -> Self {
        Self::geneses_block()
    }

    pub fn add_block(&mut self, data: String) {
        let prev_block_hash = self.hashs.last().cloned().unwrap();
        let new_block = Block::new(data, prev_block_hash);

        let hash = new_block.get_hash();
        self.storage
            .insert(hash.clone(), Bytes::from(new_block.to_string()))
            .unwrap();
        self.height += 1;
        self.hashs.push(hash);
    }

    pub fn to_string_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
