use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum QuestSet {
    Spawn,
    Progress,
    Reward,
}

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
        app.configure_sets(Update, (
            QuestSet::Spawn,
            QuestSet::Progress.after(QuestSet::Spawn),
            QuestSet::Reward.after(QuestSet::Progress),
        ))
        .add_systems(Update, (
            quest_spawn_system.in_set(QuestSet::Spawn),
            quest_progress_system.in_set(QuestSet::Progress),
            quest_reward_system.in_set(QuestSet::Reward),
        ));
    }
}

fn quest_spawn_system() { /* spawn logic */ }

fn quest_progress_system() { /* progress logic */ }

fn quest_reward_system() { /* reward logic */ }
