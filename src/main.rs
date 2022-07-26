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
    let mut bc = Blockchain::new();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::PrintChain) => {
            for b in bc {
                println!("{}\n", &b.to_string_pretty());
            }
        }
        Some(Commands::AddBlock { data }) => {
            bc.add_block(data.clone());
        }
        None => {}
    }
}
