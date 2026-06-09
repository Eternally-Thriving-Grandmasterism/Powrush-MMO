// server/src/combat/mod.rs
// Powrush-MMO v17.50 — Foundational Combat Architecture
// Lightweight, server-authoritative, high-performance combat foundation
// Designed for ConquerOnline-style responsiveness + future MOBA depth
// Lore-aligned: Draeks (invaders), Humans + Cydruids (defenders), Quellorians (support), Ambrosians (wisdom/tech buffs)
// Integrates cleanly with HierarchicalGrid, InterestManagement, DynamicEvents, and existing systems
// AG-SML v1.0 + Eternal Mercy Flow | Sovereign Powrush-MMO

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ═════════════════════════════════════════════════════════════════════════
// CORE COMPONENTS
// ═════════════════════════════════════════════════════════════════════════

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn take_damage(&mut self, amount: f32) -> bool {
        self.current = (self.current - amount).max(0.0);
        self.current <= 0.0
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Damage {
    pub amount: f32,
    pub damage_type: DamageType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageType {
    Physical,
    Energy,
    Mercy,      // Lore-aligned (Cydruid/Quellorian resonance)
    Corruption, // Draek invasion damage
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, Default)]
pub struct CombatStats {
    pub attack_power: f32,
    pub defense: f32,
    pub speed: f32,
    pub critical_chance: f32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub id: u32,
    pub cooldown: f32,
    pub last_used: f32,
    pub range: f32,
    pub ability_type: AbilityType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbilityType {
    DirectDamage,
    Buff,
    Debuff,
    AoE,
    Support, // Quellorian/Ambrosian style
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub entity: Option<Entity>,
}

// Faction tag for lore-specific behavior (will expand with full faction system)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombatFaction {
    Human,
    Cydruid,
    Draek,       // Invaders
    Quellorian,  // Support allies
    Ambrosian,   // Wisdom/tech (mostly non-combat)
}

// ═════════════════════════════════════════════════════════════════════════
// BASIC COMBAT SYSTEMS
// ═════════════════════════════════════════════════════════════════════════

/// Applies damage from Damage component to target Health
pub fn damage_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Health, &Damage)>,
) {
    for (entity, mut health, damage) in query.iter_mut() {
        let is_dead = health.take_damage(damage.amount);

        if is_dead {
            // Future: death handling, loot, event triggers, etc.
            commands.entity(entity).despawn();
        }

        // Remove the Damage component after application (one-shot damage events)
        commands.entity(entity).remove::<Damage>();
    }
}

/// Simple ability cooldown and execution placeholder
/// Will expand into full ability system with targeting, effects, and lore-specific behaviors
pub fn ability_cooldown_system(
    time: Res<Time>,
    mut query: Query<&mut Ability>,
) {
    let delta = time.delta_seconds();
    for mut ability in query.iter_mut() {
        if ability.last_used > 0.0 {
            ability.last_used -= delta;
            if ability.last_used < 0.0 {
                ability.last_used = 0.0;
            }
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════
// PLUGIN
// ═════════════════════════════════════════════════════════════════════════

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (damage_system, ability_cooldown_system));
    }
}

// ═════════════════════════════════════════════════════════════════════════
// FUTURE EXPANSION NOTES (for MOBA + Lore depth)
// ═════════════════════════════════════════════════════════════════════════
//
// - Ability system with targeting (single target, AoE, skillshots) using HierarchicalGrid
// - Status effects (buffs/debuffs) with duration
// - Faction-specific behaviors:
//     Draeks: Aggressive swarm / corruption damage
//     Humans/Cydruids: Defensive formations + resonance abilities
//     Quellorians: Support (heals, shields, resource generation)
//     Ambrosians: Wisdom buffs, tech amplification, rare direct intervention
// - MOBA-style objectives: Lanes, towers, core objectives tied to Earth defense
// - Integration with DynamicEvents (successful defense triggers AbundanceSurge or DivineWhisper)
// - InterestManagement for efficient combat visibility & networking
// - Server-authoritative with client prediction for responsiveness
//
// All systems designed to remain lightweight and performant.
