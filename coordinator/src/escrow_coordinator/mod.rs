use super::*;
use cashu_escrow_common::model::TradeContract;
use cdk::nuts::SecretKey as CDKSecretKey;
use hashes::hex::DisplayHex;
use ndk::prelude::*;
use ndk::{Filter, Kind, RelayPoolNotification};
use nostr_sdk as ndk;
use sha2::{Digest, Sha256};
use std::collections::{hash_map::Entry, HashMap, HashSet};
use tokio::sync::broadcast::error::RecvError;

pub struct EscrowCoordinator {
    nostr_client: NostrClient,
    pending_contracts: HashMap<[u8; 32], TradeContract>, // k: hash of contract json
    active_contracts: HashMap<[u8; 32], ActiveTade>,
    received_events: HashSet<EventId>,
}

struct ActiveTade {
    _trade_contract: TradeContract,
    _coordinator_secret: CDKSecretKey,
}

impl EscrowCoordinator {
    pub fn new(nostr_client: NostrClient) -> anyhow::Result<Self> {
        Ok(Self {
            nostr_client,
            pending_contracts: HashMap::new(),
            active_contracts: HashMap::new(),
            received_events: HashSet::new(),
        })
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let my_pubkey = self.nostr_client.public_key();
        let filter_note = Filter::new()
            .kind(Kind::GiftWrap)
            .pubkey(my_pubkey)
            .limit(0);

        self.nostr_client
            .client
            .subscribe(vec![filter_note], None)
            .await?;
        let mut notifications = self.nostr_client.client.notifications();

        loop {
            match notifications.recv().await {
                Ok(notification) => {
                    if let RelayPoolNotification::Event { event, .. } = notification {
                        // check if we already processed this event previously
                        match self.received_events.contains(&event.id) {
                            true => continue,
                            false => self.received_events.insert(event.id),
                        };

                        if let Ok(unwrapped_gift) =
                            self.nostr_client.client.unwrap_gift_wrap(&event).await
                        {
                            let rumor = unwrapped_gift.rumor;
                            if rumor.kind == Kind::PrivateDirectMessage {
                                if let Ok((contract_hash, contract)) =
                                    EscrowCoordinator::parse_contract(&rumor.content)
                                {
                                    debug!("Received contract: {}", &contract.trade_description);
                                    if let Entry::Vacant(e) =
                                        self.pending_contracts.entry(contract_hash)
                                    {
                                        e.insert(contract);
                                    } else {
                                        self.pending_contracts.remove(&contract_hash);
                                        let _ = self
                                            .begin_trade(&contract_hash, &contract)
                                            .await
                                            .inspect_err(|e| {
                                                error!("Got error while beginning a trade: {}", e);
                                            });
                                    }
                                }
                            }
                        }
                    } else if RelayPoolNotification::Shutdown == notification {
                        error!("Got shutdown notification, restarting nostr client!");
                        self.reconnect_nostr_client().await?;
                    }
                }
                Err(RecvError::Closed) => {
                    error!("Got closed error from channel, restarting nostr client...");
                    self.reconnect_nostr_client().await?;
                }
                Err(RecvError::Lagged(count)) => {
                    warn!("Lost {} events, resuming after that...", count);
                }
            }
        }
    }

    async fn begin_trade(
        &mut self,
        contract_hash: &[u8; 32],
        trade: &TradeContract,
    ) -> anyhow::Result<()> {
        debug!(
            "Beginning trade: {}",
            contract_hash.to_hex_string(hashes::hex::Case::Lower)
        );
        let contract_secret = CDKSecretKey::generate();
        self.active_contracts.insert(
            *contract_hash,
            ActiveTade {
                _trade_contract: trade.clone(),
                _coordinator_secret: contract_secret.clone(),
            },
        );
        self.nostr_client
            .send_escrow_registration(
                (trade.npubkey_buyer, trade.npubkey_seller),
                contract_hash,
                &contract_secret.public_key().to_hex(),
            )
            .await?;
        Ok(())
    }

    fn parse_contract(content: &str) -> anyhow::Result<([u8; 32], TradeContract)> {
        let contract: TradeContract = serde_json::from_str(content)?;

        // create a Sha256 object
        let mut hasher = Sha256::new();
        // write input message
        hasher.update(content.as_bytes());
        // read hash digest and consume hasher
        let trade_hash: [u8; 32] = hasher.finalize().into();
        Ok((trade_hash, contract))
    }

    async fn reconnect_nostr_client(&self) -> anyhow::Result<()> {
        self.nostr_client.client.disconnect().await?;
        warn!("Reconnecting nostr client in 60 seconds...");
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        self.nostr_client.client.connect().await;
        info!("Nostr client reconnected sucessfully!");
        Ok(())
    }
}
