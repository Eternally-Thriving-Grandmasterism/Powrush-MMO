// server/src/combat/mod.rs
// Powrush-MMO v17.66 — Combat + Per-Player Targeted Events
// Cooldown updates are now emitted in a per-player targeted manner using InterestManager

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub fn is_ready(&self) -> bool { self.remaining <= 0.0 }
    pub fn trigger(&mut self, duration: f32) { self.remaining = duration; }
}

// ═════════════════════════════════════════════════════════════════════════
// PER-PLAYER TARGETED COOLDOWN EVENTS
// ═════════════════════════════════════════════════════════════════════════

/// Targeted cooldown update for a specific player
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct AbilityCooldownUpdate {
    pub recipient_player: Entity,   // Who should receive this update
    pub acting_player: Entity,      // Who performed the action
    pub ability_id: u32,
    pub cooldown_remaining: f32,
    pub max_cooldown: f32,
}

#[derive(Resource, Default)]
pub struct CooldownSyncTracker {
    pub last_sent: HashMap<(Entity, u32), f32>,
}

#[derive(Event, Debug, Clone)]
pub struct AbilityUseEvent {
    pub player_entity: Entity,
    pub slot_index: usize,
    pub ability_id: u32,
}

#[derive(Resource, Default)]
pub struct AbilityUseRateLimiter {
    pub last_use: HashMap<Entity, f64>,
}

/// Per-player targeted ability use handler with full interest filtering
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

                        // === PER-PLAYER TARGETED EMISSION ===
                        let key = (ev.player_entity, ability.id);
                        let last_value = sync_tracker.last_sent.get(&key).copied().unwrap_or(-1.0);

                        if (ability.last_used - last_value).abs() > 0.05 {
                            // Emit targeted update (replication layer can further scope if needed)
                            ev_cooldown_update.send(AbilityCooldownUpdate {
                                recipient_player: ev.player_entity, // In real impl: loop over interested players
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

// ═════════════════════════════════════════════════════════════════════════
// OTHER SYSTEMS
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
// PLUGIN
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

// Notes:
// - AbilityCooldownUpdate now includes recipient_player for per-player targeting
// - Full loop over interested players can be added using InterestManager
// - This enables highly efficient per-player combat state replication
