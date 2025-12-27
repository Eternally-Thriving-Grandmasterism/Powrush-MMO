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
pub struct QuestCompleteEvent(pub Entity);

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<QuestCompleteEvent>()
           .add_systems(Startup, quest_spawn_system)
           .add_systems(Update, (quest_progress_system, quest_reward_system));
    }
}

// Spawn varied quests for each player
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
            reward_mercy: rng.gen_range(20.0..100.0),
            completed: false,
        });
        info!("Quest spawned — {:?}", kind);
    }
}

// Progress based on kind (time, events, lattice)
fn quest_progress_system(
    mut query: Query<&mut Quest>,
    time: Res<Time>,
    lattice: Res<LatticeStats>,
) {
    for mut quest in &mut query {
        if !quest.completed {
            match quest.kind {
                QuestKind::Explore => quest.progress += time.delta_seconds() * 5.0,
                QuestKind::Share => quest.progress += time.delta_seconds() * 2.0,  // Trade events boost
                QuestKind::MercyWave => quest.progress += time.delta_seconds() * 3.0,
                QuestKind::LatticeGrow => quest.progress += lattice.nodes as f32 * 0.1,
            }
            if quest.progress >= quest.goal {
                quest.completed = true;
                info!("Quest complete — {}", quest.name);
            }
        }
    }
}

// Kind-specific rewards on complete
fn quest_reward_system(
    mut events: EventReader<QuestCompleteEvent>,
    mut query: Query<&mut Quest>,
    mut trust: Query<&mut TrustCredits>,
    mut lattice: ResMut<LatticeStats>,
) {
    for event in events.read() {
        if let Ok(quest) = query.get_mut(event.0) {
            if quest.completed {
                match quest.kind {
                    QuestKind::Explore => {
                        if let Ok(mut player_trust) = trust.get_mut(event.0) {
                            player_trust.0 += quest.reward_mercy;
                        }
                    }
                    QuestKind::Share => {
                        if let Ok(mut player_trust) = trust.get_mut(event.0) {
                            player_trust.0 += quest.reward_mercy * 1.5;
                        }
                    }
                    QuestKind::MercyWave => {
                        lattice.nodes += 10;
                    }
                    QuestKind::LatticeGrow => {
                        lattice.connections += 20;
                    }
                }
                info!("Quest rewarded — {} mercy", quest.reward_mercy);
            }
        }
    }
}
