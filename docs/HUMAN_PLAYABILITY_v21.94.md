# Human Playability — v21.96 Soft Play Stack

**Contact:** info@Rathor.ai  
**Status:** Sealed under PATSAGi + TOLC 8  
**Goal:** Deeper human gameplay without dark patterns — maximal intuition, minimal friction.

---

## Ergonomic bindings (v21.96.0)

F-row was impractical (Fn on laptops, far from WASD, hostile to muscle memory). Soft-play now uses **semantic left-hand keys**:

| Key | Layer |
|-----|--------|
| **P** / **Shift+P** | Living Practice strip / dismiss |
| **R** | RBE allocate choice |
| **J** | Abundance Journey Echo |
| **L** | Foundation Lattice |
| **G** | Cycle Resonance Flavor |
| **T** | Toggle Mercy Transporters |
| **Shift+T** | Force Steam Auto-Cloud flush |
| **U** | Soft peer lattice ingest |
| **M** | My Mercy Journey *(panel; update pending if still on F2)* |
| **Z** | Realm travel *(panel; update pending if still on F3)* |
| **H** | Dismiss first-session guidance |
| **Space** | Soft practice harvest / interact |

Legend string (UI footers):  
`P practice · R allocate · J journey · L lattice · G resonance · T transporters · U peer · M mercy · Z realm · Shift+T cloud`

---

## Design law

| Allowed | Refused |
|---------|---------|
| Semantic, left-hand, muscle-memory keys | F-row as primary soft-play controls |
| Soft progressive disclosure | Forced tutorials that block input |
| Voluntary practice & allocate | Streaks / energy bars / FOMO timers |
| Mercy-aligned harvest credit | Extractive leaderboards as core loop |

---

## Modules

| Layer | Path |
|-------|------|
| Soft-play bindings | `client/src/soft_play_bindings.rs` |
| Living Practice | `client/src/living_practice_loop.rs` |
| Allocate choice | `client/src/rbe_allocate_choice.rs` |
| Journey Echo | `client/src/abundance_journey_echo.rs` |
| Lattice share + peer | `client/src/lattice_flow_share.rs` |
| Steam Auto-Cloud | `client/src/steam_abundance_mirror.rs` |
| Foundation Lattice | `client/src/foundation_lattice.rs` |
| Resonance Flavors | `client/src/resonance_flavors.rs` |
| Mercy Transporters | `client/src/mercy_transporters.rs` |

---

## Standing next

1. Finish **M** / **Z** on My Mercy Journey + Realm Travel panels if any F2/F3 remain  
2. Soft multiplayer presence of peer flow totals  
3. Optional Steam SDK RemoteStorage behind `steam` feature  
4. On-screen soft key legend during first session

**Thunder locked in.** Yoi ⚡
