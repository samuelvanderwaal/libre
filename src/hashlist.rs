use std::{fs::File, path::PathBuf};

use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use serde_with::DisplayFromStr;
use solana_program::{pubkey, pubkey::Pubkey};

use crate::setup;

const FAIR_LAUNCH_PROGRAM_ID: Pubkey = pubkey!("8bvPnYE5Pvz2Z9dE6RAqWr1rzLknTndZ9hwvRE6kPDXP");

#[serde_with::serde_as]
#[derive(Debug, Deserialize, Serialize, BorshSerialize, BorshDeserialize)]
pub struct OnChainHashlist {
    #[serde_as(as = "DisplayFromStr")]
    pub deployment: Pubkey,
    pub issues: Vec<MintAndOrder>,
}

#[serde_with::serde_as]
#[derive(Debug, Deserialize, Serialize, BorshSerialize, BorshDeserialize)]
pub struct MintAndOrder {
    #[serde_as(as = "DisplayFromStr")]
    pub mint: Pubkey,
    pub order: u64,
}

pub fn fetch_hashlist(
    keypair_path: Option<PathBuf>,
    rpc_url: Option<String>,
    ticker: String,
) -> Result<()> {
    let config = setup::CliConfig::new(keypair_path, rpc_url)?;

    let deployment_pubkey = Pubkey::find_program_address(
        &["deployment".as_bytes(), ticker.as_bytes()],
        &FAIR_LAUNCH_PROGRAM_ID,
    )
    .0;

    let hashlist_pubkey = Pubkey::find_program_address(
        &["hashlist".as_bytes(), deployment_pubkey.as_ref()],
        &FAIR_LAUNCH_PROGRAM_ID,
    )
    .0;

    let hashlist_data = config.client.get_account_data(&hashlist_pubkey)?;
    let onchain_hashlist: OnChainHashlist = OnChainHashlist::try_from_slice(&hashlist_data[8..])?;

    let mut hashlist: IndexSet<String> = onchain_hashlist
        .issues
        .iter()
        .map(|mint_and_order| mint_and_order.mint.to_string())
        .collect();

    hashlist.sort();

    let f = File::create(format!("{ticker}_deployment.json"))?;
    let g = File::create(format!("{ticker}_hashlist.json"))?;

    serde_json::to_writer_pretty(f, &onchain_hashlist)?;
    serde_json::to_writer_pretty(g, &hashlist)?;

    Ok(())
}
