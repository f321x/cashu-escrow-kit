mod buyer_utils;
pub mod general_utils;
mod seller_utils;

use super::*;

pub struct ClientEscrowMetadata {
    pub escrow_coordinator_nostr_public_key: NostrPubkey,
    pub escrow_coordinator_ecash_public_key: Option<EcashPubkey>,
    pub escrow_start_timestamp: Option<Timestamp>,
    pub mode: TradeMode,
}

impl ClientEscrowMetadata {
    pub fn from_client_cli_input(cli_input: &ClientCliInput) -> anyhow::Result<Self> {
        Ok(Self {
            escrow_coordinator_nostr_public_key: cli_input.coordinator_nostr_pubkey,
            escrow_coordinator_ecash_public_key: None,
            escrow_start_timestamp: None,
            mode: cli_input.mode,
        })
    }
}

impl EscrowClient {
    pub async fn from_cli_input(cli_input: ClientCliInput) -> anyhow::Result<Self> {
        let escrow_contract = TradeContract::from_client_cli_input(&cli_input)?;
        let escrow_metadata = ClientEscrowMetadata::from_client_cli_input(&cli_input)?;
        let nostr_instance = ClientNostrInstance::from_client_cli_input(&cli_input).await?;
        let ecash_wallet = cli_input.ecash_wallet;

        Ok(Self {
            nostr_instance,
            ecash_wallet,
            escrow_metadata,
            escrow_contract,
        })
    }

    pub async fn init_trade(&mut self) -> anyhow::Result<()> {
        Self::common_trade_flow(self).await?;
        debug!("Common trade flow completed");

        match self.escrow_metadata.mode {
            TradeMode::Buyer => {
                self.buyer_pipeline().await?;
            }
            TradeMode::Seller => {
                self.seller_pipeline().await?;
            }
        };
        Ok(())
    }

    // the common trade flow is similar for both buyer and seller
    async fn common_trade_flow(&mut self) -> anyhow::Result<()> {
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

    async fn buyer_pipeline(&self) -> anyhow::Result<()> {
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

        // either send signature or begin dispute
        Ok(())
    }

    async fn seller_pipeline(&self) -> anyhow::Result<()> {
        let escrow_contract = &self.escrow_contract;
        let client_metadata = &self.escrow_metadata;
        let wallet = &self.ecash_wallet;

        let _escrow_token = self
            .nostr_instance
            .await_and_validate_escrow_token(wallet, escrow_contract, client_metadata)
            .await?;

        // send product and proof of delivery (oracle) to seller

        // await signature or begin dispute
        Ok(())
    }
}
