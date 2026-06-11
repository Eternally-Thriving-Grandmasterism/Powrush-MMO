/*!
 * FMOD Audio Prototype (Enhanced)
 *
 * This module provides a basic but functional prototype for
 * integrating FMOD as a high-quality 3D/spatial audio backend.
 *
 * Requirements to run:
 * - FMOD Engine SDK installed and linked
 * - FMOD Studio banks in assets/fmod/Desktop/
 */

use bevy::prelude::*;
use bevy_fmod::prelude::*;

/// Resource holding the FMOD Studio instance
#[derive(Resource)]
pub struct FmodAudio {
    pub studio: Studio,
}

/// Event to play a 3D FMOD event (for prototype testing)
#[derive(Event, Debug)]
pub struct FmodPlay3DEvent {
    pub event_path: String,
    pub position: Vec3,
    pub volume: f32,
}

impl FmodPlay3DEvent {
    pub fn new(event_path: impl Into<String>, position: Vec3) -> Self {
        Self {
            event_path: event_path.into(),
            position,
            volume: 1.0,
        }
    }
}

pub struct FmodAudioPlugin;

impl Plugin for FmodAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(FmodPlugin {
                banks: vec![
                    "assets/fmod/Desktop/Master.bank".to_string(),
                    "assets/fmod/Desktop/Master.strings.bank".to_string(),
                ],
                ..default()
            })
            .init_resource::<FmodAudio>()
            .add_event::<FmodPlay3DEvent>()
            .add_systems(Startup, init_fmod)
            .add_systems(Update, play_fmod_3d_events);
    }
}

fn init_fmod(mut commands: Commands, studio: Res<Studio>) {
    commands.insert_resource(FmodAudio {
        studio: studio.clone(),
    });
    info!("[FMOD] FMOD Studio initialized (prototype)");
}

/// System that plays 3D FMOD events when FmodPlay3DEvent is sent
fn play_fmod_3d_events(
    mut events: EventReader<FmodPlay3DEvent>,
    fmod: Res<FmodAudio>,
) {
    for event in events.read() {
        if let Ok(instance) = fmod.studio.create_event_instance(&event.event_path) {
            let attributes = Attributes3D {
                position: event.position.into(),
                velocity: Vec3::ZERO.into(),
                forward: Vec3::Z.into(),
                up: Vec3::Y.into(),
            };

            if let Err(e) = instance.set_3d_attributes(attributes) {
                warn!("[FMOD] Failed to set 3D attributes: {}", e);
                continue;
            }

            if let Err(e) = instance.set_volume(event.volume as f64) {
                warn!("[FMOD] Failed to set volume: {}", e);
            }

            if let Err(e) = instance.start() {
                warn!("[FMOD] Failed to start event '{}': {}", event.event_path, e);
            } else {
                debug!("[FMOD] Played 3D event: {}", event.event_path);
            }
        }
    }
}
