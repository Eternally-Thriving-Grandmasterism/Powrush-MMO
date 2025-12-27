use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct ChatMessage {
    pub sender: String,
    pub text: String,
    pub trust_level: f32,
    pub timestamp: f64,
}

#[derive(Component)]
pub struct ChatInput {
    pub text: String,
}

#[derive(Event, Replicated)]
pub struct SendChatEvent(pub String);

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SendChatEvent>()
           .add_systems(Update, (
                chat_input_system,
                chat_send_system,
                chat_render_system,
           ))
           .insert_resource(ChatInput { text: String::new() });
    }
}

fn chat_input_system(
    mut input: ResMut<ChatInput>,
    keyboard: Res<Input<KeyCode>>,
    mut char_evr: EventReader<ReceivedCharacter>,
) {
    for ev in char_evr.read() {
        if ev.char.is_alphanumeric() || ev.char.is_whitespace() || ":)(".contains(ev.char) {
            input.text.push(ev.char);
        }
    }

    if keyboard.just_pressed(KeyCode::Back) && !input.text.is_empty() {
        input.text.pop();
    }
}

fn chat_send_system(
    mut events: EventWriter<SendChatEvent>,
    input: Res<ChatInput>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Return) && !input.text.is_empty() {
        events.send(SendChatEvent(input.text.clone()));
        // Clear input after send
        input.text.clear();
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
            bottom: Val::Px(20.0),
            width: Val::Px(600.0),
            height: Val::Px(300.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.6)),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Global Chat",
            TextStyle { font_size: 28.0, color: Color::GOLD, ..default() },
        ));

        for msg in messages.iter().rev().take(10) {
            let color = if msg.trust_level > 50.0 { Color::GOLD } else { Color::CYAN };
            parent.spawn(TextBundle::from_section(
                format!("{}: {}", msg.sender, msg.text),
                TextStyle { font_size: 20.0, color, ..default() },
            ));
        }
    });
}
