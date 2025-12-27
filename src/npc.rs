use bevy::prelude::*;
use bevy_replicon::prelude::*;
use rand::Rng;

#[derive(Component, Replicated)]
pub struct NPC {
    pub behavior: NPCBehavior,
    pub target: Option<Vec3>,
}

#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum NPCBehavior {
    Wander,
    FollowPlayer,
    GuardLattice,
}

pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, npc_ai_system);
    }
}

fn npc_ai_system(
    mut npcs: Query<(&NPC, &mut Transform)>,
    players: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    for (npc, mut trans) in &mut npcs {
        match npc.behavior {
            NPCBehavior::Wander => {
                if rng.gen_bool(0.01 * time.delta_seconds() as f64) {
                    trans.translation += Vec3::new(
                        rng.gen_range(-10.0..10.0),
                        0.0,
                        rng.gen_range(-10.0..10.0),
                    );
                }
            }
            NPCBehavior::FollowPlayer => {
                if let Ok(player_trans) = players.get_single() {
                    let dir = player_trans.translation - trans.translation;
                    trans.translation += dir.normalize_or_zero() * time.delta_seconds() * 3.0;
                }
            }
            NPCBehavior::GuardLattice => {
                // Patrol lattice nodes
            }
        }
    }
}
