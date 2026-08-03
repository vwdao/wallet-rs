export function roundUp(value) {
  const magnitude = Math.pow(10, Math.max(0, Math.floor(Math.log10(value))))
  return Math.ceil(value / magnitude) * magnitude
}

export function compact(value) {
  if (value >= 1e6) return `${(value / 1e6).toFixed(1)}M`
  if (value >= 1e3) return `${(value / 1e3).toFixed(1)}k`
  return String(Math.round(value))
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
  }
}

export function drawStatsChart(canvas, data) {
  const points = Array.isArray(data?.points) ? data.points : []
  const total = points.reduce(
    (sum, point) => sum + Number(point.success || 0) + Number(point.error || 0),
    0,
  )
  if (!total) return false

  const context = canvas.getContext('2d')
  if (!context) return false

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

  const padding = { left: 46, right: 40, top: 14, bottom: 28 }
  const chartWidth = width - padding.left - padding.right
  const chartHeight = height - padding.top - padding.bottom
  let maxRequests = 0
  let maxLatency = 0
  points.forEach((point) => {
    maxRequests = Math.max(maxRequests, Number(point.success || 0) + Number(point.error || 0))
    maxLatency = Math.max(maxLatency, Number(point.avg_latency_ms || 0))
  })
  maxRequests = Math.max(1, roundUp(maxRequests))
  maxLatency = Math.max(1, maxLatency)

  const count = points.length
  const slot = count > 1 ? chartWidth / (count - 1) : chartWidth
  const x = (index) => count > 1
    ? padding.left + slot * index
    : padding.left + chartWidth / 2
  const y = (value) => padding.top + chartHeight - chartHeight * (value / maxRequests)
  const latencyY = (value) => padding.top + chartHeight - chartHeight * (value / maxLatency)

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

  context.fillStyle = colors.muted
  context.textAlign = 'left'
  context.fillText('ms', width - padding.right + 6, padding.top + 2)
  for (let index = 1; index <= 2; index += 1) {
    const value = maxLatency * index / 2
    context.fillText(compact(value), width - padding.right + 6, latencyY(value))
  }

  const barWidth = Math.max(2, Math.min(16, slot * 0.28))
  points.forEach((point, index) => {
    const xx = x(index)
    const base = y(0)
    const success = Number(point.success || 0)
    const error = Number(point.error || 0)
    const middle = y(success)
    const top = y(success + error)
    context.fillStyle = colors.green
    context.fillRect(xx - barWidth, middle, barWidth * 2, Math.max(0, base - middle))
    context.fillStyle = colors.red
    context.fillRect(xx - barWidth, top, barWidth * 2, Math.max(0, middle - top))
  })

  context.strokeStyle = colors.yellow
  context.lineWidth = 1.5
  context.beginPath()
  points.forEach((point, index) => {
    const yy = latencyY(Number(point.avg_latency_ms || 0))
    if (index === 0) context.moveTo(x(index), yy)
    else context.lineTo(x(index), yy)
  })
  context.stroke()

  context.fillStyle = colors.muted
  context.textAlign = 'center'
  context.textBaseline = 'top'
  const labelStep = Math.max(1, Math.ceil(count / 8))
  points.forEach((point, index) => {
    if (index % labelStep !== 0 && index !== count - 1) return
    const date = new Date(point.ts * 1000)
    const longRange = data.bucket_seconds >= 3600
    const label = longRange
      ? `${date.getMonth() + 1}/${date.getDate()} ${String(date.getHours()).padStart(2, '0')}:00`
      : `${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}`
    context.fillText(label, x(index), padding.top + chartHeight + 8)
  })
  context.textBaseline = 'alphabetic'
  return true
}
