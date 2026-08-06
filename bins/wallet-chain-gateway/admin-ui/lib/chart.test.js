import { describe, expect, it, vi } from 'vitest'
import {
  compact,
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
    expect(drawIpBarChart({}, { items: [] })).toBe(false)
  })

  it('draws stacked bars per client ip', () => {
    const { canvas, context } = mockCanvas()
    const drawn = drawIpBarChart(canvas, {
      items: [
        { client_ip: '1.1.1.1', total_requests: 4, success_count: 3, error_count: 1 },
        { client_ip: '2.2.2.2', total_requests: 2, success_count: 2, error_count: 0 },
      ],
    })
    expect(drawn).toBe(true)
    expect(context.fillRect.mock.calls.length).toBeGreaterThanOrEqual(3)
  })

  it('records hit regions per client ip', () => {
    const { canvas } = mockCanvas()
    drawIpBarChart(canvas, {
      items: [
        { client_ip: '1.1.1.1', total_requests: 4, success_count: 3, error_count: 1, avg_latency_ms: 80 },
        { client_ip: '2.2.2.2', total_requests: 2, success_count: 2, error_count: 0, avg_latency_ms: 60 },
      ],
    })
    const hits = getChartHits(canvas)
    expect(hits.length).toBe(2)
    expect(hits[0].payload.client_ip).toBe('1.1.1.1')
    expect(hits[0].payload.total_requests).toBe(4)
    expect(hits[1].payload.client_ip).toBe('2.2.2.2')
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
