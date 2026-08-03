'use client'

import { useCallback, useEffect, useRef, useState } from 'react'
import { drawIpBarChart } from '../lib/chart.js'

const DEFAULT_VISIBILITY = { success: true, error: true }

export function IpStatsChart({ data, loading = false }) {
  const canvasRef = useRef(null)
  const [empty, setEmpty] = useState(true)
  const [visibility, setVisibility] = useState(DEFAULT_VISIBILITY)

  const draw = useCallback(() => {
    if (!canvasRef.current) return
    setEmpty(!drawIpBarChart(canvasRef.current, data, visibility))
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
      <div className="chart-title">按客户端 IP 请求量（Top 12）</div>
      <canvas
        ref={canvasRef}
        aria-label="按客户端 IP 的请求量柱状图"
        style={{ display: empty || loading ? 'none' : 'block', width: '100%', height: 280 }}
      />
      {loading ? (
        <div className="chart-empty">正在加载 IP 统计…</div>
      ) : empty ? (
        <div className="chart-empty">所选时间范围内暂无 IP 请求数据</div>
      ) : null}
      <div className="chart-legend" role="group" aria-label="图例显隐">
        <button
          type="button"
          className={`chart-legend-btn${visibility.success ? '' : ' off'}`}
          aria-pressed={visibility.success}
          onClick={() => toggle('success')}
        >
          <i style={{ background: 'var(--green)' }} />
          成功
        </button>
        <button
          type="button"
          className={`chart-legend-btn${visibility.error ? '' : ' off'}`}
          aria-pressed={visibility.error}
          onClick={() => toggle('error')}
        >
          <i style={{ background: 'var(--red)' }} />
          错误
        </button>
      </div>
    </div>
  )
}
