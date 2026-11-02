//! TRON protobuf types + generated tonic gRPC client.
//!
//! Message types are vendored from `tron-rs` 0.1.0's prost output so they
//! compile against prost 0.14. The tonic client wrappers for
//! `protocol.Wallet` / `protocol.WalletSolidity` / `protocol.WalletExtension`
//! are vendored from the same crate's `protocol.tonic.rs` and patched for
//! tonic 0.14 (`tonic::body::Body`, `tonic_prost::ProstCodec`).

pub mod protocol {
    #![allow(dead_code, unused_imports, clippy::all)]
    include!("protocol.rs");
    include!("wallet_client.rs");
}

pub(crate) mod client;
mod proto_value;
pub(crate) mod url;
