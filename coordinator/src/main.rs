mod escrow_coordinator;

use std::{env, str::FromStr};

use cashu_escrow_common::nostr::NostrClient;
use dotenv::dotenv;
use escrow_coordinator::EscrowCoordinator;
use nostr_sdk::{Keys, ToBech32};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let keys = Keys::from_str(&env::var("ESCROW_NSEC")?)?;
    let nostr_client = NostrClient::new(keys).await?;
    println!(
        "Coordinator npub: {}",
        nostr_client.public_key().to_bech32()?
    );
    println!("Starting service and waiting for trades...");
    return EscrowCoordinator::new(nostr_client)?.run().await;
}
