use bevy::prelude::*;
use bevy_replicon::prelude::*;
use rand::Rng;

#[derive(Component, Replicated)]
pub struct NPC {
    pub velocity: Vec3,
    pub max_speed: f32,
    pub mercy_level: f32,
}

pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, npc_steering_system);
    }
}

fn npc_steering_system(
    mut npcs: Query<(&NPC, &mut Transform)>,
    players: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    let dt = time.delta_seconds();

    for (npc, mut trans) in &mut npcs {
        let mut steering = Vec3::ZERO;

        // Seek player (mercy attraction)
        if let Ok(player_trans) = players.get_single() {
            let desired = (player_trans.translation - trans.translation).normalize();
            steering += desired * npc.max_speed - npc.velocity;
        }

        // Wander (random circle)
        let circle_center = npc.velocity.normalize() * npc.max_speed;
        let displacement = Vec3::new(
            rng.gen_range(-1.0..1.0),
            0.0,
            rng.gen_range(-1.0..1.0),
        ).normalize() * 2.0;
        let wander = circle_center + displacement;
        steering += wander;

        // Separation (avoid crowding)
        // (skip for brevity — query nearby NPCs)

        // Apply steering
        npc.velocity += steering * dt * 0.5;
        npc.velocity = npc.velocity.clamp_length(0.0, npc.max_speed);
        trans.translation += npc.velocity * dt;

        // Mercy modulation — high mercy slows aggression
        npc.max_speed = 5.0 + npc.mercy_level * 0.1;
    }
}
