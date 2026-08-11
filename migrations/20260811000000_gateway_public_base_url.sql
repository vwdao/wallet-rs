-- 网关对外 RPC 基础地址。连接页用它生成可供外部客户端调用的 RPC 链接，
-- 留空时连接页回退到当前访问地址（window.location.origin）。
INSERT INTO gateway_settings (key, value, description) VALUES
    ('gateway_public_base_url', '', '网关对外 HTTP/WebSocket RPC 基础地址（如 https://gw.example.com），连接页默认使用'),
    ('gateway_public_grpc_url', '', '网关对外 gRPC 基础地址（如 grpc://gw.example.com:50051），TRON 等支持 gRPC 的链使用')
ON CONFLICT (key) DO NOTHING;
