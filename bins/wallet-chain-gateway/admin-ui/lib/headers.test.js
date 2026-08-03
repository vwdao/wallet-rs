import { describe, it, expect } from 'vitest'
import { headersToText, headersFromText } from './headers.js'

describe('headers', () => {
  it('roundtrips name: value lines', () => {
    const text = 'Authorization: Bearer xxx\nX-Api-Key: yyy'
    expect(headersFromText(text)).toEqual({
      Authorization: 'Bearer xxx',
      'X-Api-Key': 'yyy',
    })
    expect(headersToText(headersFromText(text))).toBe(text)
  })

  it('skips invalid lines', () => {
    expect(headersFromText('nope\n: empty\nOk: 1')).toEqual({ Ok: '1' })
  })
})
