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

export function mergeChains(networks) {
  const map = new Map(
    CHAIN_OPTIONS.map((item) => [
      item.id,
      {
        id: item.id,
        name: item.name,
        family: '',
        enabled: false,
        supported_protocols: [],
        inDb: false,
      },
    ]),
  )
  for (const network of networks || []) {
    const id = Number(network.chain_index)
    if (!Number.isFinite(id)) continue
    const existing = map.get(id)
    const name = network.name || existing?.name || chainName(id)
    map.set(id, {
      id,
      name,
      family: network.family || existing?.family || '',
      enabled: network.enabled !== false,
      supported_protocols: Array.isArray(network.supported_protocols)
        ? network.supported_protocols.map(String)
        : [],
      inDb: true,
    })
  }
  return [...map.values()].sort((a, b) => a.name.localeCompare(b.name))
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
