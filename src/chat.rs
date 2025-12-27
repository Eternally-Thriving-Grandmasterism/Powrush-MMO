use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_egui::{egui, EguiContexts};

#[derive(Component, Replicated)]
pub struct ChatMessage {
    pub sender: String,
    pub text: String,
    pub trust: f32,
    pub emote: Option<String>,  // Emoji or custom
}

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            chat_input_system,
            chat_render_system,
            voice_chat_system,
            emote_system,
        ))
        .add_event::<SendChatEvent>()
        .add_event::<VoiceChatEvent>();
    }
}

fn chat_input_system(
    mut events: EventWriter<SendChatEvent>,
    mut contexts: EguiContexts,
    keyboard: Res<Input<KeyCode>>,
) {
    egui::Window::new("Chat")
        .default_pos([10.0, 400.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                let mut input = String::new();
                ui.text_edit_singleline(&mut input);
                if ui.button("Send").clicked() || keyboard.just_pressed(KeyCode::Enter) {
                    events.send(SendChatEvent(input));
                }
                // Emoji picker
                if ui.button("😊").clicked() {
                    events.send(SendChatEvent("😇".to_string()));
                }
            });
        });
}

fn emote_system(
    mut events: EventReader<SendChatEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        if event.0.starts_with('/') {
            // Custom emote (e.g., /dance)
            commands.spawn(EmoteBundle {
                emote: Emote { name: event.0.clone() },
                particle_burst: ParticleBurst::new("gold"),
            });
        }
    }
}

fn voice_chat_system(
    mut audio: ResMut<Audio>,
    microphone: Res<Microphone>,
    events: EventReader<VoiceChatEvent>,
) {
    for _ in events.read() {
        // Procedural voice modulation (pitch shift based on trust)
        audio.play(microphone.clone()).with_volume(1.0);
    }
}
