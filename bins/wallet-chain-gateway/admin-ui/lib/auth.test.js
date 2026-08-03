import { describe, it, expect, beforeEach } from 'vitest'
import { getToken, setToken, clearToken, TOKEN_KEY } from './auth.js'

beforeEach(() => {
  globalThis.localStorage = {
    store: {},
    getItem(k) { return this.store[k] ?? null },
    setItem(k, v) { this.store[k] = String(v) },
    removeItem(k) { delete this.store[k] },
  }
})

describe('auth', () => {
  it('stores and clears token', () => {
    expect(getToken()).toBe('')
    setToken('abc')
    expect(getToken()).toBe('abc')
    expect(localStorage.getItem(TOKEN_KEY)).toBe('abc')
    clearToken()
    expect(getToken()).toBe('')
  })
})
