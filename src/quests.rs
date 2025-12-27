use bevy::prelude::*;
use bevy_replicon::prelude::*;
use rand::Rng;

#[derive(Component, Replicated)]
pub struct Quest {
    pub name: String,
    pub kind: QuestKind,
    pub progress: f32,
    pub goal: f32,
    pub mercy_reward: f32,
    pub item_reward: Option<Item>,
    pub trust_bonus: f32,
}

#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum QuestKind {
    Explore,     // Trust + item
    Share,       // Mercy points
    MercyWave,   // Lattice node
    LatticeGrow, // Guild bonus
}

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<QuestCompleteEvent>()
           .add_systems(Update, (quest_progress_system, quest_kind_reward_system));
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
        }
    }
}

fn quest_kind_reward_system(
    mut events: EventReader<QuestCompleteEvent>,
    mut query: Query<&mut Quest>,
    mut trust: Query<&mut TrustCredits>,
    mut inventory: Query<&mut Inventory>,
    mut lattice: ResMut<LatticeStats>,
) {
    for event in events.read() {
        if let Ok(quest) = query.get_mut(event.0) {
            if quest.progress >= quest.goal {
                // Kind-specific rewards
                match quest.kind {
                    QuestKind::Explore => {
                        if let Ok(mut player_trust) = trust.get_mut(event.0) {
                            player_trust.0 += quest.trust_bonus;
                        }
                        if let Some(item) = &quest.item_reward {
                            if let Ok(mut inv) = inventory.get_mut(event.0) {
                                inv.items.push(item.clone());
                            }
                        }
                    }
                    QuestKind::Share => {
                        if let Ok(mut player_trust) = trust.get_mut(event.0) {
                            player_trust.0 += quest.mercy_reward;
                        }
                    }
                    QuestKind::MercyWave => {
                        lattice.nodes += 5;
                    }
                    QuestKind::LatticeGrow => {
                        lattice.connections += 10;
                    }
                }
                info!("Quest complete — {} mercy", quest.mercy_reward);
            }
        }
    }
}
