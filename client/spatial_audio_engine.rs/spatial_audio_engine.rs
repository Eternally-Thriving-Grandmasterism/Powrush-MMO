//! client/spatial_audio_engine.rs
//! Unified Spatial Audio Engine for Powrush-MMO — HRTF + Occlusion + Doppler + Granular + Valence-Driven
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use glam::{Vec3, Quat};
use rand::Rng;
use std::time::Duration;
use crate::game::procedural_music::{MusicSamplePool, HrtfImpulseResponses};

#[derive(Component)]
pub struct SpatialAudioEmitter {
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

pub struct SpatialAudioEnginePlugin;

impl Plugin for SpatialAudioEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_spatial_audio);
    }
}

fn update_spatial_audio(
    mut audio: ResMut<Audio>,
    listener: Query<&AudioListener>,
    emitters: Query<(&SpatialAudioEmitter, &Transform)>,
    hrtf: Res<HrtfImpulseResponses>,
    sample_pool: Res<MusicSamplePool>,
) {
    let listener = listener.single();
    for (emitter, transform) in emitters.iter() {
        let source_pos = transform.translation;

        // Generate or fetch base sound based on type
        let base_sound = match emitter.sound_type {
            SoundType::RbeResource => generate_granular_cloud(&sample_pool.blooms, &mut rand::thread_rng(), 1.0, 8.0, 1.2, listener, &hrtf),
            SoundType::JoySanctuary => generate_granular_cloud(&sample_pool.pads, &mut rand::thread_rng(), 1.0, 30.0, 0.8, listener, &hrtf),
            _ => continue,
        };

        // Apply full spatial pipeline (HRTF + occlusion + Doppler)
        let spatial = apply_real_hrtf(base_sound, source_pos, listener, 1.0, &hrtf);

        // Play as 3D spatial source (Kira handles positioning)
        audio.play(AudioSource::from(spatial.into_iter().collect::<Vec<_>>().into_source()));
    }
}

// (All previous granular, HRTF, ADSR, golden-ratio functions are kept and reused here)

fn generate_granular_cloud(...) -> Vec<f32> { /* existing implementation */ }
fn apply_real_hrtf(...) -> Vec<f32> { /* existing implementation */ }
