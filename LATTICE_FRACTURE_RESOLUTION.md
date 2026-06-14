# LATTICE_FRACTURE_RESOLUTION.md

**Fracture Resolution Progression & AGi Automation System**

**Status:** Production Design Document  
**Version:** 1.6  
**Last Updated:** June 14, 2026

---

## 1. Overview

**Skill Name:** Fracture Resolution

This skill governs a player’s ability to manually resolve **Lattice Fractures** — glitches and instabilities in the simulation layer of Powrush-MMO.

The skill serves two primary purposes:

1. Provides a deep, logical, and addictive puzzle-solving loop.
2. Acts as the main progression gate for unlocking **Artificial Godly intelligence (AGi)** automation.

The design philosophy follows the core Ra-Thor principle of **earned access**. Powerful automation is not given freely — it must be worked toward through demonstrated skill and progression.

---

## 2. Puzzle Mechanics & Contextual Types

Every Lattice Fracture should feel logically connected to the situation in which it appears. The puzzle type is chosen (or blended) based on the player’s current activity and world context.

### Core Design Rule

> The solution method must make rational sense for *why* the fracture occurred.

### Contextual Puzzle Types

#### 2.1 TOLC Gate Alignment
**Primary Context:** Deep simulation layer instability, high-level events, or when multiple systems are affected simultaneously.

**Core Mechanic:**
The player is presented with the 8 TOLC Mercy Gates in a circular or nodal interface. Some gates are misaligned, inverted, overpowered, or in conflict with neighboring gates. The player must rotate, reconnect, or re-sequence them until all gates satisfy their core principle without creating contradictions.

**Win Condition:** All 8 gates reach a stable collective valence score above the required threshold.

**Difficulty Scaling:** More gates become locked or hidden. Later versions add dynamic influence between gates and stricter mercy constraints.

**AGi Behavior:** The AGi can use backtracking + AC-3 to find optimal solutions.

#### 2.2 Resource Flow Balancing
**Primary Context:** Harvesting, economy nodes, or any situation involving resource production and distribution.

**Core Mechanic:**
The player sees a node-and-connection map of the local resource economy. Some nodes are overproducing, leaking, or creating negative feedback loops. The player must adjust flows, reroute resources, shut down corrupted nodes, or add stabilizers while maintaining overall system balance.

**Win Condition:** Stabilize the entire network so no node goes critically negative and all mercy/abundance thresholds are met.

**Difficulty Scaling:** More nodes, hidden constraints, and dynamic changes during the puzzle. Higher levels require satisfying multiple overlapping mercy conditions.

**AGi Behavior:** The AGi uses improved bounding and variable ordering to find good solutions efficiently.

#### 2.3 Causal Chain Reconstruction
**Primary Context:** Combat, major conflicts, timeline-sensitive events, or when a clear sequence of cause and effect has been broken.

**Core Mechanic:**
The player is shown a broken timeline or event chain with missing, duplicated, or out-of-order nodes. They must reconstruct the correct causal sequence and remove corrupted events.

**Win Condition:** Rebuild a stable, non-contradictory causal chain that explains the fracture.

**Difficulty Scaling:** Longer chains, hidden dependencies, red herrings, and paradoxes that must be resolved without creating new contradictions.

**AGi Behavior:** The AGi can reconstruct valid timelines using backtracking-style search.

#### 2.4 Pattern Purification
**Primary Context:** Data corruption, corrupted Divine Whispers, historical records, or information streams.

**Core Mechanic:**
The player is shown a grid or sequence of symbols/data fragments, some of which are corrupted. They must identify and remove the corrupted elements while preserving the underlying valid pattern.

**Win Condition:** The remaining pattern is clean, coherent, and matches the expected structure.

**Difficulty Scaling:** More noise, multiple overlapping patterns, and time pressure on higher difficulties.

**AGi Behavior:** The AGi can purify data streams using pattern-based search.

#### 2.5 Spatial Integrity Repair
**Primary Context:** World exploration, terrain distortion, broken paths, or visual glitches in the environment.

**Core Mechanic:**
The player must realign distorted geometry, reconnect broken ley lines, or restore proper spatial relationships in a 3D or top-down view.

**Win Condition:** The local space returns to stable, coherent geometry.

**Difficulty Scaling:** More complex geometry, moving pieces, and multi-layered spatial constraints.

**AGi Behavior:** The AGi can restore spatial coherence using geometric constraint solving.

#### 2.6 Consensus Alignment
**Primary Context:** Council events, social interactions, group decisions, or situations involving conflicting wills.

**Core Mechanic:**
Multiple "voices" or positions are shown with conflicting goals. The player must find alignment points and resolve contradictions until a stable, mercy-consistent consensus is reached.

**Win Condition:** All parties reach an acceptable equilibrium without violating core mercy principles.

**Difficulty Scaling:** More voices, hidden agendas, and stricter ethical constraints.

**AGi Behavior:** The AGi can mediate and produce stable consensus solutions.

---

## 3. Experience & Leveling

### How Experience is Gained

Players gain experience primarily by successfully resolving fractures manually.

**Experience Formula (Simplified):**

```
Experience = BaseXP × DifficultyMultiplier × (1 + SkillLevel × 0.015) × FirstTimeBonus
```

- **BaseXP** = 100 for a standard fracture
- **DifficultyMultiplier** = 1.0 – 3.5 depending on fracture complexity
- **FirstTimeBonus** = 1.5× when solving a new puzzle archetype for the first time

### Experience Table (Levels 1–100)

| Level | Total XP Required | XP to Next Level | Notes                              |
|-------|-------------------|------------------|------------------------------------|
| 1     | 0                 | 800              | Starting level                     |
| 10    | 7,200             | 1,100            | Basic tools unlocked               |
| 20    | 18,500            | 1,450            | Access to second puzzle type       |
| 30    | 35,000            | 1,850            | Mistake penalties reduced          |
| 40    | 58,000            | 2,300            | Access to third puzzle type        |
| **50**| **90,000**        | **2,800**        | **AGi Unlock Threshold**           |
| 60    | 132,000           | 3,400            | Strong pattern recognition         |
| 70    | 185,000           | 4,100            | Rare fractures begin appearing     |
| 80    | 252,000           | 4,900            | Advanced tools available           |
| 90    | 335,000           | 5,800            | Near-optimal solving speed         |
| 100   | 440,000           | —                | Mastery tier reached               |

After level 100, progression continues at a much slower rate as a prestige/mastery system.

---

## 4. Reward Scaling

Rewards from resolving fractures scale with both fracture difficulty and the player’s Fracture Resolution skill.

### Base Rewards
- Experience
- Resonance / Valence
- Rare resources and data fragments
- Progress toward Ra-Thor access (when applicable)

### Reward Multipliers by Skill Tier

| Skill Tier       | Reward Multiplier | Special Rewards                          |
|------------------|-------------------|------------------------------------------|
| 1–25 (Novice)    | 1.0×              | Basic resources                          |
| 26–50 (Advanced) | 1.25×             | Higher chance of rare data               |
| 51–80 (Expert)   | 1.5×              | Access to high-tier fractures            |
| 81–100 (Master)  | 1.75×             | Optimized fracture rewards               |
| 100+ (Architect) | 2.0×              | Can influence fracture type and rewards  |

---

## 5. AGi Automation Unlock

### Requirements
To unlock **Artificial Godly intelligence (AGi)** automation, the player must meet **both** conditions:

- **Fracture Resolution Level ≥ 50**
- **Ra-Thor Access Level ≥ Lite**

### What the AGi Provides
Once unlocked, the player gains access to their personal **Artificial Godly intelligence (AGi)**.

- Any discovered Lattice Fracture can be resolved **instantly and for free**.
- The AGi attempts to find an optimal solution using the available backtracking + constraint satisfaction solvers.
- Players still receive full (or near-full) rewards when an optimal solution is used.

### Post-AGi Experience
- Players may still solve fractures manually for better rewards or personal satisfaction.
- The AGi becomes the default convenience option.
- High-level players can combine AGi usage with **Fracture Optimization** for maximum efficiency.

---

## 6. Fracture Optimization (Mastery System)

At **Fracture Resolution Level 100**, players unlock **Fracture Optimization**.

This system allows high-level players to influence the type and quality of fractures they encounter, even after gaining AGi automation.

### Optimization Parameters

Players can allocate **Optimization Points** across the following categories:

| Focus                        | Effect                                              | Point Cost |
|-----------------------------|-----------------------------------------------------|------------|
| Puzzle Type Preference      | Increases chance of preferred puzzle archetypes     | 1          |
| Reward Quality              | Improves average rewards from fractures             | 2          |
| Difficulty Control          | Can slightly raise or lower fracture difficulty     | 1          |
| AGi Synergy                 | Increases rewards when using AGi automation         | 3          |

This gives meaningful endgame agency and prevents the system from becoming completely passive after automation is unlocked.

---

## 7. Design Philosophy

- **Earned Automation**: The AGi is not given freely. It requires both demonstrated skill (Level 50) and earned Ra-Thor access.
- **Meaningful Progression**: Early levels feel fast and rewarding. Later levels offer depth through optimization and mastery.
- **Rational Player Focus**: All puzzles and progression are built around logical problem-solving rather than reflexes or randomness.
- **Long-term Engagement**: Even after unlocking the AGi, players who enjoy the puzzles can continue engaging with them for better rewards or optimization.
- **Contextual Logic**: Puzzle types are chosen based on the situation so the solution method always feels rational and immersive.

---

## 8. Implementation Structures (Code-Ready)

This section defines data models, traits, and flows intended to be directly implementable in Rust (simulation crate) and Bevy ECS (client).

**Current Implementation Status (as of v1.6):**

### Solver Quality
- `TolcGateState`: Strong CSP solver with **AC-3 + Forward Checking + MRV + bitmask domains**.
- `ResourceFlowState`: Improved backtracking with **variable ordering + stronger bounding + light forward checking**.
- Both solvers support `is_solvable()` and `find_solution()`.

### Generation
- `generate_fracture()` uses the improved solvers and increases retry attempts for high-difficulty puzzles.

### AGi
- `agi.rs` attempts to use `find_solution()` for optimal resolution when available.

### Module Structure

```
simulation/src/fracture/
├── mod.rs
├── types.rs
├── puzzle_trait.rs
├── generation.rs
├── agi.rs
└── puzzles/
    ├── mod.rs
    ├── tolc_gates.rs
    └── resource_flow.rs
```

---

## 9. Design Philosophy

- **Earned Automation**: The AGi is not given freely. It requires both demonstrated skill (Level 50) and earned Ra-Thor access.
- **Meaningful Progression**: Early levels feel fast and rewarding. Later levels offer depth through optimization and mastery.
- **Rational Player Focus**: All puzzles and progression are built around logical problem-solving rather than reflexes or randomness.
- **Long-term Engagement**: Even after unlocking the AGi, players who enjoy the puzzles can continue engaging with them for better rewards or optimization.
- **Contextual Logic**: Puzzle types are chosen based on the situation so the solution method always feels rational and immersive.
- **Implementation Clarity**: Data models and traits are designed to be directly implementable with minimal translation effort.

---

**This document serves as the canonical design reference for the Lattice Fracture Resolution system and AGi automation.**

It is intended to be used for implementation across both the simulation layer and the client-side UI/UX.
