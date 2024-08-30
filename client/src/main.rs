mod cli;
mod ecash;
mod escrow_client;

use std::env;
use std::sync::Arc;

use cashu_escrow_common as common;
use cdk::amount::{Amount, SplitTarget};
use cli::trade_contract::FromClientCliInput;
use cli::ClientCliInput;
use common::model::TradeContract;
use common::{cli::get_user_input, nostr::NostrClient};
use dotenv::dotenv;
use ecash::ClientEcashWallet;
use escrow_client::*;
use log::{debug, info};
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

    let cli_input = ClientCliInput::parse().await?;

    //Ensure to have enough funds in the wallet.
    if cli_input.mode == TradeMode::Buyer {
        let mint_quote = escrow_wallet.wallet.mint_quote(Amount::from(5000)).await?;
        escrow_wallet
            .wallet
            .mint(&mint_quote.id, SplitTarget::None, None)
            .await?;
    }

    let escrow_contract =
        TradeContract::from_client_cli_input(&cli_input, escrow_wallet.trade_pubkey.clone())?;
    let nostr_client = NostrClient::new(cli_input.trader_nostr_keys).await?;
    let nostr_client_arc = Arc::new(nostr_client);
    InitEscrowClient::new(escrow_wallet, escrow_contract, cli_input.mode)
        .register_trade(nostr_client_arc.clone())
        .await?
        .exchange_trade_token(&nostr_client_arc)
        .await?
        .do_your_trade_duties()
        .await?;
    Ok(())
}
