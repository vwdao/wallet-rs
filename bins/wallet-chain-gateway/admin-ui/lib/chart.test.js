import { describe, expect, it, vi } from 'vitest'
import {
  compact,
  drawChainBarChart,
  drawIpBarChart,
  drawMethodsBarChart,
  drawStatsChart,
  formatTimeLabel,
  getChartHits,
  roundUp,
} from './chart.js'

describe('chart helpers', () => {
  it('rounds chart bounds up to the current magnitude', () => {
    expect(roundUp(45)).toBe(50)
    expect(roundUp(101)).toBe(200)
  })

  it('formats large values compactly', () => {
    expect(compact(999)).toBe('999')
    expect(compact(1500)).toBe('1.5k')
    expect(compact(2_000_000)).toBe('2.0M')
  })

  it('formats time bucket labels', () => {
    const ts = 1_700_000_000
    const date = new Date(ts * 1000)
    const short = `${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}`
    const long = `${date.getMonth() + 1}/${date.getDate()} ${String(date.getHours()).padStart(2, '0')}:00`
    expect(formatTimeLabel(ts, 300)).toBe(short)
    expect(formatTimeLabel(ts, 3600)).toBe(long)
  })
})

function mockCanvas() {
  const context = {
    setTransform: vi.fn(),
    clearRect: vi.fn(),
    beginPath: vi.fn(),
    moveTo: vi.fn(),
    lineTo: vi.fn(),
    stroke: vi.fn(),
    fillText: vi.fn(),
    fillRect: vi.fn(),
  }
  return {
    context,
    canvas: {
      width: 0,
      height: 0,
      getBoundingClientRect: () => ({ width: 600, height: 260 }),
      getContext: () => context,
    },
  }
}

describe('drawStatsChart', () => {
  it('returns false without request data', () => {
    expect(drawStatsChart({}, { points: [], bucket_seconds: 60 })).toBe(false)
    expect(drawStatsChart({}, {
      points: [{ ts: 1, success: 0, error: 0, avg_latency_ms: 0 }],
      bucket_seconds: 60,
    })).toBe(false)
  })

  it('draws request bars and returns true', () => {
    const { canvas, context } = mockCanvas()
    const drawn = drawStatsChart(canvas, {
      bucket_seconds: 60,
      points: [{ ts: 1_700_000_000, success: 3, error: 1, avg_latency_ms: 125 }],
    })

    expect(drawn).toBe(true)
    expect(context.fillRect).toHaveBeenCalledTimes(2)
    expect(context.stroke).toHaveBeenCalled()
  })

  it('hides series when visibility is off', () => {
    const { canvas, context } = mockCanvas()
    const drawn = drawStatsChart(
      canvas,
      {
        bucket_seconds: 60,
        points: [{ ts: 1_700_000_000, success: 3, error: 1, avg_latency_ms: 125 }],
      },
      { success: false, error: false, latency: true },
    )
    expect(drawn).toBe(true)
    expect(context.fillRect).not.toHaveBeenCalled()
  })

  it('records hit regions covering each bucket', () => {
    const { canvas } = mockCanvas()
    drawStatsChart(canvas, {
      bucket_seconds: 60,
      points: [
        { ts: 1_700_000_000, success: 3, error: 1, avg_latency_ms: 125 },
        { ts: 1_700_000_060, success: 1, error: 0, avg_latency_ms: 90 },
      ],
    })
    const hits = getChartHits(canvas)
    expect(hits.length).toBe(2)
    expect(hits[0].payload.ts).toBe(1_700_000_000)
    expect(hits[0].payload.success).toBe(3)
    expect(hits[0].w).toBeGreaterThan(0)
    expect(hits[0].h).toBeGreaterThan(0)
  })

  it('clears hit regions when there is no data', () => {
    const { canvas } = mockCanvas()
    drawStatsChart(canvas, { points: [], bucket_seconds: 60 })
    expect(getChartHits(canvas)).toEqual([])
  })
})

describe('drawIpBarChart', () => {
  it('returns false without ip data', () => {
    expect(drawIpBarChart({}, { series: [], points: [] })).toBe(false)
  })

  it('draws stacked bars across time buckets', () => {
    const { canvas, context } = mockCanvas()
    const drawn = drawIpBarChart(canvas, {
      bucket_seconds: 60,
      series: [
        { protocol: 'http', total_requests: 5 },
        { protocol: 'ws', total_requests: 2 },
      ],
      points: [
        {
          ts: 1_700_000_000,
          values: [
            { protocol: 'http', total_requests: 3 },
            { protocol: 'ws', total_requests: 1 },
          ],
        },
        {
          ts: 1_700_000_060,
          values: [
            { protocol: 'http', total_requests: 2 },
            { protocol: 'ws', total_requests: 1 },
          ],
        },
      ],
    })
    expect(drawn).toBe(true)
    expect(context.fillRect.mock.calls.length).toBeGreaterThanOrEqual(3)
  })

  it('records hit regions per time bucket', () => {
    const { canvas } = mockCanvas()
    drawIpBarChart(canvas, {
      bucket_seconds: 60,
      series: [
        { protocol: 'http', total_requests: 4 },
        { protocol: 'grpc', total_requests: 2 },
      ],
      points: [
        {
          ts: 1_700_000_000,
          values: [
            { protocol: 'http', total_requests: 4 },
            { protocol: 'grpc', total_requests: 2 },
          ],
        },
      ],
    })
    const hits = getChartHits(canvas)
    expect(hits.length).toBe(1)
    expect(hits[0].payload.ts).toBe(1_700_000_000)
    expect(hits[0].payload.total_requests).toBe(6)
    expect(hits[0].payload.values[0].protocol).toBe('http')
  })
})

describe('drawChainBarChart', () => {
  it('returns false without chain data', () => {
    expect(drawChainBarChart({}, { series: [], points: [] })).toBe(false)
  })

  it('draws stacked bars by chain across time buckets', () => {
    const { canvas, context } = mockCanvas()
    const drawn = drawChainBarChart(canvas, {
      bucket_seconds: 60,
      series: [
        { chain_index: 10000900, total_requests: 5 },
        { chain_index: 10000999, total_requests: 2 },
      ],
      points: [
        {
          ts: 1_700_000_000,
          values: [
            { chain_index: 10000900, total_requests: 3, label: 'AVAX' },
            { chain_index: 10000999, total_requests: 1, label: 'HYPER' },
          ],
        },
        {
          ts: 1_700_000_060,
          values: [
            { chain_index: 10000900, total_requests: 2, label: 'AVAX' },
            { chain_index: 10000999, total_requests: 1, label: 'HYPER' },
          ],
        },
      ],
    })
    expect(drawn).toBe(true)
    expect(context.fillRect.mock.calls.length).toBeGreaterThanOrEqual(3)
  })

  it('records hit regions with chain breakdown', () => {
    const { canvas } = mockCanvas()
    drawChainBarChart(canvas, {
      bucket_seconds: 60,
      series: [
        { chain_index: 10000900, total_requests: 17 },
        { chain_index: 10004663, total_requests: 4133 },
      ],
      points: [
        {
          ts: 1_700_000_000,
          values: [
            { chain_index: 10000900, total_requests: 17, label: 'AVAX' },
            { chain_index: 10004663, total_requests: 4133, label: 'ROBIN' },
          ],
        },
      ],
    })
    const hits = getChartHits(canvas)
    expect(hits.length).toBe(1)
    expect(hits[0].payload.total_requests).toBe(4150)
    expect(hits[0].payload.values[0].chain_index).toBe(10000900)
  })
})

describe('drawMethodsBarChart', () => {
  it('returns false without method data', () => {
    expect(drawMethodsBarChart({}, { items: [] })).toBe(false)
  })

  it('draws stacked bars per rpc method', () => {
    const { canvas, context } = mockCanvas()
    const drawn = drawMethodsBarChart(canvas, {
      items: [
        { method: 'eth_blockNumber', chain_index: 60, total_requests: 4, success_count: 3, error_count: 1, avg_latency_ms: 80 },
        { method: '/protocol.Wallet/GetNowBlock2', chain_index: 195, total_requests: 2, success_count: 2, error_count: 0, avg_latency_ms: 60 },
      ],
    })
    expect(drawn).toBe(true)
    expect(context.fillRect.mock.calls.length).toBeGreaterThanOrEqual(3)
  })

  it('records hit regions per method with chain and counts', () => {
    const { canvas } = mockCanvas()
    drawMethodsBarChart(canvas, {
      items: [
        { method: 'eth_blockNumber', chain_index: 60, total_requests: 4, success_count: 3, error_count: 1, avg_latency_ms: 80 },
        { method: '/protocol.Wallet/GetNowBlock2', chain_index: 195, total_requests: 2, success_count: 2, error_count: 0, avg_latency_ms: 60 },
      ],
    })
    const hits = getChartHits(canvas)
    expect(hits.length).toBe(2)
    expect(hits[0].payload.method).toBe('eth_blockNumber')
    expect(hits[0].payload.chain_index).toBe(60)
    expect(hits[0].payload.total_requests).toBe(4)
    expect(hits[0].payload.error_count).toBe(1)
  })
})
