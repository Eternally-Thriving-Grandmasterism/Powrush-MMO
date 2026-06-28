/*!
 * Audio Plugin - With Smooth Priority Ducking
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy::asset::AssetApp;
use crate::settings::audio_mixing::{AudioMixer, update_dynamic_audio_volumes, DuckingState};

// ... other imports ...

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            // ... previous inits ...
            .init_resource::<AudioMixer>()
            .init_resource::<DuckingState>()
            // ... asset registrations ...
            .add_systems(Update, (
                // ... other systems ...
                update_dynamic_audio_volumes, // Now includes smooth ducking
            ));
    }
}
