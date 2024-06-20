mod ecash;
mod escrow_client;
mod escrow_provider;
mod nostr;

use anyhow::{anyhow, Result};
use dotenv::dotenv;
use ecash::EcashWallet;
use escrow_provider::{EscrowProvider, TradeContract};
use nostr::NostrClient;
use nostr_sdk::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let ecash_wallet = EcashWallet::new().await?;
    let nostr_client = NostrClient::new().await?;

    // define sample TradeContract

    let escrow_provider = EscrowProvider::setup(nostr_client, ecash_wallet).await?;

    println!("Hello, world!");
    Ok(())
}
