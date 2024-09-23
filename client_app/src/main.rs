mod cli;

use std::env;

use cashu_escrow_client::ecash::ClientEcashWallet;
use cashu_escrow_client::escrow_client::{InitEscrowClient, TradeMode};
use cashu_escrow_common::model::TradeContract;
use cashu_escrow_common::nostr::NostrClient;
use cdk::amount::{Amount, SplitTarget};
use cli::trade_contract::FromClientCliInput;
use cli::ClientCliInput;
use dotenvy::dotenv;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::builder()
        // logging level for the own crates
        .filter_module("client_app", log::LevelFilter::Trace)
        .filter_module("cashu_escrow_client", log::LevelFilter::Trace)
        .filter_module("cashu_escrow_common", log::LevelFilter::Trace)
        // logging level of all other crates
        .filter_level(log::LevelFilter::Info)
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
    let relays = env::var("NOSTR_RELAYS")?
        .split(',')
        .map(String::from)
        .collect();
    let nostr_client = NostrClient::new(cli_input.trader_nostr_keys, relays).await?;

    InitEscrowClient::new(nostr_client, escrow_wallet, escrow_contract, cli_input.mode)
        .register_trade()
        .await?
        .exchange_trade_token()
        .await?
        .do_your_trade_duties()
        .await?;
    Ok(())
}
