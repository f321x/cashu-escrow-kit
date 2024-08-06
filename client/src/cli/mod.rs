pub mod trade_contract;

use super::*;
use cdk::nuts::nut01::PublicKey as EcashPubkey;
use nostr_sdk::Keys as NostrKeys;
use nostr_sdk::PublicKey as NostrPubkey;
use std::str::FromStr;

#[derive(Debug)]
struct RawCliInput {
    buyer_npub: String,
    seller_npub: String,
    partner_ecash_pubkey: String,
    coordinator_npub: String,
    nostr_nsec: String,
    mode: TradeMode,
}

#[derive(Debug)]
pub struct ClientCliInput {
    pub mode: TradeMode,
    pub trader_nostr_keys: NostrKeys,
    pub ecash_pubkey_partner: EcashPubkey,
    pub coordinator_nostr_pubkey: NostrPubkey,
    pub trade_partner_nostr_pubkey: NostrPubkey,
}

impl RawCliInput {
    async fn parse() -> anyhow::Result<Self> {
        // information would be communicated OOB in production
        let buyer_npub: String = env::var("BUYER_NPUB")?;
        let seller_npub: String = env::var("SELLER_NPUB")?;
        let coordinator_npub: String = env::var("ESCROW_NPUB")?;

        let partner_ecash_pubkey: String;
        let nostr_nsec: String;

        let mode = match get_user_input("Select mode: (1) buyer, (2) seller: ")
            .await?
            .as_str()
        {
            "1" => {
                nostr_nsec = env::var("BUYER_NSEC")?;
                partner_ecash_pubkey = get_user_input("Enter seller's ecash pubkey: ").await?;
                TradeMode::Buyer
            }
            "2" => {
                nostr_nsec = env::var("SELLER_NSEC")?;
                partner_ecash_pubkey = get_user_input("Enter buyer's ecash pubkey: ").await?;
                TradeMode::Seller
            }
            _ => {
                panic!("Wrong trading mode selected. Select either (1) buyer or (2) seller");
            }
        };
        Ok(Self {
            buyer_npub,
            seller_npub,
            partner_ecash_pubkey,
            coordinator_npub,
            nostr_nsec,
            mode,
        })
    }
}

impl ClientCliInput {
    pub async fn parse() -> anyhow::Result<Self> {
        let raw_input = RawCliInput::parse().await?;
        debug!("Raw parsed CLI input: {:?}", raw_input);

        let ecash_pubkey_partner = EcashPubkey::from_str(&raw_input.partner_ecash_pubkey)?;

        let trader_nostr_keys = NostrKeys::from_str(&raw_input.nostr_nsec)?;
        let coordinator_nostr_pubkey = NostrPubkey::from_str(&raw_input.coordinator_npub)?;
        let trade_partner_nostr_pubkey = match raw_input.mode {
            TradeMode::Buyer => NostrPubkey::from_bech32(&raw_input.seller_npub)?,
            TradeMode::Seller => NostrPubkey::from_bech32(&raw_input.buyer_npub)?,
        };

        Ok(Self {
            mode: raw_input.mode,
            trader_nostr_keys,
            ecash_pubkey_partner,
            coordinator_nostr_pubkey,
            trade_partner_nostr_pubkey,
        })
    }
}
