mod cli;
mod ecash;
mod escrow_client;
mod nostr;

use std::env;

use anyhow::anyhow;
use cashu_escrow_common as common;
use cli::trade_contract::FromClientCliInput;
use cli::ClientCliInput;
use common::model::TradeContract;
use common::{cli::get_user_input, nostr::NostrClient};
use dotenv::dotenv;
use ecash::ClientEcashWallet;
use escrow_client::*;
use log::{debug, info};
use nostr::ClientNostrInstance;
use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::builder()
        .filter_module("client", log::LevelFilter::Debug) // logging level of the client
        .filter_level(log::LevelFilter::Info) // logging level of all other crates
        .init();

    let mint_url = env::var("MINT_URL")?;
    let escrow_wallet = ClientEcashWallet::new(&mint_url).await?;

    //todo: Ensure to have enough funds in the wallet. The buyer must probably transfer some ecash to the escrow wallet.

    let cli_input = ClientCliInput::parse().await?;
    let escrow_contract =
        TradeContract::from_client_cli_input(&cli_input, escrow_wallet.trade_pubkey.clone())?;
    let nostr_instance = ClientNostrInstance::new(cli_input.trader_nostr_keys).await?;
    let mut escrow_client = EscrowClient::new(
        nostr_instance,
        escrow_wallet,
        escrow_contract,
        cli_input.mode,
    );

    escrow_client.register_trade().await?;
    debug!("Common trade registration completed");

    escrow_client.exchange_trade_token().await?;

    escrow_client.do_your_trade_duties().await
}
