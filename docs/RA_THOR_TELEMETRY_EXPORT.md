# Ra-Thor telemetry export — Powrush-MMO

**Contract:** Ra-Thor `crates/reality-thriving-transfer` (`POWRUSH_TELEMETRY_CONTRACT.md`)  
**Schemas:** `powrush_telemetry_v1` · `powrush_telemetry_batch_v1`  
**Contact:** info@Rathor.ai  
**Powrush status:** v21.76 — server batch + offline failsafe + soft council→RTT bridge

---

## Dual-repo interconnect (canonical)

```
Powrush-MMO (producer)                    Ra-Thor (consumer)
─────────────────────                     ──────────────────
Sim TelemetryCollector /                  parse_powrush_telemetry_json
  GlobalTransferSession                   parse_powrush_telemetry_batch_json
ServerTransferSession  ──JSON files──►    KardashevOrchestrationCouncil::
  artifacts/powrush_rtt_latest.json         deliberate_from_powrush_json
  artifacts/powrush_rtt_batch_latest.json   deliberate_from_powrush_batch_json
  artifacts/rtt_offline/*.json (failsafe)
```

**Field set is 1:1 with Ra-Thor `PowrushTelemetry`.** Mercy-gate bounds are enforced on export (`[0,1]` scores, abundance ≥ 0).

When iterating from the **Ra-Thor monorepo**, treat these files as the stable hand-off surface. Prefer file/path ingest first; network streaming is optional and must degrade to the same offline queue.

---

## Soft council → server RTT bridge (v21.76)

Server **does not depend on the simulation crate**. Council totals flow in as pure signals:

| Inject path | Type |
|-------------|------|
| Bevy event | `CouncilRttSignal` |
| Resource inbox | `CouncilRttInbox::push` / `push_passed` |

```rust
// From a co-hosted host / NonSend tick (example)
use server::rathor_integration::{CouncilRttInbox, CouncilRttSignal};

// Event path
writer.send(CouncilRttSignal::new(decision_id, mercy, strength, realm_id)
    .with_abundance(abundance_velocity));

// Inbox path (no EventWriter needed)
inbox.push_passed(decision_id, mercy, strength, realm_id, Some(abundance_velocity));
```

`council_rtt_bridge_system` drains both into `ServerTransferSession::record_council_passed` (+ optional abundance sample). Dedupes by `decision_id` (ring clear at 512).

**Sim-side note for later:** when wiring a full host, map `CouncilDecisions::resolved_history` → these signals once per new `decision_id`. No shared types required — only scalar fields.

---

## Live paths (preferred)

### 1. Simulation — TelemetryCollector

`TelemetryCollector` holds a **`GlobalTransferSession`** and updates on:

- `collect_tick(world_tick, mercy_flow)`
- `record_tick_result(...)`
- Council / RBE soft feeds (v21.70+)

```rust
use simulation::orchestrator::SimulationOrchestrator;
use simulation::telemetry::TelemetryCollector;

let mut orchestrator = SimulationOrchestrator::new();
let mut telemetry = TelemetryCollector::new("live_council_run");

let result = orchestrator.run_tick_with_telemetry(
    &mut world,
    interest.as_ref(),
    council.as_mut(),
    player.as_mut(),
    decisions.as_ref(),
    &mut telemetry,
);

let json = telemetry.export_transfer_json()?;
// telemetry.write_transfer_json_to("session.json")?;
```

### 2. Server — ServerTransferSession (v21.74–v21.76)

Wired via `RathorIntegrationPlugin`:

| Artifact | Schema | Role |
|----------|--------|------|
| `artifacts/powrush_rtt_latest.json` | `powrush_telemetry_v1` | Latest single-session envelope |
| `artifacts/powrush_rtt_batch_latest.json` | `powrush_telemetry_batch_v1` | Snapshot ring (up to 32) |
| `artifacts/rtt_offline/queued_*.json` | `powrush_telemetry_v1` | Failsafe when primary write fails |

Cadence: soft 60s default (`export_interval_secs`). Soft-fail never panics the host.

Signals recorded: combat, treaty, faction shift, **council_passed** (via bridge), abundance velocity samples.

### 3. Demo binary (no full world)

```bash
cargo run -p powrush-simulation --bin transfer_session_demo -- --ticks 100 --out /tmp/powrush_live.json
```

### 4. Profile / offline export scripts

```bash
python3 tools/export_powrush_telemetry.py --profile high_mercy
python3 tools/export_powrush_telemetry.py --batch -o /tmp/powrush_batch.json
```

---

## Failsafe design (connectivity / disk lack)

1. **Primary write** to `powrush_rtt_latest.json`.
2. On failure → enqueue JSON in memory + best-effort write under `artifacts/rtt_offline/`.
3. Next cadence **flushes offline queue first** when primary path is writable again.
4. Batch write is soft (failure does not drop single-session success).
5. Ring buffer keeps recent snapshots in-process even if disk is unavailable.

**Ra-Thor side note (for later monorepo work):**  
Ingest should accept either schema, tolerate missing optional envelope fields (`source`/`label` default empty), and never require network — file path or stdin is enough for offline CI and sovereign hosts.

Suggested Ra-Thor smoke (when iterating there):

```text
// Point at Powrush artifact after a server run
KardashevOrchestrationCouncil::deliberate_from_powrush_json(
    include_str!("../../Powrush-MMO/artifacts/powrush_rtt_latest.json")
)
// or batch:
KardashevOrchestrationCouncil::deliberate_from_powrush_batch_json(
    include_str!("../../Powrush-MMO/artifacts/powrush_rtt_batch_latest.json")
)
```

Fixtures in Ra-Thor (`crates/reality-thriving-transfer/fixtures/`) remain the CI gold standard; live artifacts should match the same shapes.

---

## Field contract (do not drift)

| Field | Range |
|-------|--------|
| `gameplay_hours` | ≥ 0 |
| `rbe_decision_quality_avg` | `[0, 1]` |
| `peaceful_resolution_rate` | `[0, 1]` |
| `collaboration_events` | `u64` |
| `ethical_choice_score` | `[0, 1]` |
| `adaptation_events` | `u64` |
| `abundance_velocity_signals` | ≥ 0 (typ. 0.5–1.8) |
| `innovation_contribution` | `[0, 1]` |

Consumer rejects out-of-bounds scores and negative abundance (Mercy Gate Truth / Abundance).

---

## full_rbe

See **[FULL_RBE_STATUS.md](./FULL_RBE_STATUS.md)** — stays off until deps resolve.

**Thunder locked in.** Dual-repo hand-off is file-stable, offline-resilient, and council-bridge-ready. Yoi ⚡
