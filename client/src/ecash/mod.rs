mod tests_ecash;

use super::*;

use cdk::secp256k1::rand::Rng;
use cdk::{
    amount::SplitTarget,
    cdk_database::WalletMemoryDatabase,
    nuts::{Conditions, CurrencyUnit, PublicKey, SecretKey, SigFlag, SpendingConditions, Token},
    wallet::Wallet,
};
use escrow_client::EscrowUser;
use std::str::FromStr;
use std::sync::Arc;

pub struct EcashWallet {
    secret: SecretKey,
    pub wallet: Wallet,
    pub trade_pubkey: String,
}

impl EcashWallet {
    pub async fn new() -> anyhow::Result<Self> {
        let localstore = WalletMemoryDatabase::default();
        let secret = SecretKey::generate();
        let trade_pubkey: String = secret.public_key().to_string();
        let seed = rand::thread_rng().gen::<[u8; 32]>();
        println!("Trade ecash pubkey: {}", trade_pubkey);

        let wallet = Wallet::new("", CurrencyUnit::Sat, Arc::new(localstore), &seed);

        Ok(Self {
            secret,
            wallet,
            trade_pubkey,
        })
    }

    async fn assemble_escrow_conditions(
        &self,
        user: &EscrowUser,
    ) -> anyhow::Result<SpendingConditions> {
        let buyer_pubkey = PublicKey::from_str(user.contract.buyer_ecash_public_key.as_str())?;
        let seller_pubkey = PublicKey::from_str(user.contract.seller_ecash_public_key.as_str())?;
        let coordinator_pubkey = user.escrow_coordinator_cashu_pk.clone();

        let spending_conditions = SpendingConditions::new_p2pk(
            seller_pubkey,
            Some(Conditions::new(
                Some(user.contract.time_limit),
                Some(vec![buyer_pubkey, coordinator_pubkey]),
                Some(vec![buyer_pubkey]),
                Some(2),
                Some(SigFlag::SigAll),
            )?),
        );
        Ok(spending_conditions)
    }

    pub async fn create_escrow_token(&self, user: &EscrowUser) -> anyhow::Result<String> {
        let spending_conditions = self.assemble_escrow_conditions(user).await?;
        let token = self
            .wallet
            .send(
                user.contract.trade_amount_sat.into(),
                Some(user.contract.trade_description.clone()),
                Some(spending_conditions),
                &SplitTarget::None,
            )
            .await?;
        Ok(token)
    }

    pub async fn validate_escrow_token(
        &self,
        token: &String,
        user: &EscrowUser,
    ) -> anyhow::Result<Token> {
        let spending_conditions = self.assemble_escrow_conditions(user).await?;
        let token = Token::from_str(&token)?;
        self.wallet.verify_token_p2pk(&token, spending_conditions)?;
        Ok(token)
    }
}
