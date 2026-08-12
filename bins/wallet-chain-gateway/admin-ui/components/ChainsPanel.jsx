'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import { chainSlug, PROTOCOL_OPTIONS } from '../lib/chains.js'
import { mergeChains } from '../lib/networks.js'
import { buildRpcLinks, LINK_META } from '../lib/rpc-links.js'
import { Badge } from './ui/Badge.jsx'
import { Button } from './ui/Button.jsx'
import { Empty } from './ui/Empty.jsx'

export function ChainsPanel({ api }) {
  const [networks, setNetworks] = useState([])
  const [keys, setKeys] = useState([])
  const [endpoints, setEndpoints] = useState([])
  const [baseUrl, setBaseUrl] = useState('')
  const [grpcUrl, setGrpcUrl] = useState('')
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [keyId, setKeyId] = useState('')
  const [filter, setFilter] = useState('all')
  const [saving, setSaving] = useState(null)
  const [copied, setCopied] = useState(null)

  useEffect(() => {
    let cancelled = false
    Promise.all([
      api('/admin/networks'),
      api('/admin/keys'),
      api('/admin/settings'),
      api('/admin/endpoints'),
    ])
      .then(([networkData, keyData, settingData, endpointData]) => {
        if (cancelled) return
        setNetworks(Array.isArray(networkData) ? networkData : [])
        const keyList = Array.isArray(keyData) ? keyData : []
        setKeys(keyList)
        if (keyList.length && !keyId) {
          setKeyId(keyList[0].id)
        }
        setEndpoints(Array.isArray(endpointData) ? endpointData : [])
        const settings = Array.isArray(settingData) ? settingData : []
        const base = settings.find((item) => item.key === 'gateway_public_base_url')?.value || ''
        setBaseUrl(base.trim().replace(/\/+$/, ''))
        const grpc = settings.find((item) => item.key === 'gateway_public_grpc_url')?.value || ''
        setGrpcUrl(grpc.trim().replace(/\/+$/, ''))
      })
      .catch((loadError) => {
        if (!cancelled) setError(loadError.message || '加载失败')
      })
      .finally(() => {
        if (!cancelled) setLoading(false)
      })
    return () => {
      cancelled = true
    }
  }, [api]) // eslint-disable-line react-hooks/exhaustive-deps

  const key = keys.find((item) => String(item.id) === String(keyId)) || null
  const apiKey = key?.api_key || ''

  const chains = useMemo(() => mergeChains(networks), [networks])

  const visibleChains = useMemo(() => {
    return chains.filter((chain) => {
      if (filter === 'enabled') return chain.enabled
      if (filter === 'disabled') return !chain.enabled
      return true
    })
  }, [chains, filter])

  const endpointCount = useCallback(
    (chainId) => endpoints.filter((item) => String(item.chain_index) === String(chainId)).length,
    [endpoints],
  )

  const chainAllowed = useCallback(
    (chainId) =>
      key
        ? !key.allowed_chains?.length ||
          key.allowed_chains.map(String).includes(String(chainId))
        : true,
    [key],
  )

  const linksFor = useCallback(
    (chain) => {
      if (!key || key.enabled === false || !chainAllowed(chain.id)) return []
      const chainEndpoints = endpoints.filter(
        (item) => String(item.chain_index) === String(chain.id),
      )
      return buildRpcLinks({
        base: baseUrl || (typeof window !== 'undefined' ? window.location.origin : ''),
        grpcBase: grpcUrl,
        chainName: chainSlug(chain),
        apiKey,
        endpoints: chainEndpoints,
        supportedProtocols: chain.supported_protocols,
      })
    },
    [baseUrl, grpcUrl, key, apiKey, endpoints, chainAllowed],
  )

  const applyNetworkUpdate = useCallback((row) => {
    setNetworks((current) => {
      const exists = current.some(
        (item) => String(item.chain_index) === String(row.chain_index),
      )
      return exists
        ? current.map((item) =>
            String(item.chain_index) === String(row.chain_index) ? row : item,
          )
        : [...current, row]
    })
  }, [])

  async function toggleEnabled(chain) {
    setError('')
    setSaving({ id: chain.id, kind: 'enabled' })
    try {
      const row = await api(`/admin/networks/${chain.id}`, {
        method: 'PUT',
        body: JSON.stringify({ enabled: !chain.enabled }),
      })
      applyNetworkUpdate(row)
    } catch (saveError) {
      setError(saveError.message || '切换启用状态失败')
    } finally {
      setSaving(null)
    }
  }

  async function toggleProtocol(chain, protocol, checked) {
    setError('')
    setSaving({ id: chain.id, kind: `protocol:${protocol}` })
    try {
      const next = checked
        ? [...new Set([...chain.supported_protocols, protocol])]
        : chain.supported_protocols.filter((item) => item !== protocol)
      const row = await api(`/admin/networks/${chain.id}`, {
        method: 'PUT',
        body: JSON.stringify({ supported_protocols: next }),
      })
      applyNetworkUpdate(row)
    } catch (saveError) {
      setError(saveError.message || '保存协议失败')
    } finally {
      setSaving(null)
    }
  }

  const copyUrl = useCallback(async (chainId, type, url) => {
    try {
      await navigator.clipboard.writeText(url)
    } catch {
      const textarea = document.createElement('textarea')
      textarea.value = url
      textarea.style.position = 'fixed'
      textarea.style.opacity = '0'
      document.body.appendChild(textarea)
      textarea.select()
      try {
        document.execCommand('copy')
      } finally {
        document.body.removeChild(textarea)
      }
    }
    setCopied({ chainId: String(chainId), type })
    window.setTimeout(() => {
      setCopied((current) =>
        current?.chainId === String(chainId) && current.type === type ? null : current,
      )
    }, 1600)
  }, [])

  return (
    <>
      <div className="toolbar">
        <h2>链</h2>
        <div className="stats-filters">
          <label className="stats-filter">
            <span>状态</span>
            <select
              value={filter}
              onChange={(event) => setFilter(event.target.value)}
              aria-label="按启用状态筛选"
            >
              <option value="all">全部</option>
              <option value="enabled">已开启</option>
              <option value="disabled">已关闭</option>
            </select>
          </label>
          <label className="stats-filter">
            <span>API 密钥</span>
            <select
              value={keyId}
              onChange={(event) => setKeyId(event.target.value)}
              aria-label="选择 API 密钥"
            >
              <option value="">不显示地址</option>
              {keys.map((item) => (
                <option key={item.id} value={String(item.id)}>
                  {item.name ? `${item.name} — ` : ''}{item.api_key}
                  {item.enabled ? '' : '（已禁用）'}
                </option>
              ))}
            </select>
          </label>
        </div>
      </div>

      {error ? <div className="alert danger" role="alert">{error}</div> : null}

      {loading ? (
        <Empty>正在加载链…</Empty>
      ) : (
        <>
          <p className="connect-intro">
            展示全部链。可开启或关闭每条链的接入，并设置该链支持的 RPC 类型
            （HTTP / WebSocket / gRPC / TCP）；关闭的链不会出现在网关路由中。
          </p>

          {visibleChains.length === 0 ? (
            <Empty>没有匹配的链</Empty>
          ) : (
            <div className="table-wrap">
              <table>
                <thead>
                  <tr>
                    <th>链</th>
                    <th>家族</th>
                    <th>节点</th>
                    <th>支持 RPC 类型</th>
                    <th>启用</th>
                    <th>RPC 地址</th>
                  </tr>
                </thead>
                <tbody>
                  {visibleChains.map((chain) => {
                    const isSaving = saving?.id === chain.id
                    const links = linksFor(chain)
                    return (
                      <tr
                        key={chain.id}
                        className={chain.enabled ? '' : 'row-disabled'}
                      >
                        <td>
                          <span className="chain-name">{chain.name}</span>
                          <span className="chain-id">
                            {chain.id} {chain.inDb ? '' : '（未配置）'}
                          </span>
                        </td>
                        <td>
                          {chain.family ? (
                            <Badge>{chain.family}</Badge>
                          ) : (
                            <span className="text-muted">—</span>
                          )}
                        </td>
                        <td>{endpointCount(chain.id)}</td>
                        <td>
                          <div className="protocol-checks">
                            {PROTOCOL_OPTIONS.map((protocol) => {
                              const checked = chain.supported_protocols.includes(protocol.value)
                              return (
                                <label
                                  key={protocol.value}
                                  className={`protocol-check${checked ? ' checked' : ''}`}
                                  title={protocol.label}
                                >
                                  <input
                                    type="checkbox"
                                    checked={checked}
                                    disabled={isSaving}
                                    onChange={(event) =>
                                      toggleProtocol(
                                        chain,
                                        protocol.value,
                                        event.target.checked,
                                      )
                                    }
                                  />
                                  <span>{protocol.value}</span>
                                </label>
                              )
                            })}
                          </div>
                        </td>
                        <td>
                          <Button
                            variant={chain.enabled ? 'ghost' : 'primary'}
                            size="sm"
                            disabled={isSaving}
                            loading={isSaving && saving?.kind === 'enabled'}
                            loadingText="保存中…"
                            onClick={() => toggleEnabled(chain)}
                          >
                            {chain.enabled ? '已开启' : '已关闭'}
                          </Button>
                        </td>
                        <td className="chain-links-cell">
                          {!key ? (
                            <span className="text-muted">选择密钥后显示</span>
                          ) : key.enabled === false ? (
                            <span className="text-muted">密钥已禁用</span>
                          ) : !chainAllowed(chain.id) ? (
                            <span className="text-muted">密钥不允许该链</span>
                          ) : links.length === 0 ? (
                            <span className="text-muted">
                              {chain.inDb ? '暂无可用节点' : '未配置节点'}
                            </span>
                          ) : (
                            <div className="chain-links">
                              {links.map((link) => (
                                <div key={link.type} className="chain-link-row">
                                  <span className="chain-link-tag">
                                    {LINK_META[link.type]?.label || link.type}
                                  </span>
                                  <code
                                    className="chain-link-url"
                                    onClick={() => copyUrl(chain.id, link.type, link.url)}
                                  >
                                    {copied?.chainId === String(chain.id) &&
                                    copied.type === link.type
                                      ? '已复制 ✓'
                                      : link.url}
                                  </code>
                                </div>
                              ))}
                            </div>
                          )}
                        </td>
                      </tr>
                    )
                  })}
                </tbody>
              </table>
            </div>
          )}
        </>
      )}

      <p className="connect-hint">
        勾选「支持 RPC 类型」后仅列出被勾选的类型；全部不勾选时按该链节点自动识别。网关地址来自「配置」页（gateway_public_base_url
        {grpcUrl ? '' : ' / gateway_public_grpc_url'}）。
      </p>
    </>
  )
}
