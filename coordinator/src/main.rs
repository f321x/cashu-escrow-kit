mod escrow_coordinator;

use std::env;

use cashu_escrow_common::nostr::NostrClient;
use dotenv::dotenv;
use escrow_coordinator::EscrowCoordinator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let nostr_client = NostrClient::new(&env::var("ESCROW_NSEC")?).await?;
    println!("Coordinator npub: {}", nostr_client.get_npub()?);
    println!("Starting service and waiting for trades...");
    return EscrowCoordinator::setup(nostr_client).await?.run().await;
}
