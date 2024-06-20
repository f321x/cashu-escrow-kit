use super::*;
use serde::Serialize;

pub struct NostrClient {
    keypair: Keys,
    pub client: Client,
}

#[derive(Serialize)]
struct PubkeyMessage {
    escrow_provider_pubkey: String,
    trade_id_hex: String,
}

impl NostrClient {
    pub async fn new() -> anyhow::Result<Self> {
        let keypair = Keys::parse(env::var("NOSTR_NSEC")?)?;

        let client = Client::new(&keypair);

        client.add_relay("wss://relay.damus.io").await?;
        client.add_relay("wss://relay.primal.net").await?;
        client.add_relay("wss://relay.nostr.band").await?;
        client
            .add_relay("wss://ftp.halifax.rwth-aachen.de/nostr")
            .await?;
        client.add_relay("wss://nostr.mom").await?;
        client.add_relay("wss://relay.nostrplebs.com").await?;

        // Connect to relays
        client.connect().await;
        Ok(Self { keypair, client })
    }

    pub async fn send_escrow_pubkeys(
        &self,
        receivers: (&String, &String),
        id: &[u8; 32],
        trade_pk: &String,
    ) -> anyhow::Result<()> {
        let message = serde_json::to_string(&PubkeyMessage {
            escrow_provider_pubkey: trade_pk.clone(),
            trade_id_hex: hex::encode(id),
        })?;
        self.client
            .send_private_msg(PublicKey::from_bech32(receivers.0)?, &message, None)
            .await?;
        self.client
            .send_private_msg(PublicKey::from_bech32(receivers.1)?, &message, None)
            .await?;
        Ok(())
    }

    pub async fn send_escrow_contract(
        &self,
        contract: &TradeContract,
        coordinator_pk_bech32: &String,
    ) -> anyhow::Result<()> {
        let message = serde_json::to_string(contract)?;

        self.client
            .send_private_msg(
                PublicKey::from_bech32(coordinator_pk_bech32)?,
                &message,
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn submit_trade_token_to_seller(
        &self,
        seller_npub: &String,
        token: &String,
    ) -> anyhow::Result<()> {
        self.client
            .send_private_msg(PublicKey::from_bech32(seller_npub)?, token, None)
            .await?;
        Ok(())
    }
}
