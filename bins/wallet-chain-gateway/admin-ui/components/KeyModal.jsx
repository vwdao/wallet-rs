'use client'

import { useEffect, useState } from 'react'
import { Button } from './ui/Button.jsx'
import { Field } from './ui/Field.jsx'
import { Modal } from './ui/Modal.jsx'

const EMPTY_FORM = {
  id: '',
  name: '',
  rate_limit_per_min: 60,
  allowed_tier: 'all',
  chainsStr: '',
  ipWhitelistStr: '',
  ipBlacklistStr: '',
  enabled: true,
}

function keyToForm(key) {
  if (!key) return EMPTY_FORM

  return {
    id: key.id || '',
    name: key.name || '',
    rate_limit_per_min: key.rate_limit_per_min ?? 60,
    allowed_tier: key.allowed_tier || 'all',
    chainsStr: Array.isArray(key.allowed_chains) ? key.allowed_chains.join(', ') : '',
    ipWhitelistStr: Array.isArray(key.ip_whitelist) ? key.ip_whitelist.join(', ') : '',
    ipBlacklistStr: Array.isArray(key.ip_blacklist) ? key.ip_blacklist.join(', ') : '',
    enabled: key.enabled ?? true,
  }
}

function parseRateLimit(value) {
  if (value === '' || value == null) return 60
  const n = Number(value)
  if (!Number.isFinite(n) || n < 0) return 60
  return Math.trunc(n)
}

export function keyToPayload(form) {
  const chainsStr = form.chainsStr ?? form.allowed_chains ?? ''
  const whitelistStr = form.ipWhitelistStr ?? form.ip_whitelist ?? ''
  const blacklistStr = form.ipBlacklistStr ?? form.ip_blacklist ?? ''

  return {
    name: form.name.trim(),
    rate_limit_per_min: parseRateLimit(form.rate_limit_per_min),
    allowed_tier: form.allowed_tier || 'all',
    allowed_chains: chainsStr
      ? chainsStr.split(',').map((value) => parseInt(value.trim(), 10)).filter(Boolean)
      : [],
    ip_whitelist: splitCsv(whitelistStr),
    ip_blacklist: splitCsv(blacklistStr),
    enabled: form.enabled === true || form.enabled === 'true',
  }
}

function splitCsv(value) {
  return value
    .split(',')
    .map((entry) => entry.trim())
    .filter(Boolean)
}

export function KeyModal({ open, apiKey, api, onClose, onSaved }) {
  const [form, setForm] = useState(EMPTY_FORM)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState('')

  useEffect(() => {
    if (!open) return
    setForm(keyToForm(apiKey))
    setError('')
    setSaving(false)
  }, [apiKey, open])

  function update(name, value) {
    setForm((current) => ({ ...current, [name]: value }))
  }

  async function handleSubmit(event) {
    event.preventDefault()
    setError('')
    setSaving(true)

    try {
      const body = keyToPayload(form)
      if (!body.name) throw new Error('名称必填')

      await api(form.id ? `/admin/keys/${form.id}` : '/admin/keys', {
        method: form.id ? 'PUT' : 'POST',
        body: JSON.stringify(body),
      })
      await onSaved?.()
    } catch (saveError) {
      setError(saveError.message || '保存 API 密钥失败')
    } finally {
      setSaving(false)
    }
  }

  return (
    <Modal
      open={open}
      title={form.id ? '编辑 API 密钥' : '新增 API 密钥'}
      onClose={onClose}
      footer={
        <>
          <Button variant="ghost" onClick={onClose} disabled={saving}>取消</Button>
          <Button type="submit" form="key-form" loading={saving}>保存</Button>
        </>
      }
    >
      <form id="key-form" onSubmit={handleSubmit}>
        <div className="form-grid">
          <div className="field-full">
            <Field label="名称">
              <input
                type="text"
                required
                placeholder="例如：内部服务"
                value={form.name}
                onChange={(event) => update('name', event.target.value)}
              />
            </Field>
          </div>
          <Field label="每分钟限流" hint="0 表示不限制">
            <input
              type="number"
              min="0"
              value={form.rate_limit_per_min}
              onChange={(event) => update('rate_limit_per_min', event.target.value)}
            />
          </Field>
          <Field label="允许层级">
            <select
              value={form.allowed_tier}
              onChange={(event) => update('allowed_tier', event.target.value)}
            >
              <option value="all">全部</option>
              <option value="free">免费</option>
              <option value="paid">付费</option>
            </select>
          </Field>
          <div className="field-full">
            <Field label="允许链" hint="链索引以逗号分隔；留空表示全部链">
              <input
                type="text"
                placeholder="60, 195, 501"
                value={form.chainsStr}
                onChange={(event) => update('chainsStr', event.target.value)}
              />
            </Field>
          </div>
          <div className="field-full">
            <Field label="IP 白名单" hint="IP 或 CIDR 以逗号分隔；留空表示不限制来源 IP">
              <input
                type="text"
                placeholder="203.0.113.0/24, 2001:db8::1"
                value={form.ipWhitelistStr}
                onChange={(event) => update('ipWhitelistStr', event.target.value)}
              />
            </Field>
          </div>
          <div className="field-full">
            <Field label="IP 黑名单" hint="IP 或 CIDR 以逗号分隔；命中即拒绝">
              <input
                type="text"
                placeholder="198.51.100.7"
                value={form.ipBlacklistStr}
                onChange={(event) => update('ipBlacklistStr', event.target.value)}
              />
            </Field>
          </div>
          <Field label="启用">
            <select
              value={String(form.enabled)}
              onChange={(event) => update('enabled', event.target.value)}
            >
              <option value="true">是</option>
              <option value="false">否</option>
            </select>
          </Field>
        </div>
        {error ? <div className="alert danger" role="alert">{error}</div> : null}
      </form>
    </Modal>
  )
}
