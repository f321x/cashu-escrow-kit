use super::*;
use cashu_escrow_common::nostr::PubkeyMessage;

pub async fn parse_escrow_pk(pk_message_json: &str) -> anyhow::Result<(EcashPubkey, Timestamp)> {
    let pkm: PubkeyMessage = serde_json::from_str(pk_message_json)?;
    let coordinator_ecash_pk = EcashPubkey::from_hex(pkm.escrow_coordinator_pubkey)?;
    Ok((coordinator_ecash_pk, pkm.escrow_start_ts))
}
