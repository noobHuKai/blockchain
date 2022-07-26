use crate::proof::{ProofOfWork, TARGET_BITS};
use crate::utils::bytes_serde_format;
use bytes::Bytes;
use chrono::Utc;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
struct BlockHeader {
    timestamp: i64,
    #[serde(with = "bytes_serde_format")]
    prev_hash: Bytes,
    nonce: u128,
}

#[derive(Debug, Serialize, Clone)]
pub struct Block {
    header: BlockHeader,
    data: String,
    #[serde(with = "bytes_serde_format")]
    hash: Bytes,
}

impl Block {
    pub fn new(data: String, prev_hash: Bytes) -> Self {
        let b = Block {
            header: BlockHeader {
                timestamp: Utc::now().timestamp(),
                prev_hash,
                nonce: 0,
            },
            data,
            hash: Bytes::new(),
        };
        let mut pow = ProofOfWork::new(TARGET_BITS, b);
        let (nonce, hash) = pow.run();
        let mut block = pow.get_block();
        block.set_nonce(nonce);
        block.set_hash(Bytes::from(hex::encode(hash)));
        block
    }
    pub fn prepare_data(&mut self) -> String {
        let mut res = String::new();
        res.push_str(&self.header.timestamp.to_string());
        res.push_str(std::str::from_utf8(&self.header.prev_hash).unwrap());
        res.push_str(&self.header.nonce.to_string());
        res.push_str(&self.data);
        res
    }
    fn set_hash(&mut self, hash: Bytes) {
        self.hash = hash;
    }

    pub fn get_hash(&self) -> Bytes {
        self.hash.clone()
    }

    pub fn set_nonce(&mut self, nonce: u128) {
        self.header.nonce = nonce;
    }

    #[allow(dead_code)]
    pub fn to_string_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
