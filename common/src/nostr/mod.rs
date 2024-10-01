use crate::model::EscrowRegistration;
use anyhow::Context;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use nostr_sdk::prelude::*;
use serde::de::DeserializeOwned;
use tokio::sync::broadcast::{error::RecvError, Receiver};

pub struct NostrClient {
    keys: Keys,
    pub client: Client,
    subscription_id: SubscriptionId,
    notifications_receiver: Receiver<RelayPoolNotification>,
    /// The nostr network is in general very fuzzy and makes only a few guaranties about message delivery.
    /// Messages can be posted several times and it is better no to do assumptions about the order of the messages.
    /// Therefore, we use a small cache of the last messages received for the case we'll need them later on.
    messages_cache: Vec<String>,
}

pub const CACHE_SIZE: usize = 10;

impl NostrClient {
    pub async fn new(keys: Keys, relays: Vec<String>) -> anyhow::Result<Self> {
        let client = Client::new(&keys);

        // Connect to relays
        for relay in &relays {
            client
                .add_relay(relay)
                .await
                .context(format!("Error adding nostr relay: {}", relay))?;
        }
        client.connect().await;

        let (_subscription_id, notifications_receiver) = init_subscription(&keys, &client).await?;

        Ok(Self {
            keys,
            client,
            subscription_id: _subscription_id,
            notifications_receiver,
            messages_cache: vec![],
        })
    }

    pub fn public_key(&self) -> PublicKey {
        self.keys.public_key()
    }

    pub async fn receive_escrow_message<T: DeserializeOwned>(
        &mut self,
        _timeout_secs: u64,
    ) -> anyhow::Result<T> {
        let hit_idx_res = self
            .messages_cache
            .iter()
            .enumerate()
            .find_map(|(idx, message)| {
                let result = serde_json::from_str::<T>(message).map_err(anyhow::Error::new);
                match result {
                    Ok(_) => Some((idx, result)),
                    _ => None,
                }
            });
        if let Some((hit_idx, result)) = hit_idx_res {
            trace!("Returning from messages cache...");
            self.messages_cache.remove(hit_idx);
            return result;
        }

        trace!("No hit in messages cache, waiting for new messages...");
        let loop_future = async {
            loop {
                match self.notifications_receiver.recv().await {
                    Ok(notification) => {
                        if let RelayPoolNotification::Event { event, .. } = notification {
                            let rumor = self.client.unwrap_gift_wrap(&event).await?.rumor;
                            if rumor.kind == Kind::PrivateDirectMessage {
                                let result = serde_json::from_str::<T>(&rumor.content)
                                    .map_err(anyhow::Error::new);
                                match result {
                                    Ok(_) => break result,
                                    _ => {
                                        trace!("Got an in this state unexpected escrow message, putting event in cache: {}", event.id);
                                        if self.messages_cache.contains(&rumor.content) {
                                            continue;
                                        }
                                        if self.messages_cache.len() == CACHE_SIZE {
                                            self.messages_cache.remove(0);
                                        }
                                        self.messages_cache.push(rumor.content);
                                    }
                                }
                            }
                        }
                    }
                    Err(RecvError::Closed) => {
                        error!("Relay pool closed subscription, restarting a new one...");
                        self.client.unsubscribe(self.subscription_id.clone()).await;
                        (self.subscription_id, self.notifications_receiver) =
                            init_subscription(&self.keys, &self.client).await?;
                    }
                    Err(RecvError::Lagged(count)) => {
                        warn!("Lost {} events, proceeding after that...", count);
                    }
                }
            }
        };
        // Tokio's time module doesn't work on the Web yet, see: https://github.com/tokio-rs/tokio/pull/6740
        #[cfg(not(target_arch = "wasm32"))]
        match tokio::time::timeout(std::time::Duration::from_secs(_timeout_secs), loop_future).await
        {
            Ok(result) => result,
            Err(e) => Err(anyhow::anyhow!("Timeout, {}", e)),
        }
        // TODO: Improve this workaround for wasm. For now we resign to timeout if it takes too long.
        #[cfg(target_arch = "wasm32")]
        loop_future.await
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

    pub fn messages_cache_len(&self) -> usize {
        self.messages_cache.len()
    }
}

async fn init_subscription(
    keys: &Keys,
    client: &Client,
) -> Result<(SubscriptionId, Receiver<RelayPoolNotification>), anyhow::Error> {
    let message_filter = Filter::new()
        .kind(Kind::GiftWrap)
        .pubkey(keys.public_key())
        .limit(0);
    let _subscription_id = client.subscribe(vec![message_filter], None).await?.val;
    let notifications_receiver = client.notifications();
    Ok((_subscription_id, notifications_receiver))
}
