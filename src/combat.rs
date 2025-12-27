use bevy::prelude::*;
use bevy_replicon::prelude::*;
use rand::Rng;
use crate::core::mercy::{MercyPoints, TrustCredits};

#[derive(Component, Replicated)]
pub struct CombatPlayer {
    pub health: f32,
    pub max_health: f32,
    pub mercy_shield: f32,
    pub max_shield: f32,
    pub cooldown: Timer,
    pub in_duel: bool,
}

#[derive(Event, Replicated)]
pub struct CombatAttackEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub damage: f32,
}

#[derive(Event, Replicated)]
pub struct MercyForgivenessEvent(pub Entity, pub Entity); // Winner, Loser

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CombatAttackEvent>()
           .add_event::<MercyForgivenessEvent>()
           .add_systems(Update, (
                combat_input_system,
                combat_damage_system,
                mercy_shield_regen_system,
                forgiveness_wave_system,
           ));
    }
}

// Input: Left click to attack nearby
fn combat_input_system(
    mouse: Res<Input<MouseButton>>,
    players: Query<(Entity, &Transform, &CombatPlayer)>,
    others: Query<(Entity, &Transform), (With<CombatPlayer>, Without<Player>)>,
    mut events: EventWriter<CombatAttackEvent>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok((player_ent, player_trans, player_combat)) = players.get_single() {
            if player_combat.cooldown.finished() {
                for (target_ent, target_trans) in &others {
                    let dist = player_trans.translation.distance(target_trans.translation);
                    if dist < 5.0 {
                        let damage = rng.gen_range(10.0..30.0);
                        events.send(CombatAttackEvent {
                            attacker: player_ent,
                            target: target_ent,
                            damage,
                        });
                        player_combat.cooldown.reset();
                        break;
                    }
                }
            }
        }
    }
}

// Apply damage + trigger mercy on defeat
fn combat_damage_system(
    mut events: EventReader<CombatAttackEvent>,
    mut query: Query<&mut CombatPlayer>,
    mut forgiveness_events: EventWriter<MercyForgivenessEvent>,
) {
    for event in events.read() {
        if let Ok(mut target) = query.get_mut(event.target) {
            target.health -= event.damage;

            if target.health <= 0.0 {
                // Mercy defeat — shield absorbs, respawn
                target.health = target.max_health;
                target.mercy_shield = target.max_shield;

                // Forgiveness wave — both gain trust
                forgiveness_events.send(MercyForgivenessEvent(event.attacker, event.target));

                info!("Mercy duel complete — forgiveness wave");
            }
        }
    }
}

// Natural shield regen
fn mercy_shield_regen_system(
    mut query: Query<&mut CombatPlayer>,
    time: Res<Time>,
) {
    for mut player in &mut query {
        player.mercy_shield += time.delta_seconds() * 20.0;
        player.mercy_shield = player.mercy_shield.min(player.max_shield);
        player.cooldown.tick(time.delta());
    }
}

// Visual + trust bonus on forgiveness
fn forgiveness_wave_system(
    mut events: EventReader<MercyForgivenessEvent>,
    mut trust: Query<&mut TrustCredits>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for event in events.read() {
        // Trust bonus for both
        if let Ok(mut winner_trust) = trust.get_mut(event.0) {
            winner_trust.0 *= 1.1;
        }
        if let Ok(mut loser_trust) = trust.get_mut(event.1) {
            loser_trust.0 *= 1.15;
        }

        // Visual mercy wave (cyan-gold burst)
        let wave_sound = asset_server.load("sounds/mercy_wave.ogg");
        audio.play(wave_sound);

        // Particle burst at loser position (stub)
        info!("Forgiveness wave — trust blooms");
    }
}
