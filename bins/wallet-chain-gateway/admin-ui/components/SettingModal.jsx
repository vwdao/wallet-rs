'use client'

import { useEffect, useState } from 'react'
import { Button } from './ui/Button.jsx'
import { Field } from './ui/Field.jsx'
import { Modal } from './ui/Modal.jsx'

const EMPTY_FORM = {
  key: '',
  value: '',
}

function settingToForm(setting) {
  if (!setting) return EMPTY_FORM

  return {
    key: setting.key || '',
    value: setting.value || '',
  }
}

export function settingToPayload(form) {
  return {
    value: form.value ?? '',
  }
}

export function SettingModal({ open, setting, api, onClose, onSaved }) {
  const [form, setForm] = useState(EMPTY_FORM)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState('')

  useEffect(() => {
    if (!open) return
    setForm(settingToForm(setting))
    setError('')
    setSaving(false)
  }, [setting, open])

  function update(name, value) {
    setForm((current) => ({ ...current, [name]: value }))
  }

  async function handleSubmit(event) {
    event.preventDefault()
    setError('')
    setSaving(true)

    try {
      if (!form.key) throw new Error('配置键缺失')

      await api(`/admin/settings/${encodeURIComponent(form.key)}`, {
        method: 'PUT',
        body: JSON.stringify(settingToPayload(form)),
      })
      await onSaved?.()
    } catch (saveError) {
      setError(saveError.message || '保存配置失败')
    } finally {
      setSaving(false)
    }
  }

  return (
    <Modal
      open={open}
      title="编辑配置"
      onClose={onClose}
      footer={
        <>
          <Button variant="ghost" onClick={onClose} disabled={saving}>取消</Button>
          <Button type="submit" form="setting-form" loading={saving}>保存</Button>
        </>
      }
    >
      <form id="setting-form" onSubmit={handleSubmit}>
        <div className="form-grid">
          <div className="field-full">
            <Field label="Key">
              <input
                type="text"
                readOnly
                value={form.key}
                style={{ opacity: 0.6 }}
              />
            </Field>
          </div>
          <div className="field-full">
            <Field label="Value">
              <input
                type="text"
                required
                value={form.value}
                onChange={(event) => update('value', event.target.value)}
              />
            </Field>
          </div>
        </div>
        {error ? <div className="alert danger" role="alert">{error}</div> : null}
      </form>
    </Modal>
  )
}
