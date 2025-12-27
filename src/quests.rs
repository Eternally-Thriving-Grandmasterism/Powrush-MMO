use bevy::prelude::*;
use bevy_replicon::prelude::*;
use rand::Rng;

#[derive(Component, Replicated)]
pub struct Quest {
    pub name: String,
    pub description: String,
    pub kind: QuestKind,
    pub progress: f32,
    pub goal: f32,
    pub reward_mercy: f32,
    pub reward_item: Option<Item>,
}

#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum QuestKind {
    Explore,      // Visit new terrain
    Share,        // Trade X items
    MercyWave,    // Trigger Y forgiveness
    LatticeGrow,  // Add Z nodes
}

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<QuestCompleteEvent>()
           .add_systems(Update, (quest_progress_system, quest_reward_system, quest_variety_spawn));
    }
}

fn quest_variety_spawn(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.01 * time.delta_seconds()) {
        for player in &players {
            let kind = match rng.gen_range(0..4) {
                0 => QuestKind::Explore,
                1 => QuestKind::Share,
                2 => QuestKind::MercyWave,
                _ => QuestKind::LatticeGrow,
            };
            commands.entity(player).insert(Quest {
                name: format!("{:?} Quest", kind),
                description: "Complete mercy task".to_string(),
                kind,
                progress: 0.0,
                goal: 10.0,
                reward_mercy: 50.0,
                reward_item: None,
            });
            info!("New quest spawned — {:?}", kind);
        }
    }
}

fn quest_progress_system(
    mut query: Query<&mut Quest>,
    time: Res<Time>,
) {
    for mut quest in &mut query {
        quest.progress += time.delta_seconds() * 0.2;  // Varied by kind in full
        if quest.progress >= quest.goal {
            quest.progress = quest.goal;
        }
    }
}

fn quest_reward_system(
    mut events: EventReader<QuestCompleteEvent>,
    mut query: Query<&mut Quest>,
    mut trust: Query<&mut TrustCredits>,
    mut inventory: Query<&mut Inventory>,
) {
    for event in events.read() {
        if let Ok(mut quest) = query.get_mut(event.0) {
            if quest.progress >= quest.goal {
                if let Ok(mut player_trust) = trust.get_mut(event.0) {
                    player_trust.0 += quest.reward_mercy;
                }
                if let Some(item) = &quest.reward_item {
                    if let Ok(mut inv) = inventory.get_mut(event.0) {
                        inv.items.push(item.clone());
                    }
                }
                info!("Quest complete — {} mercy awarded", quest.reward_mercy);
            }
        }
    }
}
