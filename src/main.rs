/*!
 * src/main.rs
 * Powrush-MMO — Example / Test binary entry (uses dynamic particle spawning)
 *
 * v19.2 — Updated to use SpawnPolicyVisualEffect event for dynamic spawning
 * instead of direct old helper. Demonstrates the new event-driven system.
 *
 * AG-SML v1.0 | Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use simulation::{SpawnPolicyVisualEffect, LissajousKnotPreset};

fn main() {
    // This is a minimal example binary.
    // The real client app is in client/src/app.rs
    println!("Powrush-MMO dynamic particle spawn example");
}

/// Example helper that now uses the dynamic event system.
/// Call this from gameplay systems when a policy activates or RBE event occurs.
pub fn trigger_policy_visual(
    mut commands: Commands,
    preset: LissajousKnotPreset,
    position: Vec3,
) {
    commands.send_event(SpawnPolicyVisualEffect {
        preset,
        position,
    });
}

// Legacy direct spawn removed — now use trigger_policy_visual + SpawnPolicyVisualEffect event.
// The reactive systems and dynamic spawner handle everything from here.
