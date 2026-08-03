import { beforeEach, describe, expect, it, vi } from 'vitest'
import { setToken, TOKEN_KEY } from './auth.js'
import { createApi } from './api.js'

beforeEach(() => {
  globalThis.localStorage = {
    store: {},
    getItem(k) { return this.store[k] ?? null },
    setItem(k, v) { this.store[k] = String(v) },
    removeItem(k) { delete this.store[k] },
  }
})

describe('createApi', () => {
  it('sends the stored token as a Bearer authorization header', async () => {
    setToken('secret')
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ ok: true }),
    })

    const { api } = createApi()
    await expect(api('/admin/endpoints')).resolves.toEqual({ ok: true })

    expect(fetch).toHaveBeenCalledWith('/admin/endpoints', {
      headers: {
        'Content-Type': 'application/json',
        Authorization: 'Bearer secret',
      },
    })
  })

  it('clears the token and reports unauthorized responses', async () => {
    setToken('expired')
    const onUnauthorized = vi.fn()
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 401,
      json: async () => ({ message: 'token expired' }),
    })

    const { api } = createApi({ onUnauthorized })
    await expect(api('/admin/stats')).rejects.toThrow('token expired')

    expect(localStorage.getItem(TOKEN_KEY)).toBeNull()
    expect(onUnauthorized).toHaveBeenCalledOnce()
  })
})
