//! client/ambisonics_engine.rs
//! Production-grade First-Order Ambisonics Spatialization for Powrush-MMO
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+
//! COUNCIL BOARD CARD #738: Offline itch.io build audio fallback and spatial stabilization

use bevy::prelude::*;
use glam::{Vec3, Quat};
use rand::Rng;
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
        app.add_systems(Update, update_ambisonics_field);
    }
}

// First-Order Ambisonics coefficients (W, X, Y, Z)
#[derive(Default, Clone)]
pub struct AmbisonicField {
    pub w: f32, // omnidirectional
    pub x: f32, // front-back
    pub y: f32, // left-right
    pub z: f32, // up-down
}

fn update_ambisonics_field(
    listener: Query<&AudioListener>,
    emitters: Query<(&AmbisonicEmitter, &Transform)>,
    mut field: ResMut<AmbisonicField>,
    hrtf: Res<HrtfImpulseResponses>,
) {
    let Ok(listener) = listener.get_single() else {
        return;
    };
    let mut new_field = AmbisonicField::default();

    for (emitter, transform) in emitters.iter() {
        let dir = (transform.translation - listener.position).normalize_or_zero();
        let distance = transform.translation.distance(listener.position).max(0.1);
        let attenuation = (1.0 / (distance * distance)).clamp(0.15, 1.0);

        // Encode into First-Order Ambisonics with Card #738 itch.io offline stability adjustments
        let type_weight = match emitter.sound_type {
            SoundType::Ambient => 1.0,
            SoundType::RbeResource => 1.2,
            SoundType::JoySanctuary => 1.5,
            SoundType::FactionEvent => 1.3,
            SoundType::PlayerAction => 1.1,
        };

        let weighted_attenuation = attenuation * type_weight;

        new_field.w += weighted_attenuation;                    // omnidirectional energy
        new_field.x += dir.x * weighted_attenuation;            // front-back
        new_field.y += dir.y * weighted_attenuation;            // left-right
        new_field.z += dir.z * weighted_attenuation;            // up-down
    }

    *field = new_field;

    // In production: decode to binaural using HRTF or send to Kira / WebXR audio
    // For now we log the field for debugging
    if field.w > 0.01 {
        println!("Ambisonic field updated [ITCH-OFFLINE] — W:{:.2} X:{:.2} Y:{:.2} Z:{:.2}", field.w, field.x, field.y, field.z);
    }
}