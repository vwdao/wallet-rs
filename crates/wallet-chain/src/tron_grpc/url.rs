//! gRPC URL parsing for chain-gateway endpoints.
//!
//! Recognises endpoints of the form `grpc(s)://host[:port]/rpc/<chain>/<apiKey>` and
//! returns the components used by the rest of the tron module to dial the
//! gateway's gRPC port and pass the api key in metadata.

/// Parsed components of a chain-gateway gRPC endpoint URL of the form
/// `grpc(s)://host[:port]/rpc/<chain>/<apiKey>`.
#[derive(Debug, Clone)]
pub(crate) struct GatewayGrpcUrl {
    /// The underlying HTTP/2 endpoint (`grpc://` → `http://`, `grpcs://` → `https://`).
    pub http_url: String,
    /// Logical chain name, e.g. `tron`. Used to detect accidental mismatches
    /// when the URL is paired with a non-Tron family.
    pub chain: String,
    /// API key sent as `x-api-key` / `TRON-PRO-API-KEY` metadata.
    pub api_key: String,
}

/// Returns `Some` if `raw` is a chain-gateway gRPC URL we can route to.
pub(crate) fn parse_gateway_grpc_url(raw: &str) -> Option<GatewayGrpcUrl> {
    let (grpc_scheme, rest) = if let Some(r) = raw.strip_prefix("grpcs://") {
        ("grpcs", r)
    } else if let Some(r) = raw.strip_prefix("grpc://") {
        ("grpc", r)
    } else {
        return None;
    };
    let (authority, path) = match rest.split_once('/') {
        Some((a, p)) => (a, p),
        None => (rest, ""),
    };
    if authority.is_empty() {
        return None;
    }
    let segments: Vec<&str> = path
        .trim_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();
    if segments.len() < 3 || segments[0] != "rpc" {
        return None;
    }
    let http_scheme = if grpc_scheme == "grpcs" { "https" } else { "http" };
    Some(GatewayGrpcUrl {
        http_url: format!("{http_scheme}://{authority}"),
        chain: segments[1].to_string(),
        api_key: segments[2].to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_gateway_grpc_url_basic() {
        let g = parse_gateway_grpc_url("grpc://127.0.0.1:50051/rpc/tron/gw_xxx").unwrap();
        assert_eq!(g.http_url, "http://127.0.0.1:50051");
        assert_eq!(g.chain, "tron");
        assert_eq!(g.api_key, "gw_xxx");
    }

    #[test]
    fn parse_gateway_grpc_url_grpcs() {
        let g = parse_gateway_grpc_url("grpcs://api.example.com:443/rpc/tron/gw_yyy").unwrap();
        assert_eq!(g.http_url, "https://api.example.com:443");
        assert_eq!(g.chain, "tron");
        assert_eq!(g.api_key, "gw_yyy");
    }

    #[test]
    fn parse_gateway_grpc_url_rejects_non_gateway_shapes() {
        assert!(parse_gateway_grpc_url("http://127.0.0.1:8545/rpc/tron/gw_xxx").is_none());
        assert!(parse_gateway_grpc_url("grpc://127.0.0.1:50051").is_none());
        assert!(parse_gateway_grpc_url("grpc://127.0.0.1:50051/other/tron/gw_xxx").is_none());
        assert!(parse_gateway_grpc_url("grpc://127.0.0.1:50051/rpc/tron").is_none());
    }
}
