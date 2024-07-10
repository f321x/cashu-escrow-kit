mod escrow_coordinator;

use std::env;

use dotenv::dotenv;
use escrow_coordinator::EscrowProvider;
use cashu_escrow_common::nostr::NostrClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let nostr_client = NostrClient::new(&env::var("ESCROW_NSEC")?).await?;
    println!("Coordinator npub: {}", nostr_client.get_npub()?);
    println!("Starting service and waiting for trades...");
    return EscrowProvider::setup(nostr_client).await?.run().await;
}
