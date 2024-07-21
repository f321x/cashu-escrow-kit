pub mod nostr;

use super::*;
use cashu_escrow_common::nostr::PubkeyMessage;
use cdk::nuts::PublicKey;
use nostr_sdk::PublicKey as NostrPubkey;

pub enum Trader {
    Buyer(ClientEscrowMetadata),
    Seller(ClientEscrowMetadata),
}

pub struct ClientEscrowMetadata {
    pub escrow_coordinator_nostr_public_key: NostrPubkey,
}

impl ClientEscrowMetadata {
    pub fn from_client_cli_input(cli_input: &ClientCliInput) -> anyhow::Result<Self> {
        Ok(Self {
            escrow_coordinator_nostr_public_key: cli_input.coordinator_nostr_pubkey,
        })
    }
}

impl Trader {
    pub async fn init_trade(&self) -> anyhow::Result<()> {
        match self {
            Trader::Buyer(config) => {
                self.buyer_pipeline(config).await?;
                Ok(())
            }
            Trader::Seller(config) => {
                self.seller_pipeline(config).await?;
                Ok(())
            }
        }
    }

    async fn buyer_pipeline(&self, config: &EscrowUser) -> anyhow::Result<()> {
        let token = config.wallet.create_escrow_token(config).await?;
        dbg!("Sending token to the seller: {}", token.as_str());

        config
            .nostr_client
            .submit_trade_token_to_seller(&config.contract.npub_seller, &token)
            .await?;

        // either send signature or begin dispute
        Ok(())
    }

    async fn seller_pipeline(&self, config: &EscrowUser) -> anyhow::Result<()> {
        let escrow_token = config.await_and_validate_trade_token().await?;

        // send product and proof of delivery (oracle) to seller

        // await signature or begin dispute
        Ok(())
    }
}

impl EscrowUser {
    pub async fn new(
        contract: TradeContract,
        wallet: EcashWallet,
        nostr_client: NostrClient,
        escrow_coordinator_npub: String,
    ) -> anyhow::Result<Self> {
        let escrow_pk_ts =
            Self::common_flow(&contract, &escrow_coordinator_npub, &nostr_client).await?;

        Ok(Self {
            escrow_coordinator_npub,
            escrow_pk_ts,
            contract,
            wallet,
            nostr_client,
        })
    }

    async fn common_flow(
        contract: &TradeContract,
        escrow_coordinator_npub: &String,
        nostr_client: &NostrClient,
    ) -> anyhow::Result<(PublicKey, Timestamp)> {
        nostr_client
            .send_escrow_contract(contract, escrow_coordinator_npub)
            .await?;

        let escrow_coordinator_pk =
            Self::receive_escrow_coordinator_pk(nostr_client, escrow_coordinator_npub).await?;
        Ok(escrow_coordinator_pk)
    }

    async fn parse_escrow_pk(pk_message_json: &String) -> anyhow::Result<(PublicKey, Timestamp)> {
        let pkm: PubkeyMessage = serde_json::from_str(pk_message_json)?;
        let public_key = PublicKey::from_hex(pkm.escrow_coordinator_pubkey)?;
        Ok((public_key, pkm.escrow_start_ts))
    }

    async fn receive_escrow_coordinator_pk(
        nostr_client: &NostrClient,
        coordinator_npub: &String,
    ) -> anyhow::Result<(PublicKey, Timestamp)> {
        let filter_note = Filter::new()
            .kind(Kind::EncryptedDirectMessage)
            .since(Timestamp::now())
            .author(nostr_sdk::PublicKey::from_bech32(coordinator_npub)?);
        nostr_client.client.subscribe(vec![filter_note], None).await;

        let mut notifications = nostr_client.client.notifications();

        while let Ok(notification) = notifications.recv().await {
            if let RelayPoolNotification::Event { event, .. } = notification {
                if let Some(decrypted) = nostr_client
                    .decrypt_msg(&event.content, &event.author())
                    .await
                {
                    dbg!("Received event: {:?}", &decrypted);
                    if let Ok(pk_ts) = Self::parse_escrow_pk(&decrypted).await {
                        nostr_client.client.unsubscribe_all().await;
                        return Ok(pk_ts);
                    }
                }
            }
        }
        Err(anyhow!("No valid escrow coordinator public key received"))
    }

    async fn await_and_validate_trade_token(&self) -> anyhow::Result<cdk::nuts::Token> {
        let filter_note = Filter::new()
            .kind(Kind::EncryptedDirectMessage)
            .since(self.escrow_pk_ts.1)
            .author(nostr_sdk::PublicKey::from_bech32(
                &self.contract.npub_buyer,
            )?);
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
                    dbg!("Received token event: {:?}", &decrypted);
                    if let Ok(escrow_token) =
                        self.wallet.validate_escrow_token(&decrypted, &self).await
                    {
                        return Ok(escrow_token);
                    }
                }
            }
        }
        Err(anyhow!("No valid escrow token received"))
    }
}
