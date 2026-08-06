import { CHAIN_OPTIONS, chainName } from './chains.js'

export function mergeChainOptions(networks) {
  const known = new Map(CHAIN_OPTIONS.map((item) => [item.id, item]))
  for (const network of networks || []) {
    const id = Number(network.chain_index)
    if (!Number.isFinite(id)) continue
    const name = network.name || chainName(id)
    known.set(id, { id, name, family: network.family })
  }
  return [...known.values()].sort((a, b) => a.name.localeCompare(b.name))
}

const PROTOCOL_ALIASES = {
  https: 'http',
  wss: 'ws',
  grpcs: 'grpc',
}

export function normalizeProtocol(protocol, url) {
  const value = String(protocol || '').trim().toLowerCase()
  if (value) return PROTOCOL_ALIASES[value] || value
  if (url) {
    try {
      const scheme = new URL(url).protocol.replace(':', '').toLowerCase()
      return PROTOCOL_ALIASES[scheme] || scheme
    } catch {
      return ''
    }
  }
  return ''
}
