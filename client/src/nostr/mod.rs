use anyhow::Result;
use cashu_escrow_common::model::EscrowRegistration;
use cdk::nuts::Token;
use nostr_sdk::PublicKey;

use super::*;

// here we can somehow make NostrInstance generic to be either a full Nostr Client or only a Nostr Signer depending on
// compilation flags or env variables?

// should we keep the sending functions in the common crate?
// As they are only used by the client.

pub struct ClientNostrInstance {
    pub nostr_client: NostrClient,
}

impl ClientNostrInstance {
    pub async fn from_client_cli_input(cli_input: &ClientCliInput) -> Result<Self> {
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
        coordinator_pk: &PublicKey,
    ) -> Result<()> {
        let coordinator_pk_bech32 = coordinator_pk.to_bech32()?;
        self.nostr_client
            .send_trade_contract(contract, &coordinator_pk_bech32)
            .await?;
        Ok(())
    }

    // await the answer to the submitted contract, once the coordinator returns the ecash public key
    // the escrow service is confirmed by the coordinator
    pub async fn receive_registration_message(
        &self,
        receiver_pk: PublicKey,
    ) -> Result<EscrowRegistration> {
        let message = self
            .nostr_client
            .receive_escrow_message(receiver_pk, 10)
            .await?;
        Ok(serde_json::from_str(&message)?)
    }

    pub async fn submit_trade_token_to_seller(
        &self,
        seller_npubkey: PublicKey,
        token: &str,
    ) -> Result<()> {
        self.nostr_client
            .send_trade_token_to_seller(seller_npubkey, token)
            .await?;
        Ok(())
    }

    pub async fn await_and_validate_escrow_token(
        &self,
        wallet: &ClientEcashWallet,
        contract: &TradeContract,
        registration: &EscrowRegistration,
    ) -> Result<Token> {
        let message = self
            .nostr_client
            .receive_escrow_message(contract.npubkey_buyer, 10)
            .await?;
        wallet.validate_escrow_token(&message, contract, registration)
    }
}
