use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct ChatMessage {
    pub channel: ChatChannel,
    pub sender: String,
    pub text: String,
    pub position: Vec3,  // For proximity
}

#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum ChatChannel {
    Global,
    Proximity(f32),  // Radius
    Guild,
}

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (proximity_chat_filter, chat_render_system));
    }
}

fn proximity_chat_filter(
    messages: Query<&ChatMessage>,
    players: Query<&Transform>,
) {
    for msg in &messages {
        if let ChatChannel::Proximity(radius) = msg.channel {
            for player_trans in &players {
                if player_trans.translation.distance(msg.position) < radius {
                    // Render only for nearby players
                }
            }
        }
    }
}

fn chat_render_system(
    mut commands: Commands,
    messages: Query<&ChatMessage>,
) {
    // Render filtered messages
    for msg in messages.iter().take(15) {
        let color = match msg.channel {
            ChatChannel::Global => Color::CYAN,
            ChatChannel::Guild => Color::GOLD,
            ChatChannel::Proximity(_) => Color::WHITE,
        };
        commands.spawn(TextBundle::from_section(
            format!("{}: {}", msg.sender, msg.text),
            TextStyle { font_size: 20.0, color, ..default() },
        ));
    }
}            width: Val::Px(600.0),
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
