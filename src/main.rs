use anyhow::Result;
use clap::Parser;

use libre_cli::{
    args::{Args, Commands},
    hashlist::fetch_hashlist,
};

fn main() -> Result<()> {
    solana_logger::setup_with_default("solana=info");

    let args = Args::parse();

    let keypair_path = args.keypair_path.clone();
    let rpc_url = args.rpc_url.clone();

    match args.command {
        Commands::Hashlist { ticker } => fetch_hashlist(keypair_path, rpc_url, ticker),
    }
}
