mod cli;

use std::env;

use cashu_escrow_client::ecash::ClientEcashWallet;
use cashu_escrow_client::escrow_client::{InitEscrowClient, TradeMode};
use cashu_escrow_common::model::TradeContract;
use cashu_escrow_common::nostr::NostrClient;
use cdk::amount::{Amount, SplitTarget};
use cli::trade_contract::FromClientCliInput;
use cli::ClientCliInput;
use dotenv::dotenv;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::builder()
        .filter_module("cashu_escrow_client", log::LevelFilter::Trace) // logging level of the client
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

    InitEscrowClient::new(nostr_client, escrow_wallet, escrow_contract, cli_input.mode)
        .register_trade()
        .await?
        .exchange_trade_token()
        .await?
        .do_your_trade_duties()
        .await?;
    Ok(())
}
