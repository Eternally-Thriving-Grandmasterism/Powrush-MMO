// server/src/combat/mod.rs
// Powrush-MMO v17.51 — Combat Architecture + Initial Integration
// Lightweight + robust foundation with basic HierarchicalGrid + InterestManagement wiring points
// Server-authoritative, high-performance, ready for MOBA + lore depth

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// Re-exports for convenience
pub use crate::hierarchical_grid::HierarchicalGrid;
pub use crate::interest_management::InterestManager;

// ═════════════════════════════════════════════════════════════════════════
// CORE COMPONENTS (stable from v17.50)
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

// ═════════════════════════════════════════════════════════════════════════
// INTEGRATION WITH EXISTING SYSTEMS
// ═════════════════════════════════════════════════════════════════════════

/// Example AoE system using HierarchicalGrid (lightweight integration pattern)
pub fn aoe_damage_system(
    grid: Res<HierarchicalGrid>,
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Damage)>,
) {
    for (attacker, transform, damage) in query.iter() {
        if damage.amount <= 0.0 { continue; }
        // TODO: Replace with real grid.query_radius(...) in production
        // This demonstrates the intended integration point
    }
}

/// Publish significant combat events to InterestManagement for scoped replication
pub fn publish_combat_to_interest(
    interest: &mut InterestManager,
    attacker: Entity,
    target: Entity,
    damage: f32,
) {
    // Future full implementation:
    // interest.publish_combat_event(attacker, target, damage);
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
            aoe_damage_system,
        ));
    }
}

// Registration in main.rs:
// mod combat;
// .add_plugins(CombatPlugin)
// Pass HierarchicalGrid and InterestManager as resources when available.
