use std::sync::Arc;

use crate::common::model::EscrowRegistration;
use cdk::nuts::Token;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TradeMode {
    Buyer,
    Seller,
}

pub struct InitEscrowClient {
    ecash_wallet: ClientEcashWallet,
    escrow_contract: TradeContract,
    trade_mode: TradeMode,
}

/// Initial Escrow Client state.
impl InitEscrowClient {
    pub fn new(
        ecash_wallet: ClientEcashWallet,
        escrow_contract: TradeContract,
        trade_mode: TradeMode,
    ) -> Self {
        Self {
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
    pub async fn register_trade(
        &self,
        nostr_client: Arc<NostrClient>,
    ) -> anyhow::Result<RegisteredEscrowClient> {
        let coordinator_pk = &self.escrow_contract.npubkey_coordinator;
        let contract_message = serde_json::to_string(&self.escrow_contract)?;
        dbg!("sending contract to coordinator...");
        nostr_client
            .client
            .send_private_msg(*coordinator_pk, &contract_message, None)
            .await?;

        let registration_message = nostr_client.receive_escrow_message(20).await?;
        let escrow_registration = serde_json::from_str(&registration_message)?;
        Ok(RegisteredEscrowClient {
            prev_state: self,
            escrow_registration,
        })
    }
}

pub struct RegisteredEscrowClient<'a> {
    prev_state: &'a InitEscrowClient,
    escrow_registration: EscrowRegistration,
}

impl<'a> RegisteredEscrowClient<'a> {
    /// Depending on the trade mode sends or receives the trade token.
    ///
    /// After this the state is token sent or received.
    pub async fn exchange_trade_token(
        &self,
        nostr_client: &NostrClient,
    ) -> anyhow::Result<TokenExchangedEscrowClient> {
        match self.prev_state.trade_mode {
            TradeMode::Buyer => {
                // todo: store the sent token in next instance
                self.send_trade_token(nostr_client).await?;
                Ok(TokenExchangedEscrowClient { _prev_state: self })
            }
            TradeMode::Seller => {
                // todo: store the received token in next instance
                self.receive_and_validate_trade_token(nostr_client).await?;
                Ok(TokenExchangedEscrowClient { _prev_state: self })
            }
        }
    }

    /// State change for the buyer. The state after that is token sent.
    ///
    /// Returns the sent trade token by this [`EscrowClient`].
    async fn send_trade_token(&self, nostr_client: &NostrClient) -> anyhow::Result<String> {
        let escrow_contract = &self.prev_state.escrow_contract;
        let escrow_token = self
            .prev_state
            .ecash_wallet
            .create_escrow_token(escrow_contract, &self.escrow_registration)
            .await?;

        debug!("Sending token to the seller: {}", escrow_token);

        nostr_client
            .client
            .send_private_msg(escrow_contract.npubkey_seller, &escrow_token, None)
            .await?;
        dbg!("Sent Token to seller");

        Ok(escrow_token)
    }

    /// State change for a seller. The state after this is token received.
    ///
    /// Returns the received trade token by this [`EscrowClient`].
    async fn receive_and_validate_trade_token(
        &self,
        nostr_client: &NostrClient,
    ) -> anyhow::Result<Token> {
        let escrow_contract = &self.prev_state.escrow_contract;
        let wallet = &self.prev_state.ecash_wallet;

        let message = nostr_client.receive_escrow_message(20).await?;
        dbg!("Received Token, vaidating it...");
        wallet.validate_escrow_token(&message, escrow_contract, &self.escrow_registration)
    }
}

pub struct TokenExchangedEscrowClient<'a> {
    _prev_state: &'a RegisteredEscrowClient<'a>,
}

impl<'a> TokenExchangedEscrowClient<'a> {
    /// Depending on the trade mode deliver product/service or sign the token after receiving the service.
    ///
    /// The state after this operation is duties fulfilled.
    pub async fn do_your_trade_duties(&self) -> anyhow::Result<()> {
        // todo: as seller send product and proof of delivery (oracle) to seller.
        // await signature or begin dispute

        // todo: as buyer either send signature or begin dispute
        match self._prev_state.prev_state.trade_mode {
            TradeMode::Buyer => {
                dbg!("Payed invoince and waiting for delivery...");
            }
            TradeMode::Seller => {
                dbg!("Got payment and proceding with delivery...");
            }
        }
        Ok(())
    }
}
