'use client'

import { useCallback, useEffect, useState } from 'react'
import { SettingModal } from './SettingModal.jsx'
import { Button } from './ui/Button.jsx'
import { Empty } from './ui/Empty.jsx'

export function SettingsPanel({ api }) {
  const [settings, setSettings] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [modalOpen, setModalOpen] = useState(false)
  const [editingSetting, setEditingSetting] = useState(null)

  const loadSettings = useCallback(async () => {
    setError('')
    try {
      const data = await api('/admin/settings')
      setSettings(Array.isArray(data) ? data : [])
    } catch (loadError) {
      setError(loadError.message || '加载配置失败')
    } finally {
      setLoading(false)
    }
  }, [api])

  useEffect(() => {
    loadSettings()
  }, [loadSettings])

  function openEdit(setting) {
    setEditingSetting(setting)
    setModalOpen(true)
  }

  function closeModal() {
    setModalOpen(false)
    setEditingSetting(null)
  }

  async function handleSaved() {
    closeModal()
    setLoading(true)
    await loadSettings()
  }

  return (
    <>
      <div className="toolbar">
        <h2>网关配置</h2>
        <Button variant="ghost" onClick={() => { setLoading(true); loadSettings() }}>
          刷新
        </Button>
      </div>

      {error ? <div className="alert danger" role="alert">{error}</div> : null}

      {loading ? (
        <Empty>正在加载配置…</Empty>
      ) : settings.length === 0 ? (
        <Empty>暂无配置项</Empty>
      ) : (
        <div className="table-wrap">
          <table>
            <thead>
              <tr>
                <th>Key</th>
                <th>Value</th>
                <th>Description</th>
                <th>操作</th>
              </tr>
            </thead>
            <tbody>
              {settings.map((setting) => (
                <tr key={setting.key}>
                  <td>
                    <code style={{ fontSize: 12 }}>{setting.key}</code>
                  </td>
                  <td>
                    <code
                      style={{
                        fontSize: 12,
                        background: 'var(--bg)',
                        padding: '2px 8px',
                        borderRadius: 4,
                      }}
                      title={setting.value}
                    >
                      {setting.value}
                    </code>
                  </td>
                  <td style={{ color: 'var(--muted)', fontSize: 12 }}>
                    {setting.description || ''}
                  </td>
                  <td>
                    <Button variant="ghost" size="sm" onClick={() => openEdit(setting)}>
                      编辑
                    </Button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      <SettingModal
        open={modalOpen}
        setting={editingSetting}
        api={api}
        onClose={closeModal}
        onSaved={handleSaved}
      />
    </>
  )
}
