use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Solana JSON-RPC endpoint. When empty, `network` selects a public endpoint.
    #[serde(default)]
    pub rpc_url: Option<String>,
    /// mainnet | devnet | testnet. Only used when `rpc_url` is empty.
    #[serde(default = "default_network")]
    pub network: String,
    /// Private key of the funding wallet, base58-encoded 64-byte keypair.
    #[serde(default)]
    pub private_key: Option<String>,
    /// Alternative to `private_key`: path to a keypair file (solana-keygen JSON
    /// array of 64 bytes, or a base58 string).
    #[serde(default)]
    pub keypair_file: Option<String>,
    /// Token mints whose ATAs are ensured for each recipient.
    pub mints: Vec<String>,
    /// auto | spl | token-2022. auto resolves the token program from the mint
    /// account owner on-chain.
    #[serde(default = "default_token_program")]
    pub token_program: String,
    /// RPC commitment used for reads and for confirming submissions.
    #[serde(default = "default_commitment")]
    pub commitment: String,
    /// Skip preflight simulation before broadcasting.
    #[serde(default)]
    pub skip_preflight: bool,
    /// Minimal pause between JSON-RPC calls, to stay under rate limits.
    #[serde(default = "default_call_delay_ms")]
    pub call_delay_ms: u64,
    /// How many times to retry transient RPC errors (429 / node behind) with
    /// exponential backoff before giving up on a read call.
    #[serde(default = "default_rpc_max_retries")]
    pub rpc_max_retries: u32,
    /// How many times to resubmit a create-ATA transaction with a fresh
    /// blockhash when the previous submission failed (BlockhashNotFound / 429).
    #[serde(default = "default_send_max_retries")]
    pub send_max_retries: u32,
    /// Destination wallet addresses that will own the created ATAs.
    #[serde(default)]
    pub recipients: Vec<String>,
}

fn default_network() -> String {
    "mainnet".into()
}

fn default_token_program() -> String {
    "auto".into()
}

fn default_commitment() -> String {
    "confirmed".into()
}

fn default_call_delay_ms() -> u64 {
    150
}

fn default_rpc_max_retries() -> u32 {
    5
}

fn default_send_max_retries() -> u32 {
    4
}

impl Config {
    pub fn rpc_endpoint(&self) -> String {
        match &self.rpc_url {
            Some(url) if !url.trim().is_empty() => url.clone(),
            _ => default_rpc_for_network(&self.network),
        }
    }
}

fn default_rpc_for_network(network: &str) -> String {
    match network.trim() {
        "devnet" => "https://api.devnet.solana.com".into(),
        "testnet" => "https://api.testnet.solana.com".into(),
        _ => "https://api.mainnet-beta.solana.com".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wallet_config::load_yaml;

    #[test]
    fn config_deserializes() {
        let cfg: Config = load_yaml("../../configs/wallet-solana-ata.yaml")
            .expect("wallet-solana-ata.yaml must deserialize");
        assert!(!cfg.mints.is_empty());
        assert!(!cfg.recipients.is_empty());
        assert_eq!(cfg.token_program, "auto");
        assert_eq!(cfg.commitment, "confirmed");
        assert_eq!(cfg.call_delay_ms, 150);
        assert!(cfg.rpc_max_retries > 0);
        assert!(cfg.send_max_retries > 0);
        assert!(!cfg.rpc_endpoint().is_empty());
    }
}