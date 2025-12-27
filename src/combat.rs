use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct CombatPlayer {
    pub health: f32,
    pub max_health: f32,
    pub mercy_shield: f32,
    pub damage_cooldown: Timer,
}

#[derive(Event, Replicated)]
pub struct CombatAttack(pub Entity, pub Entity);  // Attacker, Target

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CombatAttack>()
           .add_systems(Update, (combat_attack_system, mercy_shield_system));
    }
}

fn combat_attack_system(
    mut events: EventReader<CombatAttack>,
    mut players: Query<&mut CombatPlayer>,
) {
    for event in events.read() {
        if let Ok(mut target) = players.get_mut(event.1) {
            target.health -= 10.0;
            if target.health <= 0.0 {
                target.health = target.max_health;
                target.mercy_shield += 50.0;  // Forgiveness respawn
                info!("Mercy respawn — shield renewed");
            }
        }
    }
}

fn mercy_shield_system(
    mut query: Query<&mut CombatPlayer>,
    time: Res<Time>,
) {
    for mut player in &mut query {
        player.mercy_shield += time.delta_seconds() * 20.0;  // Regen
        player.mercy_shield = player.mercy_shield.min(100.0);
    }
}
