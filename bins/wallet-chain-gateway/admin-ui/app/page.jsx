'use client'

import { useEffect, useState } from 'react'
import { Login } from '../components/Login.jsx'
import { Shell } from '../components/Shell.jsx'
import { Empty } from '../components/ui/Empty.jsx'
import { clearToken, getToken } from '../lib/auth.js'

export default function Page() {
  const [authenticated, setAuthenticated] = useState(false)
  const [tab, setTab] = useState('endpoints')

  useEffect(() => {
    setAuthenticated(Boolean(getToken()))
  }, [])

  function logout() {
    clearToken()
    setAuthenticated(false)
  }

  if (!authenticated) {
    return <Login onSuccess={() => setAuthenticated(true)} />
  }

  return (
    <Shell tab={tab} onTab={setTab} onLogout={logout}>
      <Empty>面板将在后续任务接入</Empty>
    </Shell>
  )
}
