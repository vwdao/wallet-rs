'use client'

import { useCallback, useEffect, useRef, useState } from 'react'
import { drawStatsChart } from '../lib/chart.js'

const DEFAULT_VISIBILITY = { success: true, error: true, latency: true }

export function StatsChart({ data, loading = false, title = '请求趋势' }) {
  const canvasRef = useRef(null)
  const [empty, setEmpty] = useState(true)
  const [visibility, setVisibility] = useState(DEFAULT_VISIBILITY)

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

  return (
    <div
      className="chart-wrap"
      aria-busy={loading}
      style={{ opacity: loading ? 0.55 : 1, transition: 'opacity 0.15s' }}
    >
      {title ? <div className="chart-title">{title}</div> : null}
      <canvas
        ref={canvasRef}
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
