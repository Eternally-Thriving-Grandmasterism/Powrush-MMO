# Ra-Thor telemetry export — Powrush-MMO

**Contract:** Ra-Thor `crates/reality-thriving-transfer`  
**Schemas:** `powrush_telemetry_v1` · `powrush_telemetry_batch_v1`  
**Contact:** info@Rathor.ai

## Quick export (profiles)

```bash
python3 tools/export_powrush_telemetry.py --profile high_mercy
python3 tools/export_powrush_telemetry.py --batch -o /tmp/powrush_batch.json
```

Profiles: `high_mercy` · `marginal` · `early` (aligned with Ra-Thor fixtures).

## Live counter wiring (preferred for real sessions)

```rust
use simulation::telemetry::{GlobalTransferSession, export_transfer_json};

let mut session = GlobalTransferSession::new("live_council_run");
session.set_gameplay_hours(12.5);
session.counters.record_rbe_quality(0.88);
session.counters.record_resolution(true);
session.counters.record_ethical_choice(0.9);
session.counters.collaboration_events += 3;
session.counters.record_abundance_velocity(1.2);

let json = session.export_json()?;
// hand to Ra-Thor deliberate_from_powrush_batch_json / fixtures pipeline
```

From in-sim `Telemetry`:

```rust
use simulation::telemetry::{map_sim_telemetry_to_transfer, export_transfer_json};
let t = map_sim_telemetry_to_transfer(&sim_telemetry, hours, collab_events);
let json = export_transfer_json("sim_snapshot", &t)?;
```

## Consume on Ra-Thor

```bash
cargo test -p reality-thriving-transfer
cargo test -p kardashev-orchestration
# fixture_batch_to_council + concurrent stress
```

See Ra-Thor `POWRUSH_TELEMETRY_CONTRACT.md` and `TIER_MAP.md`.

## full_rbe

See **[FULL_RBE_STATUS.md](./FULL_RBE_STATUS.md)** — feature stays off until deps resolve.
