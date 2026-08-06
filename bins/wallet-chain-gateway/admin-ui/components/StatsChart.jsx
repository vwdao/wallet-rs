'use client'

import { useCallback, useEffect, useRef, useState } from 'react'
import { drawStatsChart, formatTimeLabel, getChartHits } from '../lib/chart.js'
import { chainName } from '../lib/chains.js'
import { ChartTooltip } from './ui/ChartTooltip.jsx'

const DEFAULT_VISIBILITY = { success: true, error: true, latency: true }

export function StatsChart({ data, loading = false, title = '请求趋势' }) {
  const canvasRef = useRef(null)
  const [empty, setEmpty] = useState(true)
  const [visibility, setVisibility] = useState(DEFAULT_VISIBILITY)
  const [tooltip, setTooltip] = useState(null)

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

  function toggle(key) {
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
    const methodRows = (Array.isArray(point.methods) ? point.methods : []).map((item) => ({
      label: `${item.method} · ${chainName(item.chain_index)}`,
      value: item.error_count
        ? `${item.total_requests}（${item.error_count} 错）`
        : String(item.total_requests),
      color: item.error_count ? 'var(--red)' : undefined,
    }))
    setTooltip({
      x: event.clientX,
      y: event.clientY,
      title: formatTimeLabel(point.ts, data?.bucket_seconds),
      rows: [
        ...methodRows,
        { divider: true },
        { label: '成功', value: point.success, color: 'var(--green)' },
        { label: '错误', value: point.error, color: 'var(--red)' },
        { label: '平均延迟', value: `${point.avg_latency_ms.toFixed(0)}ms`, color: 'var(--yellow)' },
      ],
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
      {title ? <div className="chart-title">{title}</div> : null}
      <canvas
        ref={canvasRef}
        onMouseMove={handleMouseMove}
        onMouseLeave={handleMouseLeave}
        aria-label="请求成功、错误和平均延迟趋势图"
        style={{ display: empty || loading ? 'none' : 'block', width: '100%', height: 260 }}
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
        title={tooltip?.title}
        rows={tooltip?.rows}
      />
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
