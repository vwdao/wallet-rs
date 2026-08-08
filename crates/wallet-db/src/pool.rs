use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use wallet_config::DatabaseConfig;
use wallet_error::{AppError, AppResult};

use crate::models;

#[derive(Clone)]
pub struct Db {
    inner: toasty::Db,
}

impl Db {
    pub async fn connect(cfg: &DatabaseConfig) -> AppResult<Self> {
        run_sql_migrations(&cfg.url).await?;

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
                models::SyncSetting,
                models::Dapp,
                models::SwapProvider,
                models::LatestPrice,
                models::DexQuote,
                models::AppConfig,
                models::Guide,
                models::ChainGatewayKey,
                models::ChainGatewayStats,
                models::GatewaySettings,
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

async fn run_sql_migrations(database_url: &str) -> AppResult<()> {
    let (mut client, connection) = tokio_postgres::connect(database_url, tokio_postgres::NoTls)
        .await
        .map_err(|e| AppError::Unavailable(format!("postgres connect: {e}")))?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            tracing::error!("postgres migration connection error: {e}");
        }
    });

    client
        .batch_execute(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version TEXT PRIMARY KEY,
                applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .await
        .map_err(|e| AppError::internal(format!("create schema_migrations: {e}")))?;

    let rows = client
        .query("SELECT version FROM schema_migrations", &[])
        .await
        .map_err(|e| AppError::internal(format!("load schema_migrations: {e}")))?;
    let applied: HashSet<String> = rows.into_iter().map(|row| row.get(0)).collect();

    for path in migration_files()? {
        let version = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| {
                AppError::internal(format!("invalid migration filename: {}", path.display()))
            })?
            .to_string();
        if applied.contains(&version) {
            continue;
        }

        let sql = fs::read_to_string(&path)
            .map_err(|e| AppError::internal(format!("read migration {}: {e}", path.display())))?;
        let tx = client
            .transaction()
            .await
            .map_err(|e| AppError::internal(format!("begin migration transaction: {e}")))?;
        tx.batch_execute(&sql)
            .await
            .map_err(|e| AppError::internal(format!("apply migration {version}: {e}")))?;
        tx.execute(
            "INSERT INTO schema_migrations (version) VALUES ($1) ON CONFLICT (version) DO NOTHING",
            &[&version],
        )
        .await
        .map_err(|e| AppError::internal(format!("record migration {version}: {e}")))?;
        tx.commit()
            .await
            .map_err(|e| AppError::internal(format!("commit migration {version}: {e}")))?;
    }

    Ok(())
}

fn migration_files() -> AppResult<Vec<PathBuf>> {
    let migrations_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../migrations");
    let mut files = Vec::new();

    for entry in fs::read_dir(&migrations_dir).map_err(|e| {
        AppError::internal(format!(
            "read migrations dir {}: {e}",
            migrations_dir.display()
        ))
    })? {
        let entry = entry.map_err(|e| AppError::internal(format!("read migration entry: {e}")))?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("sql") {
            files.push(path);
        }
    }

    files.sort();
    Ok(files)
}
