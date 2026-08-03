import { clearToken, getToken } from './auth.js'

export function createApi({ onUnauthorized } = {}) {
  async function api(path, opts = {}) {
    const headers = {
      'Content-Type': 'application/json',
      ...(opts.headers || {}),
    }
    const token = getToken()
    if (token) headers.Authorization = `Bearer ${token}`

    const response = await fetch(path, { ...opts, headers })
    if (response.status === 401) {
      clearToken()
      onUnauthorized?.()
      const error = await response.json().catch(() => ({}))
      throw new Error(error.message || 'session expired')
    }
    if (!response.ok) {
      const error = await response.json().catch(() => ({}))
      throw new Error(error.message || response.statusText)
    }
    return response.json()
  }

  return { api }
}
