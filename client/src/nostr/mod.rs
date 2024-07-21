mod client_nostr_utils;

use self::client_nostr_utils::*;
use super::*;

// here we can somehow make NostrInstance generic to be either a full Nostr Client or only a Nostr Signer depending on
// compilation flags or env variables?

// should we keep the sending functions in the common crate?
// As they are only used by the client.

pub struct ClientNostrInstance {
    pub nostr_client: NostrClient,
}

impl ClientNostrInstance {
    pub async fn from_client_cli_input(cli_input: &ClientCliInput) -> anyhow::Result<Self> {
        let nostr_client = NostrClient::new(
            &cli_input
                .trader_nostr_keys
                .secret_key()
                .unwrap()
                .to_bech32()?,
        )
        .await?;
        Ok(Self { nostr_client })
    }

    // here submit_escrow_contract calls send_escrow_contract to submit the contract via nostr
    // maybe this could be some kind of wasm callable function to just return a
    // signed event depending on the setup
    pub async fn submit_escrow_contract(
        &self,
        contract: &TradeContract,
        coordinator_pk: &NostrPubkey,
    ) -> anyhow::Result<()> {
        let coordinator_pk_bech32 = coordinator_pk.to_bech32()?;
        self.nostr_client
            .send_escrow_contract(contract, &coordinator_pk_bech32)
            .await?;
        Ok(())
    }

    // await the answer to the submitted contract, once the coordinator returns the ecash public key
    // the escrow service is confirmed by the coordinator
    pub async fn get_escrow_coordinator_pk(
        &self,
        coordinator_pk: &NostrPubkey,
    ) -> anyhow::Result<(EcashPubkey, Timestamp)> {
        let filter_note = Filter::new()
            .kind(Kind::EncryptedDirectMessage)
            .since(Timestamp::now())
            .author(*coordinator_pk);

        let subscription_id = self
            .nostr_client
            .client
            .subscribe(vec![filter_note], None)
            .await;

        let mut notifications = self.nostr_client.client.notifications();

        while let Ok(notification) = notifications.recv().await {
            if let RelayPoolNotification::Event { event, .. } = notification {
                if let Some(decrypted) = self
                    .nostr_client
                    .decrypt_msg(&event.content, &event.author())
                {
                    debug!("Received event: {:?}", &decrypted);
                    if let Ok(ecash_pubkey_and_timestamp) = parse_escrow_pk(&decrypted).await {
                        self.nostr_client.client.unsubscribe(subscription_id).await;
                        return Ok(ecash_pubkey_and_timestamp);
                    }
                }
            }
        }
        Err(anyhow!("No valid escrow coordinator public key received"))
    }

    pub async fn submit_trade_token_to_seller(
        &self,
        seller_npub: &str,
        token: &str,
    ) -> anyhow::Result<()> {
        self.nostr_client
            .send_trade_token_to_seller(seller_npub, token)
            .await?;
        Ok(())
    }

    pub async fn await_and_validate_escrow_token(
        &self,
        wallet: &ClientEcashWallet,
        contract: &TradeContract,
        metadata: &ClientEscrowMetadata,
    ) -> anyhow::Result<cdk::nuts::Token> {
        let filter_note = Filter::new()
            .kind(Kind::EncryptedDirectMessage)
            .since(
                metadata
                    .escrow_start_timestamp
                    .expect("Escrow timestamp not set"),
            )
            .author(nostr_sdk::PublicKey::from_bech32(&contract.npub_buyer)?);

        let subscription_id = self
            .nostr_client
            .client
            .subscribe(vec![filter_note], None)
            .await;

        let mut notifications = self.nostr_client.client.notifications();

        while let Ok(notification) = notifications.recv().await {
            if let RelayPoolNotification::Event { event, .. } = notification {
                if let Some(decrypted) = self
                    .nostr_client
                    .decrypt_msg(&event.content, &event.author())
                {
                    debug!("Received token event: {:?}", &decrypted);
                    if let Ok(escrow_token) =
                        wallet.validate_escrow_token(&decrypted, contract, metadata)
                    {
                        self.nostr_client.client.unsubscribe(subscription_id).await;
                        return Ok(escrow_token);
                    }
                }
            }
        }
        Err(anyhow!("No valid escrow token received"))
    }
}
