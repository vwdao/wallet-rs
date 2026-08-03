-- RPC endpoint explicit transport protocol override (http/ws/grpc/tcp, empty = auto from url scheme)

ALTER TABLE rpc_endpoints ADD COLUMN IF NOT EXISTS protocol TEXT NOT NULL DEFAULT '';
