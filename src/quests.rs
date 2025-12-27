// src/quests.rs — Full Mercy Quests Module
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
}

#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum QuestKind {
    Explore,
    Share,
    MercyWave,
    LatticeGrow,
}

#[derive(Event, Replicated)]
pub struct QuestCompleteEvent(pub QuestKind);

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<QuestCompleteEvent>()
           .add_systems(Startup, quest_spawn_system)
           .add_systems(Update, (
                quest_progress_system,
                quest_reward_system,
           ));
    }
}

fn quest_spawn_system(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
) {
    let mut rng = rand::thread_rng();
    for player in &players {
        let kind = match rng.gen_range(0..4) {
            0 => QuestKind::Explore,
            1 => Quest
