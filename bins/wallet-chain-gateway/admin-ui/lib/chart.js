export function roundUp(value) {
  const magnitude = Math.pow(10, Math.max(0, Math.floor(Math.log10(value))))
  return Math.ceil(value / magnitude) * magnitude
}

export function compact(value) {
  if (value >= 1e6) return `${(value / 1e6).toFixed(1)}M`
  if (value >= 1e3) return `${(value / 1e3).toFixed(1)}k`
  return String(Math.round(value))
}

const hitMap = new WeakMap()

export function getChartHits(canvas) {
  return hitMap.get(canvas) || []
}

export function formatTimeLabel(ts, bucketSeconds) {
  const date = new Date(ts * 1000)
  if (bucketSeconds >= 3600) {
    return `${date.getMonth() + 1}/${date.getDate()} ${String(date.getHours()).padStart(2, '0')}:00`
  }
  return `${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}`
}

function chartColors(canvas) {
  const root = canvas.ownerDocument?.documentElement
  const styles = root && typeof getComputedStyle === 'function'
    ? getComputedStyle(root)
    : null
  const color = (name, fallback) => styles?.getPropertyValue(name).trim() || fallback

  return {
    grid: color('--border', '#2a2d3a'),
    muted: color('--muted', '#71717a'),
    green: color('--green', '#22c55e'),
    red: color('--red', '#ef4444'),
    yellow: color('--yellow', '#eab308'),
    text: color('--text', '#e8eef9'),
  }
}

export function drawStatsChart(canvas, data, visibility = {}) {
  const showSuccess = visibility.success !== false
  const showError = visibility.error !== false
  const showLatency = visibility.latency !== false

  const points = Array.isArray(data?.points) ? data.points : []
  const total = points.reduce((sum, point) => {
    const success = showSuccess ? Number(point.success || 0) : 0
    const error = showError ? Number(point.error || 0) : 0
    return sum + success + error
  }, 0)
  const hasLatency = showLatency && points.some((point) => Number(point.avg_latency_ms || 0) > 0)
  if (!total && !hasLatency) {
    hitMap.set(canvas, [])
    return false
  }

  const context = canvas.getContext('2d')
  if (!context) {
    hitMap.set(canvas, [])
    return false
  }

  const dpr = typeof window === 'undefined' ? 1 : window.devicePixelRatio || 1
  const rect = canvas.getBoundingClientRect()
  canvas.width = Math.max(200, Math.floor(rect.width * dpr))
  canvas.height = Math.max(120, Math.floor(rect.height * dpr))
  context.setTransform(dpr, 0, 0, dpr, 0, 0)

  const width = canvas.width / dpr
  const height = canvas.height / dpr
  const colors = chartColors(canvas)
  context.clearRect(0, 0, width, height)
  context.font = '11px -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif'

  const padding = { left: 46, right: showLatency ? 40 : 16, top: 14, bottom: 28 }
  const chartWidth = width - padding.left - padding.right
  const chartHeight = height - padding.top - padding.bottom
  let maxRequests = 0
  let maxLatency = 0
  points.forEach((point) => {
    const success = showSuccess ? Number(point.success || 0) : 0
    const error = showError ? Number(point.error || 0) : 0
    maxRequests = Math.max(maxRequests, success + error)
    if (showLatency) maxLatency = Math.max(maxLatency, Number(point.avg_latency_ms || 0))
  })
  maxRequests = Math.max(1, roundUp(maxRequests || 1))
  maxLatency = Math.max(1, maxLatency)

  const count = points.length
  const slot = count > 1 ? chartWidth / (count - 1) : chartWidth
  const x = (index) => count > 1
    ? padding.left + slot * index
    : padding.left + chartWidth / 2
  const y = (value) => padding.top + chartHeight - chartHeight * (value / maxRequests)
  const latencyY = (value) => padding.top + chartHeight - chartHeight * (value / maxLatency)

  const hits = points.map((point, index) => {
    const half = slot / 2
    const methods = Array.isArray(point.methods)
      ? point.methods.map((item) => ({
          method: String(item.method || '未知'),
          chain_index: Number(item.chain_index || 0),
          total_requests: Number(item.total_requests || 0),
          error_count: Number(item.error_count || 0),
        }))
      : []
    return {
      x: x(index) - half,
      y: padding.top,
      w: slot,
      h: chartHeight,
      payload: {
        ts: point.ts,
        success: Number(point.success || 0),
        error: Number(point.error || 0),
        avg_latency_ms: Number(point.avg_latency_ms || 0),
        methods,
      },
    }
  })
  hitMap.set(canvas, hits)

  context.strokeStyle = colors.grid
  context.fillStyle = colors.muted
  context.textBaseline = 'middle'
  for (let index = 0; index <= 4; index += 1) {
    const value = maxRequests * index / 4
    const yy = y(value)
    context.beginPath()
    context.moveTo(padding.left, yy)
    context.lineTo(width - padding.right, yy)
    context.stroke()
    context.textAlign = 'right'
    context.fillText(compact(value), padding.left - 6, yy)
  }

  if (showLatency) {
    context.fillStyle = colors.muted
    context.textAlign = 'left'
    context.fillText('ms', width - padding.right + 6, padding.top + 2)
    for (let index = 1; index <= 2; index += 1) {
      const value = maxLatency * index / 2
      context.fillText(compact(value), width - padding.right + 6, latencyY(value))
    }
  }

  const barWidth = Math.max(2, Math.min(16, slot * 0.28))
  if (showSuccess || showError) {
    points.forEach((point, index) => {
      const xx = x(index)
      const base = y(0)
      const success = showSuccess ? Number(point.success || 0) : 0
      const error = showError ? Number(point.error || 0) : 0
      const middle = y(success)
      const top = y(success + error)
      if (showSuccess) {
        context.fillStyle = colors.green
        context.fillRect(xx - barWidth, middle, barWidth * 2, Math.max(0, base - middle))
      }
      if (showError) {
        context.fillStyle = colors.red
        context.fillRect(xx - barWidth, top, barWidth * 2, Math.max(0, middle - top))
      }
    })
  }

  if (showLatency) {
    context.strokeStyle = colors.yellow
    context.lineWidth = 1.5
    context.beginPath()
    points.forEach((point, index) => {
      const yy = latencyY(Number(point.avg_latency_ms || 0))
      if (index === 0) context.moveTo(x(index), yy)
      else context.lineTo(x(index), yy)
    })
    context.stroke()
  }

  context.fillStyle = colors.muted
  context.textAlign = 'center'
  context.textBaseline = 'top'
  const labelStep = Math.max(1, Math.ceil(count / 8))
  points.forEach((point, index) => {
    if (index % labelStep !== 0 && index !== count - 1) return
    context.fillText(formatTimeLabel(point.ts, data.bucket_seconds), x(index), padding.top + chartHeight + 8)
  })
  context.textBaseline = 'alphabetic'
  return true
}

function truncateLabel(label, max = 18) {
  const text = String(label || '')
  if (text.length <= max) return text
  return `${text.slice(0, max - 1)}…`
}

export function formatOriginTimeLabel(ts, bucketSeconds) {
  const date = new Date(ts * 1000)
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  const hours = String(date.getHours()).padStart(2, '0')
  const minutes = String(date.getMinutes()).padStart(2, '0')
  if (bucketSeconds >= 24 * 3600) {
    return `${month}-${day}`
  }
  if (bucketSeconds >= 3600) {
    return `${month}-${day} ${hours}:00`
  }
  return `${month}-${day} ${hours}:${minutes}`
}

export const IP_SERIES_COLORS = [
  '#3b82f6',
  '#14b8a6',
  '#a78bfa',
  '#f59e0b',
  '#f472b6',
  '#38bdf8',
]

export const PROTOCOL_SERIES_COLORS = {
  http: '#3b82f6',
  https: '#3b82f6',
  ws: '#14b8a6',
  wss: '#14b8a6',
  grpc: '#a78bfa',
  grpcs: '#a78bfa',
  tcp: '#f59e0b',
  未知: '#94a3b8',
}

export function protocolColor(protocol, index = 0) {
  const key = String(protocol || '').toLowerCase()
  return PROTOCOL_SERIES_COLORS[key] || IP_SERIES_COLORS[index % IP_SERIES_COLORS.length]
}

export function seriesColor(key, index = 0) {
  return IP_SERIES_COLORS[index % IP_SERIES_COLORS.length]
}

/**
 * Generic stacked time-series bars.
 * data.series: [{ key, total_requests }]
 * data.points: [{ ts, values: [{ key, total_requests }] }]
 * visibility: { [key]: boolean }
 * options.colorFor(key, index) -> color
 * options.enrichValue(value, color) -> payload value fields
 */
export function drawStackedTimeSeries(canvas, data, visibility = {}, options = {}) {
  const series = Array.isArray(data?.series) ? data.series : []
  const points = Array.isArray(data?.points) ? data.points : []
  const bucketSeconds = Number(data?.bucket_seconds || 60)
  const colorFor = options.colorFor || seriesColor
  const enrichValue = options.enrichValue || ((item, color) => ({
    key: item.key,
    label: item.label || item.key,
    total_requests: Number(item.total_requests || 0),
    color,
  }))
  const visibleSeries = series.filter((item) => visibility[item.key] !== false)

  const total = points.reduce((sum, point) => {
    const values = Array.isArray(point.values) ? point.values : []
    return sum + values.reduce((inner, item) => {
      if (visibility[item.key] === false) return inner
      return inner + Number(item.total_requests || 0)
    }, 0)
  }, 0)

  if (!total || !visibleSeries.length) {
    hitMap.set(canvas, [])
    return false
  }

  const context = canvas.getContext('2d')
  if (!context) {
    hitMap.set(canvas, [])
    return false
  }

  const dpr = typeof window === 'undefined' ? 1 : window.devicePixelRatio || 1
  const rect = canvas.getBoundingClientRect()
  canvas.width = Math.max(200, Math.floor(rect.width * dpr))
  canvas.height = Math.max(140, Math.floor(rect.height * dpr))
  context.setTransform(dpr, 0, 0, dpr, 0, 0)

  const width = canvas.width / dpr
  const height = canvas.height / dpr
  const colors = chartColors(canvas)
  context.clearRect(0, 0, width, height)
  context.font = '11px -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif'

  const padding = { left: 46, right: 16, top: 14, bottom: 36 }
  const chartWidth = width - padding.left - padding.right
  const chartHeight = height - padding.top - padding.bottom

  let maxRequests = 0
  points.forEach((point) => {
    const values = Array.isArray(point.values) ? point.values : []
    const bucketTotal = values.reduce((sum, item) => {
      if (visibility[item.key] === false) return sum
      return sum + Number(item.total_requests || 0)
    }, 0)
    maxRequests = Math.max(maxRequests, bucketTotal)
  })
  maxRequests = Math.max(1, roundUp(maxRequests || 1))

  const count = Math.max(points.length, 1)
  const slot = chartWidth / count
  const barWidth = Math.max(6, Math.min(36, slot * 0.62))
  const x = (index) => padding.left + slot * index + slot / 2
  const y = (value) => padding.top + chartHeight - chartHeight * (value / maxRequests)
  const colorByKey = new Map(
    series.map((item, index) => [item.key, colorFor(item.key, index)]),
  )

  const hits = points.map((point, index) => {
    const values = (Array.isArray(point.values) ? point.values : [])
      .filter((item) => visibility[item.key] !== false)
      .map((item) => enrichValue(
        item,
        colorByKey.get(item.key) || colorFor(item.key),
      ))
    return {
      x: padding.left + slot * index,
      y: padding.top,
      w: slot,
      h: chartHeight,
      payload: {
        ts: Number(point.ts || 0),
        bucket_seconds: bucketSeconds,
        total_requests: values.reduce((sum, item) => sum + Number(item.total_requests || 0), 0),
        values,
      },
    }
  })
  hitMap.set(canvas, hits)

  context.strokeStyle = colors.grid
  context.fillStyle = colors.muted
  context.textBaseline = 'middle'
  for (let index = 0; index <= 4; index += 1) {
    const value = maxRequests * index / 4
    const yy = y(value)
    context.beginPath()
    context.moveTo(padding.left, yy)
    context.lineTo(width - padding.right, yy)
    context.stroke()
    context.textAlign = 'right'
    context.fillText(compact(value), padding.left - 6, yy)
  }

  points.forEach((point, index) => {
    const xx = x(index)
    let stacked = 0
    const values = Array.isArray(point.values) ? point.values : []
    visibleSeries.forEach((seriesItem) => {
      const match = values.find((item) => item.key === seriesItem.key)
      const amount = Number(match?.total_requests || 0)
      if (!amount) return
      const bottom = y(stacked)
      stacked += amount
      const top = y(stacked)
      context.fillStyle = colorByKey.get(seriesItem.key) || colorFor(seriesItem.key)
      context.fillRect(xx - barWidth / 2, top, barWidth, Math.max(0, bottom - top))
    })
  })

  context.fillStyle = colors.muted
  context.textAlign = 'center'
  context.textBaseline = 'top'
  const labelEvery = Math.max(1, Math.ceil(points.length / 8))
  points.forEach((point, index) => {
    if (index % labelEvery !== 0 && index !== points.length - 1) return
    context.fillText(
      formatOriginTimeLabel(point.ts, bucketSeconds),
      x(index),
      padding.top + chartHeight + 8,
    )
  })

  context.textBaseline = 'alphabetic'
  return true
}

export function drawIpBarChart(canvas, data, visibility = {}) {
  const normalized = {
    bucket_seconds: data?.bucket_seconds,
    series: (Array.isArray(data?.series) ? data.series : []).map((item) => ({
      key: item.protocol,
      total_requests: item.total_requests,
    })),
    points: (Array.isArray(data?.points) ? data.points : []).map((point) => ({
      ts: point.ts,
      values: (Array.isArray(point.values) ? point.values : []).map((item) => ({
        key: item.protocol,
        label: item.protocol,
        total_requests: item.total_requests,
      })),
    })),
  }
  return drawStackedTimeSeries(canvas, normalized, visibility, {
    colorFor: protocolColor,
    enrichValue: (item, color) => ({
      protocol: item.key,
      total_requests: Number(item.total_requests || 0),
      color,
    }),
  })
}

export function drawChainBarChart(canvas, data, visibility = {}) {
  const normalized = {
    bucket_seconds: data?.bucket_seconds,
    series: (Array.isArray(data?.series) ? data.series : []).map((item) => ({
      key: String(item.chain_index),
      total_requests: item.total_requests,
    })),
    points: (Array.isArray(data?.points) ? data.points : []).map((point) => ({
      ts: point.ts,
      values: (Array.isArray(point.values) ? point.values : []).map((item) => ({
        key: String(item.chain_index),
        label: item.label || String(item.chain_index),
        total_requests: item.total_requests,
      })),
    })),
  }
  return drawStackedTimeSeries(canvas, normalized, visibility, {
    colorFor: seriesColor,
    enrichValue: (item, color) => ({
      chain_index: Number(item.key),
      label: item.label || item.key,
      total_requests: Number(item.total_requests || 0),
      color,
    }),
  })
}

export function drawMethodsBarChart(canvas, data, visibility = {}) {
  const showSuccess = visibility.success !== false
  const showError = visibility.error !== false

  const items = Array.isArray(data?.items) ? data.items : []
  const total = items.reduce((sum, item) => {
    const success = showSuccess ? Number(item.success_count || 0) : 0
    const error = showError ? Number(item.error_count || 0) : 0
    return sum + success + error
  }, 0)
  if (!total) {
    hitMap.set(canvas, [])
    return false
  }

  const context = canvas.getContext('2d')
  if (!context) {
    hitMap.set(canvas, [])
    return false
  }

  const dpr = typeof window === 'undefined' ? 1 : window.devicePixelRatio || 1
  const rect = canvas.getBoundingClientRect()
  canvas.width = Math.max(300, Math.floor(rect.width * dpr))
  canvas.height = Math.max(120, Math.floor(rect.height * dpr))
  context.setTransform(dpr, 0, 0, dpr, 0, 0)

  const width = canvas.width / dpr
  const height = canvas.height / dpr
  const colors = chartColors(canvas)
  context.clearRect(0, 0, width, height)
  context.font = '11px -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif'

  const padding = { left: 230, right: 16, top: 14, bottom: 22 }
  const chartWidth = width - padding.left - padding.right
  const chartHeight = height - padding.top - padding.bottom

  let maxRequests = 0
  items.forEach((item) => {
    const success = showSuccess ? Number(item.success_count || 0) : 0
    const error = showError ? Number(item.error_count || 0) : 0
    maxRequests = Math.max(maxRequests, success + error)
  })
  maxRequests = Math.max(1, roundUp(maxRequests || 1))

  const count = items.length
  const rowHeight = chartHeight / count
  const barX = padding.left
  const barArea = chartWidth
  const barLen = (value) => (barArea * value) / maxRequests
  const barHeight = Math.max(6, Math.min(16, rowHeight * 0.5))
  const rowY = (index) => padding.top + rowHeight * index + rowHeight / 2

  for (let index = 0; index <= 4; index += 1) {
    const value = (maxRequests * index) / 4
    const xx = barX + barLen(value)
    context.strokeStyle = colors.grid
    context.beginPath()
    context.moveTo(xx, padding.top)
    context.lineTo(xx, height - padding.bottom)
    context.stroke()
    context.fillStyle = colors.muted
    context.textAlign = 'center'
    context.textBaseline = 'top'
    context.fillText(compact(value), xx, height - padding.bottom + 6)
  }

  const hits = []
  items.forEach((item, index) => {
    const cy = rowY(index)
    const success = showSuccess ? Number(item.success_count || 0) : 0
    const error = showError ? Number(item.error_count || 0) : 0
    const totalVal = success + error

    context.fillStyle = colors.muted
    context.textAlign = 'right'
    context.textBaseline = 'middle'
    context.fillText(truncateLabel(item.label, 32), barX - 10, cy)

    if (showSuccess && success > 0) {
      context.fillStyle = colors.green
      context.fillRect(barX, cy - barHeight / 2, barLen(success), barHeight)
    }
    if (showError && error > 0) {
      context.fillStyle = colors.red
      context.fillRect(barX + barLen(success), cy - barHeight / 2, barLen(error), barHeight)
    }

    if (totalVal > 0) {
      context.fillStyle = colors.text
      context.textAlign = 'right'
      context.fillText(compact(totalVal), barX + barLen(totalVal) - 6, cy)
    }

    hits.push({
      x: 0,
      y: padding.top + rowHeight * index,
      w: width,
      h: rowHeight,
      payload: {
        method: item.method,
        chain_index: item.chain_index,
        total_requests: Number(item.total_requests || 0),
        success_count: Number(item.success_count || 0),
        error_count: Number(item.error_count || 0),
        avg_latency_ms: Number(item.avg_latency_ms || 0),
      },
    })
  })
  hitMap.set(canvas, hits)
  context.textBaseline = 'alphabetic'
  return true
}
