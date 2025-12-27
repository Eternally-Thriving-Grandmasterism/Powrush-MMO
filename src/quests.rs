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

#[derive(Event, Replicated)]
pub struct QuestCompleteEvent(pub Entity);

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
            1 => QuestKind::Share,
            2 => QuestKind::MercyWave,
            _ => QuestKind::LatticeGrow,
        };
        commands.entity(player).insert(Quest {
            name: format!("{:?} Quest", kind),
            kind,
            progress: 0.0,
            goal: rng.gen_range(10.0..50.0),
            completed: false,
            reward_mercy: rng.gen_range(20.0..100.0),
        });
        info!("Synced quest spawned — {:?}", kind);
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
                info!("Quest complete — synced to all players");
            }
        }
    }
}

fn quest_reward_system(
    mut events: EventReader<QuestCompleteEvent>,
    mut query: Query<&Quest>,
    mut trust: Query<&mut TrustCredits>,
) {
    for event in events.read() {
        if let Ok(quest) = query.get(event.0) {
            if quest.completed {
                if let Ok(mut player_trust) = trust.get_mut(event.0) {
                    player_trust.0 += quest.reward_mercy;
                    info!("Quest reward synced — +{} mercy", quest.reward_mercy);
                }
            }
        }
    }
}
