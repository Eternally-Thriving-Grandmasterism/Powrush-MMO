/*!
 * FMOD Audio Prototype (Hybrid Abstraction)
 *
 * FMOD now listens to the same GameAudioEvent as Kira,
 * demonstrating true backend-agnostic audio events.
 */

use bevy::prelude::*;
use bevy_fmod::prelude::*;

#[derive(Resource)]
pub struct FmodAudio {
    pub studio: Studio,
}

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
            .add_systems(Update, (
                play_fmod_3d_events,
                handle_game_audio_events_for_fmod,
            ));
    }
}

fn init_fmod(mut commands: Commands, studio: Res<Studio>) {
    commands.insert_resource(FmodAudio {
        studio: studio.clone(),
    });
    info!("[FMOD] FMOD Studio initialized (hybrid mode)");
}

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

            let _ = instance.set_3d_attributes(attributes);
            let _ = instance.set_volume(event.volume as f64);
            let _ = instance.start();
        }
    }
}

/// Listens to GameAudioEvent and translates to FMOD events
fn handle_game_audio_events_for_fmod(
    mut game_events: EventReader<crate::spatial_audio::GameAudioEvent>,
    mut fmod_events: EventWriter<FmodPlay3DEvent>,
) {
    for event in game_events.read() {
        match event {
            crate::spatial_audio::GameAudioEvent::Epiphany { position, intensity } => {
                fmod_events.send(FmodPlay3DEvent::new(
                    "event:/Epiphany/Impact",
                    *position,
                ));
            }
            crate::spatial_audio::GameAudioEvent::Harvest { position, .. } => {
                fmod_events.send(FmodPlay3DEvent::new(
                    "event:/World/HarvestImpact",
                    *position,
                ));
            }
        }
    }
}
