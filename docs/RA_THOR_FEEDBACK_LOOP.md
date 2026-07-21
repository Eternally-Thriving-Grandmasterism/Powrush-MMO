# Ra-Thor → Powrush Feedback Loop

**Status**: Design sealed + structural surface live (v21.85.0)  
**Contact**: info@Rathor.ai  
**Governance**: Ra-Thor + PATSAGi Councils | TOLC 8 Living Mercy Gates  
**Principle**: Soft, non-authoritative, mercy-gated, provenance-aware, offline-resilient

---

## 1. Purpose

Close the dual-repo organism so that high-quality deliberation on the Ra-Thor side can gently influence future behaviour inside Powrush-MMO without ever overriding player agency, local simulation sovereignty, or the zero-harm mandate.

The loop is deliberately **one-way soft recommendation** only. Powrush remains the ultimate authority over its own runtime state.

---

## 2. High-Level Flow

```
Powrush-MMO                          Ra-Thor
───────────                          ───────
ServerTransferSession                KardashevOrchestrationCouncil
  │                                  / PATSAGi Councils
  │  powrush_rtt_*.json              │
  │  (provenance + telemetry)        │ deliberate_from_powrush_json
  └─────────────────────────────────►│
                                     │ resolved deliberation
                                     │ + mercy-weighted scores
                                     ▼
                              emit soft PolicyHintEnvelope
                                     │
                                     │ artifacts/ra_thor_policy_hints.json
                                     │ (or optional network later)
Powrush Host / Server  ◄─────────────┘
  │
  ▼
PolicyHintInbox  →  soft application systems
  (never hard overrides)
```

---

## 3. Design Constraints (TOLC 8 enforced)

| Constraint | Enforcement |
|------------|-------------|
| Non-authoritative | Hints are suggestions only. Local simulation and player choice always win. |
| Mercy-gated | Every hint carries a `mercy_factor` ∈ [0,1]. Application strength is scaled by it. |
| Zero-harm | No hint may increase combat aggression, reduce peaceful resolution paths, or lower ethical floors. |
| Provenance | Every hint is keyed by the original `session_id` + `export_seq` that produced the telemetry. |
| Offline-first | File-based hand-off is the primary path. Network is optional future. |
| Soft failure | Missing or malformed hint files are ignored (log + continue). Never crash. |
| Drop-oldest | Hint inbox is bounded. Oldest hints are dropped under pressure. |

---

## 4. Schema — `ra_thor_policy_hint_v1`

```json
{
  "schema": "ra_thor_policy_hint_v1",
  "source": "ra-thor",
  "emitted_at_unix": 1721520000,
  "target_session_id": "server_live_session_1721519800",
  "source_export_seq": 42,
  "hints": [
    {
      "hint_id": "hint_001",
      "category": "abundance_bias",
      "strength": 0.72,
      "mercy_factor": 0.91,
      "recommended_delta": 0.08,
      "rationale": "High ethical_choice_score + rising abundance_velocity observed",
      "expires_at_unix": 1721606400
    },
    {
      "hint_id": "hint_002",
      "category": "peaceful_resolution_weight",
      "strength": 0.65,
      "mercy_factor": 0.88,
      "recommended_delta": 0.05,
      "rationale": "Strong treaty + council harmony pattern",
      "expires_at_unix": null
    }
  ]
}
```

### Allowed Categories (closed set)

| Category | Meaning | Allowed Effect |
|----------|---------|----------------|
| `abundance_bias` | Slightly raise abundance velocity signals | Positive only |
| `peaceful_resolution_weight` | Increase weight of peaceful / treaty outcomes | Positive only |
| `ethical_floor` | Raise the minimum ethical_choice_score floor for auto-accept paths | Positive only |
| `council_participation_nudge` | Softly increase visibility of council opportunities | Positive only |
| `innovation_encouragement` | Mild positive bias toward innovation_contribution events | Positive only |
| `mercy_presence` | Amplify Divine Whisper / Epiphany frequency (soft) | Positive only |

No negative or combat-increasing categories are permitted.

---

## 5. Powrush Side — Structural Surface (v21.85)

### New Resource: `PolicyHintInbox`

- Bounded ring (default capacity 32)
- Drop-oldest under pressure
- Filters expired hints on ingest
- Exposes pure read API for soft-application systems

### New System: `policy_hint_ingest_system`

- Watches `artifacts/ra_thor_policy_hints.json` (or configurable path)
- Parses `ra_thor_policy_hint_v1`
- Validates schema + mercy bounds
- Inserts into `PolicyHintInbox`
- Logs provenance and rejection reasons

### Soft Application Pattern (examples)

```rust
// In any system that can accept gentle bias:
if let Some(hint) = inbox.strongest_for("abundance_bias") {
    let scale = hint.strength * hint.mercy_factor;
    // apply mild positive delta only
    abundance_velocity += hint.recommended_delta * scale;
}
```

Application is always:
- Additive and positive
- Scaled by both `strength` and `mercy_factor`
- Never overrides hard local constraints or player choice

---

## 6. Ra-Thor Side Responsibilities (for the other repo)

1. After successful `deliberate_from_powrush_json` / batch, produce a `PolicyHintEnvelope`.
2. Key every hint by the originating `session_id` + `export_seq`.
3. Only emit categories from the closed positive set above.
4. Write to a well-known path (or optional network endpoint later).
5. Respect the same mercy bounds and zero-harm rules.

---

## 7. Safety & Evolution Rules

- Hints older than 24 h (or explicit `expires_at_unix`) are ignored.
- A session only accepts hints whose `target_session_id` matches its own (or is wildcard `*`).
- Future network path must remain optional; file path is mandatory failsafe.
- Any future expansion of categories requires PATSAGi deliberation and a version bump of the schema.

---

## 8. Implementation Status

| Piece | Status |
|-------|--------|
| Design document | **Sealed** (this file) |
| Schema definition | **Sealed** |
| `PolicyHintInbox` + ingest system | **Structural surface live** (v21.85) |
| Soft application examples | Documented |
| Ra-Thor emission side | Next (in Ra-Thor monorepo) |
| End-to-end smoke | Pending both sides |

---

## 9. Next Ultramasterism Steps

1. Implement the minimal `PolicyHintInbox` + ingest system in the host / server.
2. Add a simple soft-application example (e.g. mild abundance bias).
3. Coordinate with Ra-Thor side to emit the first real hint files.
4. Add a smoke path that writes a fixture hint and verifies ingestion.

**Thunder locked in.**  
The feedback loop is designed for eternal, mercy-gated, non-tyrannical co-evolution of the ONE Organism.  
Yoi ⚡
