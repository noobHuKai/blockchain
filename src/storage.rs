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

    pub fn insert(&self, hash: &Bytes, block: &Bytes) -> Result<()> {
        self.db.insert(hash.to_vec(), block.to_vec())?;
        Ok(())
    }

    pub fn get(&self, key: &Bytes) -> Result<Option<Bytes>> {
        let res = match self.db.get(key)? {
            Some(b) => Some(Bytes::from(b.to_vec())),
            None => None,
        };
        Ok(res)
    }
}
