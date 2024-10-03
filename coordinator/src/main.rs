mod escrow_coordinator;

use anyhow::Context;
use cashu_escrow_common::nostr::NostrClient;
use dotenvy::dotenv;
use escrow_coordinator::EscrowCoordinator;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use nostr_relay_builder::{LocalRelay, RelayBuilder};
use nostr_sdk::{Keys, ToBech32};
use std::{env, str::FromStr};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::builder()
        .filter_module("cashu_escrow_coordinator", log::LevelFilter::Trace) // level for the application itself
        .filter_level(log::LevelFilter::Info) // level for imported crates
        .init();
    let local_relay: LocalRelay; // has to stay in scope to keep the relay running

    let keys = Keys::from_str(&env::var("ESCROW_NSEC")?)?;
    let mut relays: Vec<String> = env::var("NOSTR_RELAYS")?
        .split(',')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    if relays.is_empty() {
        local_relay = run_local_relay().await?;
        relays = vec![local_relay.url()]
    };

    let nostr_client = NostrClient::new(keys, relays).await?;
    info!(
        "Coordinator npub: {}",
        nostr_client.public_key().to_bech32()?
    );
    info!("Starting service and waiting for trades...");
    return EscrowCoordinator::new(nostr_client)?.run().await;
}

/// Starts a in-memory mock relay for testing purposes
async fn run_local_relay() -> anyhow::Result<LocalRelay> {
    let builder = RelayBuilder::default();
    let local_relay = LocalRelay::run(builder)
        .await
        .context("Failed to start local relay")?;
    warn!(
        "Running in-memory relay at {}. Only for testing purposes",
        local_relay.url()
    );
    Ok(local_relay)
}
