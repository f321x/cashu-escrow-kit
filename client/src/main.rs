mod cli;
mod ecash;
mod escrow_client;
mod nostr;

use std::env;

use anyhow::anyhow;
use cashu_escrow_common as common;
use cdk::nuts::PublicKey as EcashPubkey;
use cli::{trade_contract::FromClientCliInput, ClientCliInput, TradeMode};
use common::{cli::get_user_input, nostr::NostrClient, TradeContract};
use dotenv::dotenv;
use ecash::ClientEcashWallet;
use escrow_client::*;
use log::{debug, info};
use nostr::ClientNostrInstance;
use nostr_sdk::prelude::*;
use nostr_sdk::PublicKey as NostrPubkey;

pub struct EscrowClient {
    pub nostr_instance: ClientNostrInstance, // can either be a Nostr Client or Nostr note signer (without networking)
    pub ecash_wallet: ClientEcashWallet,
    pub escrow_metadata: ClientEscrowMetadata, // data relevant for the application but not for the outcome of the trade contract
    pub escrow_contract: TradeContract,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::builder()
        .filter_module("client", log::LevelFilter::Debug) // logging level of the client
        .filter_level(log::LevelFilter::Info) // logging level of all other crates
        .init();

    let cli_input = ClientCliInput::parse().await?;
    let mut escrow_client = EscrowClient::from_cli_input(cli_input).await?;

    escrow_client.init_trade().await?;

    Ok(())
}
