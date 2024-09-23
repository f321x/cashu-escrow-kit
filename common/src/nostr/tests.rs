use super::*;

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Receive a message when only one mesage was sent by the escrow.
#[tokio::test]
async fn receive_1_message() -> anyhow::Result<()> {
    let mut buyer_nostr_client = create_nostr_client().await;
    let escrow_nostr_client = create_nostr_client().await;

    let msg1 = TestMessage1("message 1".to_string());
    escrow_nostr_client
        .client
        .send_private_msg(
            buyer_nostr_client.public_key(),
            serde_json::to_string(&msg1)?,
            None,
        )
        .await?;

    let result: TestMessage1 = buyer_nostr_client.receive_escrow_message(10).await?;
    assert_eq!(result, msg1);
    Ok(())
}

/// Send 2 messages and then receive them in reverse order, forcing to use the cache.
#[tokio::test]
async fn receive_2_messages_from_cache() -> anyhow::Result<()> {
    let mut buyer_nostr_client = create_nostr_client().await;
    let escrow_nostr_client = create_nostr_client().await;

    tokio::time::sleep(Duration::from_millis(50)).await; //needed to wait till the local test relay sets up both subxcriptions

    let msg1 = TestMessage1("message 1".to_string());
    escrow_nostr_client
        .client
        .send_private_msg(
            buyer_nostr_client.public_key(),
            serde_json::to_string(&msg1)?,
            None,
        )
        .await?;

    let msg2 = TestMessage2(1);
    escrow_nostr_client
        .client
        .send_private_msg(
            buyer_nostr_client.public_key(),
            serde_json::to_string(&msg2)?,
            None,
        )
        .await?;

    assert_eq!(buyer_nostr_client.messages_cache.len(), 0);
    let result_msg2: TestMessage2 = buyer_nostr_client.receive_escrow_message(10).await?;
    assert_eq!(result_msg2, msg2);
    assert_eq!(
        buyer_nostr_client.messages_cache.len(),
        1,
        "After receiving for the second sent message, must be 1 message left in the cache!"
    );

    let result_msg1: TestMessage1 = buyer_nostr_client.receive_escrow_message(10).await?;
    assert_eq!(result_msg1, msg1);
    assert_eq!(
        buyer_nostr_client.messages_cache.len(),
        0,
        "After receiving for the first sent message, must be no messages left in the cache!"
    );
    Ok(())
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
    assert_eq!(buyer_nostr_client.messages_cache.len(), CACHE_SIZE);
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestMessage1(String);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestMessage2(usize);

async fn create_nostr_client() -> NostrClient {
    let keys = Keys::generate();
    let relays: Vec<String> = vec!["ws://localhost:4736".to_string()];
    NostrClient::new(keys, relays).await.unwrap()
}
