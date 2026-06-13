# Discordant Ambrosian Corruption — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 12, 2026  
**Status:** Active Development  
**Related Documents:** AMBROSIAN_ATTUNEMENT_MECHANICS.md, CROWNSTONE_TRILEMMA_PATHS.md, RESONANCE_BURST_MECHANICS.md, QUELLORIAN_RESONANCE_AI_SYSTEMS.md, BOARDING_MECHANICS.md, THE_HIVELORD.md

---

## 1. Introduction & Nature of Discordant Ambrosians

The **Ambrosians** are crystalline-organic resonance beings who form the harmonic backbone of the Quellorian / Aetherion Luminari Alliance. Through attunement, they amplify resonance fields, provide psionic shielding, and enable phenomena such as the auroral effects of the Auroral Unification Nexus (TAUN) and the powerful Resonance Burst.

However, this deep symbiotic connection carries a terrible risk: **Discordant Corruption**.

A **Discordant Ambrosian** is an Ambrosian whose crystalline lattice has been warped by exposure to the Draek hivemind, the Crownstone’s psionic dominance fields, or catastrophic resonance feedback. Instead of radiating harmonious frequencies, they emit **discordant harmonics** — chaotic, painful, reality-warping vibrations that actively undermine Quellorian unity and can even strengthen nearby Draek forces.

Visually, a Discordant Ambrosian shifts from luminous blue-white crystal to jagged, cracked formations pulsing with sickly purple-red energy veins. Their once-melodic "song" becomes a shrieking, dissonant wail that physically hurts Quellorian crews and disrupts resonance networks.

---

## 2. Causes & Triggers of Corruption

Discordant corruption is not random. It occurs through several well-defined pathways:

### Primary Causes

1. **Crownstone Proximity Exposure** (Most Common)
   - During boarding actions on Draek capital ships or the Brood Spire itself.
   - When Quellorian forces attempt to capture or interact with the Crownstone.
   - The Crownstone’s psionic domination field directly attacks the Ambrosian crystalline lattice.

2. **Failed or Overloaded Attunement**
   - Attempting attunement while under heavy Draek psionic assault.
   - Resonance Burst triggered while Ambrosians are already stressed or partially exposed.
   - Over-attunement beyond safe harmonic thresholds (rare, usually only on capital ships in prolonged battles).

3. **Captured & Weaponized Ambrosians**
   - Draek forces (especially the Hivelord) can deliberately corrupt captured Ambrosians and re-deploy them as living weapons or "discord seeds" inside Quellorian fleets.
   - This is a terrifying late-game tactic once the Draek Dominion realizes the strategic value of corrupted Ambrosians.

4. **Resonance Feedback from Sabotage Path**
   - If the player chooses the **Sabotage** path in the Crownstone Trilemma, the slow-acting resonance virus can backfire and corrupt Ambrosians who were helping contain or purify the Crownstone.

5. **Hivelord’s Suit Direct Interface**
   - The Hivelord can personally project Crownstone energy through his biomechanical suit to corrupt Ambrosians at close range during boarding or duel scenarios.

---

## 3. Effects & Mechanical Impact

### On the Individual Discordant Ambrosian
- Loses all positive attunement bonuses.
- Gains aggressive "Discord Aura" that weakens nearby Quellorian resonance fields.
- Can actively attack and attempt to corrupt other Ambrosians (spreading the infection).
- Becomes highly aggressive toward former allies.
- May explode in a final "Discord Pulse" upon death, dealing area psionic damage and further corruption chance.

### On Quellorian Forces & Resonance Network
- **Harmony Level** of any ship or zone containing a Discordant Ambrosian is heavily penalized.
- Resonance AI decision-making becomes erratic (ships may refuse orders or enter panic states).
- Resonance Burst effectiveness is reduced or can backfire if too many Discordant Ambrosians are present.
- Attunement attempts on ships with Discordant presence have greatly increased failure chance.

### On Draek Forces
- Ironically, Discordant Ambrosians provide a **temporary local boost** to Draek hivemind signal strength (they act as unwilling relay nodes).
- The Hivelord’s suit can siphon energy from nearby Discordant Ambrosians to temporarily increase his own power.

### On the Crownstone & Trilemma Paths
- **Destroy Path**: High chance of creating multiple Discordant Ambrosians as a final backlash.
- **Capture & Repurpose Path**: Risk of partial corruption during early purification stages. Successful redemption requires active containment mechanics.
- **Sabotage Path**: Highest long-term risk of widespread Discordant outbreaks across the fleet.

---

## 4. Propagation & Containment

Discordant corruption can spread rapidly if left unchecked.

**Propagation Formula** (ready for simulation_integration.rs):

```rust
fn calculate_discord_spread(
    discordant_count: u32,
    total_ambrosians: u32,
    crownstone_proximity: f32,  // 0.0 = far, 1.0 = very close
    current_harmony: f32,
) -> f32 {
    let base_spread = (discordant_count as f32 / total_ambrosians as f32).powf(1.5);
    let proximity_multiplier = 1.0 + (crownstone_proximity * 2.0);
    let harmony_resistance = (1.0 - current_harmony).max(0.1);
    
    (base_spread * proximity_multiplier * harmony_resistance).clamp(0.0, 0.95)
}
```

**Containment & Purification**
- Quellorian Resonance Burst can temporarily suppress (but not cure) Discordant Ambrosians.
- Specialized "Resonance Purifier" ships or elite squads can attempt risky purification operations.
- The Ambrosian Choir on the TAUN can perform a costly "Harmonic Cleansing" ritual (long cooldown, high resource cost).
- In extreme cases, affected ships must be quarantined or even scuttled to prevent fleet-wide outbreak.

---

## 5. Narrative, Moral & RBE Weight

Allowing Discordant corruption to spread has profound consequences:

- **Moral Horror**: Players witness former allies screaming in agony as their crystalline forms crack and turn against everything they once protected.
- **RBE Abundance Penalty**: Widespread corruption reduces the effectiveness of harmony-based resource generation and cooperative systems across the simulation.
- **Long-term Narrative Branches**: Persistent Discordant outbreaks can lead to new questlines involving redemption, mercy-killing, or even attempting to create a new hybrid "Discordant Redemption" faction.
- **The Hivelord’s Interest**: The Hivelord actively seeks to create and weaponize Discordant Ambrosians, viewing them as the perfect insult to Quellorian harmony.

---

## 6. Technical Implementation Notes

### Recommended Global Resource

```rust
#[derive(Resource)]
pub struct DiscordantAmbrosianState {
    pub active_discordant: u32,
    pub total_ambrosians: u32,
    pub corruption_spread_rate: f32,
    pub last_purification_attempt: f32, // game time
    pub fleet_wide_discord_level: f32,  // 0.0 - 1.0
}
```

### Key Simulation Variables
- `ambrosian_corruption_level` (per Ambrosian entity or per ship)
- `discord_aura_strength`
- `resonance_network_integrity` (heavily impacted)
- `crownstone_exposure_risk` (increases during Crownstone-related boarding)

### ECS Recommendations
- Add a `Discordant` marker component + `CorruptionLevel` component to Ambrosian entities.
- Use spatial queries to calculate Discord Aura influence on nearby ships.
- Event-driven system: `DiscordantCreatedEvent`, `DiscordantPurifiedEvent`, `DiscordantExplodedEvent`.

### Integration Hooks
- **simulation_integration.rs**: Update `AmbrosianAttunementState` and `ResonanceNetworkState` every frame based on Discordant presence.
- **rbe_engine.rs**: Apply abundance and cooperation penalties when `fleet_wide_discord_level` rises.
- **Boarding Mechanics**: High chance of creating Discordant Ambrosians when boarding near Crownstone or Hivelord.
- **Crownstone Trilemma Resolution**: Each path has different Discordant outbreak probabilities and long-term consequences.
- **Resonance Burst**: Effectiveness formula must account for number of Discordant Ambrosians present.
- **Hivelord Biomechanical Suit**: Can actively create or empower Discordant Ambrosians at close range.

---

## 7. Vulnerabilities & Quellorian Counterplay

- **Resonance Burst** is the strongest immediate counter (though risky).
- **Isolation & Quarantine** tactics (prevent spread).
- **Elite Purification Teams** (high-risk, high-reward boarding actions on affected ships).
- **TAUN Harmonic Cleansing** (fleet-wide but expensive).
- **Preventive Measures**: Keeping Ambrosian attunement purity high and avoiding Crownstone exposure reduces initial infection chance dramatically.

Draek players or AI will actively try to create Discordant outbreaks as a force multiplier.

---

## 8. Development Priorities

1. Implement `DiscordantAmbrosianState` resource and core spread formula.
2. Create visual & audio design for Discordant Ambrosians (cracked crystal, purple-red energy, shrieking audio).
3. Add Discordant creation events during Crownstone boarding and failed attunement.
4. Build containment/purification gameplay systems.
5. Integrate Discordant effects into Resonance Burst and Quellorian Resonance AI decision making.
6. Design long-term narrative consequences and RBE abundance impact.
7. Create "Discordant Redemption" questline hooks for advanced playthroughs.

---

**End of Document**

*This document ensures that Discordant Ambrosian Corruption feels like a terrifying, meaningful, and mechanically deep systemic threat — perfectly coherent with the Crownstone Trilemma, Resonance systems, and the broader moral/RBE philosophy of Powrush-MMO.*