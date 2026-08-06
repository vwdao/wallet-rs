'use client'

import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import {
  drawChainBarChart,
  formatOriginTimeLabel,
  getChartHits,
  seriesColor,
} from '../lib/chart.js'
import { chainName } from '../lib/chains.js'
import { ChartTooltip } from './ui/ChartTooltip.jsx'

export function ChainStatsChart({ data, loading = false }) {
  const canvasRef = useRef(null)
  const [empty, setEmpty] = useState(true)
  const [visibility, setVisibility] = useState({})
  const [tooltip, setTooltip] = useState(null)

  const chartData = useMemo(() => {
    const series = (Array.isArray(data?.series) ? data.series : []).map((item) => ({
      ...item,
      label: chainName(item.chain_index),
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
    </div>
  )
}
