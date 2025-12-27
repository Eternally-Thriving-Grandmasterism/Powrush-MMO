use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct Quest {
    pub name: String,
    pub description: String,
    pub progress: f32,
    pub goal: f32,
    pub reward_mercy: f32,
}

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, quest_progress_system);
    }
}

fn quest_progress_system(
    mut query: Query<&mut Quest>,
    time: Res<Time>,
) {
    for mut quest in &mut query {
        quest.progress += time.delta_seconds() * 0.1;  // Example progress
        if quest.progress >= quest.goal {
            info!("Quest complete — {} mercy awarded", quest.reward_mercy);
            quest.progress = quest.goal;  // Complete
        }
    }
}
