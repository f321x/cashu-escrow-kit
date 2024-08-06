mod buyer_utils;
pub mod general_utils;
mod seller_utils;

use cdk::nuts::Token;

use super::*;

pub struct EscrowClientMetadata {
    pub escrow_coordinator_nostr_public_key: NostrPubkey,
    pub escrow_coordinator_ecash_public_key: Option<EcashPubkey>,
    pub escrow_start_timestamp: Option<Timestamp>,
    pub mode: TradeMode,
}

impl EscrowClientMetadata {
    pub fn from_client_cli_input(cli_input: &ClientCliInput) -> anyhow::Result<Self> {
        Ok(Self {
            escrow_coordinator_nostr_public_key: cli_input.coordinator_nostr_pubkey,
            escrow_coordinator_ecash_public_key: None,
            escrow_start_timestamp: None,
            mode: cli_input.mode,
        })
    }
}

pub struct EscrowClient {
    pub nostr_instance: ClientNostrInstance, // can either be a Nostr Client or Nostr note signer (without networking)
    pub ecash_wallet: ClientEcashWallet,
    pub escrow_metadata: EscrowClientMetadata, // data relevant for the application but not for the outcome of the trade contract
    pub escrow_contract: TradeContract,
}

// todo: model EscrowClient as an state machine (stm). This will improve testability too.
impl EscrowClient {
    // creates the inital state: the coordinator data isn't present.
    pub fn new(
        escrow_contract: TradeContract,
        escrow_metadata: EscrowClientMetadata,
        nostr_instance: ClientNostrInstance,
        ecash_wallet: ClientEcashWallet,
    ) -> Self {
        Self {
            escrow_contract,
            escrow_metadata,
            nostr_instance,
            ecash_wallet,
        }
    }

    /// The trade initialization is the same for both buyer and seller.
    ///
    /// After this the coordinator data is set, state trade registered.
    ///
    /// After this state the trade contract is effectfull as well, possible coordinator fees must be payed.
    pub async fn register_trade(&mut self) -> anyhow::Result<()> {
        let coordinator_pk = &self.escrow_metadata.escrow_coordinator_nostr_public_key;

        // submits the trade contract to the coordinator to initiate the escrow service
        self.nostr_instance
            .submit_escrow_contract(&self.escrow_contract, coordinator_pk)
            .await?;

        let escrow_coordinator_pk_ts: (EcashPubkey, Timestamp) = self
            .nostr_instance
            .get_escrow_coordinator_pk(coordinator_pk)
            .await?;

        self.escrow_metadata.escrow_coordinator_ecash_public_key = Some(escrow_coordinator_pk_ts.0);
        self.escrow_metadata.escrow_start_timestamp = Some(escrow_coordinator_pk_ts.1);
        Ok(())
    }

    /// Depending on the trade mode sends or receives the trade token.
    ///
    /// After this the state is token sent or received.
    pub async fn exchange_trade_token(&self) -> std::result::Result<(), anyhow::Error> {
        match self.escrow_metadata.mode {
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
        let client_metadata = &self.escrow_metadata;

        let escrow_token = self
            .ecash_wallet
            .create_escrow_token(escrow_contract, client_metadata)
            .await?;

        debug!("Sending token to the seller: {}", escrow_token.as_str());

        self.nostr_instance
            .submit_trade_token_to_seller(&escrow_contract.npub_seller, &escrow_token)
            .await?;

        Ok(escrow_token)
    }

    /// State change for a seller. The state after this is token received.
    ///
    /// Returns the received trade token by this [`EscrowClient`].
    async fn receive_and_validate_trade_token(&self) -> anyhow::Result<Token> {
        let escrow_contract = &self.escrow_contract;
        let client_metadata = &self.escrow_metadata;
        let wallet = &self.ecash_wallet;

        let escrow_token = self
            .nostr_instance
            // todo: split method in receive and validate steps, single responsability principle.
            .await_and_validate_escrow_token(wallet, escrow_contract, client_metadata)
            .await?;

        Ok(escrow_token)
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
