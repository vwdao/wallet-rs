-- Runtime settings for chain-gateway, hot-reloadable without restart

CREATE TABLE IF NOT EXISTS gateway_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    description TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed defaults
INSERT INTO gateway_settings (key, value, description) VALUES
    ('health_check_interval_ms', '30000', 'RPC 健康检查间隔（毫秒）'),
    ('failure_threshold', '3', '端点连续失败多少次标记为不健康'),
    ('rpc_timeout_secs', '30', '单次 RPC 请求超时（秒）'),
    ('max_retries', '3', '单次请求最大重试次数'),
    ('max_block_lag', '10', 'RPC 端点允许落后最高区块的数量'),
    ('global_rate_limit_per_min', '0', '全局每分钟限流（0=不限制）'),
    ('stats_batch_interval_ms', '1000', '统计批量写入间隔（毫秒）'),
    ('log_requests', 'false', '是否记录请求日志')
ON CONFLICT (key) DO NOTHING;
