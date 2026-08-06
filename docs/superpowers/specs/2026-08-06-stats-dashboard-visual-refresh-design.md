# Stats Dashboard Visual Refresh

Date: 2026-08-06  
Scope: `bins/wallet-chain-gateway/admin-ui` statistics page and shared admin shell presentation

## Goal

Upgrade the admin statistics experience from a flat utility view into a more visual, dashboard-like control surface with stronger hierarchy, better first-screen readability, and a clearer "main trend + supporting breakdowns + detailed logs" structure, without changing any admin API contracts or chart drawing logic.

## Decisions

| Topic | Choice |
|-------|--------|
| Style direction | Technology dashboard / monitoring console |
| Scope | `StatsPanel`, chart containers, stats table styling, `Shell` header/navigation atmosphere |
| Backend | No API or data model changes |
| Charts | Reuse existing canvas chart rendering |
| Theme | Keep dark mode; intensify layering and glow subtly |
| Interaction risk | Low; layout and styling focused |

## Current State

- `StatsPanel` already loads four datasets: summary table rows, trend series, methods distribution, and IP distribution.
- The current structure is functionally correct but visually flat:
  - toolbar at top
  - stacked charts
  - KPI cards below charts
  - detailed table at bottom
- `Shell` header and nav are clean but read as generic admin chrome rather than a dashboard frame.
- Existing color tokens already support a dark teal system and semantic green/red/yellow states.

## Design Summary

The refreshed experience should feel like an operations dashboard rather than a CRUD page with charts appended. The information hierarchy will shift so users first see system posture, then overall trend, then hotspots, then row-level detail.

## Layout

### 1. Shared shell atmosphere

Enhance `Shell` to frame the statistics page inside a more immersive console surface:

- add deeper background layering to the page shell
- give the top header a slightly elevated, glass/control-bar feel
- make navigation feel like dashboard tabs rather than plain admin tabs
- preserve current tab model and behavior

This change should remain compatible with the other tabs and not require panel-specific logic outside the stats page.

### 2. Stats hero section

Rework the top of `StatsPanel` into a hero-like summary strip:

- left side: page title plus a short descriptor such as current selected chain and time range context
- right side: range switch, chain filter, refresh action
- below or integrated into the hero: four KPI cards for total requests, total errors, error rate, and average latency

This moves the most important aggregate signals above the charts so the page communicates value immediately on load.

### 3. Primary and secondary chart zones

Reorganize the chart area into a dashboard grid:

- primary card: `StatsChart` spans the full width and acts as the anchor visualization
- secondary cards: `MethodsChart` and `IpStatsChart` appear in a two-column grid beneath the main chart on desktop
- responsive fallback: secondary cards collapse into a single column on narrower screens

Each chart card should have:

- a stronger container identity
- explicit title
- short supporting description
- consistent interior spacing
- clearer visual separation from surrounding content

### 4. Monitoring-style detail table

Demote the table from primary visual prominence while improving readability:

- place the table in a dedicated lower section with its own heading
- use darker panel styling with clear row separators and hover feedback
- keep all existing columns and values
- preserve horizontal overflow behavior for narrower widths

The table should feel like a log/details surface that supports diagnosis after the overview cards and charts.

## Visual Language

### Background and surface treatment

Introduce a stronger dashboard atmosphere through CSS only:

- layered radial gradients behind the shell and content
- subtle grid or scanline-inspired texture where it does not reduce readability
- more differentiated surface depths between page background, cards, and elevated content
- slightly brighter edge highlights around active or important panels

This should remain restrained enough for day-to-day admin use. The goal is "control room" rather than "marketing landing page."

### KPI card presentation

The summary cards should become more expressive:

- larger numeric emphasis
- smaller muted labels
- per-card accent treatments that reflect value semantics
- subtle glow, border, or top-strip treatments to add quick scannability

Examples:

- requests: neutral/teal emphasis
- errors: red emphasis
- error rate: yellow or mixed warning emphasis
- latency: cyan/yellow performance emphasis

### Chart card presentation

Wrap charts in more deliberate dashboard cards:

- stronger borders and subtle inner contrast
- visible titles and optional one-line descriptors
- legend area styled as part of the card system
- loading and empty states visually centered and integrated with the panel

No chart rendering rewrite is required; the focus is on presentation, hierarchy, and framing.

### Table presentation

Adjust the table to better match the dashboard visual system:

- darker header row
- more refined row dividers
- clearer emphasis for successful/error values
- preserve existing semantic coloring for error rate
- improve code badge readability for API key, IP, and method cells

## Component Responsibilities

| Module | Responsibility after refresh |
|--------|------------------------------|
| `components/Shell.jsx` | Adds dashboard-style shell framing in header/nav structure |
| `components/StatsPanel.jsx` | Reorders markup into hero, KPI, chart grid, and detail sections |
| `components/StatsChart.jsx` | Keeps existing chart behavior; accepts stronger surrounding card treatment |
| `components/MethodsChart.jsx` | Same behavior, styled as secondary dashboard panel |
| `components/IpStatsChart.jsx` | Same behavior, styled as secondary dashboard panel |
| `app/globals.css` | Adds new shell, stats layout, card, grid, and table visuals |

## Data Flow

No data flow changes are planned.

- existing `loadTable`, `loadSeries`, `loadByIp`, and `loadMethods` flows remain intact
- current summary calculation stays in `useMemo`
- current loading and error states remain the source of truth
- no endpoint additions, request shape changes, or state model rewrites

## Error Handling and Empty States

The redesign must preserve current behavior:

- top-level alert still aggregates load errors
- empty states remain explicit when a chart or table has no data
- loading states remain readable and visually integrated into card surfaces

No hidden states should be introduced where the user cannot tell whether data is missing, loading, or failed.

## Responsiveness

The page should remain usable on smaller widths:

- hero controls wrap cleanly
- KPI cards collapse with `auto-fit` grid behavior
- secondary charts stack vertically on smaller screens
- table remains horizontally scrollable

The desktop experience is the visual priority, but the mobile/narrow-laptop layout must not break.

## Non-Goals

- No backend statistics changes
- No new filters or drill-down features
- No chart library migration
- No animation-heavy effects that harm readability or performance
- No redesign of non-stats business panels beyond inheriting the improved shell atmosphere

## Verification

Verify the refresh with at least:

1. a local frontend build or equivalent syntax validation
2. a smoke check that the stats page still renders loading, non-empty, and empty/error states without JSX/CSS regressions
3. confirmation that existing chart interactions and range switching still work

## Success Criteria

- the stats page reads as a dashboard with clear first-screen hierarchy
- KPI values are visible before detailed charts and logs
- the main trend chart is visually dominant
- methods/IP charts become supporting panels in a responsive grid
- table remains fully functional but visually secondary
- shell header/nav contribute to a coherent control-center feel
- no admin API changes are required
