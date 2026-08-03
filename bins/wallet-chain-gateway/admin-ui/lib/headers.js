export function headersToText(h) {
  return Object.entries(h || {}).map(([k, v]) => `${k}: ${v}`).join('\n')
}

export function headersFromText(t) {
  const h = {}
  for (const line of String(t || '').split('\n')) {
    const i = line.indexOf(':')
    if (i > 0) {
      const k = line.slice(0, i).trim()
      const v = line.slice(i + 1).trim()
      if (k) h[k] = v
    }
  }
  return h
}
