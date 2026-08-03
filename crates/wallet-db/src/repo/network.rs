use uuid::Uuid;
use wallet_error::{AppError, AppResult};
use wallet_types::ChainIndex;

use crate::models::{Network, RpcEndpoint};
use crate::Db;

pub struct NetworkRepo<'a> {
    db: &'a Db,
}

impl<'a> NetworkRepo<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub async fn list(&self, enabled_only: bool) -> AppResult<Vec<Network>> {
        let mut db = self.db.clone_inner();
        let query = if enabled_only {
            Network::filter(Network::fields().enabled().eq(true))
        } else {
            Network::all()
        };
        query
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))
    }

    pub async fn get(&self, chain_index: ChainIndex) -> AppResult<Option<Network>> {
        let mut db = self.db.clone_inner();
        let rows: Vec<Network> =
            Network::filter(Network::fields().chain_index().eq(chain_index.as_i64()))
                .exec(&mut db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;
        Ok(rows.into_iter().next())
    }

    pub async fn upsert(&self, row: &Network) -> AppResult<Network> {
        let mut db = self.db.clone_inner();
        let rows: Vec<Network> =
            Network::filter(Network::fields().chain_index().eq(row.chain_index))
                .exec(&mut db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;

        if let Some(mut existing) = rows.into_iter().next() {
            existing
                .update()
                .name(&row.name)
                .family(&row.family)
                .evm_chain_id(row.evm_chain_id)
                .enabled(row.enabled)
                .exec(&mut db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;
            let rows: Vec<Network> =
                Network::filter(Network::fields().chain_index().eq(row.chain_index))
                    .exec(&mut db)
                    .await
                    .map_err(|e| AppError::internal(e.to_string()))?;
            rows.into_iter()
                .next()
                .ok_or_else(|| AppError::internal("network not found after update"))
        } else {
            toasty::create!(Network {
                chain_index: row.chain_index,
                name: &row.name,
                family: &row.family,
                evm_chain_id: row.evm_chain_id,
                enabled: row.enabled,
            })
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))
        }
    }

    pub async fn list_endpoints(&self, chain_index: ChainIndex) -> AppResult<Vec<RpcEndpoint>> {
        let mut db = self.db.clone_inner();
        RpcEndpoint::filter(RpcEndpoint::fields().chain_index().eq(chain_index.as_i64()))
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))
    }

    pub async fn upsert_endpoint(&self, row: &RpcEndpoint) -> AppResult<RpcEndpoint> {
        let mut db = self.db.clone_inner();
        match RpcEndpoint::get_by_id(&mut db, &row.id).await {
            Ok(mut existing) => {
                existing
                    .update()
                    .url(&row.url)
                    .protocol(&row.protocol)
                    .weight(row.weight)
                    .enabled(row.enabled)
                    .headers(&row.headers)
                    .exec(&mut db)
                    .await
                    .map_err(|e| AppError::internal(e.to_string()))?;
                RpcEndpoint::get_by_id(&mut db, &row.id)
                    .await
                    .map_err(|e| AppError::internal(e.to_string()))
            }
            Err(_) => toasty::create!(RpcEndpoint {
                id: row.id,
                chain_index: row.chain_index,
                url: &row.url,
                protocol: &row.protocol,
                weight: row.weight,
                enabled: row.enabled,
                headers: &row.headers,
            })
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string())),
        }
    }

    pub async fn delete_endpoint(&self, id: Uuid) -> AppResult<()> {
        let mut db = self.db.clone_inner();
        RpcEndpoint::filter(RpcEndpoint::fields().id().eq(id))
            .delete()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;
        Ok(())
    }
}
