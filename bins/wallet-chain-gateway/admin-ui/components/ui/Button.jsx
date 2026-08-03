'use client'

export function Button({
  variant = 'primary',
  size = 'md',
  loading = false,
  loadingText = '保存中…',
  disabled = false,
  onClick,
  children,
  type = 'button',
  className = '',
  ...props
}) {
  const variantClass =
    variant === 'ghost' ? 'btn-ghost' : variant === 'danger' ? 'btn-danger' : 'btn-primary'

  const classes = ['btn', variantClass, size === 'sm' ? 'btn-sm' : '', className]
    .filter(Boolean)
    .join(' ')

  return (
    <button
      type={type}
      className={classes}
      disabled={disabled || loading}
      onClick={onClick}
      {...props}
    >
      {loading ? loadingText : children}
    </button>
  )
}
