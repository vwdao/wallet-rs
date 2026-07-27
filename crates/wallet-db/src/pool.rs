use wallet_config::DatabaseConfig;
use wallet_error::{AppError, AppResult};

use crate::models;

#[derive(Clone)]
pub struct Db {
    inner: toasty::Db,
}

impl Db {
    pub async fn connect(cfg: &DatabaseConfig) -> AppResult<Self> {
        let inner = toasty::Db::builder()
            .models(toasty::models!(
                models::User,
                models::Address,
                models::Network,
                models::RpcEndpoint,
                models::Token,
                models::Tx,
                models::GasPool,
                models::SyncCursor,
                models::Dapp,
                models::SwapProvider,
                models::LatestPrice,
                models::DexQuote,
                models::AppConfig,
                models::Guide,
                models::ChainGatewayKey,
            ))
            .connect(&cfg.url)
            .await
            .map_err(|e| AppError::Unavailable(format!("toasty connect: {e}")))?;
        Ok(Self { inner })
    }

    pub async fn migrate(&self) -> AppResult<()> {
        self.inner
            .push_schema()
            .await
            .map_err(|e| AppError::internal(format!("push_schema: {e}")))?;
        Ok(())
    }

    pub fn clone_inner(&self) -> toasty::Db {
        self.inner.clone()
    }
}
