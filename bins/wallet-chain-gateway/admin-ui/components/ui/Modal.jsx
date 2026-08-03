'use client'

import { useEffect, useId } from 'react'

export function Modal({ open, title, onClose, children, footer, size = 'md' }) {
  const titleId = useId()

  useEffect(() => {
    if (!open) return
    const onKey = (e) => {
      if (e.key === 'Escape') onClose()
    }
    window.addEventListener('keydown', onKey)
    return () => window.removeEventListener('keydown', onKey)
  }, [open, onClose])

  if (!open) return null

  const modalClass = size === 'sm' ? 'modal modal-sm' : 'modal'

  return (
    <div
      className="modal-overlay open"
      onMouseDown={(e) => {
        if (e.target === e.currentTarget) onClose()
      }}
    >
      <div className={modalClass} role="dialog" aria-modal="true" aria-labelledby={titleId}>
        <h3 id={titleId}>{title}</h3>
        <div className="modal-body">{children}</div>
        {footer && <div className="modal-actions">{footer}</div>}
      </div>
    </div>
  )
}
