use base64::Engine;
use hmac::Mac;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;
use wallet_db::{ChainGatewayKey, ChainGatewayStats, GatewaySettings, RpcEndpoint};
use wallet_error::AppError;

use crate::free_rpc::{ChainSyncResult, FreeRpcSyncer};
use crate::proxy;
use crate::Gw;
const TOKEN_TTL_SECS: i64 = 24 * 60 * 60;

type HmacSha256 = hmac::Hmac<sha2::Sha256>;

fn sign_token(secret: &str, username: &str) -> String {
    let exp = jiff::Timestamp::now().as_second() + TOKEN_TTL_SECS;
    let payload = format!("{username}:{exp}");
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("hmac accepts any key");
    mac.update(payload.as_bytes());
    let sig = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());
    let body = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(payload.as_bytes());
    format!("{body}.{sig}")
}

fn verify_token(secret: &str, token: &str) -> Option<String> {
    let (body, sig) = token.split_once('.')?;
    let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(body)
        .ok()?;
    let expected = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(sig)
        .ok()?;
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).ok()?;
    mac.update(&payload);
    if mac.verify_slice(&expected).is_err() {
        return None;
    }
    let payload = std::str::from_utf8(&payload).ok()?;
    let (username, exp) = payload.rsplit_once(':')?;
    let exp: i64 = exp.parse().ok()?;
    if jiff::Timestamp::now().as_second() > exp {
        return None;
    }
    Some(username.to_string())
}

fn bearer_token(req: &Request) -> Option<String> {
    let auth = req.headers().get("authorization")?.to_str().ok()?;
    auth.strip_prefix("Bearer ")
        .or_else(|| auth.strip_prefix("bearer "))
        .map(|s| s.trim().to_string())
}

fn require_admin(req: &Request, _depot: &Depot, st: &Gw) -> Result<(), AppError> {
    if let Some(token) = bearer_token(req) {
        let secret = st
            .admin_password
            .as_deref()
            .or(st.admin_key.as_deref());
        if let Some(secret) = secret {
            if verify_token(secret, &token).is_some() {
                return Ok(());
            }
        }
    }
    if let Some(expected) = &st.admin_key {
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

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    expires_in: i64,
}

fn validate_headers(headers: &BTreeMap<String, String>) -> Result<(), AppError> {
    for (name, value) in headers {
        name.parse::<http::header::HeaderName>()
            .map_err(|e| AppError::InvalidArgument(format!("invalid header name '{name}': {e}")))?;
        http::header::HeaderValue::from_str(value).map_err(|e| {
            AppError::InvalidArgument(format!("invalid value for header '{name}': {e}"))
        })?;
    }
    Ok(())
}

#[handler]
async fn login(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<LoginResponse>, AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("state missing"))?;

        let body: LoginRequest = req
            .parse_json()
            .await
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?;

        let (username, password, secret) = match (&st.admin_username, &st.admin_password) {
            (Some(u), Some(p)) => (u.as_str(), p.as_str(), p.as_str()),
            (None, None) => match &st.admin_key {
                Some(k) => ("admin", k.as_str(), k.as_str()),
                None => return Err(AppError::Forbidden),
            },
            _ => return Err(AppError::Forbidden),
        };

        if body.username != username || body.password != password {
            return Err(AppError::Unauthorized);
        }

        Ok(Json(LoginResponse {
            token: sign_token(secret, username),
            expires_in: TOKEN_TTL_SECS,
        }))
    }
    .await;
    match result {
        Ok(j) => res.render(j),
        Err(e) => res.render(e),
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
        require_admin(req, depot, st)?;

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
        require_admin(req, depot, st)?;

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
        require_admin(req, depot, st)?;

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
        require_admin(req, depot, st)?;

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
    protocol: String,
    weight: i32,
    enabled: bool,
    tier: String,
    is_archive: bool,
    priority: i32,
    healthy: bool,
    avg_latency_ms: Option<i32>,
    error_count: i32,
    headers: BTreeMap<String, String>,
}

impl EndpointResponse {
    fn from_row(r: &RpcEndpoint) -> Self {
        Self {
            id: r.id,
            chain_index: r.chain_index,
            url: r.url.clone(),
            protocol: r.protocol.clone(),
            weight: r.weight,
            enabled: r.enabled,
            tier: r.tier.clone(),
            is_archive: r.is_archive,
            priority: r.priority,
            healthy: r.healthy,
            avg_latency_ms: r.avg_latency_ms,
            error_count: r.error_count,
            headers: serde_json::from_value(r.headers.clone()).unwrap_or_default(),
        }
    }
}

#[derive(Deserialize)]
struct CreateEndpointRequest {
    chain_index: i64,
    url: String,
    #[serde(default)]
    protocol: String,
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
    #[serde(default)]
    headers: BTreeMap<String, String>,
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
    protocol: Option<String>,
    weight: Option<i32>,
    enabled: Option<bool>,
    tier: Option<String>,
    is_archive: Option<bool>,
    priority: Option<i32>,
    headers: Option<BTreeMap<String, String>>,
}

fn validate_protocol(protocol: &str) -> Result<(), AppError> {
    if !protocol.is_empty() {
        crate::protocol::EndpointProtocol::from_config(protocol)?;
    }
    Ok(())
}

#[handler]
async fn list_endpoints(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<Vec<EndpointResponse>>, AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("state missing"))?;
        require_admin(req, depot, st)?;

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
        require_admin(req, depot, st)?;

        let body: CreateEndpointRequest = req
            .parse_json()
            .await
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?;

        proxy::validate_endpoint_url(&body.url)?;
        validate_headers(&body.headers)?;
        validate_protocol(&body.protocol)?;

        let headers = serde_json::to_value(&body.headers)
            .map_err(|e| AppError::internal(format!("encode headers: {e}")))?;

        let mut db = st.db.clone_inner();
        let row = toasty::create!(RpcEndpoint {
            chain_index: body.chain_index,
            url: &body.url,
            protocol: &body.protocol,
            weight: body.weight,
            enabled: body.enabled,
            tier: &body.tier,
            is_archive: body.is_archive,
            priority: body.priority,
            headers: &headers,
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
        require_admin(req, depot, st)?;

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
        if let Some(headers) = &body.headers {
            validate_headers(headers)?;
        }
        if let Some(protocol) = &body.protocol {
            validate_protocol(protocol)?;
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
        if let Some(protocol) = &body.protocol {
            upd = upd.protocol(protocol);
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
        if let Some(headers) = &body.headers {
            let headers = serde_json::to_value(headers)
                .map_err(|e| AppError::internal(format!("encode headers: {e}")))?;
            upd = upd.headers(&headers);
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
        require_admin(req, depot, st)?;

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
        require_admin(req, depot, st)?;

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
    client_ip: Option<String>,
    method: Option<String>,
    total_requests: i64,
    success_count: i64,
    error_count: i64,
    avg_latency_ms: f64,
}

#[handler]
async fn get_stats(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<Vec<StatsSummary>>, AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("state missing"))?;
        require_admin(req, depot, st)?;

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

        let mut map: std::collections::HashMap<(String, i64, Option<String>, Option<String>), (i64, i64, i64)> =
            std::collections::HashMap::new();

        for row in &rows {
            let entry = map
                .entry((
                    row.api_key.clone(),
                    row.chain_index,
                    row.client_ip.clone(),
                    row.method.clone(),
                ))
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
                |((api_key, chain_index, client_ip, method), (total, errors, total_latency))| {
                    StatsSummary {
                        api_key,
                        chain_index,
                        client_ip,
                        method,
                        total_requests: total,
                        success_count: total - errors,
                        error_count: errors,
                        avg_latency_ms: if total > 0 {
                            total_latency as f64 / total as f64
                        } else {
                            0.0
                        },
                    }
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

#[derive(Serialize)]
struct StatsSeriesPoint {
    ts: i64,
    success: i64,
    error: i64,
    avg_latency_ms: f64,
}

#[derive(Serialize)]
struct StatsSeries {
    range: String,
    bucket_seconds: i64,
    points: Vec<StatsSeriesPoint>,
}

#[handler]
async fn get_stats_series(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<StatsSeries>, AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("state missing"))?;
        require_admin(req, depot, st)?;

        let range: String = req.query("range").unwrap_or_else(|| "1h".into());
        let api_key: Option<String> = req.query("api_key");
        let chain_index: Option<i64> = req.query("chain_index");

        let (duration_secs, bucket_secs) = parse_stats_range(&range)?;

        let now = jiff::Timestamp::now();
        let cutoff = now
            .checked_sub(jiff::Span::new().seconds(duration_secs))
            .map_err(|e| AppError::internal(e.to_string()))?;

        let mut q = ChainGatewayStats::all()
            .filter(ChainGatewayStats::fields().created_at().ge(cutoff));
        if let Some(key) = &api_key {
            q = q.filter(ChainGatewayStats::fields().api_key().eq(key));
        }
        if let Some(ci) = chain_index {
            q = q.filter(ChainGatewayStats::fields().chain_index().eq(ci));
        }

        let mut db = st.db.clone_inner();
        let rows: Vec<ChainGatewayStats> = q
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        let mut buckets: BTreeMap<i64, (i64, i64, i64)> = BTreeMap::new();
        for row in &rows {
            let bucket = (row.created_at.as_second() / bucket_secs) * bucket_secs;
            let entry = buckets.entry(bucket).or_insert((0, 0, 0));
            entry.0 += 1;
            if row.status_code >= 400 {
                entry.1 += 1;
            }
            entry.2 += row.latency_ms as i64;
        }

        let first_bucket = (cutoff.as_second() / bucket_secs) * bucket_secs;
        let last_bucket = (now.as_second() / bucket_secs) * bucket_secs;
        let mut points = Vec::new();
        let mut bucket = first_bucket;
        while bucket <= last_bucket {
            let (total, errors, total_latency) = buckets.get(&bucket).copied().unwrap_or((0, 0, 0));
            points.push(StatsSeriesPoint {
                ts: bucket,
                success: total - errors,
                error: errors,
                avg_latency_ms: if total > 0 {
                    total_latency as f64 / total as f64
                } else {
                    0.0
                },
            });
            bucket += bucket_secs;
        }

        Ok(Json(StatsSeries {
            range,
            bucket_seconds: bucket_secs,
            points,
        }))
    }
    .await;
    match result {
        Ok(j) => res.render(j),
        Err(e) => res.render(e),
    }
}

#[derive(Serialize)]
struct StatsByIpItem {
    client_ip: String,
    total_requests: i64,
    success_count: i64,
    error_count: i64,
    avg_latency_ms: f64,
}

#[derive(Serialize)]
struct StatsByIp {
    range: String,
    items: Vec<StatsByIpItem>,
}

fn parse_stats_range(range: &str) -> Result<(i64, i64), AppError> {
    match range {
        "15m" => Ok((15 * 60, 60)),
        "1h" => Ok((60 * 60, 5 * 60)),
        "24h" => Ok((24 * 60 * 60, 60 * 60)),
        "7d" => Ok((7 * 24 * 60 * 60, 6 * 60 * 60)),
        "30d" => Ok((30 * 24 * 60 * 60, 24 * 60 * 60)),
        other => Err(AppError::InvalidArgument(format!(
            "unsupported range '{other}', expected one of 15m, 1h, 24h, 7d, 30d"
        ))),
    }
}

#[handler]
async fn get_stats_by_ip(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<StatsByIp>, AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("state missing"))?;
        require_admin(req, depot, st)?;

        let range: String = req.query("range").unwrap_or_else(|| "1h".into());
        let api_key: Option<String> = req.query("api_key");
        let chain_index: Option<i64> = req.query("chain_index");
        let limit_raw: i64 = req.query("limit").unwrap_or(12);
        let limit = limit_raw.clamp(1, 50) as usize;

        let (duration_secs, _) = parse_stats_range(&range)?;
        let now = jiff::Timestamp::now();
        let cutoff = now
            .checked_sub(jiff::Span::new().seconds(duration_secs))
            .map_err(|e| AppError::internal(e.to_string()))?;

        let mut q = ChainGatewayStats::all()
            .filter(ChainGatewayStats::fields().created_at().ge(cutoff));
        if let Some(key) = &api_key {
            q = q.filter(ChainGatewayStats::fields().api_key().eq(key));
        }
        if let Some(ci) = chain_index {
            q = q.filter(ChainGatewayStats::fields().chain_index().eq(ci));
        }

        let mut db = st.db.clone_inner();
        let rows: Vec<ChainGatewayStats> = q
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        let mut map: std::collections::HashMap<String, (i64, i64, i64)> =
            std::collections::HashMap::new();
        for row in &rows {
            let ip = row
                .client_ip
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .unwrap_or("未知")
                .to_string();
            let entry = map.entry(ip).or_insert((0, 0, 0));
            entry.0 += 1;
            if row.status_code >= 400 {
                entry.1 += 1;
            }
            entry.2 += row.latency_ms as i64;
        }

        let mut items: Vec<StatsByIpItem> = map
            .into_iter()
            .map(|(client_ip, (total, errors, total_latency))| StatsByIpItem {
                client_ip,
                total_requests: total,
                success_count: total - errors,
                error_count: errors,
                avg_latency_ms: if total > 0 {
                    total_latency as f64 / total as f64
                } else {
                    0.0
                },
            })
            .collect();
        items.sort_by(|a, b| {
            b.total_requests
                .cmp(&a.total_requests)
                .then_with(|| a.client_ip.cmp(&b.client_ip))
        });
        items.truncate(limit);

        Ok(Json(StatsByIp { range, items }))
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
        require_admin(req, depot, st)?;

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
        require_admin(req, depot, st)?;

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
        require_admin(req, depot, st)?;

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
        .push(Router::with_path("login").post(login))
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
        .push(Router::with_path("stats/series").get(get_stats_series))
        .push(Router::with_path("stats/by-ip").get(get_stats_by_ip))
        .push(Router::with_path("stats").get(get_stats))
        .push(
            Router::with_path("settings")
                .get(list_settings)
                .post(create_setting),
        )
        .push(Router::with_path("settings/{key}").put(update_setting))
}

#[cfg(test)]
mod tests {
    use super::{sign_token, verify_token};

    #[test]
    fn token_roundtrip() {
        let token = sign_token("secret", "admin");
        assert_eq!(verify_token("secret", &token).as_deref(), Some("admin"));
    }

    #[test]
    fn token_rejects_wrong_secret() {
        let token = sign_token("secret", "admin");
        assert!(verify_token("wrong", &token).is_none());
    }

    #[test]
    fn token_rejects_tampered() {
        let token = sign_token("secret", "admin");
        let mut tampered = token;
        let dot = tampered.find('.').unwrap();
        tampered.replace_range(dot + 1.., "AAAA");
        assert!(verify_token("secret", &tampered).is_none());
    }
}
