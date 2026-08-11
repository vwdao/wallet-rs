'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import { CHAIN_OPTIONS, chainName } from '../lib/chains.js'
import { buildRpcLinks, LINK_META } from '../lib/rpc-links.js'
import { Button } from './ui/Button.jsx'
import { Empty } from './ui/Empty.jsx'

export function ConnectPanel({ api }) {
  const [networks, setNetworks] = useState([])
  const [keys, setKeys] = useState([])
  const [endpoints, setEndpoints] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [chainId, setChainId] = useState('')
  const [keyId, setKeyId] = useState('')
  const [baseUrl, setBaseUrl] = useState('')
  const [grpcUrl, setGrpcUrl] = useState('')
  const [copiedType, setCopiedType] = useState(null)

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
        const networkList = Array.isArray(networkData) ? networkData : []
        const enabled = networkList.filter((network) => network.enabled !== false)
        const choices = enabled.length
          ? enabled.map((network) => ({
              id: Number(network.chain_index),
              name: network.name || chainName(Number(network.chain_index)),
              family: network.family,
            }))
          : CHAIN_OPTIONS
        setNetworks([...choices].sort((a, b) => a.name.localeCompare(b.name)))
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
  }, [api])

  const network = networks.find((item) => String(item.id) === String(chainId)) || null
  const key = keys.find((item) => String(item.id) === String(keyId)) || null
  const apiKey = key?.api_key || ''

  const chainEndpoints = useMemo(
    () => endpoints.filter((endpoint) => String(endpoint.chain_index) === String(chainId)),
    [endpoints, chainId],
  )

  const links = useMemo(() => {
    if (!network || !apiKey) return []
    return buildRpcLinks({
      base: baseUrl || (typeof window !== 'undefined' ? window.location.origin : ''),
      grpcBase: grpcUrl,
      chainName: network.name,
      apiKey,
      endpoints: chainEndpoints,
    })
  }, [baseUrl, grpcUrl, network, apiKey, chainEndpoints])

  const chainAllowed = key
    ? !key.allowed_chains?.length || key.allowed_chains.map(String).includes(String(chainId))
    : true

  const copyUrl = useCallback(async (type, url) => {
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
    setCopiedType(type)
    window.setTimeout(() => {
      setCopiedType((current) => (current === type ? null : current))
    }, 1600)
  }, [])

  return (
    <>
      <div className="toolbar">
        <h2>RPC 连接</h2>
      </div>

      {error ? <div className="alert danger" role="alert">{error}</div> : null}

      {loading ? (
        <Empty>正在加载…</Empty>
      ) : (
        <div className="connect-card">
          <p className="connect-intro">
            选择链和 API 密钥，生成可供外部客户端调用的各类 RPC 地址。链支持的 RPC
            类型不同，页面会按该链的节点自动列出可用链接。
          </p>

          <div className="form-grid">
            <label className="field">
              <span className="field-label">选择链</span>
              <select
                value={chainId}
                onChange={(event) => setChainId(event.target.value)}
                aria-label="选择链"
              >
                <option value="">请选择链</option>
                {networks.map((item) => (
                  <option key={item.id} value={String(item.id)}>
                    {item.name} ({item.id})
                  </option>
                ))}
              </select>
            </label>

            <label className="field">
              <span className="field-label">API 密钥</span>
              <select
                value={keyId}
                onChange={(event) => setKeyId(event.target.value)}
                aria-label="选择 API 密钥"
              >
                <option value="">请选择密钥</option>
                {keys.map((item) => (
                  <option key={item.id} value={String(item.id)}>
                    {item.name ? `${item.name} — ` : ''}{item.api_key}
                    {item.enabled ? '' : '（已禁用）'}
                  </option>
                ))}
              </select>
            </label>
          </div>

          {key && !key.enabled ? (
            <div className="alert danger" role="alert">所选 API 密钥已禁用，链接不可用。</div>
          ) : network && key && !chainAllowed ? (
            <div className="alert danger" role="alert">
              该密钥不允许访问 {network.name}，请先在密钥中配置该链。
            </div>
          ) : null}

          {network && apiKey && key?.enabled !== false && chainAllowed ? (
            links.length === 0 ? (
              <Empty>该链暂无可用的 RPC 链接，请先在「RPC 节点」添加节点。</Empty>
            ) : (
              <div className="connect-links">
                {links.map((link) => (
                  <div
                    key={link.type}
                    className={`connect-link${copiedType === link.type ? ' copied' : ''}`}
                  >
                    <div className="connect-link-meta">
                      <span className="connect-link-label">{LINK_META[link.type]?.label || link.type}</span>
                      <span className="connect-link-note">{LINK_META[link.type]?.note || ''}</span>
                    </div>
                    <code className="connect-link-url">{link.url}</code>
                    <Button
                      size="sm"
                      variant="ghost"
                      onClick={() => copyUrl(link.type, link.url)}
                    >
                      {copiedType === link.type ? '已复制 ✓' : '复制'}
                    </Button>
                  </div>
                ))}
              </div>
            )
          ) : null}

          <p className="connect-hint">
            网关地址来自「配置」页设置（gateway_public_base_url
            {grpcUrl ? '' : ' / gateway_public_grpc_url'}），可在该页修改。
          </p>
        </div>
      )}
    </>
  )
}
