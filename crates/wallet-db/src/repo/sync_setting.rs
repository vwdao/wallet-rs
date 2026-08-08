use wallet_error::{AppError, AppResult};
use wallet_types::ChainIndex;

use crate::models::SyncSetting;
use crate::Db;

pub struct SyncSettingRepo<'a> {
    db: &'a Db,
}

impl<'a> SyncSettingRepo<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub async fn get(&self, chain_index: ChainIndex) -> AppResult<Option<SyncSetting>> {
        let mut db = self.db.clone_inner();
        let rows: Vec<SyncSetting> =
            SyncSetting::filter(SyncSetting::fields().chain_index().eq(chain_index.as_i64()))
                .exec(&mut db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;
        Ok(rows.into_iter().next())
    }

    pub async fn upsert(&self, setting: &SyncSetting) -> AppResult<()> {
        let mut db = self.db.clone_inner();
        let rows: Vec<SyncSetting> =
            SyncSetting::filter(SyncSetting::fields().chain_index().eq(setting.chain_index))
                .exec(&mut db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;

        match rows.into_iter().next() {
            Some(mut existing) => {
                existing
                    .update()
                    .poll_interval_ms(setting.poll_interval_ms)
                    .confirmations(setting.confirmations)
                    .enabled(setting.enabled)
                    .exec(&mut db)
                    .await
                    .map_err(|e| AppError::internal(e.to_string()))?;
            }
            None => {
                toasty::create!(SyncSetting {
                    chain_index: setting.chain_index,
                    poll_interval_ms: setting.poll_interval_ms,
                    confirmations: setting.confirmations,
                    enabled: setting.enabled,
                })
                .exec(&mut db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;
            }
        }
        Ok(())
    }
}
