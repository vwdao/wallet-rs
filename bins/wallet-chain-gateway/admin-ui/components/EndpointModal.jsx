'use client'

import { useEffect, useState } from 'react'
import { headersFromText, headersToText } from '../lib/headers.js'
import { Button } from './ui/Button.jsx'
import { Field } from './ui/Field.jsx'
import { Modal } from './ui/Modal.jsx'

const EMPTY_FORM = {
  id: '',
  chain_index: '',
  priority: 0,
  url: '',
  protocol: '',
  tier: 'free',
  weight: 1,
  is_archive: false,
  enabled: true,
  headersText: '',
}

function endpointToForm(endpoint) {
  if (!endpoint) return EMPTY_FORM

  return {
    id: endpoint.id || '',
    chain_index: endpoint.chain_index ?? '',
    priority: endpoint.priority ?? 0,
    url: endpoint.url || '',
    protocol: endpoint.protocol || '',
    tier: endpoint.tier || 'free',
    weight: endpoint.weight ?? 1,
    is_archive: endpoint.is_archive ?? false,
    enabled: endpoint.enabled ?? true,
    headersText: headersToText(endpoint.headers),
  }
}

export function EndpointModal({ open, endpoint, api, onClose, onSaved }) {
  const [form, setForm] = useState(EMPTY_FORM)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState('')

  useEffect(() => {
    if (!open) return
    setForm(endpointToForm(endpoint))
    setError('')
    setSaving(false)
  }, [endpoint, open])

  function update(name, value) {
    setForm((current) => ({ ...current, [name]: value }))
  }

  async function handleSubmit(event) {
    event.preventDefault()
    setError('')
    setSaving(true)

    try {
      const body = {
        chain_index: Number(form.chain_index) || 60,
        url: form.url.trim(),
        protocol: form.protocol || '',
        weight: Number(form.weight) || 1,
        enabled: form.enabled === true || form.enabled === 'true',
        tier: form.tier || 'free',
        is_archive: form.is_archive === true || form.is_archive === 'true',
        priority: Number(form.priority) || 0,
        headers: headersFromText(form.headersText),
      }
      if (!body.url) throw new Error('URL 必填')

      if (form.id) {
        await api(`/admin/endpoints/${form.id}`, {
          method: 'PUT',
          body: JSON.stringify(body),
        })
      } else {
        await api('/admin/endpoints', {
          method: 'POST',
          body: JSON.stringify(body),
        })
      }
      await onSaved?.()
    } catch (saveError) {
      setError(saveError.message || '保存节点失败')
    } finally {
      setSaving(false)
    }
  }

  return (
    <Modal
      open={open}
      title={form.id ? '编辑 RPC 节点' : '新增 RPC 节点'}
      onClose={onClose}
      footer={
        <>
          <Button variant="ghost" onClick={onClose} disabled={saving}>取消</Button>
          <Button type="submit" form="endpoint-form" loading={saving}>保存</Button>
        </>
      }
    >
      <form id="endpoint-form" onSubmit={handleSubmit}>
        <div className="form-grid">
          <Field label="链索引">
            <input
              type="number"
              placeholder="60 (ETH)"
              value={form.chain_index}
              onChange={(event) => update('chain_index', event.target.value)}
            />
          </Field>
          <Field label="优先级">
            <input
              type="number"
              value={form.priority}
              onChange={(event) => update('priority', event.target.value)}
            />
          </Field>
          <div className="field-full">
            <Field label="URL">
              <input
                type="text"
                required
                placeholder="https://rpc.example.com"
                value={form.url}
                onChange={(event) => update('url', event.target.value)}
              />
            </Field>
          </div>
          <Field label="协议">
            <select
              value={form.protocol}
              onChange={(event) => update('protocol', event.target.value)}
            >
              <option value="">自动（根据 URL）</option>
              <option value="http">http</option>
              <option value="ws">ws</option>
              <option value="grpc">grpc</option>
              <option value="tcp">tcp</option>
            </select>
          </Field>
          <Field label="层级">
            <select value={form.tier} onChange={(event) => update('tier', event.target.value)}>
              <option value="free">免费</option>
              <option value="paid">付费</option>
            </select>
          </Field>
          <Field label="权重">
            <input
              type="number"
              value={form.weight}
              onChange={(event) => update('weight', event.target.value)}
            />
          </Field>
          <Field label="归档节点">
            <select
              value={String(form.is_archive)}
              onChange={(event) => update('is_archive', event.target.value)}
            >
              <option value="false">否</option>
              <option value="true">是</option>
            </select>
          </Field>
          <Field label="启用">
            <select
              value={String(form.enabled)}
              onChange={(event) => update('enabled', event.target.value)}
            >
              <option value="true">是</option>
              <option value="false">否</option>
            </select>
          </Field>
          <div className="field-full">
            <Field label="请求头" hint="每行填写一个 Name: value">
              <textarea
                rows="4"
                placeholder={'Authorization: Bearer xxx\nX-Api-Key: yyy'}
                value={form.headersText}
                onChange={(event) => update('headersText', event.target.value)}
              />
            </Field>
          </div>
        </div>
        {error ? <div className="alert danger" role="alert">{error}</div> : null}
      </form>
    </Modal>
  )
}
