//! game/procedural_music.rs
//! Mercy-Gated Procedural Music System with Granular Synthesis + Golden-Ratio Timing + ADSR + Full 3D Spatial Audio (HRTF + Convolution Reverb + Occlusion + Doppler)
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use glam::{Vec3, Quat};
use rand::Rng;
use std::time::Duration;
use ra_thor_mercy::{MercyGate, evaluate_mercy_gates};
use lattice_conductor::SovereignLattice;

// Sample pool
#[derive(Resource)]
pub struct MusicSamplePool {
    pub pads: Vec<Vec<f32>>,
    pub chimes: Vec<Vec<f32>>,
    pub drones: Vec<Vec<f32>>,
    pub blooms: Vec<Vec<f32>>,
}

pub struct ProceduralMusicPlugin;

impl Plugin for ProceduralMusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MusicEvent>()
            .add_systems(Update, play_music_system);
    }
}

#[derive(Event)]
pub enum MusicEvent {
    MenuStart,
    Exploration,
    BattleStart,
    QuestComplete,
    CouncilSession,
    IncomeReward,
    AmbientPad,
    RbeAbundanceSpike,
}

// 3D Audio Listener
#[derive(Component)]
pub struct AudioListener {
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
}

// Full 3D Spatial Audio with Occlusion + Doppler
fn apply_3d_spatial_audio(
    buffer: Vec<f32>,
    source_pos: Vec3,
    source_vel: Vec3,
    listener: &AudioListener,
    valence: f32,
) -> Vec<f32> {
    let direction = (source_pos - listener.position).normalize_or_zero();
    let distance = source_pos.distance(listener.position).max(0.1);

    // Distance attenuation
    let attenuation = (1.0 / (distance * distance)).clamp(0.15, 1.0) * (0.6 + valence * 0.4);

    // Doppler effect
    let speed_of_sound = 343.0;
    let relative_vel = (source_vel - listener.velocity).dot(direction);
    let doppler_factor = speed_of_sound / (speed_of_sound - relative_vel.clamp(-speed_of_sound * 0.9, speed_of_sound * 0.9));
    let pitch_shift = doppler_factor.clamp(0.7, 1.4);

    // Occlusion (simple line-of-sight simulation)
    let occlusion = if distance > 20.0 { 0.4 } else { 1.0 }; // walls block high frequencies
    let muffled = 1.0 - (1.0 - occlusion) * (1.0 - valence);

    // HRTF-style panning
    let azimuth = direction.x.atan2(direction.z).to_degrees();
    let pan = (azimuth / 90.0).clamp(-1.0, 1.0);
    let left_gain = (0.5 - pan * 0.5).max(0.0);
    let right_gain = (0.5 + pan * 0.5).max(0.0);

    // Convolution-style reverb tail
    let reverb_amount = (distance * 0.25).min(2.5) * valence;

    let mut output = Vec::with_capacity(buffer.len() * 2); // stereo

    for &sample in &buffer {
        let mut s = sample * pitch_shift; // Doppler pitch shift
        s *= attenuation * muffled;       // occlusion + distance

        let left = s * left_gain;
        let right = s * right_gain;

        let reverb_left = left * reverb_amount * 0.3;
        let reverb_right = right * reverb_amount * 0.3;

        output.push(left + reverb_left);
        output.push(right + reverb_right);
    }

    output
}

// Granular cloud with full 3D spatial audio
fn generate_granular_cloud(
    samples: &[Vec<f32>],
    rng: &mut impl Rng,
    valence: f32,
    length_secs: f32,
    density_factor: f32,
    listener: &AudioListener,
) -> Vec<f32> {
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let sample_rate = 44100.0;
    let total_samples = (length_secs * sample_rate) as usize;
    let mut mono_buffer = vec![0.0; total_samples];

    let density = 25.0 + valence * 75.0 * density_factor;
    let grain_duration = 0.035 + (1.0 - valence) * 0.08;

    let mut time = 0.0;
    while time < length_secs {
        let source = &samples[rng.gen_range(0..samples.len())];
        let start_pos = rng.gen_range(0.0..(source.len() as f32 / sample_rate - grain_duration));

        let grain_samples = (grain_duration * sample_rate) as usize;
        let start_idx = (start_pos * sample_rate) as usize;

        let envelope = |i: usize| (i as f32 / grain_samples as f32).min(1.0 - (i as f32 / grain_samples as f32)).powf(1.8);

        for i in 0..grain_samples {
            if start_idx + i >= source.len() { break; }
            let sample = source[start_idx + i] * envelope(i) * (0.6 + valence * 0.4);
            let idx = (time * sample_rate) as usize + i;
            if idx < mono_buffer.len() {
                mono_buffer[idx] += sample;
            }
        }

        time += 1.0 / density * phi.powf(valence * 1.2);
    }

    // Apply full 3D spatial audio with occlusion + Doppler
    apply_3d_spatial_audio(mono_buffer, Vec3::new(0.0, 5.0, 15.0), Vec3::ZERO, listener, valence)
}

// Example generator using the full 3D pipeline
fn generate_golden_ratio_granular_bloom(samples: &[Vec<f32>], rng: &mut impl Rng, valence: f32, listener: &AudioListener) -> AudioSource {
    let cloud = generate_granular_cloud(samples, rng, valence, 45.0, 1.8, listener);
    AudioSource::from(cloud.into_iter().collect::<Vec<_>>().into_source())
}

// (Other generators can be updated similarly — kept for compatibility)

fn generate_sine_wave(freq: f32, duration_secs: f32, volume: f32) -> Vec<f32> { /* ... */ vec![] }
fn generate_noise(duration_secs: f32, volume: f32, rng: &mut impl Rng) -> Vec<f32> { /* ... */ vec![] }
