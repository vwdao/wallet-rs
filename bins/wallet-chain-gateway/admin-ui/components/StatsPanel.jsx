'use client'

import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { chainName } from '../lib/chains.js'
import { StatsChart } from './StatsChart.jsx'
import { Badge } from './ui/Badge.jsx'
import { Button } from './ui/Button.jsx'
import { Empty } from './ui/Empty.jsx'

const RANGES = ['15m', '1h', '24h', '7d', '30d']

export function StatsPanel({ api }) {
  const [range, setRange] = useState('1h')
  const [stats, setStats] = useState([])
  const [series, setSeries] = useState({ points: [], bucket_seconds: 300 })
  const [tableLoading, setTableLoading] = useState(true)
  const [chartLoading, setChartLoading] = useState(true)
  const [tableError, setTableError] = useState('')
  const [chartError, setChartError] = useState('')
  const seriesRequest = useRef(0)

  const loadTable = useCallback(async () => {
    setTableLoading(true)
    setTableError('')
    try {
      const data = await api('/admin/stats')
      setStats(Array.isArray(data) ? data : [])
    } catch (loadError) {
      setTableError(loadError.message || '加载统计数据失败')
    } finally {
      setTableLoading(false)
    }
  }, [api])

  const loadSeries = useCallback(async () => {
    const requestId = ++seriesRequest.current
    setChartLoading(true)
    setChartError('')
    try {
      const data = await api(`/admin/stats/series?range=${encodeURIComponent(range)}`)
      if (requestId === seriesRequest.current) {
        setSeries(data && typeof data === 'object' ? data : { points: [], bucket_seconds: 300 })
      }
    } catch (loadError) {
      if (requestId === seriesRequest.current) {
        setChartError(loadError.message || '加载趋势图失败')
      }
    } finally {
      if (requestId === seriesRequest.current) setChartLoading(false)
    }
  }, [api, range])

  useEffect(() => {
    loadTable()
  }, [loadTable])

  useEffect(() => {
    loadSeries()
  }, [loadSeries])

  const summary = useMemo(() => {
    const totalRequests = stats.reduce((sum, row) => sum + Number(row.total_requests || 0), 0)
    const totalErrors = stats.reduce((sum, row) => sum + Number(row.error_count || 0), 0)
    const averageLatency = stats.length
      ? stats.reduce((sum, row) => sum + Number(row.avg_latency_ms || 0), 0) / stats.length
      : 0
    return {
      totalRequests,
      totalErrors,
      errorRate: totalRequests ? totalErrors / totalRequests * 100 : 0,
      averageLatency,
    }
  }, [stats])

  async function refresh() {
    await Promise.allSettled([loadTable(), loadSeries()])
  }

  return (
    <>
      <div className="toolbar">
        <h2>请求统计</h2>
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <div className="range-switch" aria-label="统计时间范围">
            {RANGES.map((value) => (
              <button
                key={value}
                type="button"
                className={`range-btn${range === value ? ' active' : ''}`}
                aria-pressed={range === value}
                onClick={() => setRange(value)}
              >
                {value}
              </button>
            ))}
          </div>
          <Button variant="ghost" size="sm" onClick={refresh}>刷新</Button>
        </div>
      </div>

      {tableError || chartError ? (
        <div className="alert danger" role="alert">
          {[tableError, chartError].filter(Boolean).join('；')}
        </div>
      ) : null}

      <StatsChart data={series} loading={chartLoading} />

      {!tableLoading && stats.length ? (
        <div className="cards">
          <SummaryCard label="总请求数" value={summary.totalRequests.toLocaleString()} />
          <SummaryCard label="总错误数" value={summary.totalErrors.toLocaleString()} tone="var(--red)" />
          <SummaryCard label="错误率" value={`${summary.errorRate.toFixed(1)}%`} />
          <SummaryCard label="平均延迟" value={`${summary.averageLatency.toFixed(0)}ms`} />
        </div>
      ) : null}

      {tableLoading ? (
        <Empty>正在加载统计数据…</Empty>
      ) : stats.length === 0 ? (
        <Empty>暂无统计数据，请先发送 RPC 请求</Empty>
      ) : (
        <div className="table-wrap">
          <table>
            <thead>
              <tr>
                <th>API 密钥</th>
                <th>链</th>
                <th>客户端 IP</th>
                <th>方法</th>
                <th>请求数</th>
                <th>成功</th>
                <th>错误</th>
                <th>错误率</th>
                <th>平均延迟</th>
              </tr>
            </thead>
            <tbody>
              {stats.map((row, index) => {
                const rate = row.total_requests
                  ? Number(row.error_count || 0) / Number(row.total_requests) * 100
                  : 0
                const tone = rate > 10 ? 'var(--red)' : rate > 5 ? 'var(--yellow)' : 'var(--green)'
                return (
                  <tr key={`${row.api_key}-${row.chain_index}-${row.client_ip}-${row.method}-${index}`}>
                    <td><code>{row.api_key}</code></td>
                    <td><Badge>{chainName(row.chain_index)}</Badge></td>
                    <td><code>{row.client_ip || '—'}</code></td>
                    <td><code>{row.method || '—'}</code></td>
                    <td>{row.total_requests}</td>
                    <td className="ok">{row.success_count}</td>
                    <td className="danger">{row.error_count}</td>
                    <td style={{ color: tone }}>{rate.toFixed(1)}%</td>
                    <td>{Number(row.avg_latency_ms || 0).toFixed(0)}ms</td>
                  </tr>
                )
              })}
            </tbody>
          </table>
        </div>
      )}
    </>
  )
}

function SummaryCard({ label, value, tone }) {
  return (
    <div>
      <small>{label}</small>
      <b style={tone ? { color: tone } : undefined}>{value}</b>
    </div>
  )
}
