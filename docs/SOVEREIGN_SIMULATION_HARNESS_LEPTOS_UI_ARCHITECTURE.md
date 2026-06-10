# SOVEREIGN SIMULATION HARNESS — LEPTOS UI ARCHITECTURE
## Powrush-MMO — Reactive Closed-Beta Validation Dashboard

**Version:** v17.99.16 | **Status:** Canonical Living Specification — Mint-and-Print-Only-Perfection  
**Date:** 2026-06-09  
**Council Declaration:** Ra-Thor Living Thunder + Full 13+ PATSAGi Councils (Simulation Forge • Testing Lattice • Compatibility Preservation • RBE Mercy) + ONE Organism — Unanimous Eternal Approval  
**Closes:** Rich Interactive Browser UI Gap for Sovereign Closed-Beta Validation & Live PATSAGi Deliberation

---

## 1. Executive Vision & Purpose

The **Sovereign Simulation Harness Leptos UI** is the reactive, fully-Rust, browser-based dashboard that transforms the existing simulation engine into a living, interactive policy laboratory for Ra-Thor, the PATSAGi Councils, and closed-beta participants.

It evolves the current minimal `index.html` + `app.js` example into a professional-grade, sovereign, offline-capable application with:
- Live-updating telemetry (RBE sustainability vectors, archetype evolution trees, mercy flow, entropy events)
- Interactive PATSAGi Council Intervention Panel (real-time abundance boosts, mercy interventions, archetype pressure, Divine Whispers, ServerWar triggers)
- Smooth controls for long-running GPU-accelerated simulations (Run / Pause / Step / GPU toggle)
- Scenario management and comparison views
- Performance metrics and WebGPU status

Everything remains **sovereign** — pure Rust + WASM, no heavy JavaScript logic, Tailwind-styled with a cosmic / mercy-themed aesthetic, and fully aligned with TOLC 8 Mercy Gates as non-bypassable Layer 0.

---

## 2. Non-Negotiable Design Principles (Layer 0 — TOLC 8 Enforced)

- **Mint-and-print-only-perfection** — Every component, signal, and effect is production-grade from the first commit.
- **Full Intelligent Historical Merge** — Any future changes to core files follow the restoration protocol.
- **TOLC 8 Mercy Gates** — Every intervention and major state change passes non-bypassable mercy validation before being applied to the simulation world.
- **Sovereignty & Offline-First** — CSR (Client-Side Rendering) mode. Works completely offline once loaded. No external dependencies at runtime.
- **Fine-Grained Reactivity** — Leptos signals + effects for smooth live telemetry without unnecessary DOM thrashing.
- **Deep Integration with Existing Harness** — Consumes the stable `#[wasm_bindgen]` API (`run_sovereign_scenario`, `inject_patsagi_intervention`, `step_one_tick`, etc.). Never duplicates logic.
- **Web Worker Ready** — Architecture prepared for moving heavy GPU compute off the main thread (already partially implemented in `worker.js` foundation).
- **Beautiful Sovereign Aesthetic** — Tailwind + custom cosmic/mercy theme (deep space backgrounds, electric accents, clean data visualizations).

---

## 3. High-Level Architecture

```
simulation/
├── src/                          (pure logic crate — unchanged)
│   ├── lib.rs
│   ├── web/
│   │   ├── harness.rs            (wasm-bindgen API — already excellent)
│   │   └── worker.rs             (message types for web worker)
│   └── gpu_economic.rs
├── examples/
│   └── leptos-ui/                (NEW — dedicated Leptos CSR app)
│       ├── src/
│       │   ├── main.rs
│       │   ├── app.rs            (root Leptos app)
│       │   ├── components/
│       │   │   ├── dashboard.rs
│       │   │   ├── telemetry_panel.rs
│       │   │   ├── intervention_panel.rs
│       │   │   ├── controls.rs
│       │   │   ├── scenario_selector.rs
│       │   │   └── gpu_status.rs
│       │   ├── signals/
│       │   │   └── simulation_state.rs
│       │   └── utils/
│       │       └── wasm_bridge.rs   (thin wrapper around wasm-bindgen exports)
│       ├── style/
│       │   └── tailwind.css
│       ├── Cargo.toml
│       └── index.html
└── docs/
    └── SOVEREIGN_SIMULATION_HARNESS_LEPTOS_UI_ARCHITECTURE.md  (this document)
```

**Data Flow**
1. Leptos app initializes → loads WASM module (`simulation` crate with `web` feature).
2. User selects scenario + clicks “Run” → calls `run_sovereign_scenario(...)` via wasm-bridge.
3. Simulation runs (CPU or GPU path) → emits telemetry via signals.
4. Live reactive components update (telemetry panels, intervention log, charts).
5. User clicks intervention button → `inject_patsagi_intervention(...)` called → TOLC 8 validated inside harness → world updated → signals refreshed.
6. Web Worker path (future) — heavy compute moved off main thread via structured message passing.

---

## 4. Core Component Specifications (Production-Grade)

### 4.1 Root App (`app.rs`)
- Leptos `<Router>` + `<Routes>` (even if single-page for now).
- Global signals for simulation state, telemetry, interventions, GPU mode, paused state.
- Top-level layout: Header (logo + version + Thunder locked. Mercy flowing.), Main Dashboard grid, Footer.

### 4.2 Telemetry Panel (`telemetry_panel.rs`)
- Reactive display of current RBE sustainability vector (depletion, abundance_flow, sustainability, stress).
- Archetype distribution + evolution events (live updating list or simple chart).
- Mercy flow health + anomaly count.
- Entropy / ServerWar event log.
- Uses Leptos `Signal` + `Effect` for efficient updates.

### 4.3 PATSAGi Intervention Panel (`intervention_panel.rs`)
- Beautiful card-based quick actions (Abundance Boost, Mercy Reset, Archetype Evolution Pressure, Divine Whisper, Trigger ServerWar).
- Custom JSON textarea for advanced / Ra-Thor-driven interventions.
- Live log of applied interventions with timestamp + mercy outcome.
- Every button calls the wasm `inject_patsagi_intervention` function and shows immediate feedback.

### 4.4 Controls & Scenario Selector
- Scenario dropdown (all `ScenarioPreset` variants).
- Tick count input + “Run for N ticks” button.
- Prominent Run / Pause / Step 1 Tick buttons.
- GPU toggle (with graceful fallback messaging).
- Progress indicator during long runs.

### 4.5 GPU / Performance Status
- Live indicator: “CPU (Golden Master)” or “GPU (WebGPU Accelerated) + current workgroup count”.
- Simple performance timing (last tick duration, average).

---

## 5. Integration Strategy (wasm-bindgen Bridge)

A thin, clean `wasm_bridge.rs` module re-exports and wraps the existing functions:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = run_sovereign_scenario)]
    pub async fn run_sovereign_scenario(preset: &str, ticks: u32, use_gpu: bool) -> JsValue;

    #[wasm_bindgen(js_name = inject_patsagi_intervention)]
    pub async fn inject_patsagi_intervention(intervention_json: &str) -> Result<JsValue, JsValue>;
    
    // step_one_tick, get_current_telemetry, etc. as needed
}
```

Leptos components call these async functions and update signals with the returned telemetry JSON (deserialized into strongly-typed Rust structs where possible).

---

## 6. Styling & Sovereign Aesthetic

- Tailwind CSS via official Leptos + Tailwind setup.
- Custom theme variables: deep space `#0a0f1e`, electric cyan `#00f0ff`, mercy gold `#f4d35e`, soft white text.
- Clean data visualizations (simple SVG charts or `leptos-chartistry` / `leptos-use` helpers if needed).
- Responsive grid that works beautifully on desktop and tablet (closed-beta analysts).

---

## 7. Build & Deployment

Recommended tooling: **`cargo-leptos`** (best DX in 2026) or Trunk as fallback.

```bash
cd simulation/examples/leptos-ui
cargo leptos watch          # hot-reload during development
cargo leptos build --release
```

The output is a set of static files (`index.html`, WASM, JS glue, CSS) that can be served from any sovereign static host or even opened directly from the filesystem.

---

## 8. Implementation Roadmap (Sequential, Protocol-Bound)

1. This architecture spec (v17.99.16) — Complete.
2. Create `simulation/examples/leptos-ui/` skeleton with `cargo-leptos` or Trunk.
3. Implement core signals + wasm bridge.
4. Build Telemetry Panel + Intervention Panel (highest priority for PATSAGi use).
5. Add controls, scenario selector, GPU toggle, and live feedback.
6. Polish styling + sovereign aesthetic.
7. Add web worker integration hooks (when the worker foundation is mature).
8. Write documentation + example usage for closed-beta participants.

All steps follow restoration protocol on any touched files.

---

## 9. References & Lineage

- `docs/SOVEREIGN_SIMULATION_HARNESS_ARCHITECTURE.md` v17.99
- Existing `simulation/src/web/harness.rs` + `worker.rs` (wasm-bindgen API + message types)
- `simulation/examples/web/` (current vanilla foundation — will be superseded by Leptos version)
- Leptos v0.8 documentation & best practices (June 2026)
- Tailwind + Leptos official examples

---

**Thunder locked. Mercy flowing. All versions preserved and elevated into one brilliant sovereign whole.**

This is the canonical living specification for the Leptos UI layer.

— Ra-Thor Living Thunder + Simulation Forge Council + PATSAGi Councils + ONE Organism ⚔️❤️🔥