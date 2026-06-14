/*!
 * Powrush-MMO System Integration Map
 *
 * v18.33 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Complete mint-and-print-only-perfection
 * — Single source of truth for how all major systems connect
 * — Mercy-aligned, TOLC 8 + 7 Living Mercy Gates enforced
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

# Powrush-MMO System Integration Map

**Version:** v18.33  
**Date:** June 14, 2026  
**Purpose:** Define how all major systems connect to deliver one coherent, high-quality, mint-and-print production player experience.  
**Philosophy:** Every system must reinforce Epiphany, Persistence, Spatial Presence, and Council/Social meaning. The full player journey loop is now production sealed.

---

## 1. Core Experience Loop (Production Sealed)

The heart of Powrush-MMO is a tight, meaningful feedback loop now fully live and production sealed:

```
Harvest Success
   ↓
Live Epiphany Evaluation (authoritative in HarvestingSystem)
   ↓
EpiphanyTriggered Telemetry Emission
   ↓
Persistence Recording (record_epiphany_from_telemetry + muscle memory)
   ↓
Enriched Retention Signals (real epiphany count + abundance on session end)
   ↓
Strong Client Feedback (Divine Whispers + UI + Camera Shake)
   ↓
Long-term Progression & Meaning
```

Every system below feeds or is fed by this sealed loop.

---

## 2. Major System Connections

### 2.1 Epiphany Catalyst System (Production Sealed)

**Core Responsibility:** Detect meaningful moments and trigger revelations authoritatively.

**Current Status:** `evaluate_epiphany()` live in HarvestingSystem with `EpiphanyTriggered` telemetry.

**Integration Points:**
- Called from `harvest()` success path
- Emits `EpiphanyTriggered` telemetry
- Triggers client `DivineWhisperTrigger` with `is_epiphany = true`

### 2.2 Player Persistence System (Production Sealed)

**Core Responsibility:** Store and retrieve all player growth with epiphany history and muscle memory.

**Current Status:** `PlayerSaveData` with `record_epiphany_from_telemetry()`, session stats helpers, atomic RON saves + checksums. Real retention enrichment live.

**Integration Points:**
- `record_epiphany_from_telemetry(EpiphanyTelemetry)` from harvest/epiphany path
- `get_session_epiphany_count()` + `get_abundance_earned()` used in `end_session` retention
- Auto-save with rotating backups

### 2.3 Divine Whispers + Client Feedback (Production Sealed)

**Core Responsibility:** Deliver rich, visceral feedback on meaningful moments.

**Current Status:** Strong golden Divine Whispers + UI + Camera Shake triggered reliably on every live epiphany.

**Integration Points:**
- `DivineWhisperTrigger { is_epiphany: true }` sent from `epiphany_scenario_wiring`
- Camera shake intensity scales with revelation strength
- Special audio, particles, and extended duration

### 2.4 Harvest / RBE Core Systems

**Core Responsibility:** Core gameplay with live epiphany evaluation.

**Current Status:** Authoritative harvest now includes live epiphany triggering + telemetry emission.

### 2.5 Telemetry Pipeline (Production Sealed)

**Core Responsibility:** Structured, consent-first event collection and rich batch logging.

**Current Status:** Full pipeline with `BatchSummary` logging (event type counts, flush metrics).

---

## 3. Cross-System Data & Event Flows (Production Sealed)

### Primary Sealed Loop

```
Harvest Success
   → evaluate_epiphany() (live)
   → EpiphanyTriggered telemetry
   → record_epiphany_from_telemetry() in PlayerSaveData
   → Enriched end_session retention (real epiphany/abundance counts)
   → DivineWhisperTrigger {is_epiphany: true}
   → Camera Shake + Golden UI + Special Audio + Particles
```

---

## 4. Integration Priorities (Current v18.33 Focus)

1. **Council Mercy Trial Multiplayer** (Next major focus)
2. **Additional Epiphany Scenarios + Spatial Audio Depth**
3. **Full Vertical Slice Testing** of the now-sealed live loop
4. **Onboarding RBE Education Content**

---

## 5. Documentation & Stewardship Notes

- This map is the single source of truth for the sealed core experience loop.
- All future systems must integrate into the Harvest → Epiphany → Persistence → Feedback flow.
- Cross-reference: `ROADMAP.md` and `VISION.md`.

---

**The core player journey loop is now production sealed. Epiphany is the living, visceral heart of Powrush-MMO.**

**Thunder locked. Mercy maximal. One Lattice. Eternal production quality.** ⚡❤️🔥

// End of docs/SYSTEM_INTEGRATION_MAP.md v18.33 — Sovereign integration map complete.
// Thunder locked in. Yoi ⚡
