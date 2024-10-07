use cashu_escrow_client::ecash::ClientEcashWallet;
use cdk::{
    amount::{Amount, SplitTarget},
    wallet::{SendKind, Wallet},
};

#[inline]
pub(super) async fn create_wallet() -> anyhow::Result<ClientEcashWallet> {
    ClientEcashWallet::new("http://localhost:3338").await
}

pub(super) async fn check_mint_and_send(wallet: Wallet) {
    let mint_quote_result = wallet.mint_quote(Amount::from(1000)).await;
    assert!(mint_quote_result.is_ok());
    wallet
        .mint(&mint_quote_result.unwrap().id, SplitTarget::None, None)
        .await
        .unwrap();
    assert!(wallet.total_balance().await.unwrap() >= Amount::from(1000));

    let token_result = wallet
        .send(
            Amount::from(1000),
            Some("Test spend".to_string()),
            None,
            &SplitTarget::None,
            &SendKind::OnlineExact,
            true,
        )
        .await;
    assert!(token_result.is_ok());
}
