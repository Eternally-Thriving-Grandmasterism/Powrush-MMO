# Human Playability — v21.94 Soft Play Stack

**Contact:** info@Rathor.ai  
**Status:** Sealed under PATSAGi + TOLC 8  
**Goal:** Deeper human gameplay without dark patterns, true to RBE / mercy / high-road transfer.

---

## Player journey

```
First Session (H dismiss)
  → Living Practice climates (P / Shift+P)
  → Mercy harvests → Thriving Moments
  → Allocate surplus (R): Flow outward | Steward reserve
  → Abundance Journey Echo (F4) — durable offline
  → Lattice flow share (auto) + peer ingest (F5)
```

| Key | Layer |
|-----|--------|
| **H** | Dismiss first-session guidance |
| **P** / **Shift+P** | Practice strip / dismiss |
| **R** | RBE allocate choice |
| **F2** | My Mercy Journey |
| **F3** | Realm travel |
| **F4** | Abundance Journey Echo |
| **F5** | Soft peer lattice ingest |

---

## Design law (non-negotiable)

| Allowed | Refused |
|---------|---------|
| Soft progressive disclosure | Forced tutorials that block input |
| Voluntary practice & allocate | Streaks / energy bars / FOMO timers |
| Mercy-aligned harvest credit | Extractive leaderboards as core loop |
| Offline-first share envelopes | Gamification that softens TOLC 8 |

---

## Modules

| Layer | Path |
|-------|------|
| First session | `client/src/first_session_guidance.rs` |
| Living Practice | `client/src/living_practice_loop.rs` |
| Thriving Moments | `client/src/thriving_moments.rs` |
| Allocate choice | `client/src/rbe_allocate_choice.rs` |
| Journey Echo + persist | `client/src/abundance_journey_echo.rs` |
| Lattice share + peer | `client/src/lattice_flow_share.rs` |

### Persist / share files

| File | Role |
|------|------|
| `data/powrush_abundance_journey.json` | Durable journey + totals |
| `data/powrush_lattice_flow_share.json` | Own abundance direction |
| `data/powrush_lattice_flow_share_peer.json` | Peer envelope to ingest (F5) |

---

## Standing next (councils)

1. Optional Steam Cloud mirror of journey + share blobs  
2. Thin Ra-Thor ingest adapter for `powrush_lattice_flow_share_v1`  
3. Soft multiplayer presence of peer flow totals (when net path ready)  

**Still refused:** scarcity framing, streaks, energy bars, punitive fail states.

---

**Thunder locked in.** Yoi ⚡
