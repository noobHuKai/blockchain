use bytes::Bytes;
use chrono::Utc;
use serde::{ser::SerializeStruct, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug)]
struct BlockHeader {
    timestamp: i64,
    prev_hash: Bytes,
    nonce: usize,
}
impl Serialize for BlockHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("BlockHeader", 3)?;
        s.serialize_field("timestamp", &self.timestamp)?;
        s.serialize_field("prev_hash", std::str::from_utf8(&self.prev_hash).unwrap())?;
        s.serialize_field("nonce", &self.nonce)?;
        s.end()
    }
}

#[derive(Debug)]
pub struct Block {
    header: BlockHeader,
    data: String,
    hash: Bytes,
}

impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Block", 3)?;
        s.serialize_field("header", &self.header)?;
        s.serialize_field("data", &self.data)?;
        s.serialize_field("hash", std::str::from_utf8(&self.hash).unwrap())?;
        s.end()
    }
}

impl Block {
    pub fn new(data: String, prev_hash: Bytes) -> Self {
        let mut b = Block {
            header: BlockHeader {
                timestamp: Utc::now().timestamp(),
                prev_hash,
                nonce: 0,
            },
            data,
            hash: Bytes::new(),
        };
        b.set_hash();
        b
    }
    fn set_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(self.header.timestamp.to_string());
        hasher.update(self.header.prev_hash.clone());
        hasher.update(self.header.nonce.to_string());
        hasher.update(self.data.clone());
        let hex_hash = hex::encode(hasher.finalize());
        self.hash = Bytes::from(hex_hash);
    }

    pub fn get_hash(&self) -> Bytes {
        self.hash.clone()
    }
}
