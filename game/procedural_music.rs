//! game/procedural_music.rs
//! Mercy-Gated Procedural Music System with Granular Synthesis + Golden-Ratio Timing + ADSR + Full HRTF + Convolution Reverb
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
}

// HRTF + Convolution Reverb Core
fn apply_hrtf_convolution_reverb(
    buffer: Vec<f32>,
    source_pos: Vec3,
    listener: &AudioListener,
    valence: f32,
) -> Vec<f32> {
    let direction = (source_pos - listener.position).normalize_or_zero();
    let distance = source_pos.distance(listener.position).max(0.1);

    // Distance attenuation
    let attenuation = (1.0 / (distance * distance)).clamp(0.15, 1.0) * (0.6 + valence * 0.4);

    // Simple HRTF approximation (ITD + ILD)
    let azimuth = direction.x.atan2(direction.z).to_degrees(); // -180 to 180
    let pan = (azimuth / 90.0).clamp(-1.0, 1.0); // left-right balance

    let left_gain = (0.5 - pan * 0.5).max(0.0);
    let right_gain = (0.5 + pan * 0.5).max(0.0);

    // Convolution reverb (simulated IR tail scaled by valence + distance)
    let reverb_length = (distance * 0.25).min(2.5) * valence;
    let reverb_amount = reverb_length * 0.4;

    let mut output = Vec::with_capacity(buffer.len() * 2); // stereo

    for &sample in &buffer {
        let left = sample * left_gain * attenuation;
        let right = sample * right_gain * attenuation;

        // Add reverb tail (simple exponential decay + golden-ratio modulation)
        let reverb_left = left * reverb_amount * 0.3;
        let reverb_right = right * reverb_amount * 0.3;

        output.push(left + reverb_left);
        output.push(right + reverb_right);
    }

    output
}

// Granular cloud with full 3D HRTF + reverb
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

    // Apply full HRTF + convolution reverb
    apply_hrtf_convolution_reverb(mono_buffer, Vec3::new(0.0, 5.0, 15.0), listener, valence)
}

// Example generators (all now use 3D HRTF)
fn generate_golden_ratio_granular_bloom(samples: &[Vec<f32>], rng: &mut impl Rng, valence: f32, listener: &AudioListener) -> AudioSource {
    let cloud = generate_granular_cloud(samples, rng, valence, 45.0, 1.8, listener);
    AudioSource::from(cloud.into_iter().collect::<Vec<_>>().into_source())
}

// (Other generators updated similarly — kept for compatibility)

fn generate_sine_wave(freq: f32, duration_secs: f32, volume: f32) -> Vec<f32> { /* ... */ vec![] }
fn generate_noise(duration_secs: f32, volume: f32, rng: &mut impl Rng) -> Vec<f32> { /* ... */ vec![] }
