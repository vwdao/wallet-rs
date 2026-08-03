'use client'

import { Modal } from './Modal.jsx'
import { Button } from './Button.jsx'

export function ConfirmDialog({
  open,
  title = '确认操作',
  message,
  detail,
  confirmLabel = '确定',
  cancelLabel = '取消',
  tone = 'danger',
  loading = false,
  onConfirm,
  onCancel,
}) {
  const iconClass = tone === 'danger' ? 'confirm-icon danger' : 'confirm-icon'

  return (
    <Modal
      open={open}
      title={title}
      size="sm"
      onClose={loading ? () => {} : onCancel}
      footer={
        <>
          <Button variant="ghost" disabled={loading} onClick={onCancel}>
            {cancelLabel}
          </Button>
          <Button
            variant={tone === 'danger' ? 'danger' : 'primary'}
            loading={loading}
            loadingText="处理中…"
            onClick={onConfirm}
          >
            {confirmLabel}
          </Button>
        </>
      }
    >
      <div className="confirm-body">
        <div className={iconClass} aria-hidden="true">
          !
        </div>
        <div className="confirm-copy">
          <p className="confirm-message">{message}</p>
          {detail ? <p className="confirm-detail">{detail}</p> : null}
        </div>
      </div>
    </Modal>
  )
}
