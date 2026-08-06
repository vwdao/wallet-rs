'use client'

import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { drawMethodsBarChart, getChartHits } from '../lib/chart.js'
import { chainName } from '../lib/chains.js'
import { ChartTooltip } from './ui/ChartTooltip.jsx'

const DEFAULT_VISIBILITY = { success: true, error: true }

export function MethodsChart({ data, loading = false }) {
  const canvasRef = useRef(null)
  const [empty, setEmpty] = useState(true)
  const [visibility, setVisibility] = useState(DEFAULT_VISIBILITY)
  const [tooltip, setTooltip] = useState(null)

  const chartData = useMemo(() => ({
    ...(data || {}),
    items: (Array.isArray(data?.items) ? data.items : []).map((item) => ({
      ...item,
      label: `${item.method} · ${chainName(item.chain_index)}`,
    })),
  }), [data])

  const draw = useCallback(() => {
    if (!canvasRef.current) return
    setEmpty(!drawMethodsBarChart(canvasRef.current, chartData, visibility))
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
    const item = hit.payload
    setTooltip({
      x: event.clientX,
      y: event.clientY,
      title: item.method,
      rows: [
        { label: '链', value: chainName(item.chain_index) },
        { label: '总请求', value: item.total_requests },
        { label: '成功', value: item.success_count, color: 'var(--green)' },
        { label: '错误', value: item.error_count, color: 'var(--red)' },
        { label: '平均延迟', value: `${item.avg_latency_ms.toFixed(0)}ms` },
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
      <div className="chart-title">Top 10 RPC 方法</div>
      <canvas
        ref={canvasRef}
        onMouseMove={handleMouseMove}
        onMouseLeave={handleMouseLeave}
        aria-label="Top 10 RPC 方法请求量柱状图"
        style={{ display: empty || loading ? 'none' : 'block', width: '100%', height: 300 }}
      />
      {loading ? (
        <div className="chart-empty">正在加载方法统计…</div>
      ) : empty ? (
        <div className="chart-empty">所选时间范围内暂无方法统计数据</div>
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
      <ChartTooltip
        x={tooltip?.x}
        y={tooltip?.y}
        title={tooltip?.title}
        rows={tooltip?.rows}
      />
    </div>
  )
}
