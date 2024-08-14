use crate::model::{EscrowRegistration, TradeContract};
use nostr_sdk::{nostr::nips::nip04, prelude::*};

pub struct NostrClient {
    keypair: Keys,
    pub client: Client,
}

impl NostrClient {
    pub async fn new(nsec: &String) -> anyhow::Result<Self> {
        let keypair = Keys::parse(nsec)?;

        let client = Client::new(&keypair);

        client.add_relay("wss://relay.damus.io").await?;
        client.add_relay("wss://relay.primal.net").await?;
        client.add_relay("wss://relay.nostr.band").await?;
        client
            .add_relay("wss://ftp.halifax.rwth-aachen.de/nostr")
            .await?;
        client.add_relay("wss://nostr.mom").await?;
        //client.add_relay("wss://relay.nostrplebs.com").await?; (having errors)

        // Connect to relays
        client.connect().await;
        Ok(Self { keypair, client })
    }

    pub fn get_npub(&self) -> anyhow::Result<String> {
        Ok(self.keypair.public_key().to_bech32()?)
    }

    pub fn decrypt_msg(&self, msg: &str, sender_pk: &PublicKey) -> Option<String> {
        let secret_key = self
            .keypair
            .secret_key()
            .expect("The key pair must be set if we have a valid instance.");
        nip04::decrypt(secret_key, sender_pk, msg).ok()
    }

    // coordinator specific function?
    pub async fn send_escrow_pubkeys(
        &self,
        receivers: (PublicKey, PublicKey),
        id: &[u8; 32],
        trade_pk: &str,
    ) -> anyhow::Result<()> {
        let registration_json = serde_json::to_string(&EscrowRegistration {
            escrow_id_hex: hex::encode(id),
            coordinator_escrow_pubkey: cdk::nuts::PublicKey::from_hex(trade_pk)?,
            escrow_start_time: Timestamp::now(),
        })?;
        // todo: replace deprecated method
        self.client
            .send_direct_msg(receivers.0, &registration_json, None)
            .await?;
        self.client
            .send_direct_msg(receivers.1, &registration_json, None)
            .await?;
        Ok(())
    }

    // client specific function?
    pub async fn send_escrow_contract(
        &self,
        contract: &TradeContract,
        coordinator_pk_bech32: &str,
    ) -> anyhow::Result<()> {
        let message = serde_json::to_string(contract)?;
        dbg!("sending contract to coordinator...");
        self.client
            .send_direct_msg(
                PublicKey::from_bech32(coordinator_pk_bech32)?,
                &message,
                None,
            )
            .await?;
        Ok(())
    }

    // client specific function?
    pub async fn send_trade_token_to_seller(
        &self,
        seller_npubkey: PublicKey,
        token: &str,
    ) -> anyhow::Result<()> {
        self.client
            .send_direct_msg(seller_npubkey, token, None)
            .await?;
        Ok(())
    }
}
