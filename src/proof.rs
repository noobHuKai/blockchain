use std::cmp::Ordering;

use crate::block::Block;
use crate::utils::sha256_encrypt;
use num_bigint::BigUint;

pub const TARGET_BITS: usize = 8;

#[derive(Debug)]
pub struct ProofOfWork {
    target_bytes: Vec<u8>,
    block: Block,
}

impl ProofOfWork {
    pub fn new(bits: usize, block: Block) -> Self {
        let mut target = BigUint::from(1u8);
        target <<= 256 - bits;

        let mut target_bytes = vec![0].repeat(32 - target.to_bytes_be().len());
        target_bytes.extend(target.to_bytes_be());

        Self {
            block,
            target_bytes,
        }
    }
    fn prepare_data(&mut self, nonce: u128) -> String {
        self.block.set_nonce(nonce);
        self.block.prepare_data()
    }

    pub fn run(&mut self) -> (u128, Vec<u8>) {
        let mut nonce = 0;
        let mut hash = Vec::new();

        while nonce < u128::MAX {
            let data = self.prepare_data(nonce);
            hash = sha256_encrypt(data);

            if hash.cmp(&self.target_bytes) == Ordering::Less {
                break;
            } else {
                nonce += 1;
            }
        }
        (nonce, hash)
    }

    pub fn get_block(self) -> Block {
        self.block
    }

    #[allow(dead_code)]
    pub fn validate(&mut self, nonce: u128) -> bool {
        let data = self.prepare_data(nonce);
        let hash = sha256_encrypt(data);
        hash.cmp(&self.target_bytes) == Ordering::Less
    }
}
