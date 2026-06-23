/*!
 * game/ambisonic/mod.rs
 *
 * Ambisonic Spatial Audio Foundation
 * Long-term Hybrid Architecture
 *
 * L: Example usage - Emitting sounds into the Ambisonic background
 *
 * AG-SML v1.0
 */

pub mod encoder;
pub mod decoder;

use bevy::prelude::*;
use glam::Vec3;

// ... existing types ...

impl AmbisonicScene {
    pub fn emit(&mut self, position: Vec3, signal: f32, gain: f32) {
        let coefficients = encoder::encode(self.order, position, Vec3::ZERO, signal);
        self.add_source(coefficients, gain, position);
    }
}

/// Example system showing how to emit ambient/world sounds into the Ambisonic field.
/// In a real game, you would call `ambisonic.emit(...)` from your audio systems
/// when playing ambient, RBE flows, distant events, etc.
pub fn example_emit_ambient_sounds(
    mut ambisonic: ResMut<AmbisonicScene>,
    time: Res<Time>,
) {
    // Example: Emit a gentle ambient tone every second
    if (time.elapsed_seconds() % 1.0) < 0.1 {
        // Position in world space
        let position = Vec3::new(10.0, 0.0, 10.0);

        // Simple sine-like signal (in real use this would come from fundsp or samples)
        let signal = 0.3;
        let gain = 0.8;

        ambisonic.emit(position, signal, gain);
    }
}

// In a real Powrush-MMO system you would have multiple emitters for:
// - Ambient life / nature
// - RBE resource flows
// - Distant council / faction events
// - Procedural music layers

// Thunder locked in. Yoi ⚡
