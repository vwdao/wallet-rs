use uuid::Uuid;
use wallet_error::{AppError, AppResult};
use wallet_types::{ChainIndex, NormalizedTx};

use crate::models::Tx;
use crate::Db;

/// Maximum number of rows per batched `INSERT ... ON CONFLICT` statement.
///
/// PostgreSQL caps prepared-statement parameters at 65_535; each row binds 13
/// params here, so 500 rows (6_500 params) leaves plenty of headroom and keeps
/// individual statement sizes reasonable for the planner.
const BATCH_INSERT_CHUNK: usize = 500;

/// `db::Type` constants for the `transactions` columns.
///
/// toasty's `bind` cannot infer a storage type for `NULL` (it has no Rust
/// type to draw from), so every nullable bind must go through
/// [`toasty::sql::Statement::bind_typed`] with the column's real Postgres
/// type. Keeping the map here keeps the SQL builder honest and matches the
/// table declared in `migrations/20260723100000_init.sql` +
/// `20260808040000_tx_gas_fee.sql` + `20260808070000_tx_unique_coalesce.sql`.
mod pg_ty {
    use toasty::schema::db::Type;

    pub const I64: Type = Type::Integer(8);
    pub const TEXT: Type = Type::Text;
    pub const NUMERIC: Type = Type::Numeric(None);
    pub const JSONB: Type = Type::Document { binary: true };
}

pub struct TransactionRepo<'a> {
    db: &'a Db,
}

impl<'a> TransactionRepo<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub async fn get(&self, chain_index: ChainIndex, hash: &str) -> AppResult<Option<Tx>> {
        let mut db = self.db.clone_inner();
        let rows: Vec<Tx> = Tx::filter(
            Tx::fields()
                .chain_index()
                .eq(chain_index.as_i64())
                .and(Tx::fields().hash().eq(hash)),
        )
        .exec(&mut db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;
        Ok(rows.into_iter().next())
    }

    pub async fn list_by_address(
        &self,
        chain_index: ChainIndex,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> AppResult<(Vec<Tx>, i64)> {
        let mut db = self.db.clone_inner();
        let rows: Vec<Tx> = Tx::filter(
            Tx::fields().chain_index().eq(chain_index.as_i64()).and(
                Tx::fields()
                    .from_address()
                    .eq(address)
                    .or(Tx::fields().to_address().eq(address)),
            ),
        )
        .limit(limit as usize)
        .offset(offset as usize)
        .exec(&mut db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

        let total = Tx::filter(
            Tx::fields().chain_index().eq(chain_index.as_i64()).and(
                Tx::fields()
                    .from_address()
                    .eq(address)
                    .or(Tx::fields().to_address().eq(address)),
            ),
        )
        .count()
        .exec(&mut db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

        Ok((rows, total as i64))
    }

    pub async fn insert_normalized(
        &self,
        chain_index: ChainIndex,
        tx: &NormalizedTx,
    ) -> AppResult<()> {
        let mut db = self.db.clone_inner();
        let status = tx.status.as_str();
        let contract = tx.contract_address.as_ref().map(|c| c.as_str().to_string());
        let log_index = tx.log_index.map(|i| i as i64);

        let mut filter = Tx::fields()
            .chain_index()
            .eq(chain_index.as_i64())
            .and(Tx::fields().hash().eq(tx.hash.as_str()));
        filter = match &contract {
            Some(c) => filter.and(Tx::fields().contract_address().eq(c.clone())),
            None => filter.and(Tx::fields().contract_address().is_none()),
        };
        filter = match log_index {
            Some(i) => filter.and(Tx::fields().log_index().eq(i)),
            None => filter.and(Tx::fields().log_index().is_none()),
        };

        let existing: Vec<Tx> = Tx::filter(filter)
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        if existing.is_empty() {
            toasty::create!(Tx {
                chain_index: chain_index.as_i64(),
                hash: tx.hash.as_str(),
                from_address: tx.from.as_ref().map(|a| a.as_str().to_string()),
                to_address: tx.to.as_ref().map(|a| a.as_str().to_string()),
                value: tx.value.raw,
                gas_fee: tx.gas_fee.as_ref().map(|g| g.raw),
                block_number: tx.block_number as i64,
                status: status,
                contract_address: contract,
                log_index: log_index,
                method: tx.method.clone(),
                raw: tx.raw.clone(),
            })
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;
        } else {
            let mut existing = existing.into_iter().next().unwrap();
            existing
                .update()
                .from_address(tx.from.as_ref().map(|a| a.as_str().to_string()))
                .to_address(tx.to.as_ref().map(|a| a.as_str().to_string()))
                .value(tx.value.raw)
                .gas_fee(tx.gas_fee.as_ref().map(|g| g.raw))
                .block_number(tx.block_number as i64)
                .status(status)
                .contract_address(contract)
                .log_index(log_index)
                .method(tx.method.clone())
                .raw(tx.raw.clone())
                .exec(&mut db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;
        }
        Ok(())
    }

    /// Bulk upsert a batch of normalized transactions in a single
    /// `INSERT ... ON CONFLICT DO UPDATE` statement per chunk.
    ///
    /// This replaces the per-row select+insert/update round trip with one
    /// statement per `BATCH_INSERT_CHUNK` rows, which is the main source of
    /// sync slowdown when each block contains hundreds of transactions.
    ///
    /// The conflict target mirrors the partial unique index
    /// `uq_tx_chain_hash_contract_log` so re-syncing the same block is
    /// idempotent: existing rows get refreshed with the new fields.
    pub async fn insert_normalized_batch(
        &self,
        chain_index: ChainIndex,
        txs: &[NormalizedTx],
    ) -> AppResult<()> {
        if txs.is_empty() {
            return Ok(());
        }
        let mut db = self.db.clone_inner();
        for chunk in txs.chunks(BATCH_INSERT_CHUNK) {
            upsert_chunk(&mut db, chain_index, chunk).await?;
        }
        Ok(())
    }
}

async fn upsert_chunk(
    db: &mut toasty::Db,
    chain_index: ChainIndex,
    chunk: &[NormalizedTx],
) -> AppResult<()> {
    // Column order in VALUES must match the bind order below. The `id` UUID
    // is generated here because the `transactions.id` column is `NOT NULL`
    // with no SQL default — toasty's ORM path used to fill it in, but the
    // raw-SQL batch path has to do it itself.
    let cols_per_row = 13;
    let mut placeholders = String::with_capacity(chunk.len() * 32);
    for i in 0..chunk.len() {
        if i > 0 {
            placeholders.push(',');
        }
        let base = i * cols_per_row + 1;
        placeholders.push_str(&format!(
            "(${},${},${},${},${},${},${},${},${},${},${},${},${}::jsonb)",
            base,
            base + 1,
            base + 2,
            base + 3,
            base + 4,
            base + 5,
            base + 6,
            base + 7,
            base + 8,
            base + 9,
            base + 10,
            base + 11,
            base + 12,
        ));
    }
    let sql = format!(
        "INSERT INTO transactions (id, chain_index, hash, from_address, to_address, \
            value, gas_fee, block_number, status, contract_address, log_index, \
            method, raw) \
         VALUES {placeholders} \
         ON CONFLICT (chain_index, hash, COALESCE(contract_address, ''), \
            COALESCE(log_index, -1)) DO UPDATE SET \
            from_address = EXCLUDED.from_address, \
            to_address = EXCLUDED.to_address, \
            value = EXCLUDED.value, \
            gas_fee = EXCLUDED.gas_fee, \
            block_number = EXCLUDED.block_number, \
            status = EXCLUDED.status, \
            contract_address = EXCLUDED.contract_address, \
            log_index = EXCLUDED.log_index, \
            method = EXCLUDED.method, \
            raw = EXCLUDED.raw"
    );
    let chain_idx = chain_index.as_i64();
    let mut stmt = toasty::sql::statement(&sql);
    for tx in chunk {
        let contract = tx.contract_address.as_ref().map(|c| c.as_str().to_string());
        let log_index = tx.log_index.map(|i| i as i64);
        // toasty's `bind` can't infer a storage type for `NULL`, so every
        // nullable column goes through `bind_typed` with the column's real
        // Postgres type. Non-nullable columns still use plain `bind` so the
        // driver can keep doing its own type inference.
        let raw_json = serde_json::to_string(&tx.raw)
            .map_err(|e| AppError::internal(format!("serialize tx.raw: {e}")))?;
        // The id is discarded on conflict (the existing row keeps its own
        // UUID), but it is required for the INSERT to satisfy the NOT NULL
        // constraint on first-time writes.
        let id = Uuid::new_v4();
        stmt = stmt
            .bind(id)
            .bind(chain_idx)
            .bind(tx.hash.as_str())
            .bind_optional_text(tx.from.as_ref().map(|a| a.as_str()))
            .bind_optional_text(tx.to.as_ref().map(|a| a.as_str()))
            .bind(tx.value.raw)
            .bind_optional_numeric(tx.gas_fee.as_ref().map(|g| g.raw))
            .bind(tx.block_number as i64)
            .bind(tx.status.as_str())
            .bind_optional_text(contract.as_deref())
            .bind_optional_i64(log_index)
            .bind_optional_text(tx.method.as_deref())
            .bind_typed(toasty::stmt::Value::String(raw_json), pg_ty::JSONB);
    }
    stmt.exec(db).await.map_err(|e| AppError::internal(e.to_string()))?;
    Ok(())
}

/// `bind` for an `Option<&str>` — `Some` uses the inferred `Text` type, `None`
/// must declare the column type explicitly so the driver knows what flavor of
/// NULL to send.
trait BindOptional {
    fn bind_optional_text(self, value: Option<&str>) -> Self;
    fn bind_optional_numeric(self, value: Option<rust_decimal::Decimal>) -> Self;
    fn bind_optional_i64(self, value: Option<i64>) -> Self;
}

impl BindOptional for toasty::sql::Statement {
    fn bind_optional_text(mut self, value: Option<&str>) -> Self {
        self = match value {
            Some(s) => self.bind(s),
            None => self.bind_typed(toasty::stmt::Value::Null, pg_ty::TEXT),
        };
        self
    }
    fn bind_optional_numeric(mut self, value: Option<rust_decimal::Decimal>) -> Self {
        self = match value {
            Some(d) => self.bind(d),
            None => self.bind_typed(toasty::stmt::Value::Null, pg_ty::NUMERIC),
        };
        self
    }
    fn bind_optional_i64(mut self, value: Option<i64>) -> Self {
        self = match value {
            Some(i) => self.bind(i),
            None => self.bind_typed(toasty::stmt::Value::Null, pg_ty::I64),
        };
        self
    }
}
