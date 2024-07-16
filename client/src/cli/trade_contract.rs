use super::*;

pub trait FromClientCliInput {
    fn from_client_cli_input(cli_input: &ClientCliInput) -> anyhow::Result<TradeContract>;
}

impl FromClientCliInput for TradeContract {
    fn from_client_cli_input(cli_input: &ClientCliInput) -> anyhow::Result<Self> {
        debug!("Constructing hard coded client trade contract...");
        let npub_seller: String;
        let npub_buyer: String;

        match cli_input.mode {
            TradeMode::Buyer => {
                npub_seller = cli_input.trade_partner_nostr_pubkey.to_string();
                npub_buyer = cli_input.trader_nostr_keys.public_key().to_bech32()?;
            }
            TradeMode::Seller => {
                npub_buyer = cli_input.trade_partner_nostr_pubkey.to_string();
                npub_seller = cli_input.trader_nostr_keys.public_key().to_bech32()?;
            }
        }
        Ok(TradeContract {
            trade_beginning_ts: 1718975980,
            trade_description:
                "Purchase of one Watermelon for 5000 satoshi. 3 days delivery to ...".to_string(),
            trade_mint_url: "https://mint.minibits.cash/Bitcoin".to_string(),
            trade_amount_sat: 5000,
            npub_seller,
            npub_buyer,
            time_limit: 3 * 24 * 60 * 60,
            seller_ecash_public_key: cli_input.ecash_pubkey_seller.to_string(),
            buyer_ecash_public_key: cli_input.ecash_pubkey_buyer.to_string(),
        })
    }
}
