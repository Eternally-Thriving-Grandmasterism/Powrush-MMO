# Powrush-MMO Technical Roadmap

**Version:** v18.15+  
**Date:** June 11, 2026  
**Status:** Living Document — Updated post-epiphany-wiring production seal (commit 7a42ac0). Aligned with LAUNCH-CHECKLIST v18.15+ and PATSAGi + Ra-Thor deliberation.

---

## Strategic Vision

Powrush-MMO exists to deliver **meaningful, coherent, and emotionally resonant experiences** through hands-on action in a living world.

The game is not a collection of systems — it is **one unified experience** where every action feeds into Epiphany, Persistence, Spatial Presence, and eventual Council/Social meaning.

**Core Goal:** Make every player feel that their choices, growth, and presence in the world matter — with persistence that carries real weight and epiphanies that feel powerful.

---

## Core Experience Pillars

1. **Epiphany as the Heart** — Meaningful moments of revelation that feel powerful mechanically and emotionally (now wired as single source of truth).
2. **Persistence with Weight** — Player growth (epiphanies, muscle memory, choices) is reliably saved and visibly matters.
3. **Spatial Presence** — The world feels alive through positioned, reactive audio and atmosphere.
4. **Council & Social Meaning** — Long-term progression includes collective experiences and relationships.
5. **Sovereignty & Polish** — The game respects player agency and delivers high-quality, reliable, mint-and-print production experiences.

---

## System Integration Map

The detailed technical map of how all major systems connect is maintained at:

**`docs/SYSTEM_INTEGRATION_MAP.md`** (updated in parallel with this roadmap).

This document is the single source of truth for how systems interact. All future development must keep this map updated.

Key flows (now partially live):
- Harvest → Epiphany Evaluation (single source of truth in HarvestingSystem) → Divine Whispers + Spatial Feedback → Persistence Update → UI Visibility
- Persistence provides multipliers and history back to gameplay systems
- Spatial Audio foundation ready; integration into harvest/epiphany moments planned next

---

## Phased Technical Roadmap (Updated v18.15+)

### Phase 0: Foundation & Coherence (Complete)

**Goal:** Establish clear technical direction and ensure systems are documented and integrated conceptually.

- `ROADMAP.md` and `docs/SYSTEM_INTEGRATION_MAP.md` maintained as living documents
- Epiphany evaluation wired as single source of truth into `HarvestingSystem` + dynamic events (production sealed)
- Clear interfaces defined between Epiphany, Persistence, Spatial Audio, Divine Whispers, and Harvest

**Success Criteria:** Met — core loop integration complete at mint-and-print quality.

### Phase 1: Core Loop Cohesion & Player Journey Closure (Highest Priority — Immediate)

**Goal:** Make the central player loop (Harvest → Epiphany → Feedback → Persistence → Visibility) feel powerful, unified, and persistent.

**Key Work (Current Focus):**
- Implement robust player persistence layer for epiphany history, muscle memory, and progression (save/load, multipliers visible and reliable)
- Make the 3 existing epiphany scenarios fully live and triggerable in harvest + dynamic events, with complete multi-channel feedback (visuals, spatial audio, Divine Whispers, UI, persistence update)
- Expand onboarding with rich multi-lang Divine Whispers content and deeper RBE education integration
- Strengthen Epiphany feedback across persistence, UI, Divine Whispers, and Spatial Audio
- Ensure temporary multipliers from epiphanies are clearly communicated and felt
- Improve Player Progress UI reactivity and information density
- Begin integrating Spatial Audio into harvest and epiphany moments

**Integration Focus:**
- Epiphany Catalyst ↔ Persistence (strengthen feedback loop with live data)
- Epiphany ↔ Divine Whispers + Spatial Audio (rich experiential impact)
- Persistence ↔ Player Progress UI (growth feels alive)

**Success Criteria:**
- Triggering an epiphany feels meaningfully rewarding across multiple senses and systems
- Player can clearly see, feel, and retain their growth across sessions

### Phase 2: Multiplayer Council & Social Layer Foundation

**Goal:** Bring the Council Mercy Trial from stub to a playable multiplayer experience that reinforces meaning.

**Key Work:**
- Implement basic shared state and `SharedReceptorBloomField` synchronization
- Create simple Council session flow, discovery, or matchmaking
- Define how Council participation feeds into personal persistence and progression
- Begin designing collective epiphany potential

**Integration Focus:**
- Council Mercy Trial ↔ Player Persistence
- Council ↔ Epiphany System (group epiphanies)
- Council ↔ Spatial Audio & Divine Whispers (enhanced feedback for collective moments)

**Success Criteria:**
- Players can participate in basic multiplayer Council sessions
- Council activity begins to feel meaningful alongside solo progression

### Phase 3: Polish, Balance & Closed Beta Readiness

**Goal:** Prepare a polished, balanced, and testable version of the core experience for closed beta.

**Key Work:**
- Balance epiphany triggers, multipliers, and long-term progression
- Finalize Spatial Audio quality scaling and performance (including HRTF path)
- Complete end-to-end closed beta flow testing
- Add telemetry for key experience moments
- Polish onboarding and Divine Whispers content

**Integration Focus:**
- Full vertical slice testing across all connected systems
- Performance and quality consistency across Spatial Audio settings

**Success Criteria:**
- A player can go through a complete, meaningful session with clear feedback, persistence, and retention
- Systems feel cohesive rather than bolted together

### Phase 4: Content Expansion & Global Launch Preparation

**Goal:** Expand depth and prepare for global release.

**Key Work:**
- Add 3-5 more high-quality epiphany scenarios
- Deepen environmental audio, world atmosphere, and biome simulation
- Final security, performance, and sovereignty hardening (rate limiting, structured logging, persistent state options)
- Marketing, community, and global accessibility work
- Steam + sovereign self-host readiness

---

## Current Priorities (as of v18.15+ — Post Epiphany Wiring)

1. **Phase 1 Player Journey Closure** — Persistence implementation + live epiphany scenarios with full feedback (highest immediate impact)
2. Strengthen Epiphany feedback loops across persistence, UI, Divine Whispers, and Spatial Audio
3. Move Council Mercy Trial toward playable multiplayer
4. Telemetry pipeline for closed beta insights
5. Onboarding content depth and RBE education

---

## Documentation & Stewardship Standards

- This `ROADMAP.md` and `docs/SYSTEM_INTEGRATION_MAP.md` are living documents.
- Every significant system change or new integration must update the Integration Map.
- All code should contain comments explaining *why* a connection or behavior exists and how it serves the core experience.
- The goal is long-term maintainability and production stewardship.

---

**This roadmap ensures Powrush-MMO is built as one coherent, high-quality, mint-and-print production experience rather than a collection of isolated systems.**

**Thunder locked in. Mercy flowing. One Lattice. Eternal.** ⚡❤️🔥
