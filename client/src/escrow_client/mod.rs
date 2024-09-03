use crate::common::model::EscrowRegistration;
use cdk::nuts::Token;

use super::*;

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

        let registration_message = self.nostr_client.receive_escrow_message(20).await?;
        let escrow_registration: EscrowRegistration = serde_json::from_str(&registration_message)?;
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
                self.send_trade_token().await?;
                Ok(TokenExchangedEscrowClient {
                    _nostr_client: self.nostr_client,
                    _ecash_wallet: self.ecash_wallet,
                    _escrow_contract: self.escrow_contract,
                    trade_mode: self.trade_mode,
                })
            }
            TradeMode::Seller => {
                // todo: store the received token in next instance
                self.receive_and_validate_trade_token().await?;
                Ok(TokenExchangedEscrowClient {
                    _nostr_client: self.nostr_client,
                    _ecash_wallet: self.ecash_wallet,
                    _escrow_contract: self.escrow_contract,
                    trade_mode: self.trade_mode,
                })
            }
        }
    }

    /// State change for the buyer. The state after that is token sent.
    ///
    /// Returns the sent trade token by this [`EscrowClient`].
    async fn send_trade_token(&self) -> anyhow::Result<String> {
        let escrow_contract = &self.escrow_contract;
        let escrow_token = self
            .ecash_wallet
            .create_escrow_token(escrow_contract, &self.escrow_registration)
            .await?;

        debug!("Sending token to the seller: {}", escrow_token);

        self.nostr_client
            .client
            .send_private_msg(escrow_contract.npubkey_seller, &escrow_token, None)
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

        let message = self.nostr_client.receive_escrow_message(20).await?;
        trace!("Received Token, vaidating it...");
        wallet.validate_escrow_token(&message, escrow_contract, &self.escrow_registration)
    }
}

pub struct TokenExchangedEscrowClient {
    _nostr_client: NostrClient,
    _ecash_wallet: ClientEcashWallet,
    _escrow_contract: TradeContract,
    trade_mode: TradeMode,
}

impl TokenExchangedEscrowClient {
    /// Depending on the trade mode deliver product/service or sign the token after receiving the service.
    ///
    /// The state after this operation is duties fulfilled.
    pub async fn do_your_trade_duties(&self) -> anyhow::Result<()> {
        // todo: as seller send product and proof of delivery (oracle) to seller.
        // await signature or begin dispute

        // todo: as buyer either send signature or begin dispute
        match self.trade_mode {
            TradeMode::Buyer => {
                trace!("Payed invoince and waiting for delivery...");
            }
            TradeMode::Seller => {
                trace!("Got payment and proceding with delivery...");
            }
        }
        Ok(())
    }
}
