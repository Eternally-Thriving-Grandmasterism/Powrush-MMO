use bevy::prelude::*;
use bevy_replicon::prelude::*;
use rand::Rng;

#[derive(Component, Replicated)]
pub struct NPC {
    pub mercy_level: f32,
    pub current_state: NPCState,
    pub state_timer: Timer,
    pub target_pos: Option<Vec3>,
}

#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum NPCState {
    Wander,
    FollowPlayer,
    GuardLattice,
    TradeOffer,
    QuestGiver,
    MercyInteract,
}

pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            npc_ai_decision_system,
            npc_movement_system,
        ));
    }
}

fn npc_ai_decision_system(
    mut npcs: Query<&mut NPC>,
    players: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    for mut npc in &mut npcs {
        npc.state_timer.tick(time.delta());

        if npc.state_timer.finished() {
            let new_state = match npc.mercy_level {
                mercy if mercy > 80.0 => {
                    if rng.gen_bool(0.3) {
                        NPCState::MercyInteract
                    } else {
                        NPCState::QuestGiver
                    }
                }
                mercy if mercy > 50.0 => {
                    if rng.gen_bool(0.5) {
                        NPCState::TradeOffer
                    } else {
                        NPCState::FollowPlayer
                    }
                }
                _ => NPCState::Wander,
            };
            npc.current_state = new_state;
            npc.state_timer = Timer::from_seconds(rng.gen_range(10.0..30.0), TimerMode::Once);
            info!("NPC state changed — {:?}", new_state);
        }
    }
}

fn npc_movement_system(
    mut npcs: Query<(&NPC, &mut Transform)>,
    players: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    for (npc, mut trans) in &mut npcs {
        match npc.current_state {
            NPCState::Wander => {
                trans.translation += Vec3::new(
                    rand::random::<f32>() * 2.0 - 1.0,
                    0.0,
                    rand::random::<f32>() * 2.0 - 1.0,
                ) * time.delta_seconds();
            }
            NPCState::FollowPlayer => {
                if let Ok(player_trans) = players.get_single() {
                    let dir = player_trans.translation - trans.translation;
                    trans.translation += dir.normalize_or_zero() * time.delta_seconds() * 3.0;
                }
            }
            _ => {}  // Guard/Trade/Quest — stationary or custom
        }
    }
}
