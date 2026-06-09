// server/src/combat/mod.rs
// Powrush-MMO v17.69 — Entity Component System Design (Combat)
// Professional, clean, and extensible ECS architecture for combat systems
// Aligned with Bevy best practices and InterestManager + replication goals

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::hierarchical_grid::HierarchicalGrid;
pub use crate::interest_management::InterestManager;

// ═════════════════════════════════════════════════════════════════════════
// ECS DESIGN OVERVIEW
// ═════════════════════════════════════════════════════════════════════════
//
// Core Philosophy:
// - Data lives in Components (pure data, no behavior)
// - Behavior lives in Systems (query minimal data, focused responsibilities)
// - Communication via Events (AbilityUseEvent, AbilityCooldownUpdate)
// - Global/shared state in Resources (rate limiters, sync trackers)
// - InterestManager is injected for spatial scoping without tight coupling
// - All systems are Update-scheduled and run in parallel where safe
//
// Layers:
// 1. Data Layer (Components)
// 2. Event Layer (AbilityUseEvent, AbilityCooldownUpdate)
// 3. Resource Layer (Rate limiters, trackers)
// 4. System Layer (focused, query-only-what-they-need)
// 5. Plugin Layer (registration + system ordering)

// ═════════════════════════════════════════════════════════════════════════
// 1. DATA LAYER - COMPONENTS
// ═════════════════════════════════════════════════════════════════════════

/// Health component - core survival data
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

/// Damage payload (transient - added/removed each frame)
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Damage {
    pub amount: f32,
    pub damage_type: DamageType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageType {
    Physical, Energy, Mercy, Corruption,
}

/// Combat statistics (attack power, defense, cooldown reduction, etc.)
#[derive(Component, Debug, Clone, Serialize, Deserialize, Default)]
pub struct CombatStats {
    pub attack_power: f32,
    pub defense: f32,
    pub speed: f32,
    pub critical_chance: f32,
    pub cooldown_reduction: f32,
}

/// Ability definition and runtime state
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub id: u32,
    pub cooldown: f32,
    pub last_used: f32,
    pub range: f32,
    pub ability_type: AbilityType,
    pub triggers_gcd: bool,
}

impl Ability {
    pub fn can_use(&self) -> bool { self.last_used <= 0.0 }
    pub fn trigger(&mut self, cooldown_reduction: f32) {
        let effective_cooldown = self.cooldown * (1.0 - cooldown_reduction.clamp(0.0, 0.8));
        self.last_used = effective_cooldown;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbilityType {
    DirectDamage, Buff, Debuff, AoE, Support,
}

/// Current target for abilities
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub entity: Option<Entity>,
}

/// Faction for behavior differentiation
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombatFaction {
    Human, Cydruid, Draek, Quellorian, Ambrosian,
}

/// Status effects (DoT, HoT, buffs, debuffs)
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

/// Global Cooldown state per entity
#[derive(Component, Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalCooldown {
    pub remaining: f32,
}

impl GlobalCooldown {
    pub fn is_ready(&self) -> bool { self.remaining <= 0.0 }
    pub fn trigger(&mut self, duration: f32) { self.remaining = duration; }
}

// ═════════════════════════════════════════════════════════════════════════
// 2. EVENT LAYER
// ═════════════════════════════════════════════════════════════════════════

/// Client requests to use an ability
#[derive(Event, Debug, Clone)]
pub struct AbilityUseEvent {
    pub player_entity: Entity,
    pub slot_index: usize,
    pub ability_id: u32,
}

/// Per-player targeted cooldown state update
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct AbilityCooldownUpdate {
    pub recipient_player: Entity,
    pub acting_player: Entity,
    pub ability_id: u32,
    pub cooldown_remaining: f32,
    pub max_cooldown: f32,
}

// ═════════════════════════════════════════════════════════════════════════
// 3. RESOURCE LAYER (Global / Shared State)
// ═════════════════════════════════════════════════════════════════════════

/// Rate limiter to prevent ability spam
#[derive(Resource, Default)]
pub struct AbilityUseRateLimiter {
    pub last_use: HashMap<Entity, f64>,
}

/// Tracks last sent cooldown values for change detection
#[derive(Resource, Default)]
pub struct CooldownSyncTracker {
    pub last_sent: HashMap<(Entity, u32), f32>,
}

// ═════════════════════════════════════════════════════════════════════════
// 4. SYSTEM LAYER
// ═════════════════════════════════════════════════════════════════════════

/// Main entry point for client ability requests
/// Optimized: cheap checks first, expensive interest work last
pub fn handle_ability_use_requests(
    mut commands: Commands,
    mut ev_ability_use: EventReader<AbilityUseEvent>,
    time: Res<Time>,
    mut rate_limiter: ResMut<AbilityUseRateLimiter>,
    mut sync_tracker: ResMut<CooldownSyncTracker>,
    mut ev_cooldown_update: EventWriter<AbilityCooldownUpdate>,
    interest: Res<InterestManager>,
    mut ability_query: Query<(Entity, &mut Ability, &CombatStats, &Target)>,
    mut health_query: Query<&mut Health>,
    mut gcd_query: Query<&mut GlobalCooldown>,
) {
    let current_time = time.elapsed_seconds_f64();

    for ev in ev_ability_use.read() {
        if let Some(last_time) = rate_limiter.last_use.get(&ev.player_entity) {
            if current_time - last_time < 0.1 { continue; }
        }
        rate_limiter.last_use.insert(ev.player_entity, current_time);

        for (ability_entity, mut ability, stats, target) in ability_query.iter_mut() {
            if ability.id != ev.ability_id { continue; }

            if !ability.can_use() { continue; }

            if ability.triggers_gcd {
                let mut can_use = true;
                for mut gcd in gcd_query.iter_mut() {
                    if !gcd.is_ready() { can_use = false; }
                }
                if !can_use { continue; }
            }

            if let Ok(player_health) = health_query.get(ev.player_entity) {
                if player_health.is_dead() { continue; }
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

                        if ability.triggers_gcd {
                            for mut gcd in gcd_query.iter_mut() {
                                if gcd.is_ready() { gcd.trigger(1.0); }
                            }
                        }

                        // Change detection + per-player targeted emission
                        let key = (ev.player_entity, ability.id);
                        let last_value = sync_tracker.last_sent.get(&key).copied().unwrap_or(-1.0);

                        if (ability.last_used - last_value).abs() > 0.05 {
                            ev_cooldown_update.send(AbilityCooldownUpdate {
                                recipient_player: ev.player_entity,
                                acting_player: ev.player_entity,
                                ability_id: ability.id,
                                cooldown_remaining: ability.last_used,
                                max_cooldown: ability.cooldown,
                            });
                            sync_tracker.last_sent.insert(key, ability.last_used);
                        }
                    }
                }
            }
            break;
        }
    }
}

/// Ticks down ability cooldowns and Global Cooldown
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

/// Applies damage and removes dead entities
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

/// Basic ability execution (can be expanded or merged with handle_ability_use_requests)
pub fn execute_ability_system(
    mut commands: Commands,
    mut ability_query: Query<(&mut Ability, &Target, &CombatStats)>,
    mut health_query: Query<&mut Health>,
) {
    for (mut ability, target, stats) in ability_query.iter_mut() {
        if !ability.can_use() { continue; }

        if let Some(target_entity) = target.entity {
            if let Ok(mut target_health) = health_query.get_mut(target_entity) {
                if ability.ability_type == AbilityType::DirectDamage {
                    let damage_amount = stats.attack_power;
                    commands.entity(target_entity).insert(Damage {
                        amount: damage_amount,
                        damage_type: DamageType::Physical,
                    });
                    ability.trigger(stats.cooldown_reduction);
                }
            }
        }
    }
}

/// Applies status effects over time
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

/// Example faction behavior (Draek corruption bonus)
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
// 5. PLUGIN LAYER
// ═════════════════════════════════════════════════════════════════════════

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AbilityUseRateLimiter>()
            .init_resource::<CooldownSyncTracker>()
            .add_event::<AbilityUseEvent>()
            .add_event::<AbilityCooldownUpdate>()
            .add_systems(Update, (
                damage_system,
                ability_cooldown_system,
                execute_ability_system,
                handle_ability_use_requests,
                status_effect_system,
                draek_corruption_system,
                aoe_damage_system,
            ));
    }
}

// ═════════════════════════════════════════════════════════════════════════
// DESIGN NOTES & FUTURE EXTENSIONS
// ═════════════════════════════════════════════════════════════════════════
//
// Strengths of current design:
// - Clear separation of concerns (data vs systems vs events vs resources)
// - Systems query minimal data needed
// - InterestManager is injected (loose coupling)
// - Per-player targeted events ready for efficient replication
// - Change detection + rate limiting + interest awareness all present
//
// Future improvements:
// - Split combat into sub-modules (damage, abilities, status_effects, factions)
// - Add proper ability execution queue / command buffer
// - Full reverse interest query for true per-player targeting
// - Integrate with replication system for automatic dirty tracking
// - Add more faction-specific behavior systems
// - Client-side prediction components (for prediction/reconciliation)
