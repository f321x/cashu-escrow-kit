use super::*;

pub struct ClientNostrClient {
    pub nostr_client: NostrClient,
}

impl ClientNostrClient {
    pub async fn from_client_cli_input(cli_input: &ClientCliInput) -> anyhow::Result<Self> {
        let nostr_client = NostrClient::new(
            &cli_input
                .trader_nostr_keys
                .secret_key()
                .unwrap()
                .to_bech32()?,
        )
        .await?;
        Ok(Self { nostr_client })
    }
}
