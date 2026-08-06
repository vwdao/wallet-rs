import { describe, it, expect } from 'vitest'
import { CHAIN_OPTIONS } from './chains.js'
import { mergeChainOptions, normalizeProtocol } from './networks.js'

describe('mergeChainOptions', () => {
  it('keeps static options and adds new networks', () => {
    const options = mergeChainOptions([
      { chain_index: 60, name: 'Ethereum', family: 'evm' },
      { chain_index: 9999, name: '自定义链', family: 'evm' },
    ])
    expect(options.find((o) => o.id === 60).name).toBe('Ethereum')
    expect(options.find((o) => o.id === 9999).name).toBe('自定义链')
  })

  it('handles empty or invalid input', () => {
    expect(mergeChainOptions(null)).toHaveLength(CHAIN_OPTIONS.length)
    expect(mergeChainOptions([])).toHaveLength(CHAIN_OPTIONS.length)
    expect(mergeChainOptions([{ chain_index: 'x' }])).toHaveLength(CHAIN_OPTIONS.length)
  })

  it('sorts by name', () => {
    const options = mergeChainOptions([{ chain_index: 2, name: 'ZZ', family: 'evm' }])
    const names = options.map((o) => o.name)
    expect(names).toEqual([...names].sort((a, b) => a.localeCompare(b)))
  })
})

describe('normalizeProtocol', () => {
  it('normalizes alias protocols', () => {
    expect(normalizeProtocol('https')).toBe('http')
    expect(normalizeProtocol('wss')).toBe('ws')
    expect(normalizeProtocol('grpcs')).toBe('grpc')
    expect(normalizeProtocol('  HTTP ')).toBe('http')
  })

  it('infers protocol from url when stored protocol is empty', () => {
    expect(normalizeProtocol('', 'https://rpc.example.com')).toBe('http')
    expect(normalizeProtocol('', 'wss://rpc.example.com')).toBe('ws')
    expect(normalizeProtocol('', 'grpcs://gmain.vvwallet.org:443')).toBe('grpc')
    expect(normalizeProtocol('', 'tcp://127.0.0.1:50051')).toBe('tcp')
  })

  it('returns empty when no protocol or url', () => {
    expect(normalizeProtocol('', '')).toBe('')
    expect(normalizeProtocol(undefined, undefined)).toBe('')
    expect(normalizeProtocol('', 'not-a-url')).toBe('')
  })

  it('prefers stored protocol over url', () => {
    expect(normalizeProtocol('ws', 'https://rpc.example.com')).toBe('ws')
  })
})
