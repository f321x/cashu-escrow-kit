mod escrow_coordinator;

use std::{env, str::FromStr};

use cashu_escrow_common::nostr::NostrClient;
use dotenv::dotenv;
use escrow_coordinator::EscrowCoordinator;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use nostr_sdk::{Keys, ToBech32};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::builder()
        .filter_module("coordinator", log::LevelFilter::Trace) // level for the application itself
        .filter_level(log::LevelFilter::Info) // level for imported crates
        .init();

    let keys = Keys::from_str(&env::var("ESCROW_NSEC")?)?;
    let nostr_client = NostrClient::new(keys).await?;
    info!(
        "Coordinator npub: {}",
        nostr_client.public_key().to_bech32()?
    );
    info!("Starting service and waiting for trades...");
    return EscrowCoordinator::new(nostr_client)?.run().await;
}
