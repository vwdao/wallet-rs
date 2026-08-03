'use client'

import { useCallback, useEffect, useRef, useState } from 'react'
import { drawStatsChart } from '../lib/chart.js'

export function StatsChart({ data, loading = false }) {
  const canvasRef = useRef(null)
  const [empty, setEmpty] = useState(true)

  const draw = useCallback(() => {
    if (!canvasRef.current) return
    setEmpty(!drawStatsChart(canvasRef.current, data))
  }, [data])

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

  return (
    <div
      className="chart-wrap"
      aria-busy={loading}
      style={{ opacity: loading ? 0.55 : 1, transition: 'opacity 0.15s' }}
    >
      <canvas
        ref={canvasRef}
        aria-label="请求成功、错误和平均延迟趋势图"
        style={{ display: empty ? 'none' : 'block', width: '100%', height: 260 }}
      />
      {empty ? <div className="chart-empty">所选时间范围内暂无请求数据</div> : null}
      <div style={{ display: 'flex', gap: 18, marginTop: 10, color: 'var(--muted)', fontSize: 12 }}>
        <span><i style={dotStyle('var(--green)')} />成功</span>
        <span><i style={dotStyle('var(--red)')} />错误</span>
        <span><i style={dotStyle('var(--yellow)')} />平均延迟</span>
      </div>
    </div>
  )
}

function dotStyle(background) {
  return {
    display: 'inline-block',
    width: 10,
    height: 10,
    marginRight: 6,
    borderRadius: 2,
    verticalAlign: 'middle',
    background,
  }
}
