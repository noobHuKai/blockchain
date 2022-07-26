mod block;
mod block_chain;
mod cli;
mod proof;
mod storage;
mod utils;

use block_chain::Blockchain;
use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    // let mut bc = Blockchain::new();
    // bc.add_block("data".to_string());
    // println!("{}", &bc.to_string_pretty());

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::PrintChain) => {
            println!("print chain");
        }
        Some(_) => {}
        None => {}
    }
}
