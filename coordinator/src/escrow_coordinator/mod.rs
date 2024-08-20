use super::*;
use cashu_escrow_common::model::TradeContract;
use cdk::nuts::SecretKey as CDKSecretKey;
use hashes::hex::DisplayHex;
use ndk::prelude::*;
use ndk::{Filter, Kind, RelayPoolNotification};
use nostr_sdk as ndk;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

pub struct EscrowCoordinator {
    nostr_client: NostrClient,
    pending_contracts: HashMap<[u8; 32], TradeContract>, // k: hash of contract json
    active_contracts: HashMap<[u8; 32], ActiveTade>,
}

struct ActiveTade {
    _trade_contract: TradeContract,
    _coordinator_secret: CDKSecretKey,
}

impl EscrowCoordinator {
    pub async fn setup(nostr_client: NostrClient) -> anyhow::Result<Self> {
        Ok(Self {
            nostr_client,
            pending_contracts: HashMap::new(),
            active_contracts: HashMap::new(),
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

        while let Ok(notification) = notifications.recv().await {
            if let RelayPoolNotification::Event { event, .. } = notification {
                if let Ok(unwrapped_gift) = self.nostr_client.client.unwrap_gift_wrap(&event).await
                {
                    if let Ok((contract_hash, contract)) =
                        parse_contract(&unwrapped_gift.rumor.content)
                    {
                        dbg!("Received contract: {}", &contract.trade_description);
                        if self.pending_contracts.contains_key(&contract_hash) {
                            self.pending_contracts.remove(&contract_hash);
                            self.begin_trade(&contract_hash, &contract).await?;
                        } else {
                            self.pending_contracts.insert(contract_hash, contract);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn begin_trade(
        &mut self,
        contract_hash: &[u8; 32],
        trade: &TradeContract,
    ) -> anyhow::Result<()> {
        dbg!(
            "Beginning trade: {}",
            contract_hash.to_hex_string(hashes::hex::Case::Lower)
        );
        let contract_secret = CDKSecretKey::generate();
        self.active_contracts.insert(
            contract_hash.clone(),
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

    // pub async fn subscribe(&self) -> anyhow::Result<()> {
    //     Ok(())
    // }
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
