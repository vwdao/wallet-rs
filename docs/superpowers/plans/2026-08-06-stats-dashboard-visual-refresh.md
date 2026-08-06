# Stats Dashboard Visual Refresh Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rework the Chain Gateway admin statistics experience into a technology-dashboard style layout with stronger hierarchy and shell atmosphere, while keeping existing stats behavior and backend contracts intact.

**Architecture:** Keep the current React component structure and canvas chart logic, but reorder `StatsPanel` into a hero/KPI/chart/detail hierarchy and upgrade `Shell` plus shared CSS tokens/layouts to create a more immersive operations-console frame. Changes stay entirely in frontend markup and styling, with existing data loading, summary calculations, errors, and chart rendering preserved.

**Tech Stack:** Next.js App Router, React client components, JSX, shared `globals.css`, existing canvas chart helpers

## Global Constraints

- Style direction: Technology dashboard / monitoring console
- Scope: `StatsPanel`, chart containers, stats table styling, `Shell` header/navigation atmosphere
- Backend: No API or data model changes
- Charts: Reuse existing canvas chart rendering
- Theme: Keep dark mode; intensify layering and glow subtly
- Interaction risk: Low; layout and styling focused
- No backend statistics changes
- No new filters or drill-down features
- No chart library migration
- No animation-heavy effects that harm readability or performance
- No redesign of non-stats business panels beyond inheriting the improved shell atmosphere

---

### Task 1: Reframe the shared shell as a dashboard surface

**Files:**
- Modify: `bins/wallet-chain-gateway/admin-ui/components/Shell.jsx`
- Modify: `bins/wallet-chain-gateway/admin-ui/app/globals.css`

**Interfaces:**
- Consumes: existing `Shell({ tab, onTab, onLogout, children })`
- Produces: same `Shell` API with richer semantic wrappers and CSS hooks for dashboard chrome

- [ ] **Step 1: Add shell wrappers and semantic copy hooks in `Shell.jsx`**

```jsx
export function Shell({ tab, onTab, onLogout, children }) {
  return (
    <main className="shell shell-dashboard">
      <div className="shell-backdrop" aria-hidden="true" />
      <header className="shell-header">
        <div className="shell-brand">
          <span className="shell-kicker">Admin Console</span>
          <strong>Chain Gateway Admin</strong>
        </div>
        <Button variant="ghost" size="sm" onClick={onLogout}>退出登录</Button>
      </header>
      <nav className="shell-nav" aria-label="管理后台导航">
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
```

- [ ] **Step 2: Add shell atmosphere styles in `globals.css`**

```css
.shell-dashboard {
  position: relative;
  background:
    radial-gradient(circle at top, rgba(20, 184, 166, 0.14), transparent 28%),
    radial-gradient(circle at 80% 0%, rgba(59, 130, 246, 0.12), transparent 24%),
    var(--bg);
}

.shell-backdrop {
  position: fixed;
  inset: 0;
  pointer-events: none;
  background-image:
    linear-gradient(rgba(255, 255, 255, 0.02) 1px, transparent 1px),
    linear-gradient(90deg, rgba(255, 255, 255, 0.02) 1px, transparent 1px);
  background-size: 24px 24px;
  mask-image: linear-gradient(to bottom, rgba(255, 255, 255, 0.45), transparent 75%);
}

.shell-header,
.shell-nav {
  position: relative;
  z-index: 1;
  backdrop-filter: blur(14px);
}
```

- [ ] **Step 3: Run a focused build check**

Run: `npm run build`  
Working directory: `bins/wallet-chain-gateway/admin-ui`  
Expected: build succeeds and shell class additions introduce no JSX/CSS errors

### Task 2: Rebuild the stats page hierarchy

**Files:**
- Modify: `bins/wallet-chain-gateway/admin-ui/components/StatsPanel.jsx`

**Interfaces:**
- Consumes: existing stats data loaders, `SummaryCard`, `StatsChart`, `MethodsChart`, `IpStatsChart`, `Button`, `Badge`, `Empty`
- Produces: same public `StatsPanel({ api })` API with hero, KPI, chart-grid, and detail-section markup

- [ ] **Step 1: Add a dashboard hero and move KPI cards above the charts**

```jsx
const selectedChainLabel = chainIndex === ''
  ? '全部链路'
  : chainName(Number(chainIndex))

<section className="stats-hero">
  <div className="stats-hero-copy">
    <span className="stats-kicker">Realtime Overview</span>
    <h2>请求统计指挥台</h2>
    <p>{selectedChainLabel} · 时间范围 {range}</p>
  </div>
  <div className="stats-hero-controls">
    {/* existing filter controls and refresh button */}
  </div>
</section>

<section className="stats-summary-grid">
  <SummaryCard label="总请求数" value={summary.totalRequests.toLocaleString()} accent="teal" />
  <SummaryCard label="总错误数" value={summary.totalErrors.toLocaleString()} accent="red" />
  <SummaryCard label="错误率" value={`${summary.errorRate.toFixed(1)}%`} accent="yellow" />
  <SummaryCard label="平均延迟" value={`${summary.averageLatency.toFixed(0)}ms`} accent="blue" />
</section>
```

- [ ] **Step 2: Restructure chart and detail sections without changing data behavior**

```jsx
<section className="stats-primary-panel">
  <div className="stats-panel-head">
    <div>
      <h3>请求趋势</h3>
      <p>观察成功、错误与平均延迟在当前窗口内的变化。</p>
    </div>
  </div>
  <StatsChart data={series} loading={chartLoading} title="" />
</section>

<section className="stats-secondary-grid">
  <div className="stats-subpanel">
    <div className="stats-panel-head">
      <div>
        <h3>方法热点</h3>
        <p>查看请求量最高的方法与异常分布。</p>
      </div>
    </div>
    <MethodsChart data={methods} loading={methodsLoading} />
  </div>
  <div className="stats-subpanel">
    <div className="stats-panel-head">
      <div>
        <h3>来源 IP</h3>
        <p>识别高频来源与异常客户端行为。</p>
      </div>
    </div>
    <IpStatsChart data={byIp} loading={ipChartLoading} />
  </div>
</section>

<section className="stats-detail-section">
  <div className="stats-panel-head">
    <div>
      <h3>请求明细</h3>
      <p>按 API Key、链路、IP 与方法查看聚合结果。</p>
    </div>
  </div>
  {/* existing loading, empty, and table rendering */}
</section>
```

- [ ] **Step 3: Update `SummaryCard` signature for semantic accents**

```jsx
function SummaryCard({ label, value, accent = 'teal' }) {
  return (
    <div className={`summary-card summary-card-${accent}`}>
      <small>{label}</small>
      <b>{value}</b>
    </div>
  )
}
```

- [ ] **Step 4: Run a focused build check**

Run: `npm run build`  
Working directory: `bins/wallet-chain-gateway/admin-ui`  
Expected: stats layout compiles with no JSX errors and existing imports remain valid

### Task 3: Apply dashboard visual styling to stats panels and table

**Files:**
- Modify: `bins/wallet-chain-gateway/admin-ui/app/globals.css`

**Interfaces:**
- Consumes: new class names from `Shell.jsx` and `StatsPanel.jsx`
- Produces: dashboard hero, KPI, chart grid, and table styles with responsive behavior

- [ ] **Step 1: Add stats dashboard layout styles**

```css
.stats-hero,
.stats-primary-panel,
.stats-subpanel,
.stats-detail-section,
.summary-card {
  position: relative;
  border: 1px solid rgba(94, 234, 212, 0.14);
  background: linear-gradient(180deg, rgba(17, 24, 39, 0.92), rgba(11, 18, 32, 0.96));
  box-shadow: 0 18px 40px rgba(0, 0, 0, 0.35), inset 0 1px 0 rgba(255, 255, 255, 0.04);
}

.stats-secondary-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 16px;
}

.stats-summary-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 16px;
}
```

- [ ] **Step 2: Add KPI, panel-head, and table refinements**

```css
.summary-card-red { --summary-accent: var(--red); }
.summary-card-yellow { --summary-accent: var(--yellow); }
.summary-card-blue { --summary-accent: #38bdf8; }
.summary-card-teal { --summary-accent: var(--accent); }

.summary-card::before {
  content: '';
  position: absolute;
  inset: 0 auto auto 0;
  width: 100%;
  height: 2px;
  background: linear-gradient(90deg, var(--summary-accent), transparent 75%);
}

.table-wrap table thead th {
  background: rgba(148, 163, 184, 0.08);
}

@media (max-width: 960px) {
  .stats-secondary-grid {
    grid-template-columns: 1fr;
  }
}
```

- [ ] **Step 3: Run a full build for regressions**

Run: `npm run build`  
Working directory: `bins/wallet-chain-gateway/admin-ui`  
Expected: build succeeds and responsive/dashboard CSS does not break the export

## Self-Review

- Spec coverage: shell atmosphere, stats hero, KPI promotion, main + secondary chart hierarchy, monitoring-style table, and responsive behavior each map to Tasks 1-3.
- Placeholder scan: no TODO/TBD markers or undefined task references remain.
- Type consistency: all component signatures stay the same except internal `SummaryCard` styling props; no external interface changes are introduced.

## Execution Handoff

Plan complete and saved to `docs/superpowers/plans/2026-08-06-stats-dashboard-visual-refresh.md`.

The user already chose inline execution in this session, so the next step is to use `superpowers:executing-plans` and implement the plan directly with verification checkpoints.
