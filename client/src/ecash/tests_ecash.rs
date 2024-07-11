use cdk::wallet;

#[cfg(test)]
use super::super::EcashWallet;
use super::*;

async fn _new_testing_wallet() -> EcashWallet {
    let secret = SecretKey::generate();
    let trade_pubkey: String = secret.public_key().to_string();
    EcashWallet {
        secret,
        wallet: Wallet::new(
            "",
            CurrencyUnit::Sat,
            Arc::new(WalletMemoryDatabase::default()),
            &rand::thread_rng().gen::<[u8; 32]>(),
        ),
        trade_pubkey,
    }
}

async fn _get_dummy_escrow_user() -> EscrowUser {
    let trade_beginning_ts = Timestamp::from(1720724405);
    let escrow_coordintor_npub = "";
    let escrow_coordinator_cashu_pk = PublicKey::from_hex("").unwrap();
    let contract = TradeContract {
        trade_id: 0,
        buyer_ecash_public_key: "".to_string(),
        seller_ecash_public_key: "".to_string(),
        trade_amount_sat: 0,
        trade_description: "".to_string(),
        time_limit: 0,
        trade_beginning_ts: 0,
    };
    let wallet = _new_testing_wallet().await;
    let nostr_client = NostrClient::new().await.unwrap();
    EscrowUser {}
}

#[tokio::test]
async fn test_escrow_token_creation() {
    let wallet = _new_testing_wallet().await;
}

#[tokio::test]
async fn test_invalid_escrow_token_validation() {
    let wallet = _new_testing_wallet().await;
}

#[tokio::test]
async fn test_valid_escrow_token_validation() {
    let wallet = _new_testing_wallet().await;
}
