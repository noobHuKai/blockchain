use crate::block::Block;
use crate::storage::Storage;
use crate::utils::bytes_serde_format;
use bytes::Bytes;
use serde::Serialize;

#[derive(Serialize)]
pub struct Blockchain {
    #[serde(with = "bytes_serde_format")]
    tip: Bytes,
    // for iterator
    #[serde(with = "bytes_serde_format")]
    cur_tip: Bytes,
    #[serde(skip)]
    storage: Storage,
}

impl Blockchain {
    fn geneses_block() -> Block {
        Block::new(String::from("Genesis Block"), Bytes::from("0".repeat(64)))
    }
    pub fn new() -> Self {
        // Self::geneses_block()
        let storage = Storage::new("blockchain").unwrap();

        let last_hash = match storage.get(&Bytes::from("last")).unwrap() {
            Some(b) => b,
            None => {
                let block = Self::geneses_block();
                let hash = block.get_hash();
                storage
                    .insert(&hash, &Bytes::from(block.to_string()))
                    .unwrap();
                //  set last hash
                storage.insert(&Bytes::from("last"), &hash).unwrap();
                hash
            }
        };
        Self {
            tip: last_hash.clone(),
            cur_tip: last_hash,
            storage,
        }
    }

    pub fn add_block(&mut self, data: String) {
        let prev_block_hash = self.tip.clone();
        let new_block = Block::new(data, prev_block_hash);

        let hash = new_block.get_hash();
        self.storage
            .insert(&hash, &Bytes::from(new_block.to_string()))
            .unwrap();
        //  set last hash
        self.storage.insert(&Bytes::from("last"), &hash).unwrap();

        self.tip = hash.clone();
        self.cur_tip = hash;
    }
}

impl Iterator for Blockchain {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        match self.storage.get(&self.cur_tip).unwrap() {
            Some(b) => {
                let block: Block = serde_json::from_slice(&b.to_vec()).unwrap();
                self.cur_tip = block.get_pre_hash();
                Some(block)
            }
            None => {
                self.cur_tip = self.tip.clone();
                None
            }
        }
    }
}
