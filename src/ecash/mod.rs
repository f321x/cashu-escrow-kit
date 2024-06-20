use super::*;

use cdk::{
    amount::SplitTarget,
    cdk_database::WalletMemoryDatabase,
    nuts::{Conditions, CurrencyUnit, PublicKey, SecretKey, SigFlag, SpendingConditions, Token},
    wallet::Wallet,
};
use std::str::FromStr;
use std::sync::Arc;

pub struct EcashWallet {
    secret: SecretKey,
    wallet: Wallet,
}

impl EcashWallet {
    pub async fn new() -> anyhow::Result<Self> {
        let localstore = WalletMemoryDatabase::default();
        let secret = SecretKey::generate();

        // let mint_url = UncheckedUrl::from_str("https://testnut.cashu.space").unwrap();
        // let unit = CurrencyUnit::Sat;
        // let amount = Amount::from(10);

        let wallet = Wallet::new(Arc::new(localstore), &seed, vec![]);

        // let quote = wallet
        //     .mint_quote(mint_url.clone(), unit.clone(), amount)
        //     .await
        //     .unwrap();

        // println!("Minting nuts ...");

        // loop {
        //     let status = wallet
        //         .mint_quote_status(mint_url.clone(), &quote.id)
        //         .await
        //         .unwrap();

        //     println!("Quote status: {}", status.paid);

        //     if status.paid {
        //         break;
        //     }

        //     sleep(Duration::from_secs(5)).await;
        // }

        // let _receive_amount = wallet
        //     .mint(mint_url.clone(), &quote.id, SplitTarget::default(), None)
        //     .await
        //     .unwrap();

        // let secret = SecretKey::generate();

        // let spending_conditions =
        //     SpendingConditions::new_p2pk(secret.public_key(), Conditions::default());

        // let token = wallet
        //     .send(
        //         &mint_url,
        //         unit,
        //         amount,
        //         None,
        //         Some(spending_conditions),
        //         &SplitTarget::None,
        //     )
        //     .await
        //     .unwrap();

        // println!("Created token locked to pubkey: {}", secret.public_key());
        // println!("{}", token);

        // wallet.add_p2pk_signing_key(secret).await;

        // let amount = wallet
        //     .receive(&token, &SplitTarget::default(), None)
        //     .await
        //     .unwrap();

        // println!("Redeamed locked token worth: {}", u64::from(amount));

        Ok(Self { secret, wallet })
    }

    async fn assemble_escrow_conditions(
        &self,
        trade: &TradeContract,
    ) -> anyhow::Result<SpendingConditions> {
        let buyer_pubkey = PublicKey::from_str(trade.buyer_ecash_public_key.as_str())?;
        let seller_pubkey = PublicKey::from_str(trade.seller_ecash_public_key.as_str())?;

        let public_keys = vec![buyer_pubkey, seller_pubkey, self.secret.public_key()];
        let spending_conditions = SpendingConditions::new_p2pk(
            self.secret.public_key(),
            Conditions::new(
                Some(trade.time_limit),
                Some(public_keys),
                Some(vec![buyer_pubkey]),
                Some(2),
                Some(SigFlag::SigAll),
            )?,
        );
        Ok(spending_conditions)
    }

    pub async fn create_escrow_token(
        &self,
        escrow_contract: &TradeContract,
    ) -> anyhow::Result<String> {
        let spending_conditions = self.assemble_escrow_conditions(escrow_contract).await?;
        let token = self
            .wallet
            .send(
                &cdk::UncheckedUrl::new(&escrow_contract.trade_mint_url),
                CurrencyUnit::Sat,
                escrow_contract.trade_amount_sat.into(),
                Some(escrow_contract.trade_description.clone()),
                Some(spending_conditions),
                &SplitTarget::None,
            )
            .await?;
        Ok(token)
    }

    pub async fn validate_escrow_token(
        &self,
        token: &String,
        contract: &TradeContract,
    ) -> anyhow::Result<Token> {
        let spending_conditions = self.assemble_escrow_conditions(contract).await?;
        let token = Token::from_str(&token)?;
        self.wallet.verify_token_p2pk(&token, spending_conditions)?;
        Ok(token)
    }
}
