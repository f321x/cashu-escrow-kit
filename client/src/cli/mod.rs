pub mod trade_contract;

use super::*;
use cdk::nuts::nut01::PublicKey as EcashPubkey;
use nostr_sdk::Keys as NostrKeys;
use nostr_sdk::PublicKey as NostrPubkey;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum TradeMode {
    Buyer,
    Seller,
}

#[derive(Debug)]
struct RawCliInput {
    buyer_npub: String,
    seller_npub: String,
    pub ecash_wallet: ClientEcashWallet,
    seller_ecash_pubkey: String,
    buyer_ecash_pubkey: String,
    coordinator_npub: String,
    nostr_nsec: String,
    mode: TradeMode,
    mint_url: String,
}

#[derive(Debug)]
pub struct ClientCliInput {
    pub mode: TradeMode,
    pub trader_nostr_keys: NostrKeys,
    pub ecash_wallet: ClientEcashWallet,
    pub ecash_pubkey_buyer: EcashPubkey,
    pub ecash_pubkey_seller: EcashPubkey,
    pub coordinator_nostr_pubkey: NostrPubkey,
    pub trade_partner_nostr_pubkey: NostrPubkey,
    pub mint_url: String,
}

impl RawCliInput {
    async fn parse() -> anyhow::Result<Self> {
        // information would be communicated OOB in production
        let buyer_npub: String = env::var("BUYER_NPUB")?;
        let seller_npub: String = env::var("SELLER_NPUB")?;
        let coordinator_npub: String = env::var("ESCROW_NPUB")?;
        let mint_url = env::var("MINT_URL")?;

        let ecash_wallet = ClientEcashWallet::new(&mint_url).await?;
        let seller_ecash_pubkey: String;
        let buyer_ecash_pubkey: String;
        let nostr_nsec: String;

        let mode = match get_user_input("Select mode: (1) buyer, (2) seller: ")
            .await?
            .as_str()
        {
            "1" => {
                nostr_nsec = env::var("BUYER_NSEC")?;
                buyer_ecash_pubkey = ecash_wallet.trade_pubkey.clone();
                seller_ecash_pubkey = get_user_input("Enter seller's ecash pubkey: ").await?;
                TradeMode::Buyer
            }
            "2" => {
                nostr_nsec = env::var("SELLER_NSEC")?;
                seller_ecash_pubkey = ecash_wallet.trade_pubkey.clone();
                buyer_ecash_pubkey = get_user_input("Enter buyer's ecash pubkey: ").await?;
                TradeMode::Seller
            }
            _ => {
                panic!("Wrong trading mode selected. Select either (1) buyer or (2) seller");
            }
        };
        Ok(Self {
            buyer_npub,
            seller_npub,
            ecash_wallet,
            seller_ecash_pubkey,
            buyer_ecash_pubkey,
            coordinator_npub,
            nostr_nsec,
            mode,
            mint_url,
        })
    }
}

impl ClientCliInput {
    pub async fn parse() -> anyhow::Result<Self> {
        let raw_input = RawCliInput::parse().await?;
        debug!("Raw parsed CLI input: {:?}", raw_input);

        let ecash_pubkey_buyer = EcashPubkey::from_str(&raw_input.buyer_ecash_pubkey)?;
        let ecash_pubkey_seller = EcashPubkey::from_str(&raw_input.seller_ecash_pubkey)?;

        let trader_nostr_keys = NostrKeys::from_str(&raw_input.nostr_nsec)?;
        let coordinator_nostr_pubkey = NostrPubkey::from_str(&raw_input.coordinator_npub)?;
        let trade_partner_nostr_pubkey = match raw_input.mode {
            TradeMode::Buyer => NostrPubkey::from_str(&raw_input.seller_npub)?,
            TradeMode::Seller => NostrPubkey::from_str(&raw_input.buyer_npub)?,
        };

        Ok(Self {
            mode: raw_input.mode,
            trader_nostr_keys,
            ecash_wallet: raw_input.ecash_wallet,
            ecash_pubkey_buyer,
            ecash_pubkey_seller,
            coordinator_nostr_pubkey,
            trade_partner_nostr_pubkey,
            mint_url: raw_input.mint_url,
        })
    }
}
