mod common;

use common::{check_mint_and_send, create_wallet};

#[tokio::test]
async fn send_minted_ecash() {
    let wallet_result = create_wallet().await;
    assert!(wallet_result.is_ok());

    let wallet = wallet_result.unwrap().wallet;
    check_mint_and_send(wallet).await;
}
