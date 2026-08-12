'use client'

import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import {
  drawChainBarChart,
  formatOriginTimeLabel,
  getChartHits,
  seriesColor,
} from '../lib/chart.js'
import { chainName } from '../lib/chains.js'
import { Badge } from './ui/Badge.jsx'
import { ChartTooltip } from './ui/ChartTooltip.jsx'
import { Empty } from './ui/Empty.jsx'

export function ChainStatsChart({ data, loading = false }) {
  const canvasRef = useRef(null)
  const [empty, setEmpty] = useState(true)
  const [visibility, setVisibility] = useState({})
  const [tooltip, setTooltip] = useState(null)
  const [selectedChain, setSelectedChain] = useState('')

  const chartData = useMemo(() => {
    const series = (Array.isArray(data?.series) ? data.series : []).map((item) => ({
      ...item,
      label: chainName(item.chain_index),
      methods: (Array.isArray(item.methods) ? item.methods : []).map((method) => ({
        ...method,
        chain_index: item.chain_index,
      })),
    }))
    const points = (Array.isArray(data?.points) ? data.points : []).map((point) => ({
      ...point,
      values: (Array.isArray(point.values) ? point.values : []).map((item) => ({
        ...item,
        label: chainName(item.chain_index),
      })),
    }))
    return {
      ...(data || {}),
      series,
      points,
    }
  }, [data])

  const series = chartData.series

  useEffect(() => {
    setVisibility((current) => {
      const next = {}
      for (const item of series) {
        const key = String(item.chain_index)
        next[key] = current[key] !== false
      }
      return next
    })
  }, [series])

  useEffect(() => {
    if (!selectedChain) return
    const stillExists = series.some(
      (item) => String(item.chain_index) === String(selectedChain),
    )
    if (!stillExists) setSelectedChain('')
  }, [series, selectedChain])

  const draw = useCallback(() => {
    if (!canvasRef.current) return
    setEmpty(!drawChainBarChart(canvasRef.current, chartData, visibility))
  }, [chartData, visibility])

  useEffect(() => {
    draw()
  }, [draw])

  useEffect(() => {
    if (typeof ResizeObserver === 'undefined') {
      window.addEventListener('resize', draw)
      return () => window.removeEventListener('resize', draw)
    }
    const observer = new ResizeObserver(draw)
    if (canvasRef.current) observer.observe(canvasRef.current)
    return () => observer.disconnect()
  }, [draw])

  function toggle(chainIndex) {
    const key = String(chainIndex)
    setVisibility((current) => ({ ...current, [key]: !current[key] }))
  }

  function handleMouseMove(event) {
    const canvas = canvasRef.current
    if (!canvas) return
    const rect = canvas.getBoundingClientRect()
    const offsetX = event.clientX - rect.left
    const offsetY = event.clientY - rect.top
    const hit = getChartHits(canvas).find(
      (region) => offsetX >= region.x && offsetX <= region.x + region.w
        && offsetY >= region.y && offsetY <= region.y + region.h,
    )
    if (!hit) {
      setTooltip(null)
      return
    }
    const point = hit.payload
    const rows = (Array.isArray(point.values) ? point.values : [])
      .filter((item) => item.total_requests > 0)
      .map((item) => ({
        label: item.label || chainName(item.chain_index),
        value: item.total_requests,
        color: item.color,
      }))
    if (rows.length > 1) {
      rows.push({ divider: true })
      rows.push({
        label: 'total',
        value: point.total_requests,
      })
    }
    setTooltip({
      x: event.clientX,
      y: event.clientY,
      title: formatOriginTimeLabel(point.ts, point.bucket_seconds || data?.bucket_seconds),
      rows,
    })
  }

  function handleMouseLeave() {
    setTooltip(null)
  }

  return (
    <div
      className="chart-wrap"
      aria-busy={loading}
      style={{ opacity: loading ? 0.55 : 1, transition: 'opacity 0.15s' }}
    >
      <div className="chart-title">链请求量 · Requests</div>
      <canvas
        ref={canvasRef}
        onMouseMove={handleMouseMove}
        onMouseLeave={handleMouseLeave}
        aria-label="按时间桶展示各链请求量"
        style={{ display: empty || loading ? 'none' : 'block', width: '100%', height: 280 }}
      />
      {loading ? (
        <div className="chart-empty">正在加载链统计…</div>
      ) : empty ? (
        <div className="chart-empty">所选时间范围内暂无链请求数据</div>
      ) : null}
      <div className="chart-legend" role="group" aria-label="链路图例显隐">
        {series.map((item, index) => {
          const key = String(item.chain_index)
          const active = visibility[key] !== false
          const color = seriesColor(key, index)
          return (
            <button
              key={key}
              type="button"
              className={`chart-legend-btn${active ? '' : ' off'}`}
              aria-pressed={active}
              onClick={() => toggle(item.chain_index)}
            >
              <i style={{ background: color }} />
              {item.label || chainName(item.chain_index)}
            </button>
          )
        })}
      </div>
      <ChartTooltip
        x={tooltip?.x}
        y={tooltip?.y}
        title={tooltip?.title}
        rows={tooltip?.rows}
      />
      <ChainMethodsBreakdown
        series={series}
        selectedChain={selectedChain}
        onSelectChain={setSelectedChain}
      />
    </div>
  )
}

function ChainMethodsBreakdown({ series, selectedChain, onSelectChain }) {
  const rows = useMemo(() => {
    const all = series.flatMap((item) =>
      (Array.isArray(item.methods) ? item.methods : []).map((method) => ({
        ...method,
        chain_index: item.chain_index,
      })),
    )
    if (selectedChain !== '') {
      return all.filter((item) => String(item.chain_index) === String(selectedChain))
    }
    return all
  }, [series, selectedChain])

  const totalRequests = rows.reduce((sum, item) => sum + Number(item.total_requests || 0), 0)
  const totalErrors = rows.reduce((sum, item) => sum + Number(item.error_count || 0), 0)

  return (
    <div className="stats-methods-breakdown" data-testid="chain-methods-breakdown">
      <div className="stats-methods-breakdown-head">
        <div>
          <span className="stats-methods-breakdown-kicker">链方法明细</span>
          <strong>{selectedChain === '' ? '全部链路' : chainName(Number(selectedChain))}</strong>
        </div>
        <div className="stats-methods-breakdown-summary">
          <span>
            共 <b>{rows.length}</b> 个方法 ·{' '}
            <b>{totalRequests.toLocaleString()}</b> 次请求 ·{' '}
            <b>{totalErrors.toLocaleString()}</b> 错误
          </span>
        </div>
      </div>
      <div className="chain-tags" role="group" aria-label="按链筛选方法明细">
        <button
          type="button"
          className={`chain-tag${selectedChain === '' ? ' active' : ''}`}
          aria-pressed={selectedChain === ''}
          onClick={() => onSelectChain('')}
        >
          全部链
        </button>
        {series.map((item) => {
          const key = String(item.chain_index)
          const active = selectedChain === key
          return (
            <button
              key={key}
              type="button"
              className={`chain-tag${active ? ' active' : ''}`}
              aria-pressed={active}
              onClick={() => onSelectChain(active ? '' : key)}
            >
              {item.label || chainName(item.chain_index)}
            </button>
          )
        })}
      </div>
      {rows.length === 0 ? (
        <Empty>所选链路暂无方法数据</Empty>
      ) : (
        <div className="table-wrap stats-table-wrap">
          <table>
            <thead>
              <tr>
                <th>链</th>
                <th>方法</th>
                <th>请求数</th>
                <th>成功</th>
                <th>错误</th>
                <th>错误率</th>
                <th>平均延迟</th>
              </tr>
            </thead>
            <tbody>
              {rows.map((item, index) => {
                const total = Number(item.total_requests || 0)
                const errors = Number(item.error_count || 0)
                const rate = total ? (errors / total) * 100 : 0
                const tone = rate > 10 ? 'var(--red)' : rate > 5 ? 'var(--yellow)' : 'var(--green)'
                return (
                  <tr key={`${item.chain_index}-${item.method}-${index}`}>
                    <td><Badge>{chainName(item.chain_index)}</Badge></td>
                    <td><code>{item.method}</code></td>
                    <td>{total.toLocaleString()}</td>
                    <td className="ok">{Number(item.success_count || 0).toLocaleString()}</td>
                    <td className="danger">{errors.toLocaleString()}</td>
                    <td style={{ color: tone }}>{rate.toFixed(1)}%</td>
                    <td>{Number(item.avg_latency_ms || 0).toFixed(0)}ms</td>
                  </tr>
                )
              })}
            </tbody>
          </table>
        </div>
      )}
    </div>
  )
}
