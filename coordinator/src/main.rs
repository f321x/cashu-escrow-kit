mod escrow_coordinator;

use std::{env, str::FromStr};

use cashu_escrow_common::nostr::NostrClient;
use dotenvy::dotenv;
use escrow_coordinator::EscrowCoordinator;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use nostr_sdk::{Keys, ToBech32};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::builder()
        .filter_module("cashu_escrow_coordinator", log::LevelFilter::Trace) // level for the application itself
        .filter_level(log::LevelFilter::Info) // level for imported crates
        .init();

    let keys = Keys::from_str(&env::var("ESCROW_NSEC")?)?;
    let relays: Vec<String> = env::var("NOSTR_RELAYS")?
        .split(',')
        .map(String::from)
        .collect();
    let nostr_client = NostrClient::new(keys.clone(), relays.clone()).await?;
    info!(
        "Coordinator npub: {}",
        nostr_client.public_key().to_bech32()?
    );
    info!("Starting service and waiting for trades...");
    let mut coordinator = EscrowCoordinator::new(nostr_client)?;
    while let Err(err) = coordinator.run().await {
        error!("Coordinator loop exited with error: {:?}", err);
        coordinator = coordinator
            .restart_nostr_client(keys.clone(), relays.clone())
            .await?;
    }
    Ok(())
}
