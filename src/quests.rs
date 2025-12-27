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
}

#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum QuestKind {
    Explore,     // Progress from movement
    Share,       // Progress from trades
    MercyWave,   // Progress from forgiveness
    LatticeGrow, // Progress from new nodes
}

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, quest_kind_progress_system);
    }
}

fn quest_kind_progress_system(
    mut query: Query<&mut Quest>,
    time: Res<Time>,
    trades: EventReader<TradeEvent>,
    forgiveness: EventReader<ForgivenessEvent>,
    lattice: Res<LatticeStats>,
) {
    for mut quest in &mut query {
        let delta = time.delta_seconds();
        match quest.kind {
            QuestKind::Explore => quest.progress += delta * 0.2,  // Movement
            QuestKind::Share => quest.progress += trades.len() as f32 * 5.0,
            QuestKind::MercyWave => quest.progress += forgiveness.len() as f32 * 10.0,
            QuestKind::LatticeGrow => quest.progress += lattice.nodes as f32 * 0.1,
        }
        if quest.progress >= quest.goal {
            quest.progress = quest.goal;
            info!("Quest complete — {} mercy", quest.reward_mercy);
        }
    }
}
