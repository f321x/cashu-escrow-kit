use super::*;

pub trait FromClientCliInput {
    fn from_client_cli_input(
        cli_input: &ClientCliInput,
        trade_pubkey: String,
    ) -> anyhow::Result<TradeContract>;
}

impl FromClientCliInput for TradeContract {
    fn from_client_cli_input(
        cli_input: &ClientCliInput,
        trade_pubkey: String,
    ) -> anyhow::Result<Self> {
        debug!("Constructing hard coded client trade contract...");
        let npub_seller: String;
        let npub_buyer: String;

        match cli_input.mode {
            TradeMode::Buyer => {
                npub_seller = cli_input.trade_partner_nostr_pubkey.to_bech32()?;
                npub_buyer = cli_input.trader_nostr_keys.public_key().to_bech32()?;
            }
            TradeMode::Seller => {
                npub_buyer = cli_input.trade_partner_nostr_pubkey.to_bech32()?;
                npub_seller = cli_input.trader_nostr_keys.public_key().to_bech32()?;
            }
        }

        let (ecash_pubkey_seller, ecash_pubkey_buyer) = match cli_input.mode {
            TradeMode::Seller => (trade_pubkey, cli_input.ecash_pubkey_partner.to_string()),
            TradeMode::Buyer => (cli_input.ecash_pubkey_partner.to_string(), trade_pubkey),
        };
        // hardcoded trade contract
        Ok(TradeContract {
            trade_description:
                "Purchase of one Watermelon for 5000 satoshi. 3 days delivery to ...".to_string(),
            trade_amount_sat: 5000,
            npub_seller,
            npub_buyer,
            npub_coordinator: cli_input.coordinator_nostr_pubkey,
            time_limit: 3 * 24 * 60 * 60,
            seller_ecash_public_key: ecash_pubkey_seller,
            buyer_ecash_public_key: ecash_pubkey_buyer,
        })
    }
}
