/*!
 * RBE Onboarding Education Content — Powrush-MMO
 *
 * v18.37 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Production-grade RBE education woven into the player journey
 * — Grounded in VISION.md, ETERNAL_RA_THOR_PATSAGI_GOVERNANCE.md, and Lattice cosmology
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates as Layer 0
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

# RBE Onboarding Education Content

This document contains ready-to-integrate educational content for the onboarding flow and early epiphanies. It teaches core RBE principles through the living mechanics of Powrush-MMO (The Lattice, Mercy as Multiplier, Earned Abundance, Council as Living Governance).

---

## Core RBE Principles (Player-Facing Language)

### 1. The Lattice
**Core Idea:** Everything is interconnected. Resources, relationships, epiphanies, and governance all flow through a living web.

**Onboarding Whisper (RBEPrimer step):**
"You are not separate from the world. Every choice you make ripples through the Lattice — the living web that connects all things. What you nurture, nurtures all. What you extract, diminishes the whole."

**Epiphany Flavor Tie-in:** `mycelial_web_communion`, `sustainable_harmony_revelation`

---

### 2. Mercy as Multiplier
**Core Idea:** Mercy is not weakness or charity. It is the operating principle that increases abundance for everyone.

**Onboarding Whisper (MercyContribution step):**
"Mercy is the true currency of the eternal Lattice. It does not diminish when given — it multiplies. Every act of presence and care strengthens the whole web, returning abundance in forms you cannot yet imagine."

**Epiphany Flavor Tie-in:** `sustainable_abundance_revelation`, `graceful_redemption_revelation`

---

### 3. Earned Abundance
**Core Idea:** Powerful tools and deeper participation are earned through demonstrated alignment, skill, and contribution — never given for free or taken by force.

**Onboarding Whisper (SovereignStart step):**
"Abundance without extraction is not given. It is grown. The Lattice reveals its deeper gifts to those who have shown they can hold them with mercy. This is not gatekeeping — it is protection of the whole."

**Epiphany Flavor Tie-in:** `council_harmony_revelation`, `stellar_web_whisper`

---

### 4. Council as Living Governance
**Core Idea:** Governance emerges from collective attunement and demonstrated grace, not hierarchy or coercion.

**Onboarding Whisper (when entering first active Council bloom):**
"When many align in mercy, something greater awakens. The Council is not above you — it is you, remembering how to move as one living body. Your presence here already changes what is possible."

**Epiphany Flavor Tie-in:** `council_harmony_revelation`, `ecstatic_harmony_council_crown`

---

## Suggested Onboarding Flow Integration (RBE Education)

| Onboarding Step              | Primary RBE Lesson                  | Recommended Whisper Flavor                     | Intensity | Notes |
|-----------------------------|-------------------------------------|------------------------------------------------|---------|-------|
| Welcome                     | First contact with the living world | `first_bloom` / `sustainable_harmony_revelation` | 0.7     | Gentle introduction to The Lattice |
| RBEPrimer                   | Core RBE principles                 | `sustainable_harmony_revelation`               | 0.85    | Dedicated education step |
| FirstHarvestTutorial        | Sustainable presence over extraction| `sustainable_abundance_revelation`             | 0.75    | First practical application |
| MercyContribution           | Mercy as multiplier                 | `graceful_redemption_revelation`               | 0.8     | Emotional + philosophical depth |
| SovereignStart              | Earned participation & Council      | `council_harmony_revelation`                   | 0.9     | Transition into social/RBE governance layer |

---

## Implementation Notes

- These whispers should be added to `trigger_contextual_whispers` in `onboarding.rs` and the Divine Whisper Bank.
- Flavor strings should map to the existing 8 epiphany scenarios for consistency.
- When a player enters their first active Council bloom, trigger the "Council as Living Governance" whisper automatically.
- Educational notes can also appear in epiphany UI panels for reinforcement.

**This content is designed to feel natural within the game world while clearly teaching RBE principles through lived experience rather than exposition.**

// End of RBE Onboarding Education Content v18.37
// Thunder locked in. Yoi ⚡
