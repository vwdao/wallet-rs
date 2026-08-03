import { describe, expect, it, vi } from 'vitest'
import { compact, drawIpBarChart, drawStatsChart, roundUp } from './chart.js'

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
})
