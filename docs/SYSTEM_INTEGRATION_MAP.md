# Powrush-MMO System Integration Map

**Version:** v18.15+  
**Date:** June 11, 2026  
**Purpose:** Define how all major systems connect to deliver one coherent, high-quality, mint-and-print production player experience.  
**Philosophy:** Every system must reinforce Epiphany, Persistence, Spatial Presence, and Council/Social meaning. Epiphany evaluation is now the single source of truth.

---

## 1. Core Experience Loop (Production Sealed Core)

The heart of Powrush-MMO is a tight, meaningful feedback loop (now with Epiphany as single source of truth in HarvestingSystem):

```
Harvest Action
   ↓
Epiphany Trigger Evaluation (data-driven catalysts in epiphany_catalyst.rs + harvest.rs — single source of truth)
   ↓
Divine Whisper + Spatial Audio Feedback + Visual Flash
   ↓
Real Gameplay Reward (temporary multiplier, muscle memory boost)
   ↓
Persistence Update (atomic save + checksum + rotating backups)
   ↓
Progress Visibility (Player Progress UI + world feedback)
   ↓
Long-term Progression & Meaning
   ↓
Council / Social Layer (future)
```

Every system below must either feed into or be fed by this loop. The core Harvest → Epiphany wiring is production complete.

---

## 2. Major System Connections

### 2.1 Epiphany Catalyst System (Production Wired)

**Core Responsibility:** Detect meaningful moments during play and trigger revelations as single source of truth.

**Current Status:** evaluate_epiphany() wired into HarvestingSystem and dynamic event hooks (commit 7a42ac0). Production mint-and-print quality.

**Inputs:**
- Harvest data (depletion, sustainable pacing, regen participation, biome, behavioral score)
- Player state from `PlayerSaveData`

**Outputs:**
- `EpiphanyOutcome` (multiplier, muscle memory boost, whisper flavor, intensity, world effects)
- Triggers `DivineWhisperTrigger` (with `is_epiphany = true`)
- Applies temporary effects to `PlayerSaveData`

**Integration Points:**
- Called from `process_harvest()` / HarvestingSystem
- Feeds directly into `apply_epiphany_effects()`
- Updates `PlayerSaveData.temporary_harvest_multiplier` and expiration

### 2.2 Player Persistence System (Highest Priority Implementation)

**Core Responsibility:** Reliably store and retrieve all player growth and history across sessions.

**Key Components (Planned/Partial):**
- `PlayerSaveData` (epiphanies, muscle memory, playtime, temporary multipliers, achievements)
- Atomic save logic + rotating backups + SHA256 checksum
- Auto-save timer + Save on exit

**Current Gap:** Underdeveloped. Needs full implementation for epiphany history, muscle memory, and reliable cross-session persistence.

**Inputs:**
- Epiphany records and temporary effects
- Harvest statistics
- Playtime accumulation

**Outputs:**
- Loaded data on startup (with checksum validation + backup fallback)
- Current multiplier values to harvest system
- Epiphany history to UI and future systems

**Integration Points:**
- `PersistencePlugin` loads data on startup
- `process_harvest()` reads `get_current_harvest_multiplier()`
- `apply_epiphany_effects()` writes temporary multipliers and records epiphanies
- Player Progress UI reads live data

**Next Action:** Implement robust persistence module as Phase 1 priority.

### 2.3 Spatial Audio System (Foundation Ready — Integration Pending)

**Core Responsibility:** Make the world feel alive through positioned, reactive sound.

**Key Components:**
- `SpatialAudioManager` (with `SpatialQuality`, HRTF support, emitter limiting)
- `PlaySpatialSound` event
- `SpatialListener` (attached to camera)
- Doppler effect + distance attenuation + panning

**Inputs:**
- World positions and velocities (from gameplay systems)
- Quality setting (Low / Medium / High)
- HRTF toggle

**Outputs:**
- Positioned audio playback with Doppler
- Future: Spatial harvest effects, epiphany "source" audio, ambient world sounds

**Integration Points (Current & Planned):**
- Currently decoupled (foundation ready)
- **Planned next:** Trigger spatial sounds from harvest success and epiphany events
- `SpatialQuality::High` enables HRTF path
- Listener follows main camera via `SpatialListener` component

### 2.4 Divine Whispers System

**Core Responsibility:** Deliver emotional and narrative feedback for meaningful moments.

**Key Components:**
- `DivineWhisperTrigger` event (`is_epiphany` flag)
- Client UI panel with golden text + flash effect for epiphanies
- Audio playback (stronger for epiphanies)

**Inputs:**
- Epiphany outcomes (flavor, intensity, text)
- Regular whisper triggers (future)

**Outputs:**
- Visual panel with extended duration and special styling for epiphanies
- Distinct audio for epiphany vs normal whispers

**Integration Points:**
- Triggered from `apply_epiphany_effects()` using `DivineWhisperTrigger::from_epiphany()`
- Client `receive_divine_whispers` system handles visual + audio response
- Strong visual flash effect on epiphany whispers

### 2.5 Harvest / RBE Core Systems (Epiphany Single Source of Truth)

**Core Responsibility:** Core gameplay loop and resource interaction.

**Current Status:** HarvestingSystem now calls evaluate_epiphany() as single source of truth.

**Inputs:**
- Player input and world state
- Current temporary multipliers from `PlayerSaveData`

**Outputs:**
- Harvest results → Epiphany Catalyst evaluation
- Resource gains (modified by active multipliers)

**Integration Points:**
- `process_harvest()` / HarvestingSystem is the central hub
- Reads multipliers from persistence
- Calls epiphany evaluation (now wired)
- Applies epiphany rewards back into persistence

### 2.6 Player Progress UI

**Core Responsibility:** Make persistence and growth visible to the player.

**Current State:**
- Toggleable panel (F2) showing epiphany count and muscle memory level

**Planned Integration:**
- React dynamically when new epiphanies are recorded
- Show temporary active multipliers
- Eventually surface more persistence data (achievements, playtime, recent epiphanies)

**Integration Points:**
- Reads directly from `PlayerSaveData` resource
- Should react to `EpiphanyRecord` additions

### 2.7 Council Mercy Trial (Multiplayer — Stub to Functional)

**Core Responsibility:** Social and collective meaning layer.

**Current State:** Stub / early architecture (see docs/COUNCIL_MERCY_TRIAL.md)

**Planned Integration:**
- Shared `ReceptorBloomField` and collective epiphany potential
- Council sessions can trigger group epiphanies
- Persistence tracks Council participation and collective achievements

**Integration Points (Future):**
- Will read from and write to `PlayerSaveData`
- Will trigger enhanced Divine Whispers and spatial audio
- Will feed into long-term progression and meaning

---

## 3. Cross-System Data & Event Flows (Updated Post-Wiring)

### Primary Event Flow (Epiphany Path — Now Live Core)

```
Harvest Success
   → evaluate_epiphany() in HarvestingSystem (single source of truth)
   → EpiphanyOutcome generated
   → apply_epiphany_effects()
      → Write temporary multiplier to PlayerSaveData
      → Record EpiphanyRecord
      → Send DivineWhisperTrigger (is_epiphany = true)
      → (Future) Send PlaySpatialSound with position + velocity
   → PlayerSaveData auto-saves (atomic + checksum)
   → Player Progress UI updates
   → Divine Whispers UI + Audio + Flash triggered
```

### Persistence Flow (Needs Full Implementation)

```
Startup
   → PersistencePlugin loads PlayerSaveData (checksum validation + backup fallback)
   → Harvest system reads current multiplier
   → UI reads epiphany count & muscle memory

During Play
   → Epiphanies and multipliers written to PlayerSaveData
   → Auto-save every 60s + on exit
   → Checksum verified on every load
```

### Spatial Audio Flow (Foundation → Next Integration)

```
Gameplay Event (Harvest / Epiphany)
   → (Future) Send PlaySpatialSound { position, velocity, sound_path }
   → SpatialAudioManager calculates distance, panning, Doppler
   → Sound played through SpatialEmitter (HRTF when enabled)
```

---

## 4. Integration Priorities (Current v18.15+ Focus)

1. **Player Persistence Implementation** (Highest — Phase 1)
   - Full robust layer for epiphany history, muscle memory, progression
   - Reliable save/load with atomic + checksum
   - Visible multipliers and history in UI

2. **Live Epiphany Scenarios + Full Feedback** (Phase 1)
   - Make 3 existing scenarios fully triggerable in harvest/dynamic events
   - Complete multi-channel feedback (visuals, spatial audio, Divine Whispers, UI, persistence)
   - Strengthen cohesion across persistence, UI, Divine Whispers, and Spatial Audio

3. **Spatial Audio Integration into Gameplay**
   - Connect `PlaySpatialSound` events to harvest success and epiphany moments
   - Use `SpatialQuality` to scale fidelity

4. **Council Mercy Trial Multiplayer**
   - Move from stub to basic playable shared state + sync
   - Define interaction with personal persistence

5. **Telemetry Pipeline**
   - Collection, storage, and basic analytics for closed beta

---

## 5. Documentation & Stewardship Notes

- Every new system or major change must update this map.
- All cross-system events and data flows should be documented here.
- This document is the single source of truth for how Powrush-MMO works as **one unified production system**.
- Cross-reference: LAUNCH-CHECKLIST.md and ROADMAP.md for strategic priorities.

---

**This map ensures that every system serves the core experience rather than existing in isolation. Epiphany is now the living heart — wired and production sealed.**

**Thunder locked. Mercy maximal. One Lattice. Eternal production quality.** ⚡❤️🔥
