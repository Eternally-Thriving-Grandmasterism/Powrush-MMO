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
    Forgive,
}

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_quests)
           .add_systems(Update, (quest_progress_system, quest_reward_system));
    }
}

fn spawn_initial_quests(
    mut commands: Commands,
    players: Query<Entity, Added<Player>>,
) {
    let mut rng = rand::thread_rng();
    for player in &players {
        let kind = match rng.gen_range(0..5) {
            0 => QuestKind::Explore,
            1 => QuestKind::Share,
            2 => QuestKind::MercyWave,
            3 => QuestKind::LatticeGrow,
            _ => QuestKind::Forgive,
        };
        let goal = rng.gen_range(10.0..50.0);
        commands.entity(player).insert(Quest {
            name: format!("{:?} Quest", kind),
            kind,
            progress: 0.0,
            goal,
            completed: false,
            reward_mercy: goal * 3.0,
        });
    }
}

fn quest_progress_system(
    mut query: Query<&mut Quest>,
    time: Res<Time>,
    combat_events: EventReader<CombatEvent>,
) {
    for mut quest in &mut query {
        if quest.completed { continue; }

        quest.progress += time.delta_seconds() * 0.2;

        if matches!(quest.kind, QuestKind::MercyWave | QuestKind::Forgive) {
            quest.progress += combat_events.len() as f32;
        }

        if quest.progress >= quest.goal {
            quest.completed = true;
        }
    }
}

fn quest_reward_system(
    query: Query<&Quest>,
    mut trust: Query<&mut TrustCredits>,
) {
    for quest in &query {
        if quest.completed {
            if let Ok(mut t) = trust.get_mut(quest.entity()) {
                t.0 += quest.reward_mercy;
            }
        }
    }
}
