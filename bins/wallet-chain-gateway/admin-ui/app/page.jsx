'use client'

import { useEffect, useMemo, useState } from 'react'

const tabs = [['endpoints', 'RPC 节点'], ['keys', 'API Keys'], ['stats', '统计'], ['settings', '配置']]

export default function Page() {
  const [adminKey, setAdminKey] = useState(() => typeof window === 'undefined' ? '' : sessionStorage.getItem('gateway_admin_key') || '')
  const [loggedIn, setLoggedIn] = useState(false)
  const [loginError, setLoginError] = useState('')
  const [tab, setTab] = useState('endpoints')
  const [data, setData] = useState({ endpoints: [], keys: [], stats: [], settings: [] })
  const [error, setError] = useState('')
  const [endpoint, setEndpoint] = useState(null)

  async function api(path, options = {}) {
    const response = await fetch(path, { ...options, headers: { 'Content-Type': 'application/json', 'x-admin-key': adminKey, ...(options.headers || {}) } })
    if (!response.ok) {
      const body = await response.json().catch(() => ({}))
      throw new Error(body.message || response.statusText)
    }
    return response.json()
  }

  async function loadAll() {
    try {
      setError('')
      const [endpoints, keys, stats, settings] = await Promise.all([
        api('/admin/endpoints'), api('/admin/keys'), api('/admin/stats'), api('/admin/settings'),
      ])
      setData({ endpoints, keys, stats, settings })
    } catch (e) { setError(e.message) }
  }

  async function login(event) {
    event.preventDefault()
    try {
      await api('/admin/keys')
      sessionStorage.setItem('gateway_admin_key', adminKey)
      setLoggedIn(true)
      await loadAll()
    } catch { setLoginError('Admin key 无效或服务不可用') }
  }

  function logout() {
    sessionStorage.removeItem('gateway_admin_key')
    setAdminKey('')
    setLoggedIn(false)
  }

  useEffect(() => { if (adminKey) login({ preventDefault() {} }) }, [])

  const totals = useMemo(() => ({
    requests: data.stats.reduce((n, item) => n + item.total_requests, 0),
    errors: data.stats.reduce((n, item) => n + item.error_count, 0),
  }), [data.stats])

  if (!loggedIn) return <main className="login"><form className="login-card" onSubmit={login}><h1>Chain Gateway</h1><p>管理后台</p><input value={adminKey} onChange={e => setAdminKey(e.target.value)} type="password" placeholder="Admin key" autoFocus /><button>登录</button>{loginError && <small className="danger">{loginError}</small>}</form></main>

  return <main className="shell">
    <header><div><strong>Chain Gateway</strong><span>管理后台</span></div><button className="ghost" onClick={logout}>退出</button></header>
    <nav>{tabs.map(([id, label]) => <button key={id} className={tab === id ? 'active' : ''} onClick={() => setTab(id)}>{label}</button>)}</nav>
    <section className="content">
      {error && <div className="alert danger">{error}</div>}
      {tab === 'endpoints' && <Endpoints data={data.endpoints} refresh={loadAll} api={api} editing={endpoint} setEditing={setEndpoint} />}
      {tab === 'keys' && <Keys data={data.keys} refresh={loadAll} api={api} />}
      {tab === 'stats' && <Stats data={data.stats} totals={totals} refresh={loadAll} />}
      {tab === 'settings' && <Settings data={data.settings} refresh={loadAll} api={api} />}
    </section>
  </main>
}

function chainName(id) { return ({ 0: 'BTC', 60: 'ETH', 195: 'TRON', 501: 'SOL' })[id] || id }

function Endpoints({ data, refresh, api, editing, setEditing }) {
  const blank = { chain_index: 60, url: '', weight: 1, enabled: true, tier: 'free', is_archive: false, priority: 0 }
  const item = editing || blank
  async function save(event) {
    event.preventDefault()
    const body = { ...item, chain_index: Number(item.chain_index), weight: Number(item.weight), priority: Number(item.priority) }
    await api(editing ? `/admin/endpoints/${editing.id}` : '/admin/endpoints', { method: editing ? 'PUT' : 'POST', body: JSON.stringify(body) })
    setEditing(null); await refresh()
  }
  async function disable(row) { if (confirm(`禁用 ${row.url}？`)) { await api(`/admin/endpoints/${row.id}`, { method: 'DELETE' }); await refresh() } }
  return <><div className="toolbar"><h2>RPC 节点</h2><button onClick={() => setEditing({ ...blank })}>新增节点</button></div><table><thead><tr><th>Chain</th><th>URL</th><th>Tier</th><th>优先级</th><th>延迟</th><th>状态</th><th /></tr></thead><tbody>{data.map(row => <tr key={row.id}><td>{chainName(row.chain_index)}</td><td className="url">{row.url}</td><td>{row.tier}</td><td>{row.priority}</td><td>{row.avg_latency_ms ?? '-'} ms</td><td className={row.healthy ? 'ok' : 'danger'}>{row.healthy ? 'Healthy' : 'Unhealthy'}</td><td><button className="link" onClick={() => setEditing({ ...row })}>编辑</button><button className="link danger" onClick={() => disable(row)}>禁用</button></td></tr>)}</tbody></table>{editing && <div className="modal-backdrop"><form className="modal" onSubmit={save}><h2>{editing.id ? '编辑节点' : '新增节点'}</h2><Field label="Chain index"><input required type="number" value={editing.chain_index} onChange={e => setEditing({ ...editing, chain_index: e.target.value })} /></Field><Field label="URL"><input required type="url" value={editing.url} onChange={e => setEditing({ ...editing, url: e.target.value })} /></Field><Field label="Tier"><select value={editing.tier} onChange={e => setEditing({ ...editing, tier: e.target.value })}><option>free</option><option>paid</option></select></Field><Field label="Weight"><input type="number" min="1" value={editing.weight} onChange={e => setEditing({ ...editing, weight: e.target.value })} /></Field><Field label="Priority"><input type="number" value={editing.priority} onChange={e => setEditing({ ...editing, priority: e.target.value })} /></Field><label className="check"><input type="checkbox" checked={editing.is_archive} onChange={e => setEditing({ ...editing, is_archive: e.target.checked })} /> Archive 节点</label><label className="check"><input type="checkbox" checked={editing.enabled} onChange={e => setEditing({ ...editing, enabled: e.target.checked })} /> 启用</label><div className="actions"><button type="button" className="ghost" onClick={() => setEditing(null)}>取消</button><button>保存</button></div></form></div>}</>
}

function Keys({ data, refresh, api }) {
  async function create() { const name = prompt('Key 名称'); if (!name) return; const chains = prompt('允许的 chain index，逗号分隔；留空表示全部', ''); if (chains === null) return; await api('/admin/keys', { method: 'POST', body: JSON.stringify({ name, rate_limit_per_min: 60, enabled: true, allowed_tier: 'all', allowed_chains: chains.split(',').map(Number).filter(Number.isInteger) }) }); await refresh() }
  return <><div className="toolbar"><h2>API Keys</h2><button onClick={create}>新增 Key</button></div><table><thead><tr><th>名称</th><th>Key</th><th>限流</th><th>Tier</th><th>Chains</th><th>请求数</th><th>状态</th></tr></thead><tbody>{data.map(row => <tr key={row.id}><td>{row.name}</td><td><code>{row.api_key}</code></td><td>{row.rate_limit_per_min}/min</td><td>{row.allowed_tier}</td><td>{row.allowed_chains.length ? row.allowed_chains.join(', ') : '全部'}</td><td>{row.total_requests}</td><td className={row.enabled ? 'ok' : 'danger'}>{row.enabled ? '启用' : '禁用'}</td></tr>)}</tbody></table></>
}

function Stats({ data, totals, refresh }) { return <><div className="toolbar"><h2>请求统计</h2><button className="ghost" onClick={refresh}>刷新</button></div><div className="cards"><div><small>总请求</small><b>{totals.requests}</b></div><div><small>错误数</small><b className="danger">{totals.errors}</b></div><div><small>统计分组</small><b>{data.length}</b></div></div><table><thead><tr><th>API Key</th><th>Chain</th><th>请求</th><th>错误</th><th>平均延迟</th></tr></thead><tbody>{data.map(row => <tr key={row.api_key + row.chain_index}><td>{row.api_key}</td><td>{chainName(row.chain_index)}</td><td>{row.total_requests}</td><td>{row.error_count}</td><td>{row.avg_latency_ms.toFixed(1)} ms</td></tr>)}</tbody></table></> }

function Settings({ data, refresh, api }) { async function update(row) { const value = prompt(row.key, row.value); if (value === null) return; await api(`/admin/settings/${encodeURIComponent(row.key)}`, { method: 'PUT', body: JSON.stringify({ value }) }); await refresh() } return <><div className="toolbar"><h2>动态配置</h2><button className="ghost" onClick={refresh}>刷新</button></div><table><thead><tr><th>Key</th><th>Value</th><th>说明</th><th /></tr></thead><tbody>{data.map(row => <tr key={row.key}><td>{row.key}</td><td><code>{row.value}</code></td><td>{row.description}</td><td><button className="link" onClick={() => update(row)}>修改</button></td></tr>)}</tbody></table></> }

function Field({ label, children }) { return <label>{label}{children}</label> }
