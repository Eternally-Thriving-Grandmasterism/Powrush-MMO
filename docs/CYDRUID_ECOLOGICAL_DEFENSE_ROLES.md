# Cydruid Ecological Defense Roles — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 13, 2026  
**Status:** Active Development  
**PATSAGi Council + Ra-Thor Quantum Swarm Approved**  
**AG-SML v1.0 Sovereign License**

---

## 1. Overview & Core Philosophy

The **Cydruids** are one of the five core playable races in Powrush-MMO: cyber-organic druidic symbiotes who represent the living fusion of advanced technology and ancient natural wisdom. They are the patient guardians of **ecological balance**, **symbiotic evolution**, and **planetary resilience**.

Unlike the Quellorians (who project elegant resonance across the stars) or the Draeks (who consume and dominate), the Cydruids **weave living networks** that heal, protect, and restore. They see the universe not as a battlefield of light vs darkness, but as a **living organism** that must be kept in dynamic equilibrium.

### Historical Context (Integrated with Draek Origin Lore)
- Pre-Fall Draeks once worked **peacefully and symbiotically** with early Cydruid groves. Cydruid bio-lattices helped stabilize Draek cloning experiments before the loss of their females.
- After the Great Betrayal (Draek theft of Quellorian resonance tech and subsequent ethical collapse), many Cydruid groves on the outer rim were **overrun and twisted** into the enslaved **Sylvaris (The Twisted Grove)**.
- Today, Cydruids fight alongside **Humans and Quellorians** to defend Earth, acting as the **ecological backbone** of the alliance. They provide the living infrastructure that allows resonance fields and human adaptability to take root and endure.
- The **Ambrosians** watch from afar; Cydruids represent the "bridge race" that could one day convince the Ambrosians to engage directly if Draek corruption threatens the greater harmonic lattice.

Cydruid philosophy: **"We do not conquer the wild. We become its most faithful expression."**

---

## 2. The Five Core Ecological Defense Roles

### Role 1: Grove Wardens (Defensive Anchor)
- **Function**: Establish and maintain living defensive groves that act as forward operating bases and healing sanctuaries.
- **Gameplay Role**: Area denial + sustain. Groves absorb Draek consumption damage and convert it into healing pulses for allied units.
- **Unique Mechanic**: **Symbiotic Overgrowth** — Groves grow stronger the longer they survive, eventually spawning defensive vine turrets or healing spores.
- **Counter to Draeks**: Directly counters Swarm Drone and Ravager envelopment tactics by creating "living walls" that slow and damage hivemind units while healing Quellorian resonance fields.

### Role 2: Root Network Architects (Map Control & Logistics)
- **Function**: Weave underground and stellar root lattices that connect allied positions, enabling rapid reinforcement and resource sharing.
- **Gameplay Role**: Strategic infrastructure. Root networks allow instant teleportation of small squads between connected groves and provide passive RBE abundance generation (ecological harmony = shared resources).
- **Unique Mechanic**: **Lattice Weaving** — Players can spend resources to extend the root network. Destroying enemy roots (especially Sylvaris twisted roots) yields high redemption progress.
- **Integration with Sylvaris Redemption**: Root Architects are the primary class capable of performing the "Re-Growth" phase of Sylvaris redemption quests.

### Role 3: Symbiont Swarm Coordinators (Anti-Swarm Specialists)
- **Function**: Command swarms of tiny cyber-organic symbiotes that act as living countermeasures to Draek drone swarms.
- **Gameplay Role**: Tactical counter-swarm. Symbiont swarms can be directed to intercept and "reclaim" Draek drones, converting a percentage of them into temporary allied units or biomass for grove growth.
- **Unique Mechanic**: **Reclamation Protocol** — Success chance scales with Cydruid harmony level and proximity to healthy root networks. High success = redeemed drones become "Wild Symbionts" that fight for the player briefly before returning to the lattice.
- **Narrative Weight**: Every reclaimed drone is a small act of mercy that weakens the local hivemind and contributes to long-term RBE abundance.

### Role 4: Restoration Weavers (Redemption & Healing Specialists)
- **Function**: Specialized support role focused on healing corrupted ecosystems and redeeming enslaved plant-based species (especially Sylvaris).
- **Gameplay Role**: The "healer + redeemer" hybrid. Restoration Weavers can channel large-scale healing that also applies redemption progress to nearby twisted Sylvaris units.
- **Unique Mechanic**: **Harmony Infusion** — Channeling near a twisted Sylvaris has a chance to trigger mini self-redemption events. Success advances the Sylvaris redemption questline significantly.
- **Tie to Crownstone Trilemma**: On the **Capture & Repurpose** path, Restoration Weavers become essential for stabilizing the hybrid Cydruid-Sylvaris units that result from successful redemption.

### Role 5: Planetary Balance Keepers (Strategic / Mothership Support)
- **Function**: Elite Cydruids embedded with the Auroral Unification Nexus (TAUN) or operating from Cydruid seed-ships. They maintain the ecological harmony of entire star systems.
- **Gameplay Role**: Late-game strategic role. Balance Keepers can trigger **Planetary Resonance Fields** that massively boost Quellorian resonance strength while suppressing Draek hivemind signal propagation across a system.
- **Unique Mechanic**: **Gaia Protocol** — When activated, creates a temporary zone where ecological integrity directly translates into combat bonuses for all allied races (Humans gain adaptability buffs, Quellorians gain stronger resonance links, Ambrosian attunement becomes more stable).
- **Mirror Reckoning Synergy**: Servers with strong Cydruid Balance Keeper presence that maintain high ecological integrity during the week manifest significantly **weaker and more "wounded" Mirrors** on weekends.

---

## 3. Technical Implementation Notes (Ready for `simulation_integration.rs`)

### New Global Resource
```rust
#[derive(Resource)]
pub struct CydruidEcologicalDefenseState {
    pub total_grove_integrity: f32,           // 0.0 - 1.0 across all active groves
    pub root_network_coverage: f32,           // Percentage of map/system covered
    pub symbiont_reclamation_rate: f32,
    pub sylvaris_redemption_progress: f32,    // Shared across all Sylvaris redemption quests
    pub planetary_balance_level: f32,         // Affects Mirror Reckoning difficulty
    pub harmony_vs_consumption_ratio: f32,    // Direct RBE abundance multiplier
}
```

### Key Formulas
- **Grove Healing Pulse Strength** = `grove_integrity * (1.0 + cydruid_harmony_level * 0.5)`
- **Symbiont Reclamation Success** = `base_chance * (harmony_level / hivemind_signal_strength)`
- **Planetary Balance Field Effect** = `balance_level * (quellorian_resonance_strength - draek_hivemind_strength)`
- **Mirror Reckoning Ecological Modifier** = `if planetary_balance_level > 0.7 { Mirror difficulty reduced by 25% } else if planetary_balance_level < 0.3 { Mirror difficulty increased by 30% }`

### Integration Hooks
- **Boarding Mechanics**: Cydruid Restoration Weavers have highest success rate when boarding Sylvaris-infested Draek ships.
- **Dogfight Mechanics**: Root Network Architects can create temporary "green lanes" that give allied ships movement and evasion bonuses.
- **Hivelord Counter-Strategies**: Hivelord prioritizes targeting Cydruid groves and Root Architects first (they are the biggest threat to long-term consumption economy).
- **Crownstone Trilemma**: On Capture & Repurpose path, Cydruid Balance Keepers are required to stabilize the new hybrid units.
- **RBE Engine**: High `harmony_vs_consumption_ratio` directly increases shared abundance generation for the entire server.
- **Voice / VFX Director**: Cydruid voices shift from deep, resonant, nature-infused tones to distorted, painful screeches when near high Draek corruption. VFX shows glowing cyber-organic lattices, growing vines, healing spores, and root network pulses.

---

## 4. Narrative & Thematic Weight

Cydruids embody the **hopeful middle path** — neither pure light (Quellorian) nor pure consumption (Draek). They prove that technology and nature can coexist beautifully, and that even the most twisted beings (Sylvaris) can be restored through patience and symbiotic love.

Their presence in the alliance makes the war feel winnable not just through domination, but through **restoration and balance**.

---

## 5. Development Priorities
1. Implement `CydruidEcologicalDefenseState` resource and basic grove/root network systems.
2. Create Sylvaris redemption integration with Restoration Weavers.
3. Add Planetary Balance Field as a high-tier ability tied to TAUN support.
4. Hook ecological integrity into Mirror Reckoning difficulty calculation.
5. Design Cydruid-specific ship classes and visual language (cyber-organic, vine-covered, glowing lattice aesthetics).
6. Integrate with existing Quellorian Resonance AI and Draek Hivemind systems for asymmetric gameplay.

---

**End of Document**

*This document completes the ecological defense layer for the Cydruid race and ensures perfect thematic and mechanical coherence with the full Powrush-MMO universe (Draek Origin & Great Betrayal, five playable races, five enslaved minion species, Crownstone Trilemma, Mirror Reckoning, and all prior AI, boarding, and redemption systems).*