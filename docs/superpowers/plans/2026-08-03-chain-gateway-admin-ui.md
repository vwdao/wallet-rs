# Chain Gateway Admin UI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the monolithic `static/admin.html` with a Next.js static-export admin console (dark + teal, micro-interactions) served by the gateway binary with full API parity.

**Architecture:** Rebuild `bins/wallet-chain-gateway/admin-ui` as a Next 15 App Router client SPA (`output: 'export'`, `basePath: '/admin'`). Pure helpers live in `lib/` with Vitest. Build syncs `out/` → `static/admin/`. Rust embeds that folder via Salvo `static_embed` + `rust-embed` so production stays a single binary. Admin JSON APIs under `/admin/*` keep precedence over the SPA catch-all.

**Tech Stack:** Next.js 15, React 19, plain CSS (design tokens), Vitest (lib only), Salvo `serve-static` + `rust-embed`, existing admin HTTP APIs.

## Global Constraints

- Dark theme only; accent teal ~`#14b8a6` (replace indigo `#6366f1`)
- Chinese UI labels; keep technical field names (URL, Tier, etc.) where useful
- No third-party UI component library; no light theme; no admin API contract changes
- Token storage key remains `cg_admin_token`; auth is Bearer from `POST /admin/login`
- Feature parity with current `static/admin.html` (endpoints protocol/headers, stats series + client_ip, settings)
- Production: no Node runtime; ship embedded static files only
- Spec: `docs/superpowers/specs/2026-08-03-chain-gateway-admin-ui-design.md`

## File Structure

| Path | Responsibility |
|------|----------------|
| `admin-ui/lib/auth.js` | Token get/set/clear |
| `admin-ui/lib/api.js` | JSON fetch + Bearer + 401 hook |
| `admin-ui/lib/chains.js` | Chain index → name |
| `admin-ui/lib/headers.js` | Headers object ↔ textarea text |
| `admin-ui/lib/chart.js` | Canvas stats chart drawing |
| `admin-ui/lib/*.test.js` | Vitest unit tests for pure helpers |
| `admin-ui/app/globals.css` | Tokens + shared component styles |
| `admin-ui/app/layout.jsx` | Root HTML shell |
| `admin-ui/app/page.jsx` | Auth gate + tab shell |
| `admin-ui/components/ui/*` | Modal, Button, Badge, Empty, Field |
| `admin-ui/components/{Login,Shell,Endpoints*,Keys*,Stats*,Settings*}.jsx` | Feature panels |
| `admin-ui/next.config.mjs` | export + basePath + API proxy |
| `admin-ui/package.json` | scripts: build, sync, test |
| `admin-ui/scripts/sync-static.mjs` | Copy `out/` → `../static/admin/` |
| `static/admin/` | Embedded build output (committed after sync) |
| `src/main.rs` | Embed + serve SPA; stop `include_str!` admin.html |
| `Cargo.toml` (gateway) | Enable Salvo `serve-static` / rust-embed as needed |
| Delete or stop shipping | `static/admin.html` after cutover |

---

### Task 1: Lib helpers + Vitest

**Files:**
- Create: `bins/wallet-chain-gateway/admin-ui/lib/auth.js`
- Create: `bins/wallet-chain-gateway/admin-ui/lib/chains.js`
- Create: `bins/wallet-chain-gateway/admin-ui/lib/headers.js`
- Create: `bins/wallet-chain-gateway/admin-ui/lib/auth.test.js`
- Create: `bins/wallet-chain-gateway/admin-ui/lib/chains.test.js`
- Create: `bins/wallet-chain-gateway/admin-ui/lib/headers.test.js`
- Modify: `bins/wallet-chain-gateway/admin-ui/package.json` (add vitest scripts/devDependency)

**Interfaces:**
- Produces:
  - `TOKEN_KEY = 'cg_admin_token'`
  - `getToken(): string`, `setToken(token: string): void`, `clearToken(): void`
  - `chainName(id: number|string): string`
  - `headersToText(h: Record<string,string>): string`
  - `headersFromText(t: string): Record<string,string>`

- [ ] **Step 1: Add Vitest and write failing tests**

Update `package.json` scripts/devDependencies:

```json
{
  "scripts": {
    "build": "next build && node scripts/sync-static.mjs",
    "dev": "next dev -p 3010",
    "test": "vitest run",
    "sync": "node scripts/sync-static.mjs"
  },
  "devDependencies": {
    "vitest": "^3.0.5"
  }
}
```

Create `lib/headers.test.js`:

```js
import { describe, it, expect } from 'vitest'
import { headersToText, headersFromText } from './headers.js'

describe('headers', () => {
  it('roundtrips name: value lines', () => {
    const text = 'Authorization: Bearer xxx\nX-Api-Key: yyy'
    expect(headersFromText(text)).toEqual({
      Authorization: 'Bearer xxx',
      'X-Api-Key': 'yyy',
    })
    expect(headersToText(headersFromText(text))).toBe(text)
  })

  it('skips invalid lines', () => {
    expect(headersFromText('nope\n: empty\nOk: 1')).toEqual({ Ok: '1' })
  })
})
```

Create `lib/chains.test.js`:

```js
import { describe, it, expect } from 'vitest'
import { chainName } from './chains.js'

describe('chainName', () => {
  it('maps known chains', () => {
    expect(chainName(60)).toBe('ETH')
    expect(chainName(501)).toBe('SOL')
  })
  it('falls back to id', () => {
    expect(chainName(99999)).toBe('99999')
  })
})
```

Create `lib/auth.test.js` (use vitest `localStorage` mock via happy-dom or manual):

```js
import { describe, it, expect, beforeEach } from 'vitest'
import { getToken, setToken, clearToken, TOKEN_KEY } from './auth.js'

beforeEach(() => {
  globalThis.localStorage = {
    store: {},
    getItem(k) { return this.store[k] ?? null },
    setItem(k, v) { this.store[k] = String(v) },
    removeItem(k) { delete this.store[k] },
  }
})

describe('auth', () => {
  it('stores and clears token', () => {
    expect(getToken()).toBe('')
    setToken('abc')
    expect(getToken()).toBe('abc')
    expect(localStorage.getItem(TOKEN_KEY)).toBe('abc')
    clearToken()
    expect(getToken()).toBe('')
  })
})
```

- [ ] **Step 2: Run tests — expect FAIL (modules missing)**

Run: `cd bins/wallet-chain-gateway/admin-ui && npm install && npm test`

Expected: FAIL resolving `./headers.js` / `./chains.js` / `./auth.js`

- [ ] **Step 3: Implement helpers**

`lib/headers.js`:

```js
export function headersToText(h) {
  return Object.entries(h || {}).map(([k, v]) => `${k}: ${v}`).join('\n')
}

export function headersFromText(t) {
  const h = {}
  for (const line of String(t || '').split('\n')) {
    const i = line.indexOf(':')
    if (i > 0) {
      const k = line.slice(0, i).trim()
      const v = line.slice(i + 1).trim()
      if (k) h[k] = v
    }
  }
  return h
}
```

`lib/chains.js` (copy full map from current admin.html `CHAIN_NAMES`):

```js
const CHAIN_NAMES = {
  0: 'BTC', 3: 'DOGE', 60: 'ETH', 195: 'TRON', 501: 'SOL', 966: 'POL',
  20000714: 'BSC', 8453: 'BASE', 10042221: 'ARB', 10000070: 'OP',
  10000900: 'AVAX', 10000999: 'HYPER', 10004663: 'ROBIN',
}

export function chainName(id) {
  return CHAIN_NAMES[id] ?? String(id)
}
```

`lib/auth.js`:

```js
export const TOKEN_KEY = 'cg_admin_token'

export function getToken() {
  if (typeof localStorage === 'undefined') return ''
  return localStorage.getItem(TOKEN_KEY) || ''
}

export function setToken(token) {
  localStorage.setItem(TOKEN_KEY, token)
}

export function clearToken() {
  localStorage.removeItem(TOKEN_KEY)
}
```

- [ ] **Step 4: Run tests — expect PASS**

Run: `cd bins/wallet-chain-gateway/admin-ui && npm test`

Expected: all tests PASS

- [ ] **Step 5: Commit**

```bash
git add bins/wallet-chain-gateway/admin-ui/lib bins/wallet-chain-gateway/admin-ui/package.json bins/wallet-chain-gateway/admin-ui/package-lock.json
git commit -m "$(cat <<'EOF'
feat(admin-ui): add auth/chains/headers helpers with tests

EOF
)"
```

---

### Task 2: Design tokens + shared UI primitives

**Files:**
- Create: `bins/wallet-chain-gateway/admin-ui/app/globals.css`
- Create: `bins/wallet-chain-gateway/admin-ui/components/ui/Button.jsx`
- Create: `bins/wallet-chain-gateway/admin-ui/components/ui/Badge.jsx`
- Create: `bins/wallet-chain-gateway/admin-ui/components/ui/Empty.jsx`
- Create: `bins/wallet-chain-gateway/admin-ui/components/ui/Field.jsx`
- Create: `bins/wallet-chain-gateway/admin-ui/components/ui/Modal.jsx`
- Modify: `bins/wallet-chain-gateway/admin-ui/app/layout.jsx`

**Interfaces:**
- Consumes: none from Task 1
- Produces:
  - `<Button variant="primary"|"ghost"|"danger" size="sm"|"md" loading? disabled? onClick children>`
  - `<Badge tone="green"|"red"|"yellow"|"teal">`
  - `<Empty>{children}</Empty>`
  - `<Field label hint?>{input}</Field>`
  - `<Modal open title onClose children footer>` — Esc + backdrop close; CSS enter transition

- [ ] **Step 1: Write `globals.css` tokens and base styles**

Include at minimum:

```css
:root {
  --bg: #0b1220;
  --surface: #121a2b;
  --elevated: #182338;
  --border: #273249;
  --border-hover: #3a4a66;
  --text: #e8eef9;
  --muted: #8b9bb4;
  --accent: #14b8a6;
  --accent-hover: #2dd4bf;
  --accent-ring: rgba(20, 184, 166, 0.35);
  --green: #22c55e;
  --red: #ef4444;
  --yellow: #eab308;
  --radius: 10px;
  --shadow: 0 18px 50px rgba(0, 0, 0, 0.45);
}

* { box-sizing: border-box; margin: 0; padding: 0; }
html, body { min-height: 100%; }
body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  background: var(--bg);
  color: var(--text);
}

input, select, textarea, button { font: inherit; }
input, select, textarea {
  width: 100%;
  padding: 9px 12px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  color: var(--text);
}
input:focus, select:focus, textarea:focus {
  outline: none;
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-ring);
}
textarea { resize: vertical; min-height: 96px; }

/* Plus: .btn*, .badge*, .modal-*, .table-wrap, .form-grid, login, shell, tabs, etc. */
```

Also style table row hover, range pills, login radial background, modal overlay/panel animations.

- [ ] **Step 2: Implement UI components**

`Modal.jsx` must:

```jsx
'use client'
import { useEffect } from 'react'

export function Modal({ open, title, onClose, children, footer }) {
  useEffect(() => {
    if (!open) return
    const onKey = (e) => { if (e.key === 'Escape') onClose() }
    window.addEventListener('keydown', onKey)
    return () => window.removeEventListener('keydown', onKey)
  }, [open, onClose])
  if (!open) return null
  return (
    <div className="modal-overlay open" onMouseDown={(e) => { if (e.target === e.currentTarget) onClose() }}>
      <div className="modal" role="dialog" aria-modal="true">
        <h3>{title}</h3>
        <div className="modal-body">{children}</div>
        {footer && <div className="modal-actions">{footer}</div>}
      </div>
    </div>
  )
}
```

`Button.jsx`: apply `btn btn-primary|ghost|danger`, show `保存中…` when `loading`.

`Field.jsx`: sentence-case label above children.

`Badge.jsx` / `Empty.jsx`: thin wrappers with CSS classes.

- [ ] **Step 3: Update layout**

```jsx
import './globals.css'

export const metadata = {
  title: 'Chain Gateway Admin',
  description: 'Wallet chain gateway administration console',
}

export default function RootLayout({ children }) {
  return (
    <html lang="zh-CN">
      <body>{children}</body>
    </html>
  )
}
```

- [ ] **Step 4: Smoke-check CSS loads**

Run: `cd bins/wallet-chain-gateway/admin-ui && npx next build` may still fail on unfinished page — alternatively keep stub page importing Button. Prefer temporary `page.jsx` that renders `<Button>测试</Button>` if needed, then proceed to Task 3.

- [ ] **Step 5: Commit**

```bash
git add bins/wallet-chain-gateway/admin-ui/app bins/wallet-chain-gateway/admin-ui/components/ui
git commit -m "$(cat <<'EOF'
feat(admin-ui): add teal dark tokens and shared UI primitives

EOF
)"
```

---

### Task 3: API client + Login + Shell

**Files:**
- Create: `bins/wallet-chain-gateway/admin-ui/lib/api.js`
- Create: `bins/wallet-chain-gateway/admin-ui/components/Login.jsx`
- Create: `bins/wallet-chain-gateway/admin-ui/components/Shell.jsx`
- Modify: `bins/wallet-chain-gateway/admin-ui/app/page.jsx`
- Modify: `bins/wallet-chain-gateway/admin-ui/next.config.mjs`

**Interfaces:**
- Consumes: `getToken`, `setToken`, `clearToken` from auth
- Produces:
  - `createApi({ onUnauthorized }): { api(path, opts): Promise<any> }`
  - `Login({ onSuccess })`
  - `Shell({ tab, onTab, onLogout, children })`
  - Tabs: `endpoints` | `keys` | `stats` | `settings`

- [ ] **Step 1: Implement `lib/api.js`**

```js
import { getToken, clearToken } from './auth.js'

export function createApi({ onUnauthorized } = {}) {
  async function api(path, opts = {}) {
    const headers = {
      'Content-Type': 'application/json',
      ...(opts.headers || {}),
    }
    const token = getToken()
    if (token) headers.Authorization = `Bearer ${token}`
    const r = await fetch(path, { ...opts, headers })
    if (r.status === 401) {
      clearToken()
      onUnauthorized?.()
      const e = await r.json().catch(() => ({}))
      throw new Error(e.message || 'session expired')
    }
    if (!r.ok) {
      const e = await r.json().catch(() => ({}))
      throw new Error(e.message || r.statusText)
    }
    return r.json()
  }
  return { api }
}
```

- [ ] **Step 2: Configure Next for export + basePath + API proxy**

`next.config.mjs`:

```js
/** @type {import('next').NextConfig} */
const gateway = process.env.GATEWAY_URL || 'http://127.0.0.1:8080'

const nextConfig = {
  output: 'export',
  trailingSlash: true,
  basePath: '/admin',
  async rewrites() {
    // Dev only; ignored for `output: 'export'` builds
    return [
      { source: '/login', destination: `${gateway}/admin/login` },
      { source: '/endpoints', destination: `${gateway}/admin/endpoints` },
      { source: '/endpoints/:path*', destination: `${gateway}/admin/endpoints/:path*` },
      { source: '/keys', destination: `${gateway}/admin/keys` },
      { source: '/keys/:path*', destination: `${gateway}/admin/keys/:path*` },
      { source: '/stats', destination: `${gateway}/admin/stats` },
      { source: '/stats/:path*', destination: `${gateway}/admin/stats/:path*` },
      { source: '/settings', destination: `${gateway}/admin/settings` },
      { source: '/settings/:path*', destination: `${gateway}/admin/settings/:path*` },
    ]
  },
}

export default nextConfig
```

Note: with `basePath: '/admin'`, browser `fetch('/admin/endpoints')` hits the gateway path directly when UI is served by gateway; in `next dev`, prefer absolute `/admin/...` fetches — Next rewrites with basePath are auto-prefixed. **Use fetch paths as `/admin/login` etc.** (absolute from origin). For `next dev`, set `rewrites` with `basePath: false` if needed:

```js
{ source: '/admin/login', destination: `${gateway}/admin/login`, basePath: false }
```

(repeat for other API routes). Keep UI pages under basePath.

- [ ] **Step 3: Implement Login + Shell + page auth gate**

`Login.jsx`: username/password, `POST /admin/login` body `{username,password}`, `setToken(d.token)`, call `onSuccess`. Show error on failure.

`Shell.jsx`: title「Chain Gateway Admin」, tabs in Chinese (`RPC 节点` / `API Keys` / `统计` / `配置`), Logout button.

`page.jsx`: client component; if no token show Login; else Shell with placeholder panels (`<Empty>面板将在后续任务接入</Empty>`).

- [ ] **Step 4: Manual check**

Run gateway + `npm run dev`. Login with configured admin user. Confirm tab switching and logout.

- [ ] **Step 5: Commit**

```bash
git add bins/wallet-chain-gateway/admin-ui
git commit -m "$(cat <<'EOF'
feat(admin-ui): add API client, login, and app shell

EOF
)"
```

---

### Task 4: Endpoints panel + modal

**Files:**
- Create: `bins/wallet-chain-gateway/admin-ui/components/EndpointModal.jsx`
- Create: `bins/wallet-chain-gateway/admin-ui/components/EndpointsPanel.jsx`
- Modify: `bins/wallet-chain-gateway/admin-ui/app/page.jsx`

**Interfaces:**
- Consumes: `api`, `chainName`, `headersToText`, `headersFromText`, `Modal`, `Button`, `Badge`, `Field`, `Empty`
- Produces: `<EndpointsPanel api />` with list + create/edit/disable

- [ ] **Step 1: Implement EndpointModal form fields**

Fields (parity with admin.html):
- chain_index, priority
- url (full width)
- protocol (`''|http|ws|grpc|tcp`), tier, weight
- is_archive, enabled
- headers textarea (name: value per line)

On save:

```js
const body = {
  chain_index: Number(form.chain_index) || 60,
  url: form.url.trim(),
  protocol: form.protocol || '',
  weight: Number(form.weight) || 1,
  enabled: form.enabled === true || form.enabled === 'true',
  tier: form.tier || 'free',
  is_archive: form.is_archive === true || form.is_archive === 'true',
  priority: Number(form.priority) || 0,
  headers: headersFromText(form.headersText),
}
if (!body.url) throw new Error('URL 必填')
if (form.id) await api(`/admin/endpoints/${form.id}`, { method: 'PUT', body: JSON.stringify(body) })
else await api('/admin/endpoints', { method: 'POST', body: JSON.stringify(body) })
```

Save button uses `loading` state.

- [ ] **Step 2: Implement EndpointsPanel table**

Columns: Chain, URL, Tier, Archive, Priority, Weight, Health, Latency, Enabled, Actions (编辑 / 禁用).

Disable: `confirm` then `DELETE /admin/endpoints/:id`.

- [ ] **Step 3: Wire into page.jsx for tab `endpoints`**

- [ ] **Step 4: Manual check create/edit/disable against running gateway**

- [ ] **Step 5: Commit**

```bash
git add bins/wallet-chain-gateway/admin-ui/components bins/wallet-chain-gateway/admin-ui/app/page.jsx
git commit -m "$(cat <<'EOF'
feat(admin-ui): add endpoints panel with protocol and headers

EOF
)"
```

---

### Task 5: Keys panel + modal

**Files:**
- Create: `bins/wallet-chain-gateway/admin-ui/components/KeyModal.jsx`
- Create: `bins/wallet-chain-gateway/admin-ui/components/KeysPanel.jsx`
- Modify: `bins/wallet-chain-gateway/admin-ui/app/page.jsx`

**Interfaces:**
- Consumes: `api`, `chainName`, shared UI
- Produces: `<KeysPanel api />`

- [ ] **Step 1: Implement KeyModal**

Fields: name, rate_limit_per_min, allowed_tier (`all|free|paid`), allowed_chains (comma-separated), enabled.

POST/PUT `/admin/keys` / `/admin/keys/:id` with:

```js
{
  name,
  rate_limit_per_min: Number(rate) || 60,
  allowed_tier,
  allowed_chains: chainsStr
    ? chainsStr.split(',').map((s) => parseInt(s.trim(), 10)).filter(Boolean)
    : [],
  enabled: enabled === true || enabled === 'true',
}
```

- [ ] **Step 2: Implement KeysPanel**

Columns: Name, API Key, Rate/min, Tier, Allowed Chains, Requests, Enabled, Actions.

Disable via `DELETE /admin/keys/:id` after confirm.

- [ ] **Step 3: Wire tab `keys`**

- [ ] **Step 4: Manual CRUD check**

- [ ] **Step 5: Commit**

```bash
git add bins/wallet-chain-gateway/admin-ui/components bins/wallet-chain-gateway/admin-ui/app/page.jsx
git commit -m "$(cat <<'EOF'
feat(admin-ui): add API keys panel

EOF
)"
```

---

### Task 6: Stats panel + canvas chart

**Files:**
- Create: `bins/wallet-chain-gateway/admin-ui/lib/chart.js`
- Create: `bins/wallet-chain-gateway/admin-ui/lib/chart.test.js`
- Create: `bins/wallet-chain-gateway/admin-ui/components/StatsChart.jsx`
- Create: `bins/wallet-chain-gateway/admin-ui/components/StatsPanel.jsx`
- Modify: `bins/wallet-chain-gateway/admin-ui/app/page.jsx`

**Interfaces:**
- Consumes: `api`, `chainName`
- Produces:
  - `roundUp(v: number): number`, `compact(v: number): string`
  - `drawStatsChart(canvas: HTMLCanvasElement, data: {points, bucket_seconds}): boolean` (false = empty)
  - `<StatsPanel api />`

- [ ] **Step 1: Extract chart helpers + tests**

Port `roundUp` / `compact` from admin.html into `lib/chart.js` and test:

```js
expect(compact(1500)).toBe('1.5k')
expect(roundUp(45)).toBe(50) // or whatever current admin.html math yields — match implementation exactly
```

Port `drawStatsChart` body from admin.html into `lib/chart.js` (same colors: green/red bars, yellow latency line). Use CSS variables where practical but keep hex fallbacks matching current chart.

- [ ] **Step 2: StatsChart + StatsPanel**

- Ranges: `15m|1h|24h|7d|30d` → `GET /admin/stats/series?range=`
- Table: `GET /admin/stats` with columns API Key, Chain, Client IP, Method, Requests, Success, Errors, Error Rate, Avg Latency
- Summary cards: total requests/errors/error rate/avg latency
- Loading: chart wrap opacity or skeleton class while fetching
- Empty states for table and chart

- [ ] **Step 3: Wire tab `stats`; refresh chart when tab selected**

- [ ] **Step 4: Manual check ranges + table including `client_ip`**

- [ ] **Step 5: Commit**

```bash
git add bins/wallet-chain-gateway/admin-ui
git commit -m "$(cat <<'EOF'
feat(admin-ui): add stats panel with series chart

EOF
)"
```

---

### Task 7: Settings panel + modal

**Files:**
- Create: `bins/wallet-chain-gateway/admin-ui/components/SettingModal.jsx`
- Create: `bins/wallet-chain-gateway/admin-ui/components/SettingsPanel.jsx`
- Modify: `bins/wallet-chain-gateway/admin-ui/app/page.jsx`

**Interfaces:**
- Consumes: `api`, shared UI
- Produces: `<SettingsPanel api />`

- [ ] **Step 1: Implement list + edit modal**

`GET /admin/settings` → table Key / Value / Description / 编辑  
`PUT /admin/settings/:key` body `{ value }`

Key field read-only in modal.

- [ ] **Step 2: Wire tab `settings`**

- [ ] **Step 3: Manual edit check**

- [ ] **Step 4: Commit**

```bash
git add bins/wallet-chain-gateway/admin-ui
git commit -m "$(cat <<'EOF'
feat(admin-ui): add settings panel

EOF
)"
```

---

### Task 8: Build sync script + first static export

**Files:**
- Create: `bins/wallet-chain-gateway/admin-ui/scripts/sync-static.mjs`
- Modify: `bins/wallet-chain-gateway/admin-ui/package.json` (ensure `build` runs sync)
- Create/replace: `bins/wallet-chain-gateway/static/admin/**` (generated)
- Modify: `bins/wallet-chain-gateway/admin-ui/.gitignore` — do **not** ignore `../static/admin` (that lives outside admin-ui). Keep ignoring `out/` and `.next/`.

**Interfaces:**
- Produces: `static/admin/index.html` + `_next/**` after `npm run build`

- [ ] **Step 1: Write sync script**

```js
import { cpSync, rmSync, mkdirSync, existsSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const outDir = join(root, 'out')
const dest = join(root, '..', 'static', 'admin')

if (!existsSync(outDir)) {
  console.error('Missing out/. Run next build first.')
  process.exit(1)
}

rmSync(dest, { recursive: true, force: true })
mkdirSync(dest, { recursive: true })
cpSync(outDir, dest, { recursive: true })
console.log(`Synced ${outDir} -> ${dest}`)
```

- [ ] **Step 2: Build**

Run: `cd bins/wallet-chain-gateway/admin-ui && npm run build`

Expected: `static/admin/index.html` exists; assets under `static/admin/_next/`

- [ ] **Step 3: Commit generated static assets + script**

```bash
git add bins/wallet-chain-gateway/admin-ui/scripts bins/wallet-chain-gateway/admin-ui/package.json bins/wallet-chain-gateway/static/admin
git commit -m "$(cat <<'EOF'
build(admin-ui): sync Next export into static/admin

EOF
)"
```

---

### Task 9: Rust embed + serve SPA (API precedence)

**Files:**
- Modify: `bins/wallet-chain-gateway/Cargo.toml`
- Modify: root `Cargo.toml` workspace `salvo` features **or** gateway-local feature enable for `serve-static`
- Modify: `bins/wallet-chain-gateway/src/main.rs`
- Delete (after verify): `bins/wallet-chain-gateway/static/admin.html` usage

**Interfaces:**
- Produces: `GET /admin` and `GET /admin/<asset>` serve embedded export; `POST/GET /admin/login|endpoints|keys|stats|settings...` unchanged

- [ ] **Step 1: Enable Salvo static embed**

In workspace or gateway `Cargo.toml`, ensure Salvo has `serve-static` (and embed). Prefer gateway override:

```toml
salvo = { workspace = true, features = ["serve-static"] }
```

If workspace feature merging is insufficient, add `"serve-static"` to the workspace salvo features list in root `Cargo.toml`.

- [ ] **Step 2: Replace `ADMIN_HTML` include with rust-embed**

In `main.rs`:

```rust
use rust_embed::RustEmbed;
use salvo::prelude::*;
use salvo::serve_static::{StaticEmbed, static_embed};

#[derive(RustEmbed)]
#[folder = "static/admin"]
struct AdminAssets;

// In router construction, AFTER pushing admin::admin_router() API routes,
// mount SPA catch-all. Example shape (adjust to match existing Router nesting):

// .push(admin::admin_router())  // existing: paths under "admin"/login, endpoints, ...
// .push(
//     Router::with_path("admin/{*path}")
//         .get(static_embed::<AdminAssets>().fallback("index.html"))
// )
// .push(
//     Router::with_path("admin")
//         .get(static_embed::<AdminAssets>().fallback("index.html"))
// )
```

Remove:

```rust
const ADMIN_HTML: &str = include_str!("../static/admin.html");
async fn admin_ui(...) { res.render(ADMIN_HTML); }
```

**Routing order rule:** register concrete API routes before the `{*path}` static catch-all so `/admin/endpoints` never returns `index.html`.

If `rust_embed` is re-exported only via salvo feature, import paths may be `salvo::prelude::*` + `salvo::serve_static::static_embed`. Add explicit `rust-embed` dependency on the gateway crate if the derive is required:

```toml
rust-embed = "8"
```

- [ ] **Step 3: Compile**

Run: `cargo build -p wallet-chain-gateway`

Expected: success. If embed folder empty, ensure Task 8 committed `static/admin` first.

- [ ] **Step 4: Runtime smoke**

Run gateway, open `/admin/`, login, load assets (Network tab: `/admin/_next/...` 200), hit `/admin/endpoints` JSON still works.

- [ ] **Step 5: Remove old `static/admin.html` and commit**

```bash
git rm bins/wallet-chain-gateway/static/admin.html
git add bins/wallet-chain-gateway/src/main.rs bins/wallet-chain-gateway/Cargo.toml Cargo.toml Cargo.lock
git commit -m "$(cat <<'EOF'
feat(chain-gateway): serve embedded Next admin UI from /admin

EOF
)"
```

---

### Task 10: Polish pass + parity verification

**Files:**
- Modify as needed: `admin-ui/app/globals.css`, modals, panels (focus, loading, empty, Protocol width, textarea)

**Checklist (from spec):**

- [ ] **Step 1: Walk parity list**
  - Login Bearer + logout
  - Endpoints full fields including protocol/headers
  - Keys CRUD
  - Stats ranges + chart + client_ip/method
  - Settings edit
  - Chain name map coverage
  - Modal Esc/backdrop; Save loading; no white textarea

- [ ] **Step 2: Rebuild + resync + rebuild Rust**

```bash
cd bins/wallet-chain-gateway/admin-ui && npm test && npm run build
cargo build -p wallet-chain-gateway
```

- [ ] **Step 3: Final commit if polish changes remain**

```bash
git add bins/wallet-chain-gateway
git commit -m "$(cat <<'EOF'
polish(admin-ui): finalize dark teal UX and parity fixes

EOF
)"
```

---

## Self-Review (plan vs spec)

| Spec requirement | Task |
|------------------|------|
| Next export + basePath `/admin` | 3, 8 |
| Sync to `static/admin` + Rust serve embed | 8, 9 |
| Dark teal tokens, sentence-case labels, textarea fix | 2, 10 |
| Micro-interactions (modal/loading/empty/focus) | 2, 4–7, 10 |
| Login Bearer `cg_admin_token` | 1, 3 |
| Endpoints protocol/headers | 4 |
| Keys CRUD | 5 |
| Stats series + client_ip | 6 |
| Settings | 7 |
| No API changes / no UI library / dark only | Global Constraints |
| Remove monolithic admin.html | 9 |

No intentional placeholders left; helper and API shapes match current `static/admin.html`.
