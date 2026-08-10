'use client'

import { useCallback, useEffect, useRef, useState } from 'react'
import { drawStatsChart, formatTimeLabel, getChartHits } from '../lib/chart.js'
import { chainName } from '../lib/chains.js'
import { Badge } from './ui/Badge.jsx'
import { ChartTooltip } from './ui/ChartTooltip.jsx'
import { Empty } from './ui/Empty.jsx'

const DEFAULT_VISIBILITY = { success: true, error: true, latency: true }

export function StatsChart({ data, loading = false, title = '请求趋势' }) {
  const canvasRef = useRef(null)
  const [empty, setEmpty] = useState(true)
  const [visibility, setVisibility] = useState(DEFAULT_VISIBILITY)
  const [tooltip, setTooltip] = useState(null)
  const [pinnedPoint, setPinnedPoint] = useState(null)

  const draw = useCallback(() => {
    if (!canvasRef.current) return
    setEmpty(!drawStatsChart(canvasRef.current, data, visibility))
  }, [data, visibility])

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

  useEffect(() => {
    // 时间范围 / 链路 / 数据刷新后，如果当前锁定的时间点已不在新数据中，清除锁定。
    if (!pinnedPoint) return
    const points = Array.isArray(data?.points) ? data.points : []
    const stillExists = points.some(
      (point) => Number(point.ts) === Number(pinnedPoint.ts),
    )
    if (!stillExists) setPinnedPoint(null)
  }, [data, pinnedPoint])

  function toggle(key) {
    setVisibility((current) => ({ ...current, [key]: !current[key] }))
  }

  function findPoint(offsetX, offsetY) {
    const canvas = canvasRef.current
    if (!canvas) return null
    const hit = getChartHits(canvas).find(
      (region) => offsetX >= region.x && offsetX <= region.x + region.w
        && offsetY >= region.y && offsetY <= region.y + region.h,
    )
    return hit ? hit.payload : null
  }

  function handleMouseMove(event) {
    const canvas = canvasRef.current
    if (!canvas) return
    const rect = canvas.getBoundingClientRect()
    const offsetX = event.clientX - rect.left
    const offsetY = event.clientY - rect.top
    const point = findPoint(offsetX, offsetY)
    if (!point) {
      setTooltip(null)
      return
    }
    setTooltip({
      x: event.clientX,
      y: event.clientY,
      point,
    })
  }

  function handleMouseLeave() {
    setTooltip(null)
  }

  function handleClick(event) {
    const canvas = canvasRef.current
    if (!canvas) return
    const rect = canvas.getBoundingClientRect()
    const offsetX = event.clientX - rect.left
    const offsetY = event.clientY - rect.top
    const point = findPoint(offsetX, offsetY)
    if (!point) return
    setPinnedPoint((current) =>
      current && Number(current.ts) === Number(point.ts) ? null : point,
    )
  }

  const tooltipRows = tooltip
    ? buildTooltipRows(tooltip.point, data?.bucket_seconds)
    : null

  return (
    <div
      className="chart-wrap"
      aria-busy={loading}
      style={{ opacity: loading ? 0.55 : 1, transition: 'opacity 0.15s' }}
    >
      {title ? <div className="chart-title">{title}</div> : null}
      <canvas
        ref={canvasRef}
        onMouseMove={handleMouseMove}
        onMouseLeave={handleMouseLeave}
        onClick={handleClick}
        aria-label="请求成功、错误和平均延迟趋势图，点击时间桶查看方法明细"
        style={{
          display: empty || loading ? 'none' : 'block',
          width: '100%',
          height: 260,
          cursor: 'crosshair',
        }}
      />
      {loading ? (
        <div className="chart-empty">正在加载趋势图…</div>
      ) : empty ? (
        <div className="chart-empty">所选时间范围内暂无请求数据</div>
      ) : null}
      <div className="chart-legend" role="group" aria-label="图例显隐">
        <LegendButton
          active={visibility.success}
          color="var(--green)"
          label="成功"
          onClick={() => toggle('success')}
        />
        <LegendButton
          active={visibility.error}
          color="var(--red)"
          label="错误"
          onClick={() => toggle('error')}
        />
        <LegendButton
          active={visibility.latency}
          color="var(--yellow)"
          label="平均延迟"
          onClick={() => toggle('latency')}
        />
      </div>
      <ChartTooltip
        x={tooltip?.x}
        y={tooltip?.y}
        title={tooltip ? formatTimeLabel(tooltip.point.ts, data?.bucket_seconds) : undefined}
        rows={tooltipRows}
      />
      {pinnedPoint ? (
        <PinnedMethodsPanel
          point={pinnedPoint}
          bucketSeconds={data?.bucket_seconds}
          onClear={() => setPinnedPoint(null)}
        />
      ) : null}
    </div>
  )
}

function buildTooltipRows(point, bucketSeconds) {
  const methodRows = (Array.isArray(point.methods) ? point.methods : []).map((item) => ({
    label: `${item.method} · ${chainName(item.chain_index)}`,
    value: item.error_count
      ? `${item.total_requests}（${item.error_count} 错）`
      : String(item.total_requests),
    color: item.error_count ? 'var(--red)' : undefined,
  }))
  return [
    ...methodRows,
    { divider: true },
    { label: '成功', value: point.success, color: 'var(--green)' },
    { label: '错误', value: point.error, color: 'var(--red)' },
    { label: '平均延迟', value: `${Number(point.avg_latency_ms || 0).toFixed(0)}ms`, color: 'var(--yellow)' },
  ]
}

function PinnedMethodsPanel({ point, bucketSeconds, onClear }) {
  const methods = Array.isArray(point.methods) ? point.methods : []
  const totalRequests = methods.reduce(
    (sum, item) => sum + Number(item.total_requests || 0),
    0,
  )
  const totalErrors = methods.reduce(
    (sum, item) => sum + Number(item.error_count || 0),
    0,
  )
  return (
    <div className="stats-methods-breakdown" data-testid="pinned-methods">
      <div className="stats-methods-breakdown-head">
        <div>
          <span className="stats-methods-breakdown-kicker">时间桶方法明细</span>
          <strong>{formatTimeLabel(Number(point.ts), bucketSeconds)}</strong>
        </div>
        <div className="stats-methods-breakdown-summary">
          <span>
            共 <b>{methods.length}</b> 个方法 ·{' '}
            <b>{totalRequests.toLocaleString()}</b> 次请求 ·{' '}
            <b>{totalErrors.toLocaleString()}</b> 错误
          </span>
          <button
            type="button"
            className="link danger"
            onClick={onClear}
            aria-label="清除锁定的时间桶"
          >
            清除选择
          </button>
        </div>
      </div>
      {methods.length === 0 ? (
        <Empty>该时间桶暂无方法数据</Empty>
      ) : (
        <div className="table-wrap stats-table-wrap">
          <table>
            <thead>
              <tr>
                <th>方法</th>
                <th>链</th>
                <th>请求数</th>
                <th>错误</th>
                <th>错误率</th>
              </tr>
            </thead>
            <tbody>
              {methods.map((item, index) => {
                const errors = Number(item.error_count || 0)
                const total = Number(item.total_requests || 0)
                const rate = total ? (errors / total) * 100 : 0
                const tone = rate > 10 ? 'var(--red)' : rate > 5 ? 'var(--yellow)' : 'var(--green)'
                return (
                  <tr key={`${item.method}-${item.chain_index}-${index}`}>
                    <td><code>{item.method}</code></td>
                    <td><Badge>{chainName(item.chain_index)}</Badge></td>
                    <td>{total.toLocaleString()}</td>
                    <td className="danger">{errors.toLocaleString()}</td>
                    <td style={{ color: tone }}>{rate.toFixed(1)}%</td>
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

function LegendButton({ active, color, label, onClick }) {
  return (
    <button
      type="button"
      className={`chart-legend-btn${active ? '' : ' off'}`}
      aria-pressed={active}
      onClick={onClick}
    >
      <i style={{ background: color }} />
      {label}
    </button>
  )
}
