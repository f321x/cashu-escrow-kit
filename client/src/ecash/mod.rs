use super::*;

use cdk::nuts::PublicKey as EscrowPubkey;
use cdk::{
    amount::SplitTarget,
    cdk_database::WalletMemoryDatabase,
    nuts::{Conditions, CurrencyUnit, SecretKey, SigFlag, SpendingConditions, Token},
    secp256k1::rand::Rng,
    wallet::Wallet,
};
use std::str::FromStr;
use std::sync::Arc;

pub struct ClientEcashWallet {
    secret: SecretKey,
    pub wallet: Wallet,
    pub trade_pubkey: String,
}

impl ClientEcashWallet {
    pub async fn new(mint_url: &str) -> anyhow::Result<Self> {
        let localstore = WalletMemoryDatabase::default();
        let secret = SecretKey::generate();
        let trade_pubkey: String = secret.public_key().to_string();
        let seed = rand::thread_rng().gen::<[u8; 32]>();
        info!("Trade ecash pubkey: {}", trade_pubkey);

        let wallet = Wallet::new(mint_url, CurrencyUnit::Sat, Arc::new(localstore), &seed);

        Ok(Self {
            secret,
            wallet,
            trade_pubkey,
        })
    }

    fn assemble_escrow_conditions(
        &self,
        contract: &TradeContract,
        escrow_metadata: &ClientEscrowMetadata,
    ) -> anyhow::Result<SpendingConditions> {
        let seller_pubkey = EscrowPubkey::from_str(&contract.seller_ecash_public_key)?;
        let buyer_pubkey = EscrowPubkey::from_str(&contract.buyer_ecash_public_key)?;
        let escrow_pubkey = escrow_metadata
            .escrow_coordinator_ecash_public_key
            .ok_or(anyhow!("Escrow coordinator ecash public key not set"))?;
        let start_timestamp = escrow_metadata
            .escrow_start_timestamp
            .ok_or(anyhow!("Escrow start timestamp not set"))?;

        let locktime = start_timestamp.as_u64() + contract.time_limit;

        let spending_conditions = SpendingConditions::new_p2pk(
            seller_pubkey,
            Some(Conditions::new(
                Some(locktime),
                Some(vec![buyer_pubkey, escrow_pubkey]),
                Some(vec![buyer_pubkey]),
                Some(2),
                Some(SigFlag::SigAll),
            )?),
        );
        Ok(spending_conditions)
    }

    pub async fn create_escrow_token(
        &self,
        contract: &TradeContract,
        escrow_metadata: &ClientEscrowMetadata,
    ) -> anyhow::Result<String> {
        let spending_conditions = self.assemble_escrow_conditions(contract, escrow_metadata)?;
        let token = self
            .wallet
            .send(
                contract.trade_amount_sat.into(),
                Some(contract.trade_description.clone()),
                Some(spending_conditions),
                &SplitTarget::None,
            )
            .await?;
        Ok(token)
    }

    pub fn validate_escrow_token(
        &self,
        escrow_token: &str,
        contract: &TradeContract,
        escrow_metadata: &ClientEscrowMetadata,
    ) -> anyhow::Result<Token> {
        let spending_conditions = self.assemble_escrow_conditions(contract, escrow_metadata)?;
        let token = Token::from_str(escrow_token)?;
        self.wallet.verify_token_p2pk(&token, spending_conditions)?;
        Ok(token)
    }
}
