pub mod nostr;

use super::*;
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
        let trade_beginning_ts = Timestamp::from(contract.trade_beginning_ts);
        let escrow_coordinator_cashu_pk = Self::common_flow(
            &contract,
            &escrow_coordinator_npub,
            &nostr_client,
            trade_beginning_ts,
        )
        .await?;

        Ok(Self {
            escrow_coordinator_npub,
            trade_beginning_ts,
            escrow_coordinator_cashu_pk,
            contract,
            wallet,
            nostr_client,
        })
    }

    async fn common_flow(
        contract: &TradeContract,
        escrow_coordinator_npub: &String,
        nostr_client: &NostrClient,
        trade_beginning_ts: Timestamp,
    ) -> anyhow::Result<PublicKey> {
        nostr_client
            .send_escrow_contract(contract, escrow_coordinator_npub)
            .await?;

        let escrow_coordinator_pk = Self::receive_escrow_coordinator_pk(
            nostr_client,
            trade_beginning_ts,
            escrow_coordinator_npub,
        )
        .await?;
        Ok(escrow_coordinator_pk)
    }

    async fn parse_escrow_pk(pk: &String) -> anyhow::Result<PublicKey> {
        let public_key = PublicKey::from_hex(pk)?;
        Ok(public_key)
    }

    async fn receive_escrow_coordinator_pk(
        nostr_client: &NostrClient,
        trade_beginning_ts: Timestamp,
        coordinator_npub: &String,
    ) -> anyhow::Result<PublicKey> {
        let filter_note = Filter::new()
            .kind(Kind::EncryptedDirectMessage)
            .since(trade_beginning_ts)
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
                    if let Ok(potential_mint_pk) = Self::parse_escrow_pk(&event.content).await {
                        nostr_client.client.unsubscribe_all().await;
                        return Ok(potential_mint_pk);
                    }
                }
            }
        }
        Err(anyhow!("No valid escrow coordinator public key received"))
    }

    async fn await_and_validate_trade_token(&self) -> anyhow::Result<cdk::nuts::Token> {
        let filter_note = Filter::new()
            .kind(Kind::EncryptedDirectMessage)
            .since(Timestamp::from(self.contract.trade_beginning_ts))
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
