'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import { EndpointsPanel } from '../components/EndpointsPanel.jsx'
import { Login } from '../components/Login.jsx'
import { Shell } from '../components/Shell.jsx'
import { Empty } from '../components/ui/Empty.jsx'
import { createApi } from '../lib/api.js'
import { clearToken, getToken } from '../lib/auth.js'

export default function Page() {
  const [authenticated, setAuthenticated] = useState(false)
  const [tab, setTab] = useState('endpoints')

  useEffect(() => {
    setAuthenticated(Boolean(getToken()))
  }, [])

  const logout = useCallback(() => {
    clearToken()
    setAuthenticated(false)
  }, [])

  const { api } = useMemo(() => createApi({ onUnauthorized: logout }), [logout])

  if (!authenticated) {
    return <Login onSuccess={() => setAuthenticated(true)} />
  }

  return (
    <Shell tab={tab} onTab={setTab} onLogout={logout}>
      {tab === 'endpoints' ? (
        <EndpointsPanel api={api} />
      ) : (
        <Empty>面板将在后续任务接入</Empty>
      )}
    </Shell>
  )
}
