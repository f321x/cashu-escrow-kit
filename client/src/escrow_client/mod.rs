use crate::common::model::EscrowRegistration;
use cdk::nuts::Token;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum TradeMode {
    Buyer,
    Seller,
}

pub struct EscrowClient {
    nostr_client: NostrClient, // can either be a Nostr Client or Nostr note signer (without networking)
    ecash_wallet: ClientEcashWallet,
    escrow_registration: Option<EscrowRegistration>,
    escrow_contract: TradeContract,
    trade_mode: TradeMode,
}

// todo: model EscrowClient as an state machine (stm). This will improve testability too.
impl EscrowClient {
    // creates the inital state: the coordinator data isn't present.
    pub fn new(
        nostr_client: NostrClient,
        ecash_wallet: ClientEcashWallet,
        escrow_contract: TradeContract,
        trade_mode: TradeMode,
    ) -> Self {
        Self {
            nostr_client,
            ecash_wallet,
            escrow_registration: None,
            escrow_contract,
            trade_mode,
        }
    }

    /// The trade initialization is the same for both buyer and seller.
    ///
    /// After this the coordinator data is set, state trade registered.
    ///
    /// After this state the trade contract is effectfull as well, possible coordinator fees must be payed.
    pub async fn register_trade(&mut self) -> anyhow::Result<()> {
        let coordinator_pk = &self.escrow_contract.npubkey_coordinator;
        let message = serde_json::to_string(&self.escrow_contract)?;
        dbg!("sending contract to coordinator...");
        self.nostr_client
            .client
            .send_private_msg(*coordinator_pk, &message, None)
            .await?;

        let my_pubkey = self.nostr_client.public_key();
        let message = self
            .nostr_client
            .receive_escrow_message(my_pubkey, 10)
            .await?;
        self.escrow_registration = Some(serde_json::from_str(&message)?);
        Ok(())
    }

    /// Depending on the trade mode sends or receives the trade token.
    ///
    /// After this the state is token sent or received.
    pub async fn exchange_trade_token(&self) -> std::result::Result<(), anyhow::Error> {
        match self.trade_mode {
            TradeMode::Buyer => {
                // todo: store the sent token in this instance
                self.send_trade_token().await?;
                Ok(())
            }
            TradeMode::Seller => {
                // todo: store the received token in this instance
                self.receive_and_validate_trade_token().await?;
                Ok(())
            }
        }
    }

    /// State change for the buyer. The state after that is token sent.
    ///
    /// Returns the sent trade token by this [`EscrowClient`].
    async fn send_trade_token(&self) -> anyhow::Result<String> {
        let escrow_contract = &self.escrow_contract;
        let escrow_registration = self
            .escrow_registration
            .as_ref()
            .ok_or(anyhow!("Escrow registration not set, wrong state"))?;

        let escrow_token = self
            .ecash_wallet
            .create_escrow_token(escrow_contract, escrow_registration)
            .await?;

        debug!("Sending token to the seller: {}", escrow_token.as_str());

        self.nostr_client
            .client
            .send_private_msg(escrow_contract.npubkey_seller, &escrow_token, None)
            .await?;

        Ok(escrow_token)
    }

    /// State change for a seller. The state after this is token received.
    ///
    /// Returns the received trade token by this [`EscrowClient`].
    async fn receive_and_validate_trade_token(&self) -> anyhow::Result<Token> {
        let escrow_contract = &self.escrow_contract;
        let client_registration = self
            .escrow_registration
            .as_ref()
            .ok_or(anyhow!("Escrow registration not set, wrong state"))?;
        let wallet = &self.ecash_wallet;

        let message = self
            .nostr_client
            .receive_escrow_message(escrow_contract.npubkey_buyer, 10)
            .await?;
        wallet.validate_escrow_token(&message, escrow_contract, client_registration)
    }

    /// Depending on the trade mode deliver product/service or sign the token after receiving the service.
    ///
    /// The state after this operation is duties fulfilled.
    pub async fn do_your_trade_duties(&self) -> anyhow::Result<()> {
        // todo: as seller send product and proof of delivery (oracle) to seller.
        // await signature or begin dispute

        // todo: as buyer either send signature or begin dispute
        Ok(())
    }
}
