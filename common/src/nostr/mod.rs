use std::time::Duration;

use crate::model::{EscrowRegistration, TradeContract};
use anyhow::anyhow;
use nostr_sdk::prelude::*;
use tokio::time::timeout;

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

    pub async fn receive_escrow_message(
        &self,
        receiver_pubkey: PublicKey,
        timeout_secs: u64,
    ) -> anyhow::Result<String> {
        let message_filter = Filter::new()
            .kind(Kind::GiftWrap)
            .pubkey(receiver_pubkey)
            .limit(0);

        let subscription_id = self.client.subscribe(vec![message_filter], None).await?.val;

        let mut notifications = self.client.notifications();

        let loop_future = async {
            loop {
                if let Ok(notification) = notifications.recv().await {
                    if let RelayPoolNotification::Event { event, .. } = notification {
                        let rumor = self.client.unwrap_gift_wrap(&event).await?.rumor;
                        if rumor.kind == Kind::PrivateDirectMessage {
                            break Ok(rumor.content) as anyhow::Result<String>;
                        }
                    }
                }
            }
        };
        let result = match timeout(Duration::from_secs(timeout_secs), loop_future).await {
            Ok(result) => result,
            Err(e) => Err(anyhow!("Timeout, {}", e)),
        };
        self.client.unsubscribe(subscription_id.clone()).await;

        result
    }

    // coordinator specific function?
    pub async fn send_escrow_registration(
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
            .send_private_msg(receivers.0, &registration_json, None)
            .await?;
        self.client
            .send_private_msg(receivers.1, &registration_json, None)
            .await?;
        Ok(())
    }

    // client specific function?
    pub async fn send_trade_contract(
        &self,
        contract: &TradeContract,
        coordinator_pk_bech32: &str,
    ) -> anyhow::Result<()> {
        let message = serde_json::to_string(contract)?;
        dbg!("sending contract to coordinator...");
        self.client
            .send_private_msg(
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
            .send_private_msg(seller_npubkey, token, None)
            .await?;
        Ok(())
    }
}
