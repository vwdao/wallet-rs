pub mod clickhouse;
pub mod models;
pub mod pool;
pub mod repo;

pub use models::*;
pub use pool::Db;
pub use repo::*;

pub type UserRow = User;
pub type AddressRow = Address;
pub type NetworkRow = Network;
pub type RpcEndpointRow = RpcEndpoint;
pub type TokenRow = Token;
pub type TxRow = Tx;
pub type DappRow = Dapp;
pub type GasPoolRow = GasPool;
