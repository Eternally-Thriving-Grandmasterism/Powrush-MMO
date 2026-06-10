/*!
 * Divine Whispers - Client Side
 *
 * Receives DivineWhisperTrigger events from the server and displays them.
 */

use bevy::prelude::*;
use simulation::divine_whispers::DivineWhisperTrigger;

pub struct DivineWhispersPlugin;

impl Plugin for DivineWhispersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DivineWhisperTrigger>()
            .add_systems(Update, receive_divine_whispers);
    }
}

fn receive_divine_whispers(
    mut events: EventReader<DivineWhisperTrigger>,
    // In real implementation, this would update UI, play sound, etc.
) {
    for event in events.read() {
        println!(
            "[Client] Received Divine Whisper for player {}: '{}' (flavor: {}, intensity: {:.2})",
            event.player_id, event.text, event.flavor, event.intensity
        );

        // TODO: Update Divine Whispers UI panel
        // TODO: Play subtle sound / particle effect based on flavor and intensity
        // TODO: Display with proper duration (event.duration_seconds)
    }
}
