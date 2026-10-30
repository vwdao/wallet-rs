use chrono::Utc;
use std::collections::HashMap;
use wallet_error::AppResult;

#[derive(Debug, Clone)]
pub struct RecipientRules {
    pub version: String,
    pub similarity_prefix: usize,
    pub similarity_suffix: usize,
    pub black_holes: HashMap<i64, Vec<String>>,
}

impl Default for RecipientRules {
    fn default() -> Self {
        let mut black_holes = HashMap::new();
        // EVM chains (chain_index 60 + generic EVM 0x0 / 0xdead)
        black_holes.insert(
            60,
            vec![
                "0x0000000000000000000000000000000000000000".to_string(),
                "0x000000000000000000000000000000000000dead".to_string(),
            ],
        );
        // Bitcoin (chain_index 0)
        black_holes.insert(0, vec!["1111111111111111111114oLvT2".to_string()]);
        // TRON (chain_index 195)
        black_holes.insert(
            195,
            vec!["T9yD14Nj9j7xAB4dbGeiX9h8unkKHxuWwb".to_string()],
        );
        // Solana (chain_index 501)
        black_holes.insert(
            501,
            vec!["11111111111111111111111111111111".to_string()],
        );

        Self {
            version: "1".to_string(),
            similarity_prefix: 4,
            similarity_suffix: 4,
            black_holes,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RecipientCheckRequest {
    pub chain_index: i64,
    pub address: String,
    pub sender_address: String,
    pub current_token_contract: String,
    pub reference_addresses: Vec<String>,
    pub trusted_recipient: bool,
}

#[derive(Debug, Clone)]
pub struct RecipientCheckItem {
    pub kind: String,
    pub outcome: String,
    pub action: String,
    pub source: String,
    pub checked_at_ms: i64,
    pub expires_at_ms: i64,
}

#[derive(Debug, Clone)]
pub struct RecipientCheckResult {
    pub action: String, // pass, warn, block
    pub reason: String,
    pub complete: bool,
    pub rules_version: String,
    pub valid_until_ms: i64,
    pub items: Vec<RecipientCheckItem>,
    pub similarity_prefix: i32,
    pub similarity_suffix: i32,
    pub black_hole_addresses: Vec<String>,
}

pub struct RecipientChecker {
    rules: RecipientRules,
}

impl RecipientChecker {
    pub fn new() -> Self {
        Self {
            rules: RecipientRules::default(),
        }
    }

    pub fn with_rules(rules: RecipientRules) -> Self {
        Self { rules }
    }

    pub fn is_evm_chain(chain_index: i64) -> bool {
        // Standard EVM chain_indexes: 60 (ETH), 56 (BSC), 137 (Polygon), 42161 (Arb), 10 (OP), etc.
        chain_index != 0 && chain_index != 195 && chain_index != 501 && chain_index != 3
    }

    pub fn normalize_address(chain_index: i64, address: &str) -> String {
        let trimmed = address.trim();
        if Self::is_evm_chain(chain_index) {
            trimmed.to_lowercase()
        } else {
            trimmed.to_string()
        }
    }

    pub fn validate_address(chain_index: i64, address: &str) -> bool {
        let addr = address.trim();
        if addr.is_empty() {
            return false;
        }

        if Self::is_evm_chain(chain_index) {
            let hex_part = addr.strip_prefix("0x").or_else(|| addr.strip_prefix("0X"));
            match hex_part {
                Some(h) => h.len() == 40 && h.chars().all(|c| c.is_ascii_hexdigit()),
                None => false,
            }
        } else if chain_index == 195 {
            // TRON address: typically starts with T, 34 base58 characters
            addr.starts_with('T') && addr.len() == 34
        } else if chain_index == 501 {
            // Solana: Base58 string, 32-44 chars
            addr.len() >= 32 && addr.len() <= 44
        } else {
            // UTXO or other chains
            addr.len() >= 26 && addr.len() <= 100
        }
    }

    pub fn check_similarity(
        chain_index: i64,
        address: &str,
        references: &[String],
        prefix: usize,
        suffix: usize,
    ) -> bool {
        let a = Self::normalize_address(chain_index, address);
        if a.len() <= prefix + suffix {
            return false;
        }

        for r in references {
            let b = Self::normalize_address(chain_index, r);
            if a != b && b.len() > prefix + suffix {
                let a_pre = &a[..prefix];
                let a_suf = &a[a.len() - suffix..];
                if b.starts_with(a_pre) && b.ends_with(a_suf) {
                    return true;
                }
            }
        }
        false
    }

    pub fn check(&self, req: &RecipientCheckRequest) -> AppResult<RecipientCheckResult> {
        let now = Utc::now().timestamp_millis();
        let valid_until = now + 30_000; // 30 seconds

        let target = Self::normalize_address(req.chain_index, &req.address);
        let sender = Self::normalize_address(req.chain_index, &req.sender_address);
        let token_contract = Self::normalize_address(req.chain_index, &req.current_token_contract);

        let mut items = Vec::new();
        let mut action = "pass".to_string();
        let mut reason = String::new();

        // 1. Validation check
        if !Self::validate_address(req.chain_index, &req.address) {
            items.push(RecipientCheckItem {
                kind: "formatValidation".to_string(),
                outcome: "matched".to_string(),
                action: "block".to_string(),
                source: "rules".to_string(),
                checked_at_ms: now,
                expires_at_ms: valid_until,
            });
            return Ok(RecipientCheckResult {
                action: "block".to_string(),
                reason: "invalidRecipientFormat".to_string(),
                complete: true,
                rules_version: self.rules.version.clone(),
                valid_until_ms: valid_until,
                items,
                similarity_prefix: self.rules.similarity_prefix as i32,
                similarity_suffix: self.rules.similarity_suffix as i32,
                black_hole_addresses: vec![],
            });
        }

        // 2. Token contract match check (prevent sending funds directly to token contract)
        if !token_contract.is_empty() && target == token_contract {
            items.push(RecipientCheckItem {
                kind: "tokenContract".to_string(),
                outcome: "matched".to_string(),
                action: "block".to_string(),
                source: "request".to_string(),
                checked_at_ms: now,
                expires_at_ms: valid_until,
            });
            action = "block".to_string();
            reason = "tokenContract".to_string();
        }

        // 3. Self-transfer check
        if !sender.is_empty() && target == sender {
            items.push(RecipientCheckItem {
                kind: "selfTransfer".to_string(),
                outcome: "matched".to_string(),
                action: "warn".to_string(),
                source: "request".to_string(),
                checked_at_ms: now,
                expires_at_ms: valid_until,
            });
            if action == "pass" {
                action = "warn".to_string();
                reason = "selfTransfer".to_string();
            }
        }

        // 4. Black hole check
        let mut black_holes = Vec::new();
        if let Some(list) = self.rules.black_holes.get(&req.chain_index) {
            black_holes.extend(list.clone());
        }
        if Self::is_evm_chain(req.chain_index) {
            if let Some(evm_list) = self.rules.black_holes.get(&60) {
                for bh in evm_list {
                    if !black_holes.contains(bh) {
                        black_holes.push(bh.clone());
                    }
                }
            }
        }

        for bh in &black_holes {
            if target == Self::normalize_address(req.chain_index, bh) {
                items.push(RecipientCheckItem {
                    kind: "blackHole".to_string(),
                    outcome: "matched".to_string(),
                    action: "block".to_string(),
                    source: "rules".to_string(),
                    checked_at_ms: now,
                    expires_at_ms: valid_until,
                });
                action = "block".to_string();
                reason = "blackHole".to_string();
                break;
            }
        }

        // 5. Similarity phishing attack detection
        if Self::check_similarity(
            req.chain_index,
            &req.address,
            &req.reference_addresses,
            self.rules.similarity_prefix,
            self.rules.similarity_suffix,
        ) {
            items.push(RecipientCheckItem {
                kind: "phishingSimilarity".to_string(),
                outcome: "matched".to_string(),
                action: "warn".to_string(),
                source: "similarity".to_string(),
                checked_at_ms: now,
                expires_at_ms: valid_until,
            });
            if action == "pass" {
                action = "warn".to_string();
                reason = "phishingSimilarity".to_string();
            }
        }

        // 6. Chain-specific checks
        if req.chain_index == 195 {
            // TRON activation check note
            items.push(RecipientCheckItem {
                kind: "tronActivation".to_string(),
                outcome: "not_matched".to_string(),
                action: "none".to_string(),
                source: "chain".to_string(),
                checked_at_ms: now,
                expires_at_ms: valid_until,
            });
        }

        Ok(RecipientCheckResult {
            action,
            reason,
            complete: true,
            rules_version: self.rules.version.clone(),
            valid_until_ms: valid_until,
            items,
            similarity_prefix: self.rules.similarity_prefix as i32,
            similarity_suffix: self.rules.similarity_suffix as i32,
            black_hole_addresses: black_holes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blackhole_detection() {
        let checker = RecipientChecker::new();
        let req = RecipientCheckRequest {
            chain_index: 60,
            address: "0x0000000000000000000000000000000000000000".to_string(),
            sender_address: "0x1111111111111111111111111111111111111111".to_string(),
            current_token_contract: "".to_string(),
            reference_addresses: vec![],
            trusted_recipient: false,
        };
        let res = checker.check(&req).unwrap();
        assert_eq!(res.action, "block");
        assert_eq!(res.reason, "blackHole");
    }

    #[test]
    fn test_similarity_attack() {
        let checker = RecipientChecker::new();
        let legitimate = "0x1234567890abcdef1234567890abcdef1234abcd".to_string();
        let attacker = "0x123400000000000000000000000000000000abcd".to_string();
        let req = RecipientCheckRequest {
            chain_index: 60,
            address: attacker,
            sender_address: "0x9999999999999999999999999999999999999999".to_string(),
            current_token_contract: "".to_string(),
            reference_addresses: vec![legitimate],
            trusted_recipient: false,
        };
        let res = checker.check(&req).unwrap();
        assert_eq!(res.action, "warn");
        assert_eq!(res.reason, "phishingSimilarity");
    }
}
