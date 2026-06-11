/*!
 * FMOD Audio Prototype (bevy_fmod exploration)
 *
 * This is a starting point for integrating FMOD as a potential
 * high-quality 3D/spatial audio backend.
 *
 * Requirements:
 * - FMOD Engine SDK
 * - FMOD Studio banks (.bank files)
 * - bevy_fmod crate
 */

use bevy::prelude::*;
use bevy_fmod::prelude::*;

/// Resource to hold the FMOD Studio system
#[derive(Resource)]
pub struct FmodAudio {
    pub studio: Studio,
}

pub struct FmodAudioPlugin;

impl Plugin for FmodAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(FmodPlugin {
                // Path to your FMOD banks
                banks: vec![
                    "assets/fmod/Desktop/Master.bank".to_string(),
                    "assets/fmod/Desktop/Master.strings.bank".to_string(),
                ],
                ..default()
            })
            .init_resource::<FmodAudio>()
            .add_systems(Startup, init_fmod);
    }
}

fn init_fmod(mut commands: Commands, studio: Res<Studio>) {
    commands.insert_resource(FmodAudio {
        studio: studio.clone(),
    });

    info!("[FMOD] FMOD Studio initialized (prototype)");
}

/// Example system to play a 3D spatial event
pub fn play_3d_fmod_event(
    fmod: Res<FmodAudio>,
    position: Vec3,
    event_path: &str,
) {
    if let Ok(event) = fmod.studio.create_event_instance(event_path) {
        // Set 3D attributes (position, forward, up)
        let attributes = Attributes3D {
            position: position.into(),
            velocity: Vec3::ZERO.into(),
            forward: Vec3::Z.into(),
            up: Vec3::Y.into(),
        };

        if let Err(e) = event.set_3d_attributes(attributes) {
            warn!("Failed to set 3D attributes: {}", e);
        }

        if let Err(e) = event.start() {
            warn!("Failed to start FMOD event: {}", e);
        }
    }
}
