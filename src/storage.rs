use anyhow::{Ok, Result};
use bytes::Bytes;

pub struct Storage {
    db: sled::Db,
}

impl Storage {
    pub fn new(name: &str) -> Result<Self> {
        let db: sled::Db = sled::open(name)?;
        Ok(Self { db })
    }

    pub fn insert(&self, hash: Bytes, block: Bytes) -> Result<()> {
        self.db.insert(hash.to_vec(), block.to_vec())?;
        Ok(())
    }
}
