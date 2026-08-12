const CHAIN_NAMES = {
  0: 'BTC', 3: 'DOGE', 60: 'ETH', 133: 'ZEC', 195: 'TRON', 501: 'SOL',
  607: 'TON', 784: 'SUI', 966: 'POL',
  20000714: 'BSC', 8453: 'BASE', 10042221: 'ARB', 10000070: 'OP',
  10009000: 'AVAX', 10000999: 'HYPER', 10004663: 'ROBIN',
}

export const CHAIN_OPTIONS = Object.entries(CHAIN_NAMES)
  .map(([id, name]) => ({ id: Number(id), name }))
  .sort((a, b) => a.name.localeCompare(b.name))

export const PROTOCOL_OPTIONS = [
  { value: 'http', label: 'HTTP' },
  { value: 'ws', label: 'WebSocket' },
  { value: 'grpc', label: 'gRPC' },
  { value: 'tcp', label: 'TCP' },
]

export function chainName(id) {
  return CHAIN_NAMES[id] ?? String(id)
}

/**
 * Canonical lowercase RPC path segment for a chain, matching the gateway
 * route (`/rpc/arb/...`, `/rpc/eth/...`). Known chains always use the
 * canonical short name regardless of the network's display name; custom
 * chains fall back to their display name so routing keeps working.
 */
export function chainSlug(chain) {
  const id = Number(chain?.id ?? chain?.chain_index)
  const known = CHAIN_NAMES[id]
  if (known) return known.toLowerCase()
  return String(chain?.name || id).trim().toLowerCase()
}
