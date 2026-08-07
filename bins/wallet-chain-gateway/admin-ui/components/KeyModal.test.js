import { describe, expect, it } from 'vitest'
import { keyToPayload } from './KeyModal.jsx'

describe('keyToPayload', () => {
  it('normalizes key form values for the API', () => {
    expect(keyToPayload({
      name: '  partner  ',
      rate_limit_per_min: '120',
      allowed_tier: 'paid',
      allowed_chains: '60, 195, nope, 0',
      ipWhitelistStr: ' 203.0.113.0/24, 2001:db8::1 ',
      ipBlacklistStr: '198.51.100.7,',
      enabled: 'false',
    })).toEqual({
      name: 'partner',
      rate_limit_per_min: 120,
      allowed_tier: 'paid',
      allowed_chains: [60, 195],
      ip_whitelist: ['203.0.113.0/24', '2001:db8::1'],
      ip_blacklist: ['198.51.100.7'],
      enabled: false,
    })
  })

  it('uses the documented defaults for empty values', () => {
    expect(keyToPayload({
      name: 'internal',
      rate_limit_per_min: '',
      allowed_tier: '',
      allowed_chains: '',
      ipWhitelistStr: '',
      ipBlacklistStr: '',
      enabled: true,
    })).toEqual({
      name: 'internal',
      rate_limit_per_min: 60,
      allowed_tier: 'all',
      allowed_chains: [],
      ip_whitelist: [],
      ip_blacklist: [],
      enabled: true,
    })
  })

  it('treats missing list strings as empty lists', () => {
    expect(keyToPayload({ name: 'x', enabled: true })).toEqual({
      name: 'x',
      rate_limit_per_min: 60,
      allowed_tier: 'all',
      allowed_chains: [],
      ip_whitelist: [],
      ip_blacklist: [],
      enabled: true,
    })
  })
})
