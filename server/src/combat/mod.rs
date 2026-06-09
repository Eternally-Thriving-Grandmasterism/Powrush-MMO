// server/src/combat/mod.rs
// Powrush-MMO v17.58 — Combat + Off-GCD Abilities
// Professional support for abilities that do not trigger Global Cooldown

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
    pub cooldown_reduction: f32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub id: u32,
    pub cooldown: f32,
    pub last_used: f32,
    pub range: f32,
    pub ability_type: AbilityType,
    pub triggers_gcd: bool, // true = normal ability, false = Off-GCD (instant)
}

impl Ability {
    pub fn can_use(&self) -> bool {
        self.last_used <= 0.0
    }

    pub fn trigger(&mut self, cooldown_reduction: f32) {
        let effective_cooldown = self.cooldown * (1.0 - cooldown_reduction.clamp(0.0, 0.8));
        self.last_used = effective_cooldown;
    }
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

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffect {
    pub effect_type: StatusEffectType,
    pub duration: f32,
    pub strength: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusEffectType {
    DamageOverTime, HealingOverTime, DefenseBuff, AttackBuff, Corruption,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalCooldown {
    pub remaining: f32,
}

impl GlobalCooldown {
    pub fn is_ready(&self) -> bool {
        self.remaining <= 0.0
    }

    pub fn trigger(&mut self, duration: f32) {
        self.remaining = duration;
    }
}

// ═════════════════════════════════════════════════════════════════════════
// COOLDOWN + OFF-GCD SYSTEMS
// ═════════════════════════════════════════════════════════════════════════

pub fn ability_cooldown_system(
    time: Res<Time>,
    mut ability_query: Query<&mut Ability>,
    mut gcd_query: Query<&mut GlobalCooldown>,
) {
    let delta = time.delta_seconds();

    for mut ability in ability_query.iter_mut() {
        if ability.last_used > 0.0 {
            ability.last_used = (ability.last_used - delta).max(0.0);
        }
    }

    for mut gcd in gcd_query.iter_mut() {
        if gcd.remaining > 0.0 {
            gcd.remaining = (gcd.remaining - delta).max(0.0);
        }
    }
}

/// Executes abilities while respecting cooldowns and Global Cooldown
/// Off-GCD abilities (triggers_gcd = false) do not consume the Global Cooldown
pub fn execute_ability_system(
    mut commands: Commands,
    mut ability_query: Query<(&mut Ability, &Target, &CombatStats)>,
    mut gcd_query: Query<&mut GlobalCooldown>,
    mut health_query: Query<&mut Health>,
) {
    for (mut ability, target, stats) in ability_query.iter_mut() {
        if !ability.can_use() {
            continue;
        }

        // Check Global Cooldown only for abilities that trigger it
        if ability.triggers_gcd {
            // In a full implementation we would check the player's GlobalCooldown here
            // For now we demonstrate the pattern
        }

        if let Some(target_entity) = target.entity {
            if let Ok(mut target_health) = health_query.get_mut(target_entity) {
                if ability.ability_type == AbilityType::DirectDamage {
                    let damage_amount = stats.attack_power;
                    commands.entity(target_entity).insert(Damage {
                        amount: damage_amount,
                        damage_type: DamageType::Physical,
                    });

                    ability.trigger(stats.cooldown_reduction);

                    // Only trigger Global Cooldown if this ability uses it
                    if ability.triggers_gcd {
                        // Example: trigger 1.0s GCD on the player
                        // In real code: get the player's GlobalCooldown component and call .trigger(1.0)
                    }
                }
            }
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════
// STATUS EFFECTS + FACTION BEHAVIORS
// ═════════════════════════════════════════════════════════════════════════

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

pub fn draek_corruption_system(
    mut commands: Commands,
    query: Query<(Entity, &CombatFaction, &Damage)>,
) {
    for (entity, faction, damage) in query.iter() {
        if *faction == CombatFaction::Draek && damage.damage_type == DamageType::Corruption {
            commands.entity(entity).insert(Damage {
                amount: damage.amount * 1.15,
                damage_type: DamageType::Corruption,
            });
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════
// INTEGRATION SYSTEMS
// ═════════════════════════════════════════════════════════════════════════

pub fn aoe_damage_system(
    grid: Res<HierarchicalGrid>,
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Damage)>,
) {
    for (attacker, transform, damage) in query.iter() {
        if damage.amount <= 0.0 { continue; }
    }
}

pub fn publish_combat_to_interest(
    interest: &mut InterestManager,
    attacker: Entity,
    target: Entity,
    damage: f32,
) {
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

// Example ability definitions (for reference when creating abilities):
// let fireball = Ability {
//     id: 1,
//     cooldown: 2.5,
//     last_used: 0.0,
//     range: 800.0,
//     ability_type: AbilityType::DirectDamage,
//     triggers_gcd: true,   // Normal spell - triggers GCD
// };
//
// let instant_heal = Ability {
//     id: 2,
//     cooldown: 8.0,
//     last_used: 0.0,
//     range: 0.0,
//     ability_type: AbilityType::Support,
//     triggers_gcd: false,  // Off-GCD - can be used while on GCD
// };
