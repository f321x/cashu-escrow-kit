use super::*;
use cashu_escrow_common::nostr::RegistrationMessage;

pub async fn parse_registration_message(
    pk_message_json: &str,
) -> anyhow::Result<RegistrationMessage> {
    Ok(serde_json::from_str(pk_message_json)?)
}
