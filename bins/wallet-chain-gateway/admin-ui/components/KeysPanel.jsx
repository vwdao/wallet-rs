'use client'

import { useCallback, useEffect, useState } from 'react'
import { chainName } from '../lib/chains.js'
import { KeyModal } from './KeyModal.jsx'
import { Badge } from './ui/Badge.jsx'
import { Button } from './ui/Button.jsx'
import { ConfirmDialog } from './ui/ConfirmDialog.jsx'
import { Empty } from './ui/Empty.jsx'

const TIER_LABELS = {
  all: '全部',
  free: '免费',
  paid: '付费',
}

export function KeysPanel({ api }) {
  const [keys, setKeys] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [modalOpen, setModalOpen] = useState(false)
  const [editingKey, setEditingKey] = useState(null)
  const [pendingDisable, setPendingDisable] = useState(null)
  const [togglingId, setTogglingId] = useState(null)

  const loadKeys = useCallback(async () => {
    setError('')
    try {
      const data = await api('/admin/keys')
      setKeys(Array.isArray(data) ? data : [])
    } catch (loadError) {
      setError(loadError.message || '加载 API 密钥失败')
    } finally {
      setLoading(false)
    }
  }, [api])

  useEffect(() => {
    loadKeys()
  }, [loadKeys])

  function openCreate() {
    setEditingKey(null)
    setModalOpen(true)
  }

  function openEdit(apiKey) {
    setEditingKey(apiKey)
    setModalOpen(true)
  }

  function closeModal() {
    setModalOpen(false)
    setEditingKey(null)
  }

  async function handleSaved() {
    closeModal()
    setLoading(true)
    await loadKeys()
  }

  async function confirmDisable() {
    if (!pendingDisable) return
    setError('')
    setTogglingId(pendingDisable.id)
    try {
      await api(`/admin/keys/${pendingDisable.id}`, { method: 'DELETE' })
      setPendingDisable(null)
      await loadKeys()
    } catch (disableError) {
      setError(disableError.message || '禁用 API 密钥失败')
    } finally {
      setTogglingId(null)
    }
  }

  async function enableKey(apiKey) {
    setError('')
    setTogglingId(apiKey.id)
    try {
      await api(`/admin/keys/${apiKey.id}`, {
        method: 'PUT',
        body: JSON.stringify({ enabled: true }),
      })
      await loadKeys()
    } catch (enableError) {
      setError(enableError.message || '启用 API 密钥失败')
    } finally {
      setTogglingId(null)
    }
  }

  return (
    <>
      <div className="toolbar">
        <h2>API 密钥</h2>
        <Button onClick={openCreate}>+ 新增密钥</Button>
      </div>

      {error ? <div className="alert danger" role="alert">{error}</div> : null}

      {loading ? (
        <Empty>正在加载 API 密钥…</Empty>
      ) : keys.length === 0 ? (
        <Empty>暂无 API 密钥</Empty>
      ) : (
        <div className="table-wrap">
          <table>
            <thead>
              <tr>
                <th>名称</th>
                <th>API 密钥</th>
                <th>每分钟限流</th>
                <th>层级</th>
                <th>允许链</th>
                <th>请求数</th>
                <th>启用</th>
                <th>操作</th>
              </tr>
            </thead>
            <tbody>
              {keys.map((apiKey) => (
                <tr key={apiKey.id}>
                  <td>{apiKey.name}</td>
                  <td className="url" title={apiKey.api_key}>{apiKey.api_key}</td>
                  <td>{apiKey.rate_limit_per_min}</td>
                  <td>
                    <Badge tone={apiKey.allowed_tier === 'paid' ? 'yellow' : 'green'}>
                      {TIER_LABELS[apiKey.allowed_tier] || apiKey.allowed_tier}
                    </Badge>
                  </td>
                  <td>
                    {apiKey.allowed_chains?.length
                      ? apiKey.allowed_chains.map((chain) => chainName(chain)).join(', ')
                      : '全部'}
                  </td>
                  <td>{apiKey.total_requests ?? 0}</td>
                  <td>
                    <Badge tone={apiKey.enabled ? 'green' : 'red'}>
                      {apiKey.enabled ? '是' : '否'}
                    </Badge>
                  </td>
                  <td>
                    <div style={{ display: 'flex', gap: 8 }}>
                      <Button variant="ghost" size="sm" onClick={() => openEdit(apiKey)}>
                        编辑
                      </Button>
                      {apiKey.enabled ? (
                        <Button
                          variant="danger"
                          size="sm"
                          disabled={togglingId === apiKey.id}
                          onClick={() => setPendingDisable(apiKey)}
                        >
                          禁用
                        </Button>
                      ) : (
                        <Button
                          variant="primary"
                          size="sm"
                          disabled={togglingId === apiKey.id}
                          loading={togglingId === apiKey.id}
                          loadingText="启用中…"
                          onClick={() => enableKey(apiKey)}
                        >
                          启用
                        </Button>
                      )}
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      <KeyModal
        open={modalOpen}
        apiKey={editingKey}
        api={api}
        onClose={closeModal}
        onSaved={handleSaved}
      />

      <ConfirmDialog
        open={Boolean(pendingDisable)}
        title="禁用 API 密钥"
        message={`确定要禁用「${pendingDisable?.name || ''}」吗？`}
        detail={pendingDisable?.api_key}
        confirmLabel="禁用"
        loading={Boolean(pendingDisable && togglingId === pendingDisable.id)}
        onCancel={() => {
          if (!togglingId) setPendingDisable(null)
        }}
        onConfirm={confirmDisable}
      />
    </>
  )
}
