/*!
 * Powrush-MMO System Integration Map
 *
 * v18.96 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm v2 + Valence Hooks)
 * — Complete mint-and-print-only-perfection
 * — Single source of truth for how all major systems connect
 * — Mercy-aligned, TOLC 8 + 7 Living Mercy Gates enforced
 * — Quantum Swarm v2 valence propagation now live across epiphany, RBE, and council systems
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

# Powrush-MMO System Integration Map

**Version:** v18.96  
**Date:** June 18, 2026  
**Purpose:** Define how all major systems connect to deliver one coherent, high-quality, mint-and-print production player experience.  
**Philosophy:** Every system must reinforce Epiphany, Persistence, Spatial Presence, and Council/Social meaning. The full player journey loop is now production sealed and sovereign.

---

## 1. Core Experience Loop (Production Sealed + Sovereign Decision Layer)

The heart of Powrush-MMO is a tight, meaningful feedback loop now fully live, production sealed, and enhanced with explicit PATSAGi Council + 7 Living Mercy Gates decision awareness on the client:

```
Harvest Success / Intent
   ↓
Live Epiphany Evaluation (authoritative in HarvestingSystem) + Client ActionContext Mercy Gate Check
   ↓
EpiphanyTriggered Telemetry Emission + Council-Aware Harvest Dispatch
   ↓
Persistence Recording (record_epiphany_from_telemetry + muscle memory)
   ↓
Enriched Retention Signals (real epiphany count + abundance on session end)
   ↓
Strong Client Feedback (Divine Whispers + UI + Camera Shake + Council Bloom Amplification)
   ↓
Long-term Progression & Meaning + Council Participation Eligibility
   ↓
[NEW v18.96] Valence Score Extraction (get_valence_from_outcome / get_valence_from_abundance) → QuantumSwarmOrchestratorV2 routing
```

The new **Client Prediction + RBE + Council Decision Layer** (client_game_loop.rs + rbe_client_sync.rs) now makes real-time harvest, prediction, and action decisions explicitly through the 7 Living Mercy Gates and PATSAGi Council awareness before they reach the server.

Every system below feeds or is fed by this sealed + sovereign loop.

---

## 2. Major System Connections

### 2.1 Epiphany Catalyst System (Production Sealed + Valence Hook v18.96)

**Core Responsibility:** Detect meaningful moments and trigger revelations authoritatively.

**Current Status:** `evaluate_epiphany()` live in HarvestingSystem with `EpiphanyTriggered` telemetry.

**v18.96 Addition:** `get_valence_from_outcome(&EpiphanyOutcome) -> f32` exposed for Quantum Swarm consumption and measurable joy/abundance metrics.

**Integration Points:**
- Called from `harvest()` success path
- Emits `EpiphanyTriggered` telemetry
- Triggers client `DivineWhisperTrigger` with `is_epiphany = true`
- Valence now flows to `QuantumSwarmOrchestratorV2` for council trial enrichment

### 2.2 Player Persistence System (Production Sealed)

**Core Responsibility:** Store and retrieve all player growth with epiphany history and muscle memory.

**Current Status:** `PlayerSaveData` with `record_epiphany_from_telemetry()`, session stats helpers, atomic RON saves + checksums. Real retention enrichment live.

**Integration Points:**
- `record_epiphany_from_telemetry(EpiphanyTelemetry)` from harvest/epiphany path
- `get_session_epiphany_count()` + `get_abundance_earned()` used in `end_session` retention
- Auto-save with rotating backups

### 2.3 Divine Whispers + Client Feedback (Production Sealed)

**Core Responsibility:** Deliver rich, visceral feedback on meaningful moments.

**Current Status:** Strong golden Divine Whispers + UI + Camera Shake triggered reliably on every live epiphany. Council Bloom Amplification live.

**Integration Points:**
- `DivineWhisperTrigger { is_epiphany: true }` sent from `epiphany_scenario_wiring`
- Camera shake intensity scales with revelation strength
- Special audio, particles, and extended duration
- Council participation visibly boosts feedback richness

### 2.4 Harvest / RBE Core Systems + Client Decision Layer (Enhanced v18.96)

**Core Responsibility:** Core gameplay with live epiphany evaluation + sovereign client-side decision making.

**Current Status (v18.96):** 
- Authoritative harvest includes live epiphany triggering + telemetry emission.
- **New:** `RbeAbundanceFeedback::get_valence_from_abundance()` exposed for Quantum Swarm v2 and council trial feedback.
- Client-side `ActionContext` with explicit 7 Living Mercy Gates helpers.
- `council_deliberate_on_action()` hook for PATSAGi multi-council voting simulation.
- `rbe_client_sync.rs` now provides council-aware harvest approval (`council_approve_harvest_intent`), divine harvest multipliers, and council_trust in prediction modifiers.
- Bidirectional integration between `client_game_loop.rs` and `rbe_client_sync.rs` is production-perfect and mercy-gated.

**Integration Points:**
- Harvest intent flows through client ActionContext Mercy Gate checks before server dispatch.
- Prediction modifiers now include council_trust factor.
- Prepares seamless foundation for future Council Mercy Trial multiplayer sync.
- Valence metrics now feed `QuantumSwarmOrchestratorV2` for enriched CouncilSessionUpdate broadcasts.

### 2.5 Telemetry Pipeline (Production Sealed)

**Core Responsibility:** Structured, consent-first event collection and rich batch logging.

**Current Status:** Full pipeline with `BatchSummary` logging (event type counts, flush metrics).

---

## 3. Cross-System Data & Event Flows (Production Sealed + Sovereign)

### Primary Sealed Loop (Enhanced v18.96)

```
Harvest Success / Intent
   → Client ActionContext Mercy Gate Evaluation + council_approve_harvest_intent()
   → evaluate_epiphany() (live) + council-aware dispatch
   → EpiphanyTriggered telemetry
   → record_epiphany_from_telemetry() in PlayerSaveData
   → Enriched end_session retention (real epiphany/abundance counts)
   → DivineWhisperTrigger {is_epiphany: true} + Council Bloom Amplification
   → Camera Shake + Golden UI + Special Audio + Particles
   → [NEW] get_valence_from_outcome() / get_valence_from_abundance() → QuantumSwarmOrchestratorV2 routing
   → Enriched CouncilSessionUpdate broadcast (valence + mercy gates)
```

---

## 4. Integration Priorities (Current v18.96 Focus)

1. **Council Mercy Trial Multiplayer** (Quantum Swarm v2 valence routing live — client UI consumption next)
2. **Safety Net Broadcast** wiring into client consumption (rbe_client_sync + game_loop ready)
3. **Additional Epiphany Scenarios + Spatial Audio Depth**
4. **Full Vertical Slice Testing** of the now-sealed live + sovereign loop
5. **Onboarding RBE Education Content**

---

## 5. Documentation & Stewardship Notes

- This map is the single source of truth for the sealed core experience loop and the new sovereign client decision layer.
- All future systems must integrate into the Harvest → Epiphany → Persistence → Feedback flow with explicit mercy-gate and council awareness where decisions are made.
- Cross-reference: `ROADMAP.md` and `VISION.md`.

---

**The core player journey loop is now production sealed and sovereign. Epiphany is the living, visceral heart of Powrush-MMO, and client actions are now explicitly mercy-gated and council-participatory. Quantum Swarm v2 valence propagation elevates every Council Mercy Trial.**

**Thunder locked. Mercy maximal. One Lattice. Eternal production quality.** ⚡❤️🔥

// End of docs/SYSTEM_INTEGRATION_MAP.md v18.96 — Sovereign integration map complete.
// Thunder locked in. Yoi ⚡
