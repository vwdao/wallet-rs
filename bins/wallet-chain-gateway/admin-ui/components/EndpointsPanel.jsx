'use client'

import { useCallback, useEffect, useState } from 'react'
import { chainName } from '../lib/chains.js'
import { EndpointModal } from './EndpointModal.jsx'
import { Badge } from './ui/Badge.jsx'
import { Button } from './ui/Button.jsx'
import { ConfirmDialog } from './ui/ConfirmDialog.jsx'
import { Empty } from './ui/Empty.jsx'

export function EndpointsPanel({ api }) {
  const [endpoints, setEndpoints] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [modalOpen, setModalOpen] = useState(false)
  const [editingEndpoint, setEditingEndpoint] = useState(null)
  const [pendingDisable, setPendingDisable] = useState(null)
  const [togglingId, setTogglingId] = useState(null)

  const loadEndpoints = useCallback(async () => {
    setError('')
    try {
      const data = await api('/admin/endpoints')
      setEndpoints(Array.isArray(data) ? data : [])
    } catch (loadError) {
      setError(loadError.message || '加载节点失败')
    } finally {
      setLoading(false)
    }
  }, [api])

  useEffect(() => {
    loadEndpoints()
  }, [loadEndpoints])

  function openCreate() {
    setEditingEndpoint(null)
    setModalOpen(true)
  }

  function openEdit(endpoint) {
    setEditingEndpoint(endpoint)
    setModalOpen(true)
  }

  function closeModal() {
    setModalOpen(false)
    setEditingEndpoint(null)
  }

  async function handleSaved() {
    closeModal()
    setLoading(true)
    await loadEndpoints()
  }

  async function confirmDisable() {
    if (!pendingDisable) return
    setError('')
    setTogglingId(pendingDisable.id)
    try {
      await api(`/admin/endpoints/${pendingDisable.id}`, { method: 'DELETE' })
      setPendingDisable(null)
      await loadEndpoints()
    } catch (disableError) {
      setError(disableError.message || '禁用节点失败')
    } finally {
      setTogglingId(null)
    }
  }

  async function enableEndpoint(endpoint) {
    setError('')
    setTogglingId(endpoint.id)
    try {
      await api(`/admin/endpoints/${endpoint.id}`, {
        method: 'PUT',
        body: JSON.stringify({ enabled: true }),
      })
      await loadEndpoints()
    } catch (enableError) {
      setError(enableError.message || '启用节点失败')
    } finally {
      setTogglingId(null)
    }
  }

  return (
    <>
      <div className="toolbar">
        <h2>RPC 节点</h2>
        <Button onClick={openCreate}>+ 新增节点</Button>
      </div>

      {error ? <div className="alert danger" role="alert">{error}</div> : null}

      {loading ? (
        <Empty>正在加载节点…</Empty>
      ) : endpoints.length === 0 ? (
        <Empty>暂无 RPC 节点</Empty>
      ) : (
        <div className="table-wrap">
          <table>
            <thead>
              <tr>
                <th>链</th>
                <th>URL</th>
                <th>层级</th>
                <th>归档</th>
                <th>优先级</th>
                <th>权重</th>
                <th>健康状态</th>
                <th>延迟</th>
                <th>启用</th>
                <th>操作</th>
              </tr>
            </thead>
            <tbody>
              {endpoints.map((endpoint) => (
                <tr key={endpoint.id}>
                  <td><Badge>{chainName(endpoint.chain_index)}</Badge></td>
                  <td className="url" title={endpoint.url}>{endpoint.url}</td>
                  <td>
                    <Badge tone={endpoint.tier === 'paid' ? 'yellow' : 'green'}>
                      {endpoint.tier === 'paid' ? '付费' : '免费'}
                    </Badge>
                  </td>
                  <td>{endpoint.is_archive ? <Badge>是</Badge> : '—'}</td>
                  <td>{endpoint.priority}</td>
                  <td>{endpoint.weight}</td>
                  <td>
                    <Badge tone={endpoint.healthy ? 'green' : 'red'}>
                      {endpoint.healthy ? '健康' : '异常'}
                    </Badge>
                  </td>
                  <td>{endpoint.avg_latency_ms != null ? `${endpoint.avg_latency_ms}ms` : '—'}</td>
                  <td>
                    <Badge tone={endpoint.enabled ? 'green' : 'red'}>
                      {endpoint.enabled ? '是' : '否'}
                    </Badge>
                  </td>
                  <td>
                    <div style={{ display: 'flex', gap: 8 }}>
                      <Button variant="ghost" size="sm" onClick={() => openEdit(endpoint)}>
                        编辑
                      </Button>
                      {endpoint.enabled ? (
                        <Button
                          variant="danger"
                          size="sm"
                          disabled={togglingId === endpoint.id}
                          onClick={() => setPendingDisable(endpoint)}
                        >
                          禁用
                        </Button>
                      ) : (
                        <Button
                          variant="primary"
                          size="sm"
                          disabled={togglingId === endpoint.id}
                          loading={togglingId === endpoint.id}
                          loadingText="启用中…"
                          onClick={() => enableEndpoint(endpoint)}
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

      <EndpointModal
        open={modalOpen}
        endpoint={editingEndpoint}
        api={api}
        onClose={closeModal}
        onSaved={handleSaved}
      />

      <ConfirmDialog
        open={Boolean(pendingDisable)}
        title="禁用节点"
        message={`确定要禁用 ${pendingDisable ? chainName(pendingDisable.chain_index) : ''} 节点吗？`}
        detail={pendingDisable?.url}
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
