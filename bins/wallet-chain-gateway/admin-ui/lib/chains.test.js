import { describe, it, expect } from 'vitest'
import { chainName, chainSlug } from './chains.js'

describe('chainName', () => {
  it('maps known chains', () => {
    expect(chainName(60)).toBe('ETH')
    expect(chainName(133)).toBe('ZEC')
    expect(chainName(501)).toBe('SOL')
    expect(chainName(607)).toBe('TON')
    expect(chainName(784)).toBe('SUI')
  })
  it('falls back to id', () => {
    expect(chainName(99999)).toBe('99999')
  })
})

describe('chainSlug', () => {
  it('uses the canonical lowercase slug for known chains', () => {
    expect(chainSlug({ id: 10042221, name: 'Arbitrum One' })).toBe('arb')
    expect(chainSlug({ id: 60, name: 'Ethereum' })).toBe('eth')
    expect(chainSlug({ id: 10009000, name: 'Avalanche' })).toBe('avax')
  })

  it('falls back to the display name for custom chains', () => {
    expect(chainSlug({ id: 9999, name: 'My Custom Chain' })).toBe('my custom chain')
    expect(chainSlug({ id: 9999, name: '自定义链' })).toBe('自定义链')
  })

  it('accepts chain_index-shaped objects', () => {
    expect(chainSlug({ chain_index: 607, name: 'TON Mainnet' })).toBe('ton')
  })
})
