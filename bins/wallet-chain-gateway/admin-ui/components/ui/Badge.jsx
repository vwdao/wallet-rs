export function Badge({ tone = 'teal', children }) {
  return <span className={`badge badge-${tone}`}>{children}</span>
}
