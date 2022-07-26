use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
#[clap(rename_all = "lower")]
pub enum Commands {
    PrintChain,
    AddBlock {
        #[clap(value_parser)]
        data: String,
    },
}
