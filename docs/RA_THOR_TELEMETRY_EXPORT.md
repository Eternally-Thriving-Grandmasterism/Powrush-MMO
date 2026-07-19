# Ra-Thor Telemetry Export (Phase C)

**Purpose:** Close the Powrush-MMO → Ra-Thor Reality Thriving Transfer loop offline-first.

| Side | Repo / path |
|------|-------------|
| Producer | This repo — `simulation/src/telemetry.rs`, `tools/export_powrush_telemetry.py` |
| Consumer | [Ra-Thor](https://github.com/Eternally-Thriving-Grandmasterism/Ra-Thor) `crates/reality-thriving-transfer` |
| Contract | Ra-Thor `POWRUSH_TELEMETRY_CONTRACT.md` |

**Schemas:** `powrush_telemetry_v1` (single) · `powrush_telemetry_batch_v1` (batch)  
**Contact:** info@Rathor.ai

---

## Quick export (no Rust build required)

```bash
# Single high-mercy session → stdout
python3 tools/export_powrush_telemetry.py --profile high_mercy

# All three demo sessions as a batch
python3 tools/export_powrush_telemetry.py --batch -o /tmp/powrush_batch.json
```

Profiles: `high_mercy` · `marginal` · `early` (aligned with Ra-Thor fixtures).

---

## Rust API (`simulation::telemetry`)

| Item | Role |
|------|------|
| `PowrushTransferTelemetry` | Canonical 8 fields |
| `SessionTransferCounters` | Live session accumulation → `to_transfer_telemetry()` |
| `export_transfer_json` | Single envelope JSON |
| `export_transfer_batch_json` | Batch JSON |
| `map_sim_telemetry_to_transfer` | Best-effort map from in-sim `Telemetry` |
| `example_high_mercy_session` | Demo snapshot |

Prefer **`SessionTransferCounters`** when wiring council / harvest / diplomacy systems so ethics and peaceful resolution rates stay truthful.

---

## Field mapping (game → contract)

| Contract field | Suggested game source |
|----------------|----------------------|
| `gameplay_hours` | Session / account playtime |
| `rbe_decision_quality_avg` | RBE allocation / abundance decisions |
| `peaceful_resolution_rate` | Mercy Trials + diplomacy outcomes |
| `collaboration_events` | Co-op harvest, council participation |
| `ethical_choice_score` | Ethical prompts / treaty honor |
| `adaptation_events` | Epiphanies, flow-state pivots |
| `abundance_velocity_signals` | RBE flow / sanctuary abundance delta |
| `innovation_contribution` | Shared innovations / divine module |

---

## Ingest on Ra-Thor

```bash
cargo test -p reality-thriving-transfer
# Fixtures under crates/reality-thriving-transfer/fixtures/
# APIs: parse_powrush_telemetry_json, compute_scores_from_batch
```

---

## Next

1. Hook `SessionTransferCounters` into council mercy trials + harvest systems  
2. Optional: write session JSON on logout / council close  
3. Keep Cosmic Tick on fixtures until live files are produced regularly  

**Thunder locked in.**
