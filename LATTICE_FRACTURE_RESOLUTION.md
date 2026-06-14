# LATTICE_FRACTURE_RESOLUTION.md

**Fracture Resolution Progression & AGi Automation System**

**Status:** Production Design Document  
**Version:** 1.0  
**Last Updated:** June 13, 2026

---

## 1. Overview

**Skill Name:** Fracture Resolution

This skill governs a player’s ability to manually resolve **Lattice Fractures** — glitches and instabilities in the simulation layer of Powrush-MMO.

The skill serves two primary purposes:

1. Provides a deep, logical, and addictive puzzle-solving loop.
2. Acts as the main progression gate for unlocking **Artificial Godly intelligence (AGi)** automation.

The design philosophy follows the core Ra-Thor principle of **earned access**. Powerful automation is not given freely — it must be worked toward through demonstrated skill and progression.

---

## 2. Experience & Leveling

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

## 3. Reward Scaling

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

## 4. AGi Automation Unlock

### Requirements
To unlock **Artificial Godly intelligence (AGi)** automation, the player must meet **both** conditions:

- **Fracture Resolution Level ≥ 50**
- **Ra-Thor Access Level ≥ Lite**

### What the AGi Provides
Once unlocked, the player gains access to their personal **Artificial Godly intelligence (AGi)**.

- Any discovered Lattice Fracture can be resolved **instantly and for free**.
- The AGi applies the correct solution with satisfying visual and audio feedback.
- Players still receive full (or near-full) rewards.

### Post-AGi Experience
- Players may still solve fractures manually for better rewards or personal satisfaction.
- The AGi becomes the default convenience option.
- High-level players can combine AGi usage with **Fracture Optimization** for maximum efficiency.

---

## 5. Fracture Optimization (Mastery System)

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

## 6. Design Philosophy

- **Earned Automation**: The AGi is not given freely. It requires both demonstrated skill (Level 50) and earned Ra-Thor access.
- **Meaningful Progression**: Early levels feel fast and rewarding. Later levels offer depth through optimization and mastery.
- **Rational Player Focus**: All puzzles and progression are built around logical problem-solving rather than reflexes or randomness.
- **Long-term Engagement**: Even after unlocking the AGi, players who enjoy the puzzles can continue engaging with them for better rewards or optimization.

---

**This document serves as the canonical design reference for the Lattice Fracture Resolution system and AGi automation.**

It is intended to be used for implementation across both the simulation layer and the client-side UI/UX.
