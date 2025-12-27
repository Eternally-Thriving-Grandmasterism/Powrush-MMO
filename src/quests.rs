use bevy::prelude::*;
use bevy_replicon::prelude::*;
use rand::Rng;

#[derive(Component, Replicated)]
pub struct Quest {
    pub name: String,
    pub kind: QuestKind,
    pub progress: f32,
    pub goal: f32,
    pub reward_mercy: f32,
    pub completed: bool,
    pub ai_state: AIState,  // AI-driven adaptation
}

#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum QuestKind {
    Explore,
    Share,
    MercyWave,
    LatticeGrow,
}

#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum AIState {
    Easy,
    Adaptive,
    Challenging,
}

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ai_quest_spawn_system)
           .add_systems(Update, (
                ai_quest_progress_system,
                ai_adapt_system,
                quest_reward_system,
           ));
    }
}

// AI spawn — adaptive to player trust
fn ai_quest_spawn_system(
    mut commands: Commands,
    players: Query<(Entity, &TrustCredits)>,
) {
    let mut rng = rand::thread_rng();
    for (player, trust) in &players {
        let state = if trust.0 > 500.0 {
            AIState::Challenging
        } else if trust.0 > 100.0 {
            AIState::Adaptive
        } else {
            AIState::Easy
        };
        let kind = match rng.gen_range(0..4) {
            0 => QuestKind::Explore,
            1 => QuestKind::Share,
            2 => QuestKind::MercyWave,
            _ => QuestKind::LatticeGrow,
        };
        let goal = 10.0 + trust.0 * 0.01;  // AI scales goal
        commands.entity(player).insert(Quest {
            name: format!("AI {:?} Quest", kind),
            kind,
            progress: 0.0,
            goal,
            reward_mercy: goal * 2.0,
            completed: false,
            ai_state: state,
        });
        info!("AI quest spawned — {:?}", state);
    }
}

// AI progress — kind + state adaptive
fn ai_quest_progress_system(
    mut query: Query<&mut Quest>,
    time: Res<Time>,
) {
    for mut quest in &mut query {
        if !quest.completed {
            let multiplier = match quest.ai_state {
                AIState::Easy => 0.3,
                AIState::Adaptive => 0.2,
                AIState::Challenging => 0.1,
            };
            quest.progress += time.delta_seconds() * multiplier;
            if quest.progress >= quest.goal {
                quest.completed = true;
                info!("AI quest complete — {:?}", quest.ai_state);
            }
        }
    }
}

// AI adapt — difficulty shifts based on player performance
fn ai_adapt_system(
    mut query: Query<&mut Quest>,
    trust: Query<&TrustCredits>,
) {
    for mut quest in &mut query {
        if let Ok(t) = trust.get(quest.entity()) {
            quest.ai_state = if t.0 > 1000.0 {
                AIState::Challenging
            } else if t.0 > 200.0 {
                AIState::Adaptive
            } else {
                AIState::Easy
            };
        }
    }
}

fn quest_reward_system(
    mut query: Query<&mut Quest>,
    mut trust: Query<&mut TrustCredits>,
) {
    for quest in &mut query {
        if quest.completed {
            if let Ok(mut player_trust) = trust.get_mut(quest.entity()) {
                player_trust.0 += quest.reward_mercy;
            }
        }
    }
}
