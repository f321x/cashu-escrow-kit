use super::*;

use cashu_escrow_common::model::{EscrowRegistration, TradeContract};
use cdk::{
    amount::SplitTarget,
    cdk_database::WalletMemoryDatabase,
    nuts::{Conditions, CurrencyUnit, PublicKey, SecretKey, SigFlag, SpendingConditions, Token},
    secp256k1::rand::Rng,
    wallet::Wallet,
};
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug)]
pub struct ClientEcashWallet {
    _secret: SecretKey,
    pub wallet: Wallet,
    pub trade_pubkey: String,
}

impl ClientEcashWallet {
    pub async fn new(mint_url: &str) -> anyhow::Result<Self> {
        let localstore = WalletMemoryDatabase::default();
        let _secret = SecretKey::generate();
        let trade_pubkey: String = _secret.public_key().to_string();
        let seed = rand::thread_rng().gen::<[u8; 32]>();
        info!("Trade ecash pubkey: {}", trade_pubkey);

        let wallet = Wallet::new(mint_url, CurrencyUnit::Sat, Arc::new(localstore), &seed);

        Ok(Self {
            _secret,
            wallet,
            trade_pubkey,
        })
    }

    fn assemble_escrow_conditions(
        &self,
        contract: &TradeContract,
        escrow_registration: &EscrowRegistration,
    ) -> anyhow::Result<SpendingConditions> {
        let seller_pubkey = PublicKey::from_str(&contract.seller_ecash_public_key)?;
        let buyer_pubkey = PublicKey::from_str(&contract.buyer_ecash_public_key)?;
        let coordinator_escrow_pubkey = escrow_registration.coordinator_escrow_pubkey;
        let start_timestamp = escrow_registration.escrow_start_time;

        let locktime = start_timestamp.as_u64() + contract.time_limit;

        let spending_conditions = SpendingConditions::new_p2pk(
            seller_pubkey,
            Some(Conditions::new(
                Some(locktime),
                Some(vec![buyer_pubkey, coordinator_escrow_pubkey]),
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
        escrow_registration: &EscrowRegistration,
    ) -> anyhow::Result<String> {
        let spending_conditions = self.assemble_escrow_conditions(contract, escrow_registration)?;
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
        escrow_registration: &EscrowRegistration,
    ) -> anyhow::Result<Token> {
        let spending_conditions = self.assemble_escrow_conditions(contract, escrow_registration)?;
        let token = Token::from_str(escrow_token)?;
        self.wallet.verify_token_p2pk(&token, spending_conditions)?;
        Ok(token)
    }
}
