use bevy::prelude::*;
use bevy_replicon::prelude::*;
use rand::Rng;

#[derive(Component, Replicated)]
pub struct Quest {
    pub name: String,
    pub kind: QuestKind,
    pub progress: f32,
    pub goal: f32,
    pub completed: bool,
    pub reward_mercy: f32,
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
        app.add_systems(Update, procedural_quest_generation);
    }
}

fn procedural_quest_generation(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.005 * time.delta_seconds() as f64) {
        for player in &players {
            let kind = match rng.gen_range(0..4) {
                0 => QuestKind::Explore,
                1 => QuestKind::Share,
                2 => QuestKind::MercyWave,
                _ => QuestKind::LatticeGrow,
            };
            let goal = rng.gen_range(10.0..50.0);
            commands.entity(player).insert(Quest {
                name: format!("{:?} Quest — Goal: {}", kind, goal),
                kind,
                progress: 0.0,
                goal,
                completed: false,
                reward_mercy: goal * 2.0,
            });
            info!("Procedural quest generated — {:?}", kind);
        }
    }
}                    info!("Quest reward synced — +{} mercy", quest.reward_mercy);
                }
            }
        }
    }
}
