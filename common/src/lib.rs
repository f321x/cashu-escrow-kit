pub mod cli;
pub mod model;
pub mod nostr;

mod cdk_pubkey_serde {
    use cdk::nuts::PublicKey;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(pk: &PublicKey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&pk.to_hex())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<PublicKey, D::Error>
    where
        D: Deserializer<'de>,
    {
        let pubkey_hex = String::deserialize(deserializer)?;
        PublicKey::from_hex(pubkey_hex).map_err(serde::de::Error::custom)
    }
}
