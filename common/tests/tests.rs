mod common;

use cashu_escrow_common::nostr::CACHE_SIZE;
use common::*;

/// Receive a message when only one message was sent by the escrow.
#[tokio::test]
pub async fn receive_1_message() -> anyhow::Result<()> {
    test_receive_1_message().await
}

/// Send 2 messages and then receive them in reverse order, forcing to use the cache.
#[tokio::test]
async fn receive_2_messages_from_cache() -> anyhow::Result<()> {
    test_receive_2_messages_from_cache().await
}

#[tokio::test]
async fn spam_protection() -> anyhow::Result<()> {
    let mut buyer_nostr_client = create_nostr_client().await;
    let escrow_nostr_client = create_nostr_client().await;

    for i in 0..CACHE_SIZE + 1 {
        escrow_nostr_client
            .client
            .send_private_msg(
                buyer_nostr_client.public_key(),
                serde_json::to_string(&TestMessage2(i))?,
                None,
            )
            .await?;
    }

    let msg_res = buyer_nostr_client
        .receive_escrow_message::<TestMessage1>(10)
        .await;
    assert!(msg_res.is_err()); //Timeout
    assert_eq!(buyer_nostr_client.messages_cache_len(), CACHE_SIZE);
    Ok(())
}
