mod cli;
mod ecash;
mod escrow_client;

use std::env;

use crate::cli::trade_contract::FromClientCliInput;
use anyhow::anyhow;
use cashu_escrow_common as common;
use cli::ClientCliInput;
use common::cli::get_user_input;
use common::nostr::NostrClient;
use common::TradeContract;
use dotenv::dotenv;
use ecash::EcashWallet;
use escrow_client::{nostr::ClientNostrClient, ClientEscrowMetadata};
use log::{debug, error, info};
use nostr_sdk::prelude::*;

pub struct ClientEscrow {
    pub nostr_client: Option<ClientNostrClient>, // option as we intend to make nostr functionality optional
    pub ecash_wallet: EcashWallet,
    pub escrow_metadata: ClientEscrowMetadata,
    pub escrow_contract: TradeContract,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::builder()
        .filter_module("client", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Info)
        .init();

    let cli_input = ClientCliInput::parse().await?;
    let escrow_contract = TradeContract::from_client_cli_input(&cli_input)?;
    let escrow_metadata = ClientEscrowMetadata::from_client_cli_input(&cli_input)?;
    let nostr_client = ClientNostrClient::from_client_cli_input(&cli_input).await?;

    Ok(())
}
