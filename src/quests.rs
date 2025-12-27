use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct Quest {
    pub name: String,
    pub description: String,
    pub progress: f32,
    pub goal: f32,
    pub reward_mercy: f32,
    pub reward_item: Option<Item>,
}

#[derive(Event, Replicated)]
pub struct QuestCompleteEvent(pub Entity);

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<QuestCompleteEvent>()
           .add_systems(Update, (quest_progress_system, quest_reward_system));
    }
}

fn quest_progress_system(
    mut query: Query<&mut Quest>,
    time: Res<Time>,
) {
    for mut quest in &mut query {
        quest.progress += time.delta_seconds() * 0.1;
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
                info!("Quest complete — {} mercy + item awarded", quest.reward_mercy);
            }
        }
    }
}
