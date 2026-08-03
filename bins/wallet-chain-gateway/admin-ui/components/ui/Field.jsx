import { cloneElement, isValidElement, useId } from 'react'

export function Field({ label, hint, children }) {
  const generatedId = useId()
  const controlId = isValidElement(children) && children.props.id
    ? children.props.id
    : `field-${generatedId.replace(/:/g, '')}`
  const hintId = hint ? `${controlId}-hint` : undefined
  const control = isValidElement(children)
    ? cloneElement(children, {
        id: controlId,
        'aria-describedby': children.props['aria-describedby'] || hintId,
      })
    : children

  return (
    <div className="field">
      <label className="field-label" htmlFor={controlId}>{label}</label>
      {control}
      {hint ? <small className="field-hint" id={hintId}>{hint}</small> : null}
    </div>
  )
}
