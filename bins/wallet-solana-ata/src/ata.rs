use anyhow::{anyhow, bail, Context, Result};
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use spl_associated_token_account::get_associated_token_address_with_program_id;
use std::str::FromStr;
use std::time::Duration;

use crate::config::Config;
use crate::rpc::{CallError, Confirmation, Rpc};

const SPL_TOKEN_PROGRAM: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const TOKEN_2022_PROGRAM: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";

pub async fn run(cfg: &Config) -> Result<()> {
    if cfg.mints.is_empty() {
        bail!("config has no mints");
    }
    let payer = load_keypair(cfg)?;
    let payer_pubkey = payer.pubkey();
    let rpc = Rpc::new(
        cfg.rpc_endpoint(),
        cfg.commitment.clone(),
        cfg.skip_preflight,
        cfg.call_delay_ms,
        cfg.rpc_max_retries,
    );

    let recipients: Vec<(usize, Pubkey)> = cfg
        .recipients
        .iter()
        .enumerate()
        .filter_map(|(i, raw)| match Pubkey::from_str(raw.trim()) {
            Ok(p) => Some((i, p)),
            Err(e) => {
                tracing::error!(index = i + 1, recipient = %raw, error = %e, "invalid recipient, skipping");
                None
            }
        })
        .collect();

    tracing::info!(
        payer = %payer_pubkey,
        mints = cfg.mints.len(),
        recipients = cfg.recipients.len(),
        valid_recipients = recipients.len(),
        rpc = %cfg.rpc_endpoint(),
        "ensuring associated token accounts"
    );

    for (m_idx, mint_raw) in cfg.mints.iter().enumerate() {
        let mint_total = cfg.mints.len();
        let mint = match Pubkey::from_str(mint_raw.trim()) {
            Ok(p) => p,
            Err(e) => {
                tracing::error!(mint_index = m_idx + 1, mint_total, mint = %mint_raw, error = %e, "invalid mint, skipping");
                continue;
            }
        };

        let token_program = match resolve_token_program(&rpc, &mint, &cfg.token_program).await {
            Ok(tp) => tp,
            Err(e) => {
                tracing::error!(mint_index = m_idx + 1, mint_total, mint = %mint, error = %e, "cannot resolve token program, skipping mint");
                continue;
            }
        };

        tracing::info!(
            mint_index = m_idx + 1,
            mint_total,
            mint = %mint,
            token_program = %token_program,
            "preparing mint"
        );

        let recipient_total = recipients.len();
        for &(orig_idx, ref recipient) in recipients.iter() {
            let ata = derive_ata(recipient, &mint, &token_program);

            match rpc.get_account(&ata).await {
                Ok(Some(acct)) => {
                    tracing::info!(
                        recipient_index = orig_idx + 1,
                        recipient_total,
                        recipient = %recipient,
                        mint = %mint,
                        ata = %ata,
                        lamports = acct.lamports,
                        owner = %acct.owner,
                        "ATA already exists, skipping"
                    );
                }
                Ok(None) => {
                    let result = create_ata(
                        &rpc,
                        &payer,
                        recipient,
                        &mint,
                        &token_program,
                        &ata,
                        cfg.send_max_retries,
                    )
                    .await;
                    match result {
                        Ok((sig, Confirmation::Confirmed)) => {
                            tracing::info!(
                                recipient_index = orig_idx + 1,
                                recipient_total,
                                recipient = %recipient,
                                mint = %mint,
                                ata = %ata,
                                signature = %sig,
                                "ATA created"
                            );
                        }
                        Ok((sig, Confirmation::TimedOut)) => {
                            tracing::warn!(
                                recipient_index = orig_idx + 1,
                                recipient_total,
                                recipient = %recipient,
                                mint = %mint,
                                ata = %ata,
                                signature = %sig,
                                "ATA created tx sent but confirmation timed out"
                            );
                        }
                        Err(e) => {
                            tracing::error!(
                                recipient_index = orig_idx + 1,
                                recipient_total,
                                recipient = %recipient,
                                mint = %mint,
                                ata = %ata,
                                error = %e,
                                "failed to create ATA"
                            );
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(
                        recipient_index = orig_idx + 1,
                        recipient_total,
                        recipient = %recipient,
                        mint = %mint,
                        ata = %ata,
                        error = %e,
                        "failed to check ATA existence"
                    );
                }
            }
        }
    }
    Ok(())
}

fn derive_ata(owner: &Pubkey, mint: &Pubkey, token_program: &Pubkey) -> Pubkey {
    get_associated_token_address_with_program_id(owner, mint, token_program)
}

fn create_ata_instruction(
    payer: &Pubkey,
    owner: &Pubkey,
    mint: &Pubkey,
    token_program: &Pubkey,
) -> Instruction {
    spl_associated_token_account::instruction::create_associated_token_account_idempotent(
        payer, owner, mint, token_program,
    )
}

async fn create_ata(
    rpc: &Rpc,
    payer: &Keypair,
    owner: &Pubkey,
    mint: &Pubkey,
    token_program: &Pubkey,
    ata: &Pubkey,
    send_max_retries: u32,
) -> Result<(solana_sdk::signature::Signature, Confirmation)> {
    let mut attempt = 0u32;
    loop {
        let blockhash = rpc.get_latest_blockhash().await?;
        let ix = create_ata_instruction(&payer.pubkey(), owner, mint, token_program);
        let tx =
            Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[payer], blockhash);

        let sent = rpc.send_transaction(&tx).await;
        match &sent {
            Ok(sig) => {
                tracing::info!(signature = %sig, ata = %ata, "created ATA tx submitted");
                let confirmation = rpc.confirm(sig, Duration::from_secs(30)).await?;
                return Ok((*sig, confirmation));
            }
            Err(CallError::BlockhashNotFound) => {
                attempt += 1;
                if attempt >= send_max_retries {
                    bail!("sendTransaction failed after {send_max_retries} attempts: blockhash not found");
                }
                tracing::warn!(
                    attempt,
                    backoff_ms = 1000 * 2u64.pow(attempt),
                    ata = %ata,
                    "blockhash expired, resubmitting with a fresh blockhash"
                );
                tokio::time::sleep(Duration::from_millis(1000 * 2u64.pow(attempt))).await;
            }
            Err(CallError::Transient(msg)) => {
                attempt += 1;
                if attempt >= send_max_retries {
                    bail!("sendTransaction failed after {send_max_retries} attempts: {msg}");
                }
                tracing::warn!(
                    attempt,
                    backoff_ms = 1000 * 2u64.pow(attempt),
                    error = %msg,
                    ata = %ata,
                    "resubmitting ATA tx with a fresh blockhash"
                );
                tokio::time::sleep(Duration::from_millis(1000 * 2u64.pow(attempt))).await;
            }
            Err(CallError::Rejected(msg)) => {
                return Err(anyhow!("sendTransaction rejected: {msg}"));
            }
        }
    }
}

async fn resolve_token_program(rpc: &Rpc, mint: &Pubkey, configured: &str) -> Result<Pubkey> {
    let spl = Pubkey::from_str(SPL_TOKEN_PROGRAM).unwrap();
    let token_2022 = Pubkey::from_str(TOKEN_2022_PROGRAM).unwrap();
    match configured.trim() {
        "spl" | "spl-token" => Ok(spl),
        "token-2022" | "token2022" => Ok(token_2022),
        "auto" | "" => {
            let acct = rpc
                .get_account(mint)
                .await?
                .ok_or_else(|| anyhow!("mint account {mint} not found on chain"))?;
            if acct.owner == spl || acct.owner == token_2022 {
                return Ok(acct.owner);
            }
            bail!("mint {mint} is owned by {}, not a known token program", acct.owner)
        }
        other => bail!("unknown token_program {other:?} (expect auto|spl|token-2022)"),
    }
}

fn load_keypair(cfg: &Config) -> Result<Keypair> {
    if let Some(key) = cfg.private_key.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        return decode_keypair(key).context("invalid private_key");
    }
    if let Some(path) = cfg.keypair_file.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        let raw = std::fs::read_to_string(path).with_context(|| format!("read keypair file {path}"))?;
        let key = raw.trim();
        if key.starts_with('[') {
            let bytes: Vec<u8> =
                serde_json::from_str(key).with_context(|| format!("parse keypair JSON in {path}"))?;
            return Keypair::try_from(bytes.as_slice()).context("invalid keypair bytes");
        }
        return decode_keypair(key).context("invalid keypair_file");
    }
    bail!("neither `private_key` nor `keypair_file` is configured")
}

fn decode_keypair(key: &str) -> Result<Keypair> {
    let bytes = bs58::decode(key)
        .into_vec()
        .context("keypair must be valid base58")?;
    match bytes.len() {
        64 => Keypair::try_from(bytes.as_slice()).context("invalid keypair"),
        n => bail!("expected a 64-byte base58 keypair, got {n} bytes"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ATA_PROGRAM: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

    #[test]
    fn drives_standard_pda_derivation() {
        let mint = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        let owner = Pubkey::from_str("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin").unwrap();
        let spl = Pubkey::from_str(SPL_TOKEN_PROGRAM).unwrap();
        let ata_program = Pubkey::from_str(ATA_PROGRAM).unwrap();

        let ata = derive_ata(&owner, &mint, &spl);
        let (manual, _bump) = Pubkey::find_program_address(
            &[owner.as_ref(), spl.as_ref(), mint.as_ref()],
            &ata_program,
        );
        assert_eq!(ata, manual);
    }

    #[test]
    fn spl_and_token_2022_derivations_differ() {
        let mint = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        let owner = Pubkey::from_str("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin").unwrap();
        let spl = Pubkey::from_str(SPL_TOKEN_PROGRAM).unwrap();
        let token_2022 = Pubkey::from_str(TOKEN_2022_PROGRAM).unwrap();

        let spl_ata = derive_ata(&owner, &mint, &spl);
        let t22_ata = derive_ata(&owner, &mint, &token_2022);
        assert_ne!(spl_ata, t22_ata);
    }

    #[test]
    fn rejects_invalid_base58_keypair() {
        assert!(decode_keypair("REPLACE_WITH_BASE58_KEYPAIR").is_err());
        assert!(decode_keypair("1").is_err());
        assert!(decode_keypair("").is_err());
    }

    #[test]
    fn decodes_64_byte_base58_keypair() {
        let kp = Keypair::new();
        let encoded = bs58::encode(kp.to_bytes()).into_string();
        let decoded = decode_keypair(&encoded).expect("valid keypair must decode");
        assert_eq!(decoded.to_bytes(), kp.to_bytes());
        assert_eq!(decoded.pubkey(), kp.pubkey());
    }

    #[test]
    fn idempotent_instruction_targets_derived_ata() {
        let mint = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        let owner = Pubkey::from_str("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin").unwrap();
        let payer = Pubkey::from_str("EnqkLXQv6qMfaX3REHNUqzqydE1eV1XSqNZQ8bBp8cJY").unwrap();
        let spl = Pubkey::from_str(SPL_TOKEN_PROGRAM).unwrap();
        let ata_program = Pubkey::from_str(ATA_PROGRAM).unwrap();

        let ix = create_ata_instruction(&payer, &owner, &mint, &spl);
        assert_eq!(ix.program_id, ata_program);
        assert_eq!(ix.accounts.len(), 6);
        assert_eq!(ix.accounts[0].pubkey, payer);
        assert_eq!(ix.accounts[1].pubkey, derive_ata(&owner, &mint, &spl));
        assert_eq!(ix.accounts[2].pubkey, owner);
        assert_eq!(ix.accounts[3].pubkey, mint);
        assert!(ix.accounts[0].is_signer && ix.accounts[0].is_writable);
        assert_eq!(ix.data[0], 1);
    }
}