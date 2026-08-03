import { describe, expect, it } from 'vitest'
import { keyToPayload } from './KeyModal.jsx'

describe('keyToPayload', () => {
  it('normalizes key form values for the API', () => {
    expect(keyToPayload({
      name: '  partner  ',
      rate_limit_per_min: '120',
      allowed_tier: 'paid',
      allowed_chains: '60, 195, nope, 0',
      enabled: 'false',
    })).toEqual({
      name: 'partner',
      rate_limit_per_min: 120,
      allowed_tier: 'paid',
      allowed_chains: [60, 195],
      enabled: false,
    })
  })

  it('uses the documented defaults for empty values', () => {
    expect(keyToPayload({
      name: 'internal',
      rate_limit_per_min: '',
      allowed_tier: '',
      allowed_chains: '',
      enabled: true,
    })).toEqual({
      name: 'internal',
      rate_limit_per_min: 60,
      allowed_tier: 'all',
      allowed_chains: [],
      enabled: true,
    })
  })
})
