'use client'

import { useState } from 'react'
import { setToken } from '../lib/auth.js'
import { Button } from './ui/Button.jsx'
import { Field } from './ui/Field.jsx'

export function Login({ onSuccess }) {
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const [error, setError] = useState('')
  const [submitting, setSubmitting] = useState(false)

  async function handleSubmit(event) {
    event.preventDefault()
    setError('')
    setSubmitting(true)

    try {
      const response = await fetch('/admin/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username, password }),
      })
      const data = await response.json().catch(() => ({}))
      if (!response.ok) {
        throw new Error(data.message || '用户名、密码错误或服务不可用')
      }
      if (!data.token) {
        throw new Error('登录响应缺少访问令牌')
      }

      setToken(data.token)
      onSuccess?.()
    } catch (loginError) {
      setError(loginError.message || '登录失败，请稍后重试')
    } finally {
      setSubmitting(false)
    }
  }

  return (
    <main className="login">
      <form className="login-card" onSubmit={handleSubmit}>
        <h1>Chain Gateway Admin</h1>
        <p>请输入管理员账号登录</p>
        <Field label="用户名">
          <input
            type="text"
            autoComplete="username"
            autoFocus
            required
            value={username}
            onChange={(event) => setUsername(event.target.value)}
          />
        </Field>
        <Field label="密码">
          <input
            type="password"
            autoComplete="current-password"
            required
            value={password}
            onChange={(event) => setPassword(event.target.value)}
          />
        </Field>
        {error ? <div className="alert danger" role="alert">{error}</div> : null}
        <Button type="submit" disabled={submitting}>
          {submitting ? '登录中…' : '登录'}
        </Button>
      </form>
    </main>
  )
}
