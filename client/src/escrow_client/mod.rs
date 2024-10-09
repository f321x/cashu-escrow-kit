use super::*;

use cashu_escrow_common::{
    model::{EscrowRegistration, TradeContract},
    nostr::NostrClient,
};
use cdk::nuts::Token;
use ecash::ClientEcashWallet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TradeMode {
    Buyer,
    Seller,
}

pub struct InitEscrowClient {
    nostr_client: NostrClient,
    ecash_wallet: ClientEcashWallet,
    escrow_contract: TradeContract,
    trade_mode: TradeMode,
}

/// Initial Escrow Client state.
impl InitEscrowClient {
    pub fn new(
        nostr_client: NostrClient,
        ecash_wallet: ClientEcashWallet,
        escrow_contract: TradeContract,
        trade_mode: TradeMode,
    ) -> Self {
        Self {
            nostr_client,
            ecash_wallet,
            escrow_contract,
            trade_mode,
        }
    }

    /// The trade initialization is the same for both buyer and seller.
    ///
    /// After this the coordinator data is set, state trade registered.
    ///
    /// After this state the trade contract is effectfull as well, possible coordinator fees must be payed.
    pub async fn register_trade(mut self) -> anyhow::Result<RegisteredEscrowClient> {
        let coordinator_pk = &self.escrow_contract.npubkey_coordinator;
        let contract_message = serde_json::to_string(&self.escrow_contract)?;
        debug!("sending contract to coordinator...");
        self.nostr_client
            .client
            .send_private_msg(*coordinator_pk, &contract_message, None)
            .await?;

        let escrow_registration: EscrowRegistration =
            self.nostr_client.receive_escrow_message(20).await?;
        debug!(
            "Received registration: {}",
            &escrow_registration.escrow_id_hex
        );
        Ok(RegisteredEscrowClient {
            nostr_client: self.nostr_client,
            ecash_wallet: self.ecash_wallet,
            escrow_contract: self.escrow_contract,
            trade_mode: self.trade_mode,
            escrow_registration,
        })
    }
}

pub struct RegisteredEscrowClient {
    nostr_client: NostrClient,
    ecash_wallet: ClientEcashWallet,
    escrow_contract: TradeContract,
    trade_mode: TradeMode,
    escrow_registration: EscrowRegistration,
}

impl RegisteredEscrowClient {
    /// Depending on the trade mode sends or receives the trade token.
    ///
    /// After this the state is token sent or received.
    pub async fn exchange_trade_token(mut self) -> anyhow::Result<TokenExchangedEscrowClient> {
        match self.trade_mode {
            TradeMode::Buyer => {
                // todo: store the sent token in next instance
                let trade_token = self.send_trade_token().await?;
                Ok(TokenExchangedEscrowClient {
                    _nostr_client: self.nostr_client,
                    _ecash_wallet: self.ecash_wallet,
                    _escrow_contract: self.escrow_contract,
                    trade_mode: self.trade_mode,
                    _unsigned_token: trade_token,
                })
            }
            TradeMode::Seller => {
                // todo: store the received token in next instance
                let trade_token = self.receive_and_validate_trade_token().await?;
                Ok(TokenExchangedEscrowClient {
                    _nostr_client: self.nostr_client,
                    _ecash_wallet: self.ecash_wallet,
                    _escrow_contract: self.escrow_contract,
                    trade_mode: self.trade_mode,
                    _unsigned_token: trade_token,
                })
            }
        }
    }

    /// State change for the buyer. The state after that is token sent.
    ///
    /// Returns the sent trade token by this [`EscrowClient`].
    async fn send_trade_token(&self) -> anyhow::Result<Token> {
        let escrow_contract = &self.escrow_contract;
        let escrow_token = self
            .ecash_wallet
            .create_escrow_token(escrow_contract, &self.escrow_registration)
            .await?;

        debug!("Sending token to the seller: {}", escrow_token);

        self.nostr_client
            .client
            .send_private_msg(
                escrow_contract.npubkey_seller,
                &serde_json::to_string(&escrow_token)?,
                None,
            )
            .await?;
        trace!("Sent Token to seller");

        Ok(escrow_token)
    }

    /// State change for a seller. The state after this is token received.
    ///
    /// Returns the received trade token by this [`EscrowClient`].
    async fn receive_and_validate_trade_token(&mut self) -> anyhow::Result<Token> {
        let escrow_contract = &self.escrow_contract;
        let wallet = &self.ecash_wallet;

        let escrow_token = self.nostr_client.receive_escrow_message(20).await?;
        trace!("Received Token, validating it...");
        wallet.validate_escrow_token(&escrow_token, escrow_contract, &self.escrow_registration)?;
        Ok(escrow_token)
    }
}

pub struct TokenExchangedEscrowClient {
    _nostr_client: NostrClient,
    _ecash_wallet: ClientEcashWallet,
    _escrow_contract: TradeContract,
    trade_mode: TradeMode,
    _unsigned_token: Token,
}

impl TokenExchangedEscrowClient {
    /// Depending on the trade mode deliver product/service or sign the token after receiving the service.
    ///
    /// The state after this operation is duties fulfilled.
    pub async fn finalize_successful_trade(&self) -> anyhow::Result<()> {
        match self.trade_mode {
            TradeMode::Buyer => {
                // sign escrow token for seller to unlock his funds
                let signed_token = self
                    ._ecash_wallet
                    .sign_token(self._unsigned_token.clone())?;

                // send signature to seller
                // self.nostr_client.client.send_private_msg(self._escrow_contract.npubkey_seller, &signature, None).await?;
                trace!("Sent signature to seller, trade is complete.");
            }
            TradeMode::Seller => {
                // receive signed token from buyer
                // let buyer_signed_token =

                // add sellers own signature
                // let fully_signed_token
                trace!("Received signature from buyer, trade is complete.");
                // now withdraw the sats from inbuilt ecash wallet
            }
        }
        Ok(())
    }

    pub async fn request_mediation(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

pub struct PartiallyFinalizedEscrowClient {
    _ecash_wallet: ClientEcashWallet,
    _escrow_contract: TradeContract,
    _trade_mode: TradeMode,
    _partially_signed_token: Token,
}
