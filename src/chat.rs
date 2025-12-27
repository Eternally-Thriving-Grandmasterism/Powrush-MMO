use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct ChatMessage {
    pub channel: ChatChannel,
    pub sender: String,
    pub text: String,
    pub position: Vec3,
}

#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum ChatChannel {
    Global,
    Proximity(f32),  // Radius in meters
    Guild,
}

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, proximity_chat_filter);
    }
}

fn proximity_chat_filter(
    messages: Query<&ChatMessage>,
    players: Query<&GlobalTransform, With<Player>>,
) {
    let player_pos = players.single().translation();
    for msg in &messages {
        if let ChatChannel::Proximity(radius) = msg.channel {
            if player_pos.distance(msg.position) <= radius {
                // Render message
            }
        } else {
            // Global/guild always render
        }
    }
}
