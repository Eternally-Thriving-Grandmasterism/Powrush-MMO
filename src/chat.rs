use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct ChatMessage {
    pub sender: String,
    pub text: String,
    pub trust: f32,
}

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (chat_input_system, chat_render_system));
    }
}

fn chat_input_system(
    mut events: EventWriter<SendChatEvent>,
    keyboard: Res<Input<KeyCode>>,
) {
    // Simple enter to send (full UI in next)
    if keyboard.just_pressed(KeyCode::Return) {
        events.send(SendChatEvent("Hello lattice!".to_string()));
    }
}

#[derive(Event)]
pub struct SendChatEvent(pub String);

fn chat_render_system(
    mut commands: Commands,
    messages: EventReader<SendChatEvent>,
) {
    for event in messages.read() {
        commands.spawn(TextBundle::from_section(
            event.0.clone(),
            TextStyle { font_size: 24.0, color: Color::CYAN, ..default() },
        ));
    }
}
