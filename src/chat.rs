use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_kira_audio::prelude::*;
use egui;

#[derive(Component, Replicated)]
pub struct ChatMessage {
    pub channel: ChatChannel,
    pub sender: String,
    pub text: String,
    pub trust: f32,
}

#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum ChatChannel {
    Global,
    Guild,
    Whisper(Entity),
}

#[derive(Component)]
pub struct ChatInput {
    pub text: String,
    pub channel: ChatChannel,
}

#[derive(Event, Replicated)]
pub struct SendChatEvent(pub String, pub ChatChannel);

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SendChatEvent>()
           .insert_resource(ChatInput { text: String::new(), channel: ChatChannel::Global })
           .add_systems(Update, (chat_input_system, chat_send_system, chat_render_system, voice_chat_system));
    }
}

fn chat_input_system(
    mut input: ResMut<ChatInput>,
    mut egui_ctx: EguiContexts,
    keyboard: Res<Input<KeyCode>>,
) {
    egui::Window::new("Chat")
        .default_pos([10.0, 400.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut input.channel, ChatChannel::Global, "Global");
                ui.selectable_value(&mut input.channel, ChatChannel::Guild, "Guild");
                ui.text_edit_singleline(&mut input.text);
                if ui.button("Send").clicked() || keyboard.just_pressed(KeyCode::Return) {
                    if !input.text.is_empty() {
                        app.world_mut().send_event(SendChatEvent(input.text.clone(), input.channel));
                        input.text.clear();  // Fixed clearing
                    }
                }
                // Emoji picker
                if ui.button("😇").clicked() {
                    input.text.push('😇');
                }
            });
        });
}

fn chat_send_system(
    mut events: EventWriter<SendChatEvent>,
    input: Res<ChatInput>,
) {
    if !input.text.is_empty() {
        events.send(SendChatEvent(input.text.clone(), input.channel));
    }
}

fn chat_render_system(
    mut commands: Commands,
    messages: Query<&ChatMessage>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            bottom: Val::Px(100.0),
            width: Val::Px(600.0),
            height: Val::Px(300.0),
            flex_direction: FlexDirection::ColumnReverse,
            ..default()
        },
        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.7)),
        ..default()
    }).with_children(|parent| {
        for msg in messages.iter().rev().take(15) {
            let color = if msg.trust > 50.0 { Color::GOLD } else { Color::CYAN };
            let prefix = match msg.channel {
                ChatChannel::Global => "[Global]",
                ChatChannel::Guild => "[Guild]",
                ChatChannel::Whisper(_) => "[Whisper]",
            };
            parent.spawn(TextBundle::from_section(
                format!("{} {}: {}", prefix, msg.sender, msg.text),
                TextStyle { font_size: 20.0, color, ..default() },
            ));
        }
    });
}

fn voice_chat_system(
    audio: Res<Audio>,
    microphone: Res<Microphone>,
    trust: Query<&TrustCredits>,
) {
    let trust_level = trust.single().0;
    audio.play(microphone.clone()).with_volume(trust_level / 100.0);  // Trust-modulated volume
}
