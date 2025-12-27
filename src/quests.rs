use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct Quest {
    pub name: String,
    pub kind: QuestKind,
    pub progress: f32,
    pub goal: f32,
    pub completed: bool,
}

#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum QuestKind {
    Explore,
    Share,
    MercyWave,
    LatticeGrow,
}

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (quest_progress_system, quest_reward_system));
    }
}

fn quest_progress_system(
    mut query: Query<&mut Quest>,
    time: Res<Time>,
) {
    for mut quest in &mut query {
        if !quest.completed {
            quest.progress += time.delta_seconds() * 0.2;
            if quest.progress >= quest.goal {
                quest.completed = true;
                info!("Quest complete — {}", quest.name);
            }
        }
    }
}

fn quest_reward_system(
    mut query: Query<&Quest>,
    mut trust: Query<&mut TrustCredits>,
) {
    for quest in &query {
        if quest.completed {
            if let Ok(mut player_trust) = trust.get_mut(quest.entity()) {
                player_trust.0 += 50.0;  // Mercy reward
            }
        }
    }
}
