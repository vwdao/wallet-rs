use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wallet_db::{ChainGatewayKey, ChainGatewayStats, GatewaySettings, RpcEndpoint};
use wallet_error::AppError;

use crate::free_rpc::{ChainSyncResult, FreeRpcSyncer};
use crate::proxy;
use crate::Gw;

fn require_admin(
    req: &Request,
    _depot: &Depot,
    admin_key: &Option<String>,
) -> Result<(), AppError> {
    if let Some(expected) = admin_key {
        let provided = req
            .headers()
            .get("x-admin-key")
            .and_then(|v| v.to_str().ok());
        match provided {
            Some(k) if k == expected.as_str() => Ok(()),
            _ => Err(AppError::Unauthorized),
        }
    } else {
        Err(AppError::Forbidden)
    }
}

// ─── Keys ──────────────────────────────────────────────

#[derive(Serialize)]
struct KeyResponse {
    id: Uuid,
    api_key: String,
    name: String,
    rate_limit_per_min: i32,
    enabled: bool,
    allowed_chains: Vec<i64>,
    allowed_tier: String,
    total_requests: i64,
}

impl KeyResponse {
    fn from_row(r: &ChainGatewayKey) -> Self {
        Self {
            id: r.id,
            api_key: r.api_key.clone(),
            name: r.name.clone(),
            rate_limit_per_min: r.rate_limit_per_min,
            enabled: r.enabled,
            allowed_chains: r.allowed_chains.clone(),
            allowed_tier: r.allowed_tier.clone(),
            total_requests: r.total_requests,
        }
    }
}

#[derive(Deserialize)]
struct CreateKeyRequest {
    name: String,
    #[serde(default = "default_rate_limit")]
    rate_limit_per_min: i32,
    #[serde(default = "default_true")]
    enabled: bool,
    #[serde(default)]
    allowed_chains: Vec<i64>,
    #[serde(default = "default_tier_all")]
    allowed_tier: String,
}

fn default_rate_limit() -> i32 {
    60
}
fn default_true() -> bool {
    true
}
fn default_tier_all() -> String {
    "all".into()
}

#[derive(Deserialize)]
struct UpdateKeyRequest {
    name: Option<String>,
    rate_limit_per_min: Option<i32>,
    enabled: Option<bool>,
    allowed_chains: Option<Vec<i64>>,
    allowed_tier: Option<String>,
}

#[handler]
async fn list_keys(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<Vec<KeyResponse>>, AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("state missing"))?;
        require_admin(req, depot, &st.admin_key)?;

        let mut db = st.db.clone_inner();
        let rows: Vec<ChainGatewayKey> = ChainGatewayKey::all()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(Json(rows.iter().map(KeyResponse::from_row).collect()))
    }
    .await;
    match result {
        Ok(j) => res.render(j),
        Err(e) => res.render(e),
    }
}

#[handler]
async fn create_key(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<KeyResponse>, AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("state missing"))?;
        require_admin(req, depot, &st.admin_key)?;

        let body: CreateKeyRequest = req
            .parse_json()
            .await
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?;

        let key = format!("gw_{}", Uuid::new_v4().to_string().replace('-', ""));
        let mut db = st.db.clone_inner();
        let row = toasty::create!(ChainGatewayKey {
            api_key: &key,
            name: &body.name,
            rate_limit_per_min: body.rate_limit_per_min,
            enabled: body.enabled,
            allowed_chains: body.allowed_chains,
            allowed_tier: &body.allowed_tier,
        })
        .exec(&mut db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(Json(KeyResponse::from_row(&row)))
    }
    .await;
    match result {
        Ok(j) => res.render(j),
        Err(e) => res.render(e),
    }
}

#[handler]
async fn update_key(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<KeyResponse>, AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("state missing"))?;
        require_admin(req, depot, &st.admin_key)?;

        let id_str: String = req
            .param("id")
            .ok_or_else(|| AppError::InvalidArgument("missing id".into()))?;
        let id: Uuid = id_str
            .parse()
            .map_err(|_| AppError::InvalidArgument("invalid uuid".into()))?;
        let body: UpdateKeyRequest = req
            .parse_json()
            .await
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?;

        let mut db = st.db.clone_inner();
        let mut row: ChainGatewayKey = ChainGatewayKey::get_by_id(&mut db, &id)
            .await
            .map_err(|_| AppError::NotFound("key not found".into()))?;

        let mut upd = row.update();
        if let Some(name) = &body.name {
            upd = upd.name(name);
        }
        if let Some(limit) = body.rate_limit_per_min {
            upd = upd.rate_limit_per_min(limit);
        }
        if let Some(enabled) = body.enabled {
            upd = upd.enabled(enabled);
        }
        if let Some(chains) = &body.allowed_chains {
            upd = upd.allowed_chains(chains.clone());
        }
        if let Some(tier) = &body.allowed_tier {
            upd = upd.allowed_tier(tier);
        }
        upd.exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        let row = ChainGatewayKey::get_by_id(&mut db, &id)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(Json(KeyResponse::from_row(&row)))
    }
    .await;
    match result {
        Ok(j) => res.render(j),
        Err(e) => res.render(e),
    }
}

#[handler]
async fn delete_key(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<(), AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("state missing"))?;
        require_admin(req, depot, &st.admin_key)?;

        let id_str: String = req
            .param("id")
            .ok_or_else(|| AppError::InvalidArgument("missing id".into()))?;
        let id: Uuid = id_str
            .parse()
            .map_err(|_| AppError::InvalidArgument("invalid uuid".into()))?;

        let mut db = st.db.clone_inner();
        let mut row: ChainGatewayKey = ChainGatewayKey::get_by_id(&mut db, &id)
            .await
            .map_err(|_| AppError::NotFound("key not found".into()))?;

        row.update()
            .enabled(false)
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(())
    }
    .await;
    match result {
        Ok(()) => res.render(Json(serde_json::json!({"ok": true}))),
        Err(e) => res.render(e),
    }
}

// ─── Endpoints ─────────────────────────────────────────

#[derive(Serialize)]
struct EndpointResponse {
    id: Uuid,
    chain_index: i64,
    url: String,
    weight: i32,
    enabled: bool,
    tier: String,
    is_archive: bool,
    priority: i32,
    healthy: bool,
    avg_latency_ms: Option<i32>,
    error_count: i32,
}

impl EndpointResponse {
    fn from_row(r: &RpcEndpoint) -> Self {
        Self {
            id: r.id,
            chain_index: r.chain_index,
            url: r.url.clone(),
            weight: r.weight,
            enabled: r.enabled,
            tier: r.tier.clone(),
            is_archive: r.is_archive,
            priority: r.priority,
            healthy: r.healthy,
            avg_latency_ms: r.avg_latency_ms,
            error_count: r.error_count,
        }
    }
}

#[derive(Deserialize)]
struct CreateEndpointRequest {
    chain_index: i64,
    url: String,
    #[serde(default = "default_weight")]
    weight: i32,
    #[serde(default = "default_true")]
    enabled: bool,
    #[serde(default = "default_tier_free")]
    tier: String,
    #[serde(default)]
    is_archive: bool,
    #[serde(default)]
    priority: i32,
}

fn default_weight() -> i32 {
    1
}
fn default_tier_free() -> String {
    "free".into()
}

#[derive(Deserialize)]
struct UpdateEndpointRequest {
    url: Option<String>,
    weight: Option<i32>,
    enabled: Option<bool>,
    tier: Option<String>,
    is_archive: Option<bool>,
    priority: Option<i32>,
}

#[handler]
async fn list_endpoints(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<Vec<EndpointResponse>>, AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("state missing"))?;
        require_admin(req, depot, &st.admin_key)?;

        let mut db = st.db.clone_inner();
        let rows: Vec<RpcEndpoint> = RpcEndpoint::all()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(Json(rows.iter().map(EndpointResponse::from_row).collect()))
    }
    .await;
    match result {
        Ok(j) => res.render(j),
        Err(e) => res.render(e),
    }
}

#[handler]
async fn create_endpoint(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<EndpointResponse>, AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("state missing"))?;
        require_admin(req, depot, &st.admin_key)?;

        let body: CreateEndpointRequest = req
            .parse_json()
            .await
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?;

        proxy::validate_endpoint_url(&body.url)?;

        let mut db = st.db.clone_inner();
        let row = toasty::create!(RpcEndpoint {
            chain_index: body.chain_index,
            url: &body.url,
            weight: body.weight,
            enabled: body.enabled,
            tier: &body.tier,
            is_archive: body.is_archive,
            priority: body.priority,
        })
        .exec(&mut db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

        st.router.invalidate_cache(body.chain_index);

        Ok(Json(EndpointResponse::from_row(&row)))
    }
    .await;
    match result {
        Ok(j) => res.render(j),
        Err(e) => res.render(e),
    }
}

#[handler]
async fn update_endpoint(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<EndpointResponse>, AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("state missing"))?;
        require_admin(req, depot, &st.admin_key)?;

        let id_str: String = req
            .param("id")
            .ok_or_else(|| AppError::InvalidArgument("missing id".into()))?;
        let id: Uuid = id_str
            .parse()
            .map_err(|_| AppError::InvalidArgument("invalid uuid".into()))?;
        let body: UpdateEndpointRequest = req
            .parse_json()
            .await
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?;

        if let Some(url) = &body.url {
            proxy::validate_endpoint_url(url)?;
        }

        let mut db = st.db.clone_inner();
        let mut row: RpcEndpoint = RpcEndpoint::get_by_id(&mut db, &id)
            .await
            .map_err(|_| AppError::NotFound("endpoint not found".into()))?;

        let chain_index = row.chain_index;
        let mut upd = row.update();
        if let Some(url) = &body.url {
            upd = upd.url(url);
        }
        if let Some(weight) = body.weight {
            upd = upd.weight(weight);
        }
        if let Some(enabled) = body.enabled {
            upd = upd.enabled(enabled);
        }
        if let Some(tier) = &body.tier {
            upd = upd.tier(tier);
        }
        if let Some(is_archive) = body.is_archive {
            upd = upd.is_archive(is_archive);
        }
        if let Some(priority) = body.priority {
            upd = upd.priority(priority);
        }
        upd.exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        st.router.invalidate_cache(chain_index);

        let row = RpcEndpoint::get_by_id(&mut db, &id)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(Json(EndpointResponse::from_row(&row)))
    }
    .await;
    match result {
        Ok(j) => res.render(j),
        Err(e) => res.render(e),
    }
}

#[handler]
async fn delete_endpoint(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<(), AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("state missing"))?;
        require_admin(req, depot, &st.admin_key)?;

        let id_str: String = req
            .param("id")
            .ok_or_else(|| AppError::InvalidArgument("missing id".into()))?;
        let id: Uuid = id_str
            .parse()
            .map_err(|_| AppError::InvalidArgument("invalid uuid".into()))?;

        let mut db = st.db.clone_inner();
        let mut row: RpcEndpoint = RpcEndpoint::get_by_id(&mut db, &id)
            .await
            .map_err(|_| AppError::NotFound("endpoint not found".into()))?;

        let chain_index = row.chain_index;
        row.update()
            .enabled(false)
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        st.router.invalidate_cache(chain_index);

        Ok(())
    }
    .await;
    match result {
        Ok(()) => res.render(Json(serde_json::json!({"ok": true}))),
        Err(e) => res.render(e),
    }
}

#[handler]
async fn sync_free_endpoints(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<Vec<ChainSyncResult>>, AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("state missing"))?;
        require_admin(req, depot, &st.admin_key)?;

        let syncer = FreeRpcSyncer::new(st.db.clone(), st.http.clone());
        let chain_index: Option<i64> = req.query("chain_index");

        let results = if let Some(chain_index) = chain_index {
            vec![syncer.sync_chain(chain_index).await?]
        } else {
            syncer.sync_all_enabled().await?
        };

        if !results.is_empty() {
            st.router.invalidate_all();
        }

        Ok(Json(results))
    }
    .await;
    match result {
        Ok(j) => res.render(j),
        Err(e) => res.render(e),
    }
}

// ─── Stats ─────────────────────────────────────────────

#[derive(Serialize)]
struct StatsSummary {
    api_key: String,
    chain_index: i64,
    total_requests: i64,
    error_count: i64,
    avg_latency_ms: f64,
}

#[handler]
async fn get_stats(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<Vec<StatsSummary>>, AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("state missing"))?;
        require_admin(req, depot, &st.admin_key)?;

        let api_key: Option<String> = req.query("api_key");
        let chain_index: Option<i64> = req.query("chain_index");
        let limit: i64 = req.query("limit").unwrap_or(100);
        let mut db = st.db.clone_inner();

        let mut q = ChainGatewayStats::all();
        if let Some(key) = &api_key {
            q = q.filter(ChainGatewayStats::fields().api_key().eq(key));
        }
        if let Some(ci) = chain_index {
            q = q.filter(ChainGatewayStats::fields().chain_index().eq(ci));
        }

        let rows: Vec<ChainGatewayStats> = q
            .limit(limit as usize)
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        let mut map: std::collections::HashMap<(String, i64), (i64, i64, i64)> =
            std::collections::HashMap::new();

        for row in &rows {
            let entry = map
                .entry((row.api_key.clone(), row.chain_index))
                .or_insert((0, 0, 0));
            entry.0 += 1;
            if row.status_code >= 400 {
                entry.1 += 1;
            }
            entry.2 += row.latency_ms as i64;
        }

        let summaries: Vec<StatsSummary> = map
            .into_iter()
            .map(
                |((api_key, chain_index), (total, errors, total_latency))| StatsSummary {
                    api_key,
                    chain_index,
                    total_requests: total,
                    error_count: errors,
                    avg_latency_ms: if total > 0 {
                        total_latency as f64 / total as f64
                    } else {
                        0.0
                    },
                },
            )
            .collect();

        Ok(Json(summaries))
    }
    .await;
    match result {
        Ok(j) => res.render(j),
        Err(e) => res.render(e),
    }
}

// ─── Settings ────────────────────────────────────────

#[derive(Serialize)]
struct SettingResponse {
    key: String,
    value: String,
    description: Option<String>,
}

impl SettingResponse {
    fn from_row(r: &GatewaySettings) -> Self {
        Self {
            key: r.key.clone(),
            value: r.value.clone(),
            description: r.description.clone(),
        }
    }
}

#[derive(Deserialize)]
struct UpdateSettingRequest {
    value: String,
}

#[handler]
async fn list_settings(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<Vec<SettingResponse>>, AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("state missing"))?;
        require_admin(req, depot, &st.admin_key)?;

        let mut db = st.db.clone_inner();
        let rows: Vec<GatewaySettings> = GatewaySettings::all()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(Json(rows.iter().map(SettingResponse::from_row).collect()))
    }
    .await;
    match result {
        Ok(j) => res.render(j),
        Err(e) => res.render(e),
    }
}

#[handler]
async fn update_setting(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<SettingResponse>, AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("state missing"))?;
        require_admin(req, depot, &st.admin_key)?;

        let key: String = req
            .param("key")
            .ok_or_else(|| AppError::InvalidArgument("missing key".into()))?;
        let body: UpdateSettingRequest = req
            .parse_json()
            .await
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?;

        let mut db = st.db.clone_inner();
        let mut row: GatewaySettings = GatewaySettings::get_by_key(&mut db, &key)
            .await
            .map_err(|_| AppError::NotFound(format!("setting '{key}' not found")))?;

        row.update()
            .value(&body.value)
            .updated_at(jiff::Timestamp::now())
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        let row = GatewaySettings::get_by_key(&mut db, &key)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(Json(SettingResponse::from_row(&row)))
    }
    .await;
    match result {
        Ok(j) => res.render(j),
        Err(e) => res.render(e),
    }
}

#[handler]
async fn create_setting(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<SettingResponse>, AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("state missing"))?;
        require_admin(req, depot, &st.admin_key)?;

        #[derive(Deserialize)]
        struct CreateSetting {
            key: String,
            value: String,
            #[serde(default)]
            description: Option<String>,
        }
        let body: CreateSetting = req
            .parse_json()
            .await
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?;

        let mut db = st.db.clone_inner();
        let row = toasty::create!(GatewaySettings {
            key: &body.key,
            value: &body.value,
            description: body.description,
        })
        .exec(&mut db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(Json(SettingResponse::from_row(&row)))
    }
    .await;
    match result {
        Ok(j) => res.render(j),
        Err(e) => res.render(e),
    }
}

// ─── Router ────────────────────────────────────────────

pub fn admin_router() -> Router {
    Router::with_path("admin")
        .push(Router::with_path("keys").get(list_keys).post(create_key))
        .push(
            Router::with_path("keys/{id}")
                .put(update_key)
                .delete(delete_key),
        )
        .push(
            Router::with_path("endpoints")
                .get(list_endpoints)
                .post(create_endpoint),
        )
        .push(Router::with_path("endpoints/sync-free").post(sync_free_endpoints))
        .push(
            Router::with_path("endpoints/{id}")
                .put(update_endpoint)
                .delete(delete_endpoint),
        )
        .push(Router::with_path("stats").get(get_stats))
        .push(
            Router::with_path("settings")
                .get(list_settings)
                .post(create_setting),
        )
        .push(Router::with_path("settings/{key}").put(update_setting))
}
