use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct DialogueTree {
    pub current_node: usize,
    pub nodes: Vec<DialogueNode>,
}

#[derive(Clone, Replicated)]
pub struct DialogueNode {
    pub text: String,
    pub choices: Vec<DialogueChoice>,
}

#[derive(Clone, Replicated)]
pub struct DialogueChoice {
    pub text: String,
    pub next_node: usize,
    pub mercy_reward: f32,
}

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, dialogue_interaction_system);
    }
}

fn dialogue_interaction_system(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    npcs: Query<(Entity, &DialogueTree)>,
) {
    if keyboard.just_pressed(KeyCode::E) {
        for (entity, tree) in &npcs {
            let node = &tree.nodes[tree.current_node];
            info!("NPC says: {}", node.text);
            // Player chooses → mercy reward + next node
            commands.entity(entity).insert(tree.clone());
        }
    }
}
