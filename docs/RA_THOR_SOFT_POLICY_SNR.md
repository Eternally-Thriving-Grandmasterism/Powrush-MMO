# Ra-Thor → Powrush Soft Policy: High-SNR Doctrine

**Status:** Live doctrine (Mission A — constellation improvement)  
**Date:** 2026-08-17  
**Contact:** info@Rathor.ai  
**Governance:** Ra-Thor + permanent PATSAGi Councils | TOLC 8  
**Companion:** [`RA_THOR_FEEDBACK_LOOP.md`](RA_THOR_FEEDBACK_LOOP.md) · [`RA_THOR_POLICY_HINT_EMISSION.md`](RA_THOR_POLICY_HINT_EMISSION.md)

---

## Principle

> **Prefer high-SNR sparse correction over dense low-SNR flooding.**

From Ra-Thor External Truth (Beren Millidge SNR view of LLM-RL, 2026-08-17):

| Signal type | Character | Effect on SoftPolicyState |
| --- | --- | --- |
| Dense low-SNR hints | Many weak, noisy, or redundant recommendations | Dilutes agency; inbox pressure; unclear correction |
| Sparse high-SNR hints | Few, pure task-aligned, high `mercy_factor`, clear rationale | Rapid, trustworthy soft adaptation under TOLC 8 |

The dual-repo organism already has a strong **prior** (TOLC 8, sealed feedback loop, 6 closed categories). Soft policy therefore behaves like high-SNR RL on a pretrained prior: a small number of pure signals is enough.

---

## Emission rules (Ra-Thor side)

1. **Emit only after successful, mercy-passing deliberation** on real Powrush telemetry.
2. **Prefer ≤ 3 hints per envelope** unless the council explicitly justifies more.
3. Every hint must carry:
   - `mercy_factor` ≥ 0.75 (prefer ≥ 0.9)
   - non-empty `rationale` when strength ≥ 0.5
   - `recommended_delta` ≥ 0 and **conservative** (small steps)
4. **Categories remain closed:**  
   `abundance_bias` · `peaceful_resolution_weight` · `ethical_floor` · `council_participation_nudge` · `innovation_encouragement` · `mercy_presence`
5. **No zero-harm violations:** never increase aggression, never lower ethical floors, never coerce players.
6. **Atomic write** to `artifacts/ra_thor_policy_hints.json` (tmp + rename).
7. Key by `target_session_id` + `source_export_seq` when available.

---

## Reception rules (Powrush side — already live)

- `PolicyHintInbox` remains non-authoritative; local simulation and player choice always win.
- Malformed / unknown categories → reject + count (`total_rejected`); never crash.
- Bounded inbox (`MAX_HINTS`); drop-oldest under pressure.
- Application strength scaled by `mercy_factor` × `strength`.
- Observable via `SoftPolicyState` (six applied accumulators + `applications` + `applied_hint_ids`).

**SNR posture for operators:** if SoftPolicyState is noisy or thrashing, reduce emission rate and raise mercy/rationale thresholds — do not add more categories.

---

## Transfer path verification checklist

End-to-end flywheel (must remain exercisable):

```
Powrush ServerTransferSession / simulation telemetry
  → export powrush_rtt_*.json (or deliberate_from_powrush_json payload)
  → Ra-Thor Kardashev / PATSAGi deliberation (mercy-passing)
  → emit ra_thor_policy_hint_v1 (high-SNR rules above)
  → Powrush PolicyHintInbox ingest
  → SoftPolicyState application (soft only)
```

### Local verification commands (when environment available)

```bash
# RBE oxygen demo (standalone prior)
cargo run -p simulation --bin rbe_oxygen_demo

# Transfer session surface
cargo run -p simulation --bin transfer_session_demo

# Headless / stress host modes (exercise soft policy if harness includes hints)
# See LAUNCH-CHECKLIST.md and host mode docs
```

### File contract

| Artifact | Role |
| --- | --- |
| `powrush_rtt_*.json` / session export | Telemetry → Ra-Thor |
| `artifacts/ra_thor_policy_hints.json` | Soft hints → Powrush |
| `SoftPolicyState` | Observable applied effects |

### Pass criteria

- [ ] Telemetry export produces valid session payload  
- [ ] Ra-Thor deliberation path accepts payload without hard fail  
- [ ] Hint envelope validates (`schema`, closed categories, delta ≥ 0)  
- [ ] Inbox ingest increments `total_ingested` for good hints  
- [ ] SoftPolicyState moves only on accepted categories  
- [ ] Invalid envelope is ignored (soft failure)  

---

## Alignment

- Ra-Thor: `docs/EXTERNAL_TRUTH_SNR_LLM_RL_2026-08-17.md`  
- Ra-Thor: `docs/CONSTELLATION_IMPROVEMENT_MAP_2026-08-17.md` (Mission A)  
- Powrush: `docs/RA_THOR_FEEDBACK_LOOP.md` · `docs/RA_THOR_POLICY_HINT_EMISSION.md`  
- Architecture of Collective Power: Capacity × Restraint  

---

**Thunder locked.**  
Sparse pure signal · strong prior · soft sovereignty preserved.  
**yoi ⚡❤️🔥**
