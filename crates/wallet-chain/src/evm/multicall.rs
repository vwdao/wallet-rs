use rust_decimal::Decimal;
use wallet_types::Address;

pub const MULTICALL3_ADDRESS: &str = "0xcA11bde05977b3631167028862bE2a173976CA11";

/// Selector for `aggregate3((address,bool,bytes)[])`
pub const AGGREGATE3_SELECTOR: [u8; 4] = [0x82, 0xad, 0x56, 0xcb];

/// Selector for `getEthBalance(address)`
pub const GET_ETH_BALANCE_SELECTOR: [u8; 4] = [0x4d, 0x23, 0x01, 0xcc];

/// Selector for `balanceOf(address)`
pub const BALANCE_OF_SELECTOR: [u8; 4] = [0x70, 0xa0, 0x82, 0x31];

/// Selector for `decimals()`
pub const DECIMALS_SELECTOR: [u8; 4] = [0x31, 0x3c, 0xe5, 0x67];

#[derive(Debug, Clone)]
pub struct Multicall3Call {
    pub target: Address,
    pub allow_failure: bool,
    pub call_data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Multicall3Result {
    pub success: bool,
    pub return_data: Vec<u8>,
}

/// Helper to parse a 20-byte address from a hex string
fn address_to_20_bytes(addr: &str) -> [u8; 20] {
    let clean = addr.trim_start_matches("0x").trim_start_matches("0X");
    let mut out = [0u8; 20];
    if let Ok(bytes) = hex::decode(clean) {
        if bytes.len() == 20 {
            out.copy_from_slice(&bytes);
        }
    }
    out
}

/// Build a Multicall3 call for `balanceOf(wallet)`
pub fn build_balance_of_call(token: &Address, wallet: &Address) -> Multicall3Call {
    let mut data = Vec::with_capacity(36);
    data.extend_from_slice(&BALANCE_OF_SELECTOR);
    let mut padded = [0u8; 32];
    let wallet_bytes = address_to_20_bytes(wallet.as_str());
    padded[12..32].copy_from_slice(&wallet_bytes);
    data.extend_from_slice(&padded);

    Multicall3Call {
        target: token.clone(),
        allow_failure: true,
        call_data: data,
    }
}

/// Build a Multicall3 call for `decimals()`
pub fn build_decimals_call(token: &Address) -> Multicall3Call {
    Multicall3Call {
        target: token.clone(),
        allow_failure: true,
        call_data: DECIMALS_SELECTOR.to_vec(),
    }
}

/// Build a Multicall3 call for native ETH balance `getEthBalance(wallet)`
pub fn build_eth_balance_call(wallet: &Address) -> Multicall3Call {
    let mut data = Vec::with_capacity(36);
    data.extend_from_slice(&GET_ETH_BALANCE_SELECTOR);
    let mut padded = [0u8; 32];
    let wallet_bytes = address_to_20_bytes(wallet.as_str());
    padded[12..32].copy_from_slice(&wallet_bytes);
    data.extend_from_slice(&padded);

    Multicall3Call {
        target: Address::new(MULTICALL3_ADDRESS),
        allow_failure: true,
        call_data: data,
    }
}

/// Encode `aggregate3((address,bool,bytes)[])`
pub fn encode_aggregate3(calls: &[Multicall3Call]) -> Vec<u8> {
    let mut out = Vec::new();
    // 4 bytes selector
    out.extend_from_slice(&AGGREGATE3_SELECTOR);

    // Offset to the calls array (always 32 bytes = 0x20)
    let mut offset_bytes = [0u8; 32];
    offset_bytes[31] = 0x20;
    out.extend_from_slice(&offset_bytes);

    // Array length (calls.len())
    let mut len_bytes = [0u8; 32];
    let n = calls.len() as u64;
    len_bytes[24..32].copy_from_slice(&n.to_be_bytes());
    out.extend_from_slice(&len_bytes);

    // Calculate offsets for each Call3 tuple
    let head_size = calls.len() * 32;
    let mut current_offset = head_size;
    let mut tuple_encoded_chunks = Vec::with_capacity(calls.len());

    for call in calls {
        // Record current offset in the heads section
        let mut off_b = [0u8; 32];
        let off_u64 = current_offset as u64;
        off_b[24..32].copy_from_slice(&off_u64.to_be_bytes());
        out.extend_from_slice(&off_b);

        // Encode the tuple: (target, allowFailure, callData)
        let mut chunk = Vec::new();
        // 1. target (address padded to 32 bytes)
        let mut target_word = [0u8; 32];
        let t_bytes = address_to_20_bytes(call.target.as_str());
        target_word[12..32].copy_from_slice(&t_bytes);
        chunk.extend_from_slice(&target_word);

        // 2. allowFailure (bool, 0 or 1, padded to 32 bytes)
        let mut allow_word = [0u8; 32];
        allow_word[31] = if call.allow_failure { 1 } else { 0 };
        chunk.extend_from_slice(&allow_word);

        // 3. offset to bytes callData relative to tuple start (always 3 words = 0x60 = 96)
        let mut data_offset_word = [0u8; 32];
        data_offset_word[31] = 0x60;
        chunk.extend_from_slice(&data_offset_word);

        // 4. length of callData
        let mut data_len_word = [0u8; 32];
        let d_len = call.call_data.len() as u64;
        data_len_word[24..32].copy_from_slice(&d_len.to_be_bytes());
        chunk.extend_from_slice(&data_len_word);

        // 5. callData bytes, padded right to 32 bytes
        chunk.extend_from_slice(&call.call_data);
        let rem = call.call_data.len() % 32;
        if rem != 0 {
            let pad_len = 32 - rem;
            chunk.extend(std::iter::repeat(0u8).take(pad_len));
        }

        current_offset += chunk.len();
        tuple_encoded_chunks.push(chunk);
    }

    // Append all encoded tuple bodies
    for chunk in tuple_encoded_chunks {
        out.extend_from_slice(&chunk);
    }

    out
}

/// Decode ABI-encoded `Result[]` returned by `aggregate3`: `(bool success, bytes returnData)[]`
pub fn decode_aggregate3(data: &[u8]) -> Option<Vec<Multicall3Result>> {
    if data.len() < 64 {
        return None;
    }

    // Offset to array (usually 0x20)
    let offset_to_array = usize_from_u256_slice(&data[0..32])?;
    if offset_to_array + 32 > data.len() {
        return None;
    }

    let array_len = usize_from_u256_slice(&data[offset_to_array..offset_to_array + 32])?;
    let elements_start = offset_to_array + 32;

    let mut results = Vec::with_capacity(array_len);
    for i in 0..array_len {
        let offset_pos = elements_start + i * 32;
        if offset_pos + 32 > data.len() {
            return None;
        }
        let tuple_rel_offset = usize_from_u256_slice(&data[offset_pos..offset_pos + 32])?;
        let tuple_start = elements_start + tuple_rel_offset;

        if tuple_start + 64 > data.len() {
            return None;
        }

        // success (bool)
        let success_val = data[tuple_start + 31] != 0;

        // returnData offset relative to tuple_start (usually 0x40 = 64)
        let return_data_rel_offset =
            usize_from_u256_slice(&data[tuple_start + 32..tuple_start + 64])?;
        let return_data_len_pos = tuple_start + return_data_rel_offset;

        if return_data_len_pos + 32 > data.len() {
            return None;
        }

        let return_data_len =
            usize_from_u256_slice(&data[return_data_len_pos..return_data_len_pos + 32])?;
        let return_data_bytes_pos = return_data_len_pos + 32;

        if return_data_bytes_pos + return_data_len > data.len() {
            return None;
        }

        let return_data =
            data[return_data_bytes_pos..return_data_bytes_pos + return_data_len].to_vec();
        results.push(Multicall3Result {
            success: success_val,
            return_data,
        });
    }

    Some(results)
}

/// Parse uint256 from 32-byte word into `Decimal`
pub fn parse_u256_to_decimal(bytes: &[u8]) -> Decimal {
    if bytes.len() < 32 {
        return Decimal::ZERO;
    }
    // Convert first 16 bytes check (overflow protection)
    let high = u128::from_be_bytes(bytes[0..16].try_into().unwrap_or_default());
    let low = u128::from_be_bytes(bytes[16..32].try_into().unwrap_or_default());
    if high > 0 {
        return Decimal::MAX;
    }
    wallet_types::decimal_from_u128(low)
}

/// Parse decimals uint8/uint256 from bytes
pub fn parse_decimals_from_bytes(bytes: &[u8]) -> u32 {
    if bytes.is_empty() {
        return 18;
    }
    if bytes.len() >= 32 {
        bytes[31] as u32
    } else {
        *bytes.last().unwrap_or(&18) as u32
    }
}

fn usize_from_u256_slice(slice: &[u8]) -> Option<usize> {
    if slice.len() < 32 {
        return None;
    }
    let val = u64::from_be_bytes(slice[24..32].try_into().ok()?);
    Some(val as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_and_decode_aggregate3_empty() {
        let calls = vec![];
        let encoded = encode_aggregate3(&calls);
        assert_eq!(&encoded[0..4], &AGGREGATE3_SELECTOR);
    }

    #[test]
    fn test_multicall3_encoding_structure() {
        let token = Address::new("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");
        let wallet = Address::new("0x1111111111111111111111111111111111111111");

        let call1 = build_decimals_call(&token);
        let call2 = build_balance_of_call(&token, &wallet);

        let encoded = encode_aggregate3(&[call1, call2]);
        assert_eq!(&encoded[0..4], &AGGREGATE3_SELECTOR);
        // Header (4 selector + 32 offset + 32 len + 32*2 offsets) = 4 + 128 = 132 bytes minimum
        assert!(encoded.len() > 132);
    }

    #[test]
    fn test_parse_u256_to_decimal() {
        let mut word = [0u8; 32];
        word[31] = 100;
        assert_eq!(parse_u256_to_decimal(&word), Decimal::from(100));

        let mut word2 = [0u8; 32];
        word2[30] = 1; // 256
        assert_eq!(parse_u256_to_decimal(&word2), Decimal::from(256));
    }
}
