mod cli;
mod ecash;
mod escrow_client;
mod escrow_provider;
mod nostr;

use anyhow::{anyhow, Result};
use cli::get_user_input;
use dotenv::dotenv;
use ecash::EcashWallet;
use escrow_client::{EscrowUser, Trader};
use escrow_provider::{EscrowProvider, TradeContract};
use nostr::NostrClient;
use nostr_sdk::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    // parsing was hacked together last minute :)
    let mut buyer_npub: String = String::new();
    let mut seller_npub: String = String::new();
    let mut coordinator_npub: String = String::new();
    let ecash_wallet = EcashWallet::new().await?;
    let mut seller_ecash_pubkey: String = String::new();
    let mut buyer_ecash_pubkey: String = String::new();
    let nostr_client = NostrClient::new(&get_user_input("Enter nostr nsec: ").await?).await?;

    let mode = match get_user_input("Select mode: buyer, seller, provider: ")
        .await?
        .as_str()
    {
        "buyer" => {
            buyer_npub = nostr_client.get_npub().await?;
            println!("Buyer npub: {}", &buyer_npub);
            seller_npub = get_user_input("Enter seller's npub: ").await?;
            seller_ecash_pubkey = get_user_input("Enter seller's ecash pubkey: ").await?;
            coordinator_npub = get_user_input("Enter coordinator's npub: ").await?;
            buyer_ecash_pubkey = ecash_wallet.trade_pubkey.clone();
            String::from("buyer")
        }
        "seller" => {
            seller_npub = nostr_client.get_npub().await?;
            println!("Seller npub: {}", &seller_npub);
            seller_ecash_pubkey = ecash_wallet.trade_pubkey.clone();
            buyer_npub = get_user_input("Enter buyer's npub: ").await?;
            buyer_ecash_pubkey = get_user_input("Enter buyer's ecash pubkey: ").await?;
            coordinator_npub = get_user_input("Enter coordinator's npub: ").await?;
            String::from("seller")
        }
        "provider" => {
            println!("Coordinator npub: {}", nostr_client.get_npub().await?);
            let mut escrow_provider = EscrowProvider::setup(nostr_client, ecash_wallet).await?;
            escrow_provider.run().await?;
            return Ok(());
        }
        _ => String::from("none"),
    };

    let contract = TradeContract {
        trade_beginning_ts: 1718975980,
        trade_description: "Purchase of one Watermelon for 5000 satoshi. 3 days delivery to ..."
            .to_string(),
        trade_mint_url: "https://mint.minibits.cash/Bitcoin".to_string(),
        trade_amount_sat: 5000,
        npub_seller: seller_npub,
        npub_buyer: buyer_npub,
        time_limit: 3 * 24 * 60 * 60,
        seller_ecash_public_key: seller_ecash_pubkey,
        buyer_ecash_public_key: buyer_ecash_pubkey,
    };

    let escrow_user =
        EscrowUser::new(contract, ecash_wallet, nostr_client, coordinator_npub).await?;

    match mode.as_str() {
        "buyer" => Trader::Buyer(escrow_user).init_trade().await?,
        "seller" => Trader::Seller(escrow_user).init_trade().await?,
        _ => return Err(anyhow!("Invalid mode")),
    }

    Ok(())
}
