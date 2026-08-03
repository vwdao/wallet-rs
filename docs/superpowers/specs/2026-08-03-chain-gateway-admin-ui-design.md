# Chain Gateway Admin UI Redesign

Date: 2026-08-03  
Scope: `bins/wallet-chain-gateway` admin console (frontend + static serving)

## Goal

Rebuild the admin console as a maintainable Next.js static-export app with a polished dark theme (teal accent) and micro-interactions, while preserving full feature parity with the current production `static/admin.html` and without changing admin API contracts.

## Decisions

| Topic | Choice |
|-------|--------|
| Scope | Experience upgrade: visual system + loading/empty/focus/modal interactions |
| Theme | Dark only |
| Accent | Teal / cyan (replace indigo `#6366f1`) |
| Architecture | Multi-file Next.js App Router with `output: 'export'` |
| Language | Chinese UI labels; keep technical field names (URL, Tier, etc.) where useful |
| Backend | No API changes |

## Current State

- Production UI: single file `bins/wallet-chain-gateway/static/admin.html`, embedded via `include_str!` in `main.rs` and served at `GET /admin`.
- Auth: username/password → `POST /admin/login` → Bearer token in `localStorage` (`cg_admin_token`).
- Incomplete stub: `bins/wallet-chain-gateway/admin-ui` (Next 15, `output: 'export'`) — missing `globals.css`, outdated auth (`x-admin-key`), missing protocol/headers/stats chart/client IP, not wired into the Rust binary.

## Architecture

```
bins/wallet-chain-gateway/
  admin-ui/                 # Next.js source
    app/
      layout.jsx
      page.jsx              # shell: auth gate + tabs
      globals.css           # tokens + shared styles
    components/
      Login.jsx
      Shell.jsx             # topbar + tabs
      EndpointsPanel.jsx
      EndpointModal.jsx
      KeysPanel.jsx
      KeyModal.jsx
      StatsPanel.jsx
      StatsChart.jsx
      SettingsPanel.jsx
      SettingModal.jsx
      ui/                   # Modal, Button, Badge, Empty, Table wrappers
    lib/
      api.js                # fetch + Bearer + 401 logout
      auth.js               # token storage helpers
      chains.js             # chain index → display name
  static/admin/             # build output synced from admin-ui/out
  src/main.rs               # serve static/admin (HTML + _next assets)
```

### Delivery

1. Next config uses `output: 'export'` and `basePath: '/admin'` so the SPA and `_next` assets are rooted under `/admin`.
2. `admin-ui`: `next build` produces static `out/`.
3. Sync script copies `out/` → `static/admin/` (committed or produced in CI/release build before compiling the gateway).
4. Rust serves `GET /admin` → `static/admin/index.html`, and `GET /admin/<path>` for exported assets under `static/admin/` (replace the current single-file `include_str!` of `admin.html`).
5. Dev: `next.dev` rewrites/proxies existing API paths (`/admin/login`, `/admin/endpoints`, `/admin/keys`, `/admin/stats`, `/admin/settings`, …) to the running gateway; UI routes stay on the Next dev server.
6. No Node runtime in production; binary ships only static files from `static/admin/`.

### Explicit non-goals

- No third-party UI component library
- No light theme / theme toggle
- No admin API contract changes
- No separately deployed admin SPA host
- No new backend features beyond what `static/admin.html` already uses

## Visual system

### Color tokens

- Backgrounds: `--bg` (page) → `--surface` (panels/tables) → `--elevated` (modals/cards)
- Borders: `--border`, `--border-hover`
- Text: `--text`, `--muted`
- Accent: teal (~`#14b8a6`) + brighter hover; used for primary buttons, active tabs, focus rings, selected range pills
- Semantic: green / red / yellow for health, errors, warnings (keep existing meanings)

### Typography & controls

- System font stack (ops console, not marketing)
- Labels: sentence case (not all-caps)
- Shared styling for `input`, `select`, `textarea` (dark fill, same radius, teal focus ring)
- Fix headers textarea light-browser-default clash by explicitly styling `textarea`

### Layout

- Login: centered card on subtle radial gradient background
- App: compact topbar + tabs; content max-width; tables in horizontal scroll containers
- Modals: consistent 2-column grid; full-width URL and Headers; wider Protocol select; actions row bottom-right (Cancel ghost / Save primary)

## Interactions

- Modal: backdrop fade + short panel enter transition; close on Esc and backdrop click
- Buttons: hover / active / disabled; Save shows loading (disabled + label change)
- Table row hover; clear empty states
- Stats range switch selected state; chart area loading or fade-in
- Visible focus rings; keyboard-usable modals and primary flows

## Feature parity checklist

Must match current `static/admin.html` behavior:

- [ ] Login via `POST /admin/login`; persist Bearer token; logout clears session
- [ ] Endpoints list: chain, URL, tier, archive, priority, weight, health, latency, enabled, actions
- [ ] Endpoint create/edit: chain, priority, URL, protocol (auto/http/ws/grpc/tcp), tier, weight, archive, enabled, headers (name: value per line)
- [ ] Endpoint delete/disable flow as today
- [ ] API keys CRUD: name, rate/min, tier, allowed chains, enabled, request count display
- [ ] Stats: ranges 15m / 1h / 24h / 7d / 30d; success/error/latency chart; summary cards; table including Client IP and method aggregation
- [ ] Settings list + edit value modal
- [ ] Chain display names via shared map (at least current `CHAIN_NAMES` coverage)

Chart implementation: keep canvas-based drawing (extract helper/hook); chart library optional only if it clearly reduces code without bloating the export.

## Component responsibilities

| Module | Responsibility |
|--------|----------------|
| `lib/api.js` | JSON fetch, Authorization header, 401 → logout |
| `lib/auth.js` | Read/write/clear `cg_admin_token` (same key as today) |
| `lib/chains.js` | `chainName(id)` map |
| `Login` | Username/password form |
| `Shell` | Topbar, tabs, active panel |
| `EndpointsPanel` / `EndpointModal` | List + modal form |
| `KeysPanel` / `KeyModal` | List + modal form |
| `StatsPanel` / `StatsChart` | Range, chart, cards, table |
| `SettingsPanel` / `SettingModal` | List + edit |
| Shared UI | Modal shell, buttons, badges, empty state, table chrome |

## Migration plan

1. Flesh out `admin-ui` to full parity + new visual system.
2. Add sync/build step and Rust static asset serving for export output.
3. Verify `/admin` in production-like run (login, all four tabs, modals, stats).
4. Remove or stop embedding the monolithic `static/admin.html` once export path is the source of truth (keep a short fallback only if needed during cutover).

## Success criteria

- Admin console is componentized under `admin-ui` and builds to static files served by the gateway.
- Dark teal theme is consistent across login, panels, tables, and modals (including textarea).
- Feature parity with current admin.html checklist above.
- Micro-interactions listed above are present and do not block basic CRUD if JS animations fail.
- No admin API changes required to ship.
