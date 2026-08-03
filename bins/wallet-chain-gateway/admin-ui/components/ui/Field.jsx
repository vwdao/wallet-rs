export function Field({ label, hint, children }) {
  return (
    <div className="field">
      <label className="field-label">{label}</label>
      {children}
      {hint ? <small className="field-hint">{hint}</small> : null}
    </div>
  )
}
