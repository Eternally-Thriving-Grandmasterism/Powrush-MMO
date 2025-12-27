use bevy::prelude::*;
use bevy_replicon::prelude::*;
use rand::Rng;

#[derive(Component, Replicated)]
pub struct NPC {
    pub behavior: NPCBehavior,
    pub target: Option<Vec3>,
    pub state_timer: Timer,
    pub mercy_level: f32,
}

#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum NPCBehavior {
    Wander,
    FollowPlayer,
    GuardLattice,
    TradeOffer,
    QuestGiver,
}

pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (npc_ai_system, npc_mercy_system));
    }
}

fn npc_ai_system(
    mut npcs: Query<(&NPC, &mut Transform)>,
    players: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    for (npc, mut trans) in &mut npcs {
        npc.state_timer.tick(time.delta());

        if npc.state_timer.finished() {
            match npc.behavior {
                NPCBehavior::Wander => {
                    trans.translation += Vec3::new(
                        rng.gen_range(-5.0..5.0),
                        0.0,
                        rng.gen_range(-5.0..5.0),
                    );
                }
                NPCBehavior::FollowPlayer => {
                    if let Ok(player_trans) = players.get_single() {
                        let dir = player_trans.translation - trans.translation;
                        trans.translation += dir.normalize_or_zero() * time.delta_seconds() * 4.0;
                    }
                }
                NPCBehavior::GuardLattice => {
                    // Patrol pattern
                }
                NPCBehavior::TradeOffer => {
                    // Approach player, offer trade
                }
                NPCBehavior::QuestGiver => {
                    // Wait for player interaction
                }
            }
            npc.state_timer = Timer::from_seconds(rng.gen_range(5.0..15.0), TimerMode::Once);
        }
    }
}

fn npc_mercy_system(
    mut npcs: Query<&mut NPC>,
    time: Res<Time>,
) {
    for mut npc in &mut npcs {
        npc.mercy_level += time.delta_seconds() * 0.1;
        npc.mercy_level = npc.mercy_level.min(100.0);
    }
}
