use cashu_escrow_client::ecash::ClientEcashWallet;
use cdk::{amount::SplitTarget, wallet::SendKind, Amount};
use std::assert_eq;

#[tokio::test]
async fn mint_ecash() -> anyhow::Result<()> {
    let wallet_result = ClientEcashWallet::new("http://localhost:3338").await;
    assert!(wallet_result.is_ok());

    let wallet = wallet_result?.wallet;
    let mint_quote_result = wallet.mint_quote(Amount::from(1000)).await;
    assert!(mint_quote_result.is_ok());

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
    assert!(token_result.is_err());
    assert_eq!(
        &token_result.err().unwrap().to_string(),
        "Insufficient funds not expected"
    );
    Ok(())
}
