const TOOLTIP_WIDTH = 480

export function UrlTooltip({ x, y, url }) {
  if (!url) return null
  const viewport = typeof window === 'undefined' ? null : window
  const left = viewport
    ? Math.max(8, Math.min(x + 16, viewport.innerWidth - TOOLTIP_WIDTH - 8))
    : x + 16
  const top = viewport ? Math.max(8, y + 18) : y + 18
  return (
    <div className="url-tooltip" style={{ left, top }}>
      <div className="url-tooltip-title">URL</div>
      <div className="url-tooltip-value">{url}</div>
      <div className="url-tooltip-hint">点击单元格复制 URL</div>
    </div>
  )
}
