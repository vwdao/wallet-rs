'use client'

import { Button } from './ui/Button.jsx'

const tabs = [
  ['endpoints', 'RPC 节点'],
  ['keys', 'API Keys'],
  ['stats', '统计'],
  ['settings', '配置'],
]

export function Shell({ tab, onTab, onLogout, children }) {
  return (
    <main className="shell">
      <header>
        <div>
          <strong>Chain Gateway Admin</strong>
        </div>
        <Button variant="ghost" size="sm" onClick={onLogout}>退出登录</Button>
      </header>
      <nav aria-label="管理后台导航">
        {tabs.map(([id, label]) => (
          <button
            key={id}
            type="button"
            className={tab === id ? 'active' : ''}
            aria-current={tab === id ? 'page' : undefined}
            onClick={() => onTab(id)}
          >
            {label}
          </button>
        ))}
      </nav>
      <section className="content">{children}</section>
    </main>
  )
}
