// server/src/combat/mod.rs
// Powrush-MMO v17.55 — Combat Foundation Expanded (Basic Abilities + Faction Behaviors)
// Lightweight, server-authoritative, high-performance
// Lore-aligned: Draeks (aggressive invaders), Humans + Cydruids (defenders),
// Quellorians (support), Ambrosians (wisdom/tech buffs, rare direct combat)
// Ready for MOBA-style objectives and deeper faction mechanics

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub use crate::hierarchical_grid::HierarchicalGrid;
pub use crate::interest_management::InterestManager;

// ═════════════════════════════════════════════════════════════════════════
// CORE COMPONENTS
// ═════════════════════════════════════════════════════════════════════════

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self { Self { current: max, max } }
    pub fn take_damage(&mut self, amount: f32) -> bool {
        self.current = (self.current - amount).max(0.0);
        self.current <= 0.0
    }
    pub fn heal(&mut self, amount: f32) { self.current = (self.current + amount).min(self.max); }
    pub fn is_dead(&self) -> bool { self.current <= 0.0 }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Damage {
    pub amount: f32,
    pub damage_type: DamageType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageType {
    Physical, Energy, Mercy, Corruption,
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
    DirectDamage, Buff, Debuff, AoE, Support,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub entity: Option<Entity>,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombatFaction {
    Human, Cydruid, Draek, Quellorian, Ambrosian,
}

// Basic status effect support
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffect {
    pub effect_type: StatusEffectType,
    pub duration: f32,
    pub strength: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusEffectType {
    DamageOverTime,
    HealingOverTime,
    DefenseBuff,
    AttackBuff,
    Corruption,
}

// ═════════════════════════════════════════════════════════════════════════
// BASIC ABILITY & EFFECT SYSTEMS
// ═════════════════════════════════════════════════════════════════════════

/// Simple ability execution system (expandable)
pub fn execute_ability_system(
    mut commands: Commands,
    time: Res<Time>,
    mut ability_query: Query<(&mut Ability, &Target, &CombatStats)>,
    mut health_query: Query<&mut Health>,
) {
    let delta = time.delta_seconds();

    for (mut ability, target, stats) in ability_query.iter_mut() {
        if ability.last_used > 0.0 {
            ability.last_used -= delta;
            continue;
        }

        if let Some(target_entity) = target.entity {
            if let Ok(mut target_health) = health_query.get_mut(target_entity) {
                if ability.ability_type == AbilityType::DirectDamage {
                    let damage_amount = stats.attack_power * 1.0; // Base scaling
                    commands.entity(target_entity).insert(Damage {
                        amount: damage_amount,
                        damage_type: DamageType::Physical,
                    });
                    ability.last_used = ability.cooldown;
                }
            }
        }
    }
}

/// Apply status effects over time
pub fn status_effect_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Health, &mut StatusEffect)>,
) {
    let delta = time.delta_seconds();

    for (entity, mut health, mut effect) in query.iter_mut() {
        effect.duration -= delta;

        match effect.effect_type {
            StatusEffectType::DamageOverTime => {
                commands.entity(entity).insert(Damage {
                    amount: effect.strength * delta,
                    damage_type: DamageType::Corruption,
                });
            }
            StatusEffectType::HealingOverTime => {
                health.heal(effect.strength * delta);
            }
            _ => {}
        }

        if effect.duration <= 0.0 {
            commands.entity(entity).remove::<StatusEffect>();
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════
// FACTION BEHAVIOR HOOKS (Initial)
// ═════════════════════════════════════════════════════════════════════════

/// Example: Draek corruption bonus (aggressive invader behavior)
pub fn draek_corruption_system(
    mut commands: Commands,
    query: Query<(Entity, &CombatFaction, &Damage)>,
) {
    for (entity, faction, damage) in query.iter() {
        if *faction == CombatFaction::Draek && damage.damage_type == DamageType::Corruption {
            // Draeks deal bonus corruption damage
            commands.entity(entity).insert(Damage {
                amount: damage.amount * 1.15,
                damage_type: DamageType::Corruption,
            });
        }
    }
}

// Future expansions:
// - Human/Cydruid defensive formations and resonance abilities
// - Quellorian support (heals, shields, resource generation)
// - Ambrosian wisdom/tech amplification (rare direct intervention)
// - MOBA-style lane objectives and tower mechanics

// ═════════════════════════════════════════════════════════════════════════
// INTEGRATION SYSTEMS (from previous iterations)
// ═════════════════════════════════════════════════════════════════════════

pub fn aoe_damage_system(
    grid: Res<HierarchicalGrid>,
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Damage)>,
) {
    for (attacker, transform, damage) in query.iter() {
        if damage.amount <= 0.0 { continue; }
        // Placeholder for real grid.query_radius implementation
    }
}

pub fn publish_combat_to_interest(
    interest: &mut InterestManager,
    attacker: Entity,
    target: Entity,
    damage: f32,
) {
    // Future: interest.publish_combat_event(...)
}

// ═════════════════════════════════════════════════════════════════════════
// CORE SYSTEMS
// ═════════════════════════════════════════════════════════════════════════

pub fn damage_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Health, &Damage)>,
) {
    for (entity, mut health, damage) in query.iter_mut() {
        if health.take_damage(damage.amount) {
            commands.entity(entity).despawn();
        }
        commands.entity(entity).remove::<Damage>();
    }
}

pub fn ability_cooldown_system(
    time: Res<Time>,
    mut query: Query<&mut Ability>,
) {
    let delta = time.delta_seconds();
    for mut ability in query.iter_mut() {
        if ability.last_used > 0.0 {
            ability.last_used = (ability.last_used - delta).max(0.0);
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════
// PLUGIN
// ═════════════════════════════════════════════════════════════════════════

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            damage_system,
            ability_cooldown_system,
            execute_ability_system,
            status_effect_system,
            draek_corruption_system,
            aoe_damage_system,
        ));
    }
}

// Registration reminder:
// In main.rs: mod combat;
// .add_plugins(CombatPlugin)
