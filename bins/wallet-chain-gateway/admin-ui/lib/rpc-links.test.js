import { describe, expect, it } from 'vitest'
import {
  availableLinkTypes,
  buildRpcLinks,
  endpointProtocol,
} from './rpc-links.js'

describe('endpointProtocol', () => {
  it('normalizes stored protocol values', () => {
    expect(endpointProtocol({ protocol: 'https' })).toBe('http')
    expect(endpointProtocol({ protocol: 'wss' })).toBe('ws')
    expect(endpointProtocol({ protocol: 'grpcs' })).toBe('grpc')
    expect(endpointProtocol({ protocol: 'TCP' })).toBe('tcp')
  })

  it('infers protocol from the url scheme', () => {
    expect(endpointProtocol({ url: 'https://rpc.eth.example' })).toBe('http')
    expect(endpointProtocol({ url: 'ws://host:8080' })).toBe('ws')
    expect(endpointProtocol({ url: 'grpc://host:50051' })).toBe('grpc')
  })

  it('returns null for unknown or empty', () => {
    expect(endpointProtocol({})).toBe(null)
    expect(endpointProtocol(null)).toBe(null)
    expect(endpointProtocol({ protocol: 'foo', url: 'nope' })).toBe(null)
  })
})

describe('availableLinkTypes', () => {
  it('http only when all endpoints are http', () => {
    expect(availableLinkTypes([{ protocol: 'http' }, { url: 'https://x' }])).toEqual(['http'])
  })

  it('adds ws when tunnel-capable endpoints exist', () => {
    expect(availableLinkTypes([{ protocol: 'ws' }])).toEqual(['http', 'ws'])
    expect(availableLinkTypes([{ url: 'tcp://1.2.3.4:50051' }])).toEqual(['http', 'ws'])
  })

  it('adds grpc when grpc endpoints exist', () => {
    expect(availableLinkTypes([{ protocol: 'grpc' }])).toEqual(['http', 'ws', 'grpc'])
  })

  it('returns empty for no endpoints', () => {
    expect(availableLinkTypes([])).toEqual([])
    expect(availableLinkTypes(null)).toEqual([])
  })

  it('restricts types to the chain allowlist when set', () => {
    expect(availableLinkTypes([{ protocol: 'grpc' }], ['grpc'])).toEqual(['grpc'])
    expect(availableLinkTypes([{ protocol: 'http' }], ['http'])).toEqual(['http'])
    expect(availableLinkTypes([{ protocol: 'http' }], ['grpc'])).toEqual([])
    expect(availableLinkTypes([{ protocol: 'ws' }], ['http', 'ws'])).toEqual(['http', 'ws'])
  })

  it('treats empty allowlist as auto-detect', () => {
    expect(availableLinkTypes([{ protocol: 'grpc' }], [])).toEqual(['http', 'ws', 'grpc'])
    expect(availableLinkTypes([{ protocol: 'grpc' }], null)).toEqual(['http', 'ws', 'grpc'])
  })
})

describe('buildRpcLinks', () => {
  const base = 'http://127.0.0.1:8545'
  const grpcBase = 'grpc://127.0.0.1:50051'

  it('builds http + ws + grpc links for grpc chains', () => {
    const links = buildRpcLinks({
      base,
      grpcBase,
      chainName: 'TON',
      apiKey: 'gw_abc',
      endpoints: [{ protocol: 'grpc' }],
    })
    expect(links.map((l) => l.type)).toEqual(['http', 'ws', 'grpc'])
    expect(links[0].url).toBe('http://127.0.0.1:8545/rpc/ton/gw_abc')
    expect(links[1].url).toBe('ws://127.0.0.1:8545/rpc/ton/gw_abc')
    expect(links[2].url).toBe('grpc://127.0.0.1:50051/rpc/ton/gw_abc')
  })

  it('omits grpc link when no grpc base is configured', () => {
    const links = buildRpcLinks({
      base,
      grpcBase: '',
      chainName: 'TRON',
      apiKey: 'gw_abc',
      endpoints: [{ protocol: 'grpc' }],
    })
    expect(links.map((l) => l.type)).toEqual(['http', 'ws'])
  })

  it('derives wss from https base', () => {
    const links = buildRpcLinks({
      base: 'https://gw.example.com',
      grpcBase,
      chainName: 'ETH',
      apiKey: 'gw_abc',
      endpoints: [{ protocol: 'ws' }],
    })
    expect(links[0].url).toBe('https://gw.example.com/rpc/eth/gw_abc')
    expect(links[1].url).toBe('wss://gw.example.com/rpc/eth/gw_abc')
  })

  it('url-encodes arbitrary chain names with spaces', () => {
    const links = buildRpcLinks({
      base,
      grpcBase: '',
      chainName: 'My Custom Chain',
      apiKey: 'gw_abc',
      endpoints: [{ protocol: 'http' }],
    })
    expect(links[0].url).toBe('http://127.0.0.1:8545/rpc/my%20custom%20chain/gw_abc')
  })

  it('keeps canonical short slugs (e.g. arb) as-is', () => {
    const links = buildRpcLinks({
      base,
      grpcBase: '',
      chainName: 'arb',
      apiKey: 'gw_abc',
      endpoints: [{ protocol: 'http' }],
    })
    expect(links[0].url).toBe('http://127.0.0.1:8545/rpc/arb/gw_abc')
  })

  it('returns no links when the chain has no endpoints', () => {
    expect(
      buildRpcLinks({ base, grpcBase, chainName: 'ETH', apiKey: 'gw_abc', endpoints: [] }),
    ).toEqual([])
  })

  it('respects the chain supported protocols allowlist', () => {
    const links = buildRpcLinks({
      base,
      grpcBase,
      chainName: 'TRON',
      apiKey: 'gw_abc',
      endpoints: [{ protocol: 'grpc' }, { protocol: 'http' }],
      supportedProtocols: ['grpc'],
    })
    expect(links.map((l) => l.type)).toEqual(['grpc'])
  })
})
