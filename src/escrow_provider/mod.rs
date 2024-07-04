use super::*;
use cdk::nuts::SecretKey;
use hashes::hex::DisplayHex;
use nostr_sdk::{Filter, Kind, RelayPoolNotification};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

pub struct EscrowProvider {
    nostr_client: NostrClient,
    wallet: EcashWallet,
    pending_contracts: HashMap<[u8; 32], TradeContract>, // k: hash of contract json
    active_contracts: HashMap<[u8; 32], ActiveTade>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeContract {
    pub trade_beginning_ts: u64,
    pub trade_description: String,
    pub trade_mint_url: String,
    pub trade_amount_sat: u64,
    pub npub_seller: String,
    pub npub_buyer: String,
    pub time_limit: u64,
    pub seller_ecash_public_key: String,
    pub buyer_ecash_public_key: String,
}

pub struct ActiveTade {
    pub trade_contract: TradeContract,
    pub provider_secret: SecretKey,
}

impl EscrowProvider {
    pub async fn setup(nostr_client: NostrClient, wallet: EcashWallet) -> anyhow::Result<Self> {
        Ok(Self {
            nostr_client,
            wallet,
            pending_contracts: HashMap::new(),
            active_contracts: HashMap::new(),
        })
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let filter_note = Filter::new()
            .kind(Kind::EncryptedDirectMessage)
            .custom_tag(
                SingleLetterTag::lowercase(Alphabet::P),
                [PublicKey::from_bech32(&self.nostr_client.get_npub()?)?.to_hex()],
            )
            .since(Timestamp::now());

        self.nostr_client
            .client
            .subscribe(vec![filter_note], None)
            .await;
        let mut notifications = self.nostr_client.client.notifications();

        while let Ok(notification) = notifications.recv().await {
            if let RelayPoolNotification::Event { event, .. } = notification {
                if let Some(decrypted) = self
                    .nostr_client
                    .decrypt_msg(&event.content, &event.author())
                    .await
                {
                    println!("Received event: {:?}", &decrypted);
                    if let Ok((contract_hash, contract)) = self.parse(&event.content).await {
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

    async fn parse(&self, content: &str) -> anyhow::Result<([u8; 32], TradeContract)> {
        let trade: TradeContract = serde_json::from_str(content)?;

        // create a Sha256 object
        let mut hasher = Sha256::new();
        // write input message
        hasher.update(content.as_bytes());
        // read hash digest and consume hasher
        let trade_hash: [u8; 32] = hasher.finalize().into();
        Ok((trade_hash, trade))
    }

    async fn begin_trade(
        &mut self,
        contract_hash: &[u8; 32],
        trade: &TradeContract,
    ) -> anyhow::Result<()> {
        println!("Beginning trade: {}", contract_hash.to_hex_string(hashes::hex::Case::Lower));
        let contract_secret = SecretKey::generate();
        self.active_contracts.insert(
            contract_hash.clone(),
            ActiveTade {
                trade_contract: trade.clone(),
                provider_secret: contract_secret.clone(),
            },
        );
        self.nostr_client
            .send_escrow_pubkeys(
                (&trade.npub_buyer, &trade.npub_seller),
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
