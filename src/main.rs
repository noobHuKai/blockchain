mod block;
mod block_chain;

use block_chain::Blockchain;

fn main() {
    let mut bc = Blockchain::new();
    bc.add_block("data".to_string());
    println!("{}", &bc.to_string_pretty());
}
