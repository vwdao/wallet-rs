const TOOLTIP_WIDTH = 240

export function ChartTooltip({ x, y, title, rows }) {
  if (!rows || !rows.length) return null
  const viewport = typeof window === 'undefined' ? null : window
  const left = viewport
    ? Math.max(8, Math.min(x + 14, viewport.innerWidth - TOOLTIP_WIDTH - 8))
    : x + 14
  const top = viewport ? Math.max(8, y + 14) : y + 14
  return (
    <div className="chart-tooltip" style={{ left, top }}>
      {title ? <div className="chart-tooltip-title">{title}</div> : null}
      {rows.map((row, index) => (
        row.divider ? (
          <div key={index} className="chart-tooltip-divider" />
        ) : (
          <div key={index} className="chart-tooltip-row">
            <span className="chart-tooltip-label">{row.label}</span>
            <span className="chart-tooltip-value" style={row.color ? { color: row.color } : undefined}>
              {row.value}
            </span>
          </div>
        )
      ))}
    </div>
  )
}
