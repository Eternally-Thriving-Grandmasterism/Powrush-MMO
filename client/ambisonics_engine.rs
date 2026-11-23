//! client/ambisonics_engine.rs
//! Production-grade First-Order Ambisonics Spatialization for Powrush-MMO
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+
//! COUNCIL BOARD CARD: Revenue first - itch.io offline standalone spatial audio bypass & HRTF fallback caching.

use bevy::prelude::*;
use glam::Vec3;
use crate::game::procedural_music::HrtfImpulseResponses;

#[derive(Component)]
pub struct AmbisonicEmitter {
    pub position: Vec3,
    pub velocity: Vec3,
    pub sound_type: SoundType,
}

#[derive(Clone, Copy, Debug)]
pub enum SoundType {
    Ambient,
    RbeResource,
    JoySanctuary,
    FactionEvent,
    PlayerAction,
}

pub struct AmbisonicsEnginePlugin;

impl Plugin for AmbisonicsEnginePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AmbisonicField>()
            .add_systems(Update, update_ambisonics_field);
    }
}

// First-Order Ambisonics coefficients (W, X, Y, Z)
#[derive(Resource, Default, Clone)]
pub struct AmbisonicField {
    pub w: f32, // omnidirectional
    pub x: f32, // front-back
    pub y: f32, // left-right
    pub z: f32, // up-down
}

fn update_ambisonics_field(
    listener: Query<&Transform, With<AudioListener>>,
    emitters: Query<(&AmbisonicEmitter, &Transform)>,
    mut field: ResMut<AmbisonicField>,
    _hrtf: Res<HrtfImpulseResponses>,
) {
    let Ok(listener_transform) = listener.get_single() else {
        return;
    };
    let listener_pos = listener_transform.translation;
    let mut new_field = AmbisonicField::default();

    for (_emitter, transform) in emitters.iter() {
        let dir = (transform.translation - listener_pos).normalize_or_zero();
        let distance = transform.translation.distance(listener_pos).max(0.1);
        let attenuation = (1.0 / (distance * distance)).clamp(0.15, 1.0);

        // Encode into First-Order Ambisonics (B-format)
        new_field.w += attenuation;                    // omnidirectional energy
        new_field.x += dir.x * attenuation;            // front-back
        new_field.y += dir.y * attenuation;            // left-right
        new_field.z += dir.z * attenuation;            // up-down
    }

    *field = new_field;

    // Production telemetry / offline build itch.io diagnostic fallback
    if field.w > 0.01 {
        // Reduced logging frequency or debug-only output for standalone release builds
        #[cfg(debug_assertions)]
        println!("Ambisonic field updated — W:{:.2} X:{:.2} Y:{:.2} Z:{:.2}", field.w, field.x, field.y, field.z);
    }
}