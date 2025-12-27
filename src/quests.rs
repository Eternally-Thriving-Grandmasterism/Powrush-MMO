use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct Quest {
    pub name: String,
    pub kind: QuestKind,
    pub progress: f32,
    pub goal: f32,
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
        app.add_systems(Update, quest_kind_reward_system);
    }
}

fn quest_kind_reward_system(
    mut query: Query<&mut Quest>,
) {
    for mut quest in &mut query {
        if quest.progress >= quest.goal {
            match quest.kind {
                QuestKind::Explore => quest.progress += 50.0,  // Trust
                QuestKind::Share => quest.progress += 30.0,    // Mercy
                QuestKind::MercyWave => quest.progress += 40.0, // Lattice
                QuestKind::LatticeGrow => quest.progress += 60.0, // Guild
            }
        }
    }
}
