mod block;
mod block_chain;
mod proof;
mod utils;

use block_chain::Blockchain;

fn main() {
    let mut bc = Blockchain::new();
    bc.add_block("data".to_string());
    println!("{}", &bc.to_string_pretty());

    let p = proof::ProofOfWork::new(24);
}
