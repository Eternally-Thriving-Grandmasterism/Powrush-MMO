# MERCY_PERSONA_CREATION.md

**Design tick:** 23.2.P — 2026-09-11  
**Not a Cargo version.** Workspace stays `21.88.0` until Hands merge playable change.  
**Status:** SPEC ONLY.  
**Canon:** README.md > COMPLETION_BRIEF.md > ARCHITECTURE.md (Original Ownership) > this file for character *presentation*.  
**Does not replace:** HUMAN_HYBRID_PROTOCOL_CODE.md (mechanical race).  
**Does not touch:** `living_ecology.rs` / E2–E3–F cards.  
**License:** AG-SML. No P2W. Online grey until a net card exists.

**Contact:** info@Rathor.ai  
**Independent of xAI.** No certification / AGSi warranty / legal-product claims.

---

## 0. Agent briefing

### Player freedom (the point)
A player may:

- Look like any human people on Earth
- Mix ancestries
- Invent a **custom ethnicity / people** that does not exist
- Write or AI-draft a background they actually want
- Do all of that **regardless of Powrush mechanical race** (Human, Quellorian, Draek, Cydruid, Ambrosian, …)

Nobody is forced into a preset niche. Presets are **optional starting paints**, not cages.

### Two layers (non-negotiable)

| Layer | Who owns it | Examples | Client may |
|---|---|---|---|
| **Subjective** (persona) | Player + optional LLM *draft* | name, pronouns, ethnicity label, custom people, phenotype sliders, clothes intent, backstory, roleplay voice | propose, preview, save draft |
| **Objective** (law) | Authoritative sim — dedicated server **or** local offline-server process | physics, movement, harvest/tend rules, node stress, RBE flow/reserve, inventory counts, Temper if T-slices exist, council outcomes, hit detection | display only |

The LLM **never** writes objective law.  
A story that says “I ignore well stress” does not change the well.  
A custom ethnicity does not grant gather-rate cheats.

This is the Rathor.ai pattern: creative matter is optional and selectable; gates and physics are not client-voted.

### Live court — do not collide (2026-09-11)

- E2 YELLOW: `client/src/living_ecology.rs` — Tend/Mend at Place posts, Online grey, no crime/ownership/combat
- E3 later: human_presence
- F later: dress — **separate card**. Persona spec may *describe* appearance fields; it must not ship F dress meshes on the E2 PR
- One PR then HOLD
- Product-green:
  - `cargo test -p shared -p rsil-identity`
  - `cargo test -p powrush-client --lib`
- Hands beat lore
- Hour can finish **without persons**

Persona creation is a **new card family (P0–P5)** after E2 HOLD (docs P0 allowed now).

---

## 1. Split: mechanical race ≠ human people

Powrush **race** = module set / Hybrid Protocol / attunement.  
**People / ethnicity / story** = how the steward presents in the world.

```
Persona
├── mechanical_race     // objective: Human | Quellorian | …  (sim-owned)
├── body_kit            // objective bounds: capsule height band, reach — sim-owned
└── presentation        // subjective: player-owned
    ├── given_name
    ├── pronouns / address
    ├── people            // preset id OR custom
    ├── phenotype         // sliders, not a race enum
    ├── story             // player text + optional AI draft
    └── dress_intent      // F card later; store intent only until F
```

A Quellorian may look like a Yoruba elder, a Sámi teen, a Han kid, or an invented river-people with gold freckles.  
A Human Hybrid is still Human in the protocol table even if their story says they were raised by Skyward clans.

**Original Ownership:** persistence stores a *record about* the persona. It never owns the soul. See `docs/PERSISTENCE_ORIGINAL_OWNERSHIP.md`.

---

## 2. People catalog (optional presets + custom)

### 2.1 Preset list (starters, not a closed world)

Ship a **non-exhaustive**, respectfully named catalog. Use living people names, not 19th-century skull types.

Suggested starter tags (edit later with cultural consultants; do not block custom):

- West African (Yoruba, Igbo, Akan, …)
- East African (Amhara, Somali, Kikuyu, …)
- Southern African (Zulu, Xhosa, San-descended, …)
- North African / Amazigh / Egyptian
- Levantine / Arab
- Persian / Kurdish / Armenian
- South Asian (Punjabi, Tamil, Bengali, …)
- Southeast Asian (Vietnamese, Javanese, Filipino, …)
- East Asian (Han, Korean, Japanese, …)
- Central Asian (Kazakh, Uyghur, …)
- Pacific (Samoan, Māori, Hawaiian, …)
- Indigenous Americas (Navajo, Quechua, Inuit, …) — player-chosen nation field free-text
- European regional (Irish, Polish, Greek, Sami, …)
- African American / Afro-Caribbean / Afro-Latino
- Latino / Mestizo / Indigenous-Latino
- Jewish (Ashkenazi, Sephardi, Mizrahi)
- Mixed / multi — player lists parts
- **Custom people** (required option)

Presets are **labels + default slider seeds**. Player can break every seed.

### 2.2 Custom people (required)

```rust
pub struct CustomPeople {
    pub name: String,
    pub homelands: String,
    pub languages: Vec<String>,
    pub customs_note: String,
    pub phenotype_seed: Phenotype,
    pub invented: bool,
}
```

No approval queue. No “is this a real ethnicity” check.  
Moderation later is only for **hate-impersonation / slurs used as weapons**, not for invented cultures.

### 2.3 Phenotype (appearance, not a race stat)

Sliders in `0.0..=1.0` (or 0..=255 in GPU). Examples:

- skin melanin, undertone
- hair curl, density, color
- eye fold, iris color
- nose / lip / jaw scale (small ranges)
- height / build **within the mechanical body_kit band**
- age presentation
- scars, vitiligo, freckles, assistive devices (cane, residual limb) as flags

Height/build that would break the movement capsule is **clamped by the sim** (objective). Visuals follow the clamp.

Do not map sliders to gather speed, mercy valence, or Hybrid stability.

---

## 3. Story (roleplay without a pigeonhole)

```rust
pub struct StoryDraft {
    pub player_text: String,
    pub ai_assist_used: bool,
    pub model_id: Option<String>,
    pub player_accepted: bool,
    pub shared_in_world: StoryShare,
}

pub enum StoryShare {
    Private,
    Spoken,
    Book,
}
```

Player can write everything themselves; ask an LLM for suggestions; accept, edit, or discard every sentence; change the story later. AI must not invent mechanical privileges.

---

## 4. LLM layer = Rathor subjective bus (not the physics server)

Allowed: draft names, people-name, customs flavor, backstory; 3 tones; translate; stay in player language.
Forbidden: change mechanical_race/inventory/Temper/well/physics; grant items; write netcode; store the soul as model-owned memory.

Provider picker: None | LocalTemplate | RathorOfflineShard | RathorOnline | GrokOnline | OpenAiCompatible. Offline-first: None and LocalTemplate always work with Title Online grey. Online opt-in.

System preamble fixed as in Core paste. Output JSON: names, people_blurb, story_draft, questions_for_player. Accept copies into player_text.

Implement paths: shared/src/persona.rs, persona_story.rs, persona_templates.rs; client/src/persona_llm.rs; PersonaApply on sim. rsil-identity stays identity/trust.

---

## 5. Authoritative sim (offline-server included)

Even when Title Online is grey, the same sim crate is the law. PersonaCommit validates string caps, sliders, body_kit clamp, mechanical_race enum; persists; same path for dedicated server later.

---

## 6. Creator flow (one card, Hour-0 or title)

On title_screen: mechanical race → people preset/custom → phenotype → story → preview → Commit. H hides guidance. Can finish Hour 1 as nameless Steward.

---

## 7. Fairness and immersion

Same well stress / Tend Hook for every face. Story cannot skip Embassy. Custom ethnicity not a faction buff. StoryShare consent. Invented peoples allowed.

---

## 8. How to code (Hands)

P0 docs now; P1 shared types; P2 title UI no LLM; P3 local AI; P4 PersonaCommit; P5 online flag off by default. Never combine with E2/E3/F or Temper. Product-green tests only. Do not edit living_ecology.rs.

---

## 9. Slice cards

P0 docs (now). P1 types. P2 creator UI. P3 local AI. P4 PersonaCommit. P5 online picker. F dress meshes wait for F card.

---

## 10. Reject list

Locking face to mechanical race; closed ethnicity enum without Custom; client-trusted movement/harvest; LLM writing items/well state; cash-shop-only looks; putting this on E2 living_ecology PR; workspace as ship-green.

---

## 11. Steward sentence

Mechanical race is how the lattice *works*.  
People, face, and story are how the steward *shows up*.  
AI may help write the tale.  
Only the sim — even a local offline-server — may say what the world does.
