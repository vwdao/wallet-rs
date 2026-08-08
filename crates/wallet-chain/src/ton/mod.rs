//! TRON protobuf types + generated tonic gRPC client.
//!
//! The protobuf message definitions are re-exported from the
//! [`tron_rs`] crate (which only ships the `.rs` for the prost
//! `Message` types), and the `tonic`-generated gRPC client wrappers
//! for `protocol.Wallet`, `protocol.WalletSolidity`, `protocol.WalletExtension`
//! etc. are vendored from `tron-rs`'s `prost/protocol.tonic.rs`.
//!
//! Both halves are re-exported under a single `protocol` module so
//! the `super::X` references in the vendored file resolve to the
//! protobuf types via the `pub use tron_rs::tron::protocol::*` line.

pub mod protocol {
    pub use tron_rs::tron::protocol::*;

    include!("wallet_client.rs");
}

pub(crate) mod client;
mod proto_value;
pub(crate) mod url;
