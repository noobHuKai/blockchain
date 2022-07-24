use std::cmp::Ordering;

use crate::block::Block;
use bytes::Bytes;
use crate::utils::sha256_encrypt;
use num_bigint::BigUint;

#[derive(Debug)]
pub struct ProofOfWork {
    target: BigUint,
}

impl ProofOfWork {
    pub fn new(bits: usize) -> Self {
        let mut target = BigUint::from(1u8);
        let target = target << (256-bits);
        // target = target.checked_shl((256 - bits) as u32).unwrap();
        Self { target }
    }
    fn prepare_data(block: &mut Block, nonce: u128) -> String {
        block.set_nonce(nonce);
        block.prepare_data()
    }

    pub fn run(&self, block: &mut Block) {
        let mut nonce = 0;
        while nonce < usize::MAX {
            let data = Self::prepare_data(block, nonce as u128);
            let hash = sha256_encrypt(data);
            // if hash.cmp(&self.target) == Ordering::Less{

            // }
        }
    }
}
