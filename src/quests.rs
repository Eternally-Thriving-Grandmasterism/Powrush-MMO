use bevy::prelude::*;
use bevy_replicon::prelude::*;
use rand::Rng;

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
        app.add_systems(Startup, quest_spawn_system)
           .add_systems(Update, quest_progress_system);
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
            1 => QuestKind::Share,
            2 => QuestKind::MercyWave,
            _ => QuestKind::LatticeGrow,
        };
        commands.entity(player).insert(Quest {
            name: format!("{:?} Quest", kind),
            kind,
            progress: 0.0,
            goal: 10.0,
        });
        info!("Quest spawned — {:?}", kind);
    }
}

fn quest_progress_system(
    mut query: Query<&mut Quest>,
    time: Res<Time>,
) {
    for mut quest in &mut query {
        quest.progress += time.delta_seconds() * 0.2;
        if quest.progress >= quest.goal {
            quest.progress = quest.goal;
            info!("Quest complete — mercy rewarded");
        }
    }
}
