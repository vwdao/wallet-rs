#![allow(clippy::all)]
#![allow(dead_code)]

pub mod wallet {
    pub mod v1 {
        tonic::include_proto!("wallet.v1");
        pub mod admin {
            tonic::include_proto!("wallet.v1.admin");
        }
    }
}

pub use wallet::v1::*;
