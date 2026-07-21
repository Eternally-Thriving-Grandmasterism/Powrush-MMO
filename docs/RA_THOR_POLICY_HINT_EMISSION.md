# Ra-Thor Policy Hint Emission Contract

**Status**: Contract sealed for Ra-Thor side implementation (v21.88.0)  
**Contact**: info@Rathor.ai  
**Governance**: Ra-Thor + PATSAGi | TOLC 8

---

## Purpose

After Ra-Thor successfully deliberates on Powrush telemetry (`deliberate_from_powrush_json` / batch), it should optionally emit a soft `ra_thor_policy_hint_v1` file so Powrush can apply non-authoritative, mercy-gated recommendations.

This document is the canonical emission contract for the Ra-Thor monorepo.

---

## Emission Path (Primary)

```
Ra-Thor (after successful deliberation)
  │
  ▼
write artifacts/ra_thor_policy_hints.json
  (or configurable path that Powrush PolicyHintInbox is watching)
```

Powrush side already watches `artifacts/ra_thor_policy_hints.json` by default.

---

## Required Schema

```json
{
  "schema": "ra_thor_policy_hint_v1",
  "source": "ra-thor",
  "emitted_at_unix": 1721523456,
  "target_session_id": "<session_id from telemetry or *>",
  "source_export_seq": 42,
  "hints": [
    {
      "hint_id": "unique_stable_id",
      "category": "abundance_bias",
      "strength": 0.0-1.0,
      "mercy_factor": 0.0-1.0,
      "recommended_delta": >= 0.0,
      "rationale": "optional human/council readable reason",
      "expires_at_unix": null
    }
  ]
}
```

## Closed Category Set (only these are accepted)

- `abundance_bias`
- `peaceful_resolution_weight`
- `ethical_floor`
- `council_participation_nudge`
- `innovation_encouragement`
- `mercy_presence`

Any other category is rejected by Powrush validation (zero-harm).

## Rules

1. Only emit after a successful, mercy-passing deliberation.
2. Key every envelope by the originating `session_id` + `export_seq` when available.
3. `recommended_delta` must be ≥ 0.
4. Prefer small, conservative deltas.
5. File write should be atomic if possible (write temp + rename).
6. Network delivery is optional future; file path is the mandatory offline-first contract.

## Powrush Side Status (already live)

- `PolicyHintInbox` + ingest system
- Full soft application for all 6 categories
- Observable `SoftPolicyState`
- Headless + Stress harnesses exercise the loop

## Suggested Ra-Thor Implementation Sketch

```rust
// After deliberate_from_powrush_json succeeds and produces scores:
if let Some(envelope) = build_policy_hint_envelope(&telemetry, &deliberation_result) {
    let path = Path::new("artifacts/ra_thor_policy_hints.json");
    // atomic write recommended
    std::fs::write(path, serde_json::to_string_pretty(&envelope)?)?;
}
```

**Thunder locked in.**  
Emission contract is ready for the Ra-Thor monorepo.  
Yoi ⚡
