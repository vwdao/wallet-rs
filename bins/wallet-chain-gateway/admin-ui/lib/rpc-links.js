const PROTOCOL_ALIASES = {
  http: 'http',
  https: 'http',
  ws: 'ws',
  wss: 'ws',
  grpc: 'grpc',
  grpcs: 'grpc',
  tcp: 'tcp',
}

export const LINK_META = {
  http: { label: 'HTTP JSON-RPC', note: 'POST 请求，标准 JSON-RPC' },
  ws: { label: 'WebSocket', note: 'WebSocket 隧道' },
  grpc: { label: 'gRPC', note: 'gRPC 接口' },
}

export function endpointProtocol(endpoint) {
  const stored = String(endpoint?.protocol || '').trim().toLowerCase()
  if (stored) return PROTOCOL_ALIASES[stored] || null
  try {
    const scheme = new URL(endpoint?.url || '').protocol.replace(':', '').toLowerCase()
    return PROTOCOL_ALIASES[scheme] || null
  } catch {
    return null
  }
}

export function availableLinkTypes(endpoints) {
  const protocols = new Set()
  for (const endpoint of endpoints || []) {
    const protocol = endpointProtocol(endpoint)
    if (protocol) protocols.add(protocol)
  }
  if (protocols.size === 0) return []
  const types = ['http']
  if (protocols.has('ws') || protocols.has('grpc') || protocols.has('tcp')) {
    types.push('ws')
  }
  if (protocols.has('grpc')) {
    types.push('grpc')
  }
  return types
}

function cleanBase(base) {
  return String(base || '').trim().replace(/\/+$/, '')
}

function schemeOf(base) {
  const match = String(base || '').match(/^([a-z][a-z0-9+.-]*):\/\//i)
  return match ? match[1].toLowerCase() : null
}

function withScheme(base, scheme) {
  const clean = cleanBase(base)
  if (schemeOf(clean)) {
    return clean.replace(/^[a-z][a-z0-9+.-]*:\/\//i, `${scheme}://`)
  }
  return `${scheme}://${clean}`
}

function ensureScheme(base, fallback) {
  const clean = cleanBase(base)
  return schemeOf(clean) ? clean : `${fallback}://${clean}`
}

export function buildRpcLinks({ base, grpcBase, chainName: rawName, apiKey, endpoints }) {
  const path = encodeURIComponent(String(rawName || '').trim().toLowerCase())
  const key = String(apiKey || '').trim()
  const links = []

  for (const type of availableLinkTypes(endpoints)) {
    if (type === 'http') {
      links.push({
        type,
        url: `${ensureScheme(base, 'http')}/rpc/${path}/${key}`,
      })
    } else if (type === 'ws') {
      const wsScheme = schemeOf(base) === 'https' ? 'wss' : 'ws'
      links.push({
        type,
        url: `${withScheme(base, wsScheme)}/rpc/${path}/${key}`,
      })
    } else if (type === 'grpc') {
      const grpcClean = cleanBase(grpcBase)
      if (grpcClean) {
        links.push({
          type,
          url: `${ensureScheme(grpcClean, 'grpc')}/rpc/${path}/${key}`,
        })
      }
    }
  }
  return links
}
