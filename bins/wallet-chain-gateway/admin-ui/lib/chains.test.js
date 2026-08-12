import { describe, it, expect } from 'vitest'
import { chainName } from './chains.js'

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
