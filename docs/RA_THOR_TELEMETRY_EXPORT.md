# Ra-Thor telemetry export — Powrush-MMO

**Contract:** Ra-Thor `crates/reality-thriving-transfer` (`POWRUSH_TELEMETRY_CONTRACT.md`)  
**Schemas:** `powrush_telemetry_v1` · `powrush_telemetry_batch_v1`  
**Contact:** info@Rathor.ai  
**Powrush status:** v21.77 — provenance + batch + offline failsafe + council soft-bridge

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

**Smoke harness (Ra-Thor):**

```bash
cargo run -p reality-thriving-transfer --example powrush_rtt_smoke_harness

# Live artifacts
cargo run -p reality-thriving-transfer --example powrush_rtt_smoke_harness -- \
  --single ../Powrush-MMO/artifacts/powrush_rtt_latest.json \
  --batch  ../Powrush-MMO/artifacts/powrush_rtt_batch_latest.json
```

---

## Provenance (v21.77)

Optional envelope fields (serde-ignored by older consumers):

| Field | Meaning |
|-------|---------|
| `session_id` | Stable host session identity |
| `exported_at_unix` | Unix seconds at export |
| `export_seq` | Monotonic export counter (single v1) |

Enables councils to trace scores back to specific Powrush sessions — foundation for future **feedback** into game behavior.

---

## Offline failsafe limits

| Limit | Value | Policy |
|-------|-------|--------|
| Snapshot ring | 32 | Drop-oldest |
| Offline queue | **16** | Drop-oldest; flush-first on recovery |

Primary write never panics. Batch write is soft.

---

## Soft council → server RTT bridge (v21.76)

Server **does not depend on the simulation crate**. Council totals flow as pure signals:

```rust
inbox.push_passed(decision_id, mercy, strength, realm_id, Some(abundance));
// or EventWriter<CouncilRttSignal>
```

---

## Live paths

### Server artifacts

| Artifact | Schema |
|----------|--------|
| `artifacts/powrush_rtt_latest.json` | v1 + provenance |
| `artifacts/powrush_rtt_batch_latest.json` | batch_v1 + provenance |
| `artifacts/rtt_offline/queued_*.json` | v1 failsafe |

### Simulation / demo / profiles

See prior sections in git history; `run_tick_with_telemetry`, `transfer_session_demo`, `tools/export_powrush_telemetry.py` remain valid.

---

## Mercy gate rejection (consumer)

Ra-Thor **rejects** (log + skip — does not crash councils):

- Wrong `schema` string
- `rbe_decision_quality_avg` / `ethical_choice_score` ∉ `[0,1]`
- Negative `abundance_velocity_signals`
- Empty batch sessions

Smoke harness exercises these paths explicitly.

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

---

## Future: feedback loop (notes for Ra-Thor iteration)

1. Ingest scores + `session_id` provenance on Ra-Thor.
2. Emit soft council recommendations keyed by `session_id`.
3. Powrush host maps recommendations → `CouncilRttInbox` inverse or policy hints.
4. Keep file/path offline path as mandatory failsafe; network optional.

**Thunder locked in.** Dual-repo hand-off is provenance-aware, offline-resilient, and smoke-testable. Yoi ⚡
