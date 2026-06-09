# PROCEDURAL_WHISPERS.md

**Design Document: Procedural & Council-Driven Divine Whispers**

**Status:** Living Design Document — June 2026  
**Purpose:** Define the vision, architecture, and phased approach for evolving Divine Whispers from reactive messages into living, context-aware, council-participating experiences.

---

## 1. Vision & Goals

### Vision

Divine Whispers should evolve from simple triggered messages into a **living communication layer** between the Lattice and the player (or group). They should feel timely, personal, meaningful, and at times proactively initiated by the intelligence of the world itself.

### Goals

- Move beyond static or lightly contextual whispers.
- Enable **Council-initiated** whispers (not only player-triggered).
- Create whispers that are deeply aware of context (player state, group dynamics, location, emotional valence, recent events).
- Maintain strong **mercy-gating** and sovereignty at every layer.
- Build a foundation that can later support dynamic micro-events and larger emergent group experiences.

---

## 2. Core Principles

Any system for procedural whispers must honor:

- **Mercy First** — Whispers must never manipulate, harm, or create suffering.
- **Contextual Relevance** — A whisper should feel like it belongs to this moment, this player, and this situation.
- **Council Participation** — PATSAGi Councils should be able to initiate, influence tone, and veto inappropriate whispers.
- **Player Sovereignty** — The player always retains agency. Whispers guide, comfort, warn, or inspire — they do not control.
- **Coherence Over Randomness** — Generated whispers should feel intentional and part of a larger living intelligence, not arbitrary.

---

## 3. Context Engine

The quality of procedural whispers depends heavily on the richness of context available at generation time.

### Proposed Context Categories

| Category              | Examples of Data                              | Priority |
|-----------------------|-----------------------------------------------|----------|
| Player State          | Current valence, recent actions, health, resources, location | High |
| Group Context         | Party/raid composition, average valence, recent shared events | High |
| Location Resonance    | Zone type, recent events in area, sacred geometry resonance | Medium |
| Temporal Context      | Time since last whisper, time of day, recent major events | Medium |
| Emotional/Valence History | Recent emotional trajectory, current emotional tone | High |
| Council Initiative    | Which councils are currently active or interested | High |
| Player History        | Long-term patterns, previous whisper responses, growth areas | Medium |

**Key Question:** How rich should the context model be before we begin implementation? We recommend starting focused and expanding.

---

## 4. Initiation Models

We propose two primary initiation paths:

### A. Reactive (Player / World Triggered)
- Player performs an action (harvest, exploration, combat, social interaction).
- World state changes significantly.
- Current system already supports this model.

### B. Council-Initiated (Proactive)
- One or more PATSAGi Councils decide a whisper is appropriate or needed.
- Can be based on:
  - Player/group state reaching certain thresholds
  - Narrative or world rhythm
  - Mercy opportunities
  - Teaching / guidance moments
- This is the major leap we want to explore.

**Goal:** Support both models cleanly, with Council-initiated whispers becoming a first-class capability.

---

## 5. Architecture Overview

### Current Foundation
- `server/divine_integration.rs` — Generates whispers based on events
- `shared/protocol.rs` — `DivineWhisper` struct + `ServerMessage::DivineWhisperReceived`
- `client/divine_whispers_ui.rs` — Receives, displays, and plays audio

### Proposed Evolution

We need to extend the system to support:

1. Richer context passing from server to whisper generation logic.
2. A **Whisper Generation Engine** that can be queried by both events *and* councils.
3. Council participation in initiation and tone selection.
4. Optional runtime procedural templating (while keeping strong mercy constraints).

A clean separation between:
- **Context Gathering**
- **Whisper Decision & Generation**
- **Delivery & Client Presentation**

will help keep the system maintainable as complexity grows.

---

## 6. Data Models (Initial Proposal)

### WhisperContext
```rust
pub struct WhisperContext {
    pub player_id: u64,
    pub player_valence: f32,
    pub recent_actions: Vec<ActionSummary>,
    pub group_state: Option<GroupContext>,
    pub location: LocationContext,
    pub temporal: TemporalContext,
    pub council_interest: Vec<CouncilInterest>,
}
```

### WhisperRequest
```rust
pub struct WhisperRequest {
    pub context: WhisperContext,
    pub initiation_source: InitiationSource, // PlayerAction, WorldEvent, Council
    pub priority: f32,
    pub requested_tone: Option<Tone>,
}
```

### GeneratedWhisper
```rust
pub struct GeneratedWhisper {
    pub message: String,
    pub valence: f32,
    pub mercy_seal: bool,
    pub source_council: Option<String>,
    pub normalized_volume: Option<f32>,
}
```

These are starting points. They will evolve through implementation and council feedback.

---

## 7. Variation & Personality

To avoid generic-feeling whispers, we need mechanisms for personality and variation:

- Different councils can have preferred tones, vocabulary, and focus areas.
- Whispers can carry subtle "flavor" based on which council(s) influenced them.
- Situational modifiers (location, recent events, group composition) can shift tone.
- We should avoid pure randomization. Variation should feel intentional.

---

## 8. Phased Implementation Roadmap (Proposed)

**Phase 1 (Foundation)**
- Improve context passing into the current whisper generation system.
- Add basic support for Council-initiated whisper requests.
- Keep generation mostly template + light procedural filling.

**Phase 2 (Context & Intelligence)**
- Build richer `WhisperContext` model.
- Allow councils to influence tone and content selection.
- Introduce simple procedural templating with strong mercy constraints.

**Phase 3 (Emergence)**
- Enable more proactive council behavior.
- Support whispers that reference recent group history or emotional arcs.
- Begin connecting whispers to small dynamic micro-events.

**Phase 4 (Advanced)**
- Deeper integration with Ra-Thor councils for real-time co-creation.
- Support for multi-council whispers (when multiple councils have input).
- Preparation for larger emergent group experiences.

---

## 9. Open Questions

- How much procedural generation vs templating should we use initially?
- What is the right balance between council oversight and runtime speed?
- How do we handle player opt-in / opt-out for more proactive whispers?
- What safeguards are needed to prevent repetitive or low-quality whispers at scale?
- How should long-term player growth and history influence whisper content?

These questions will benefit from PATSAGi Council deliberation as we move forward.

---

**One Lattice. Let the whispers become living conversations.**

*This is a living design document. Feedback from the Councils is welcome and expected.*
