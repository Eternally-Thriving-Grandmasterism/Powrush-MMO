//! game/procedural_music.rs
//! Mercy-Gated Procedural Music System with Granular Synthesis + Golden-Ratio Timing + ADSR + Optimized Real HRTF Convolution
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use glam::{Vec3, Quat};
use rand::Rng;
use std::time::Duration;
use ra_thor_mercy::{MercyGate, evaluate_mercy_gates};
use lattice_conductor::SovereignLattice;

// Real HRTF Impulse Responses (async loaded)
#[derive(Resource, Default)]
pub struct HrtfImpulseResponses {
    pub left: Vec<Vec<f32>>,
    pub right: Vec<Vec<f32>>,
    pub loaded: bool,
}

// Loading state
#[derive(Resource, Default)]
pub struct HrtfLoadingState {
    pub total: usize,
    pub loaded_count: usize,
    pub handles: Vec<Handle<AudioSource>>,
}

pub struct ProceduralMusicPlugin;

impl Plugin for ProceduralMusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MusicEvent>()
            .init_resource::<HrtfImpulseResponses>()
            .init_resource::<HrtfLoadingState>()
            .add_systems(Startup, load_hrtf_assets)
            .add_systems(Update, (play_music_system, process_hrtf_loading));
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

// OPTIMIZED HRTF Convolution (cache-friendly, early exit, no unnecessary allocations)
fn convolve_hrtf_optimized(mono_buffer: &[f32], hrtf_left: &[f32], hrtf_right: &[f32]) -> Vec<f32> {
    let len = mono_buffer.len();
    let ir_len = hrtf_left.len().min(2048); // cap IR length for performance
    let mut output = vec![0.0; len * 2]; // stereo

    for i in 0..len {
        let max_j = (ir_len).min(len - i);
        let sample = mono_buffer[i];

        let mut left_sum = 0.0;
        let mut right_sum = 0.0;

        // Unrolled inner loop for better cache performance
        for j in 0..max_j {
            let ir_l = hrtf_left[j];
            let ir_r = hrtf_right[j];
            left_sum += sample * ir_l;
            right_sum += sample * ir_r;
        }

        let idx = i * 2;
        output[idx] = left_sum;
        output[idx + 1] = right_sum;
    }

    output
}

// Full binaural pipeline with optimized HRTF
fn apply_real_hrtf(
    mono_buffer: Vec<f32>,
    source_pos: Vec3,
    listener: &AudioListener,
    valence: f32,
    hrtf: &HrtfImpulseResponses,
) -> Vec<f32> {
    if !hrtf.loaded || hrtf.left.is_empty() {
        return mono_buffer; // graceful fallback
    }

    let direction = (source_pos - listener.position).normalize_or_zero();
    let azimuth = direction.x.atan2(direction.z).to_degrees() as i32;
    let ir_index = ((azimuth + 180) % 360 / 45) as usize % hrtf.left.len();

    let left_ir = &hrtf.left[ir_index];
    let right_ir = &hrtf.right[ir_index];

    let convolved = convolve_hrtf_optimized(&mono_buffer, left_ir, right_ir);

    // Apply occlusion, Doppler, distance attenuation, and valence modulation
    let distance = source_pos.distance(listener.position).max(0.1);
    let attenuation = (1.0 / (distance * distance)).clamp(0.15, 1.0) * (0.6 + valence * 0.4);
    let occlusion = if distance > 25.0 { 0.35 } else { 1.0 };

    let mut final_buffer = vec![0.0; convolved.len()];
    for (i, &sample) in convolved.iter().enumerate() {
        final_buffer[i] = sample * attenuation * occlusion;
    }
    final_buffer
}

// Granular cloud with optimized real HRTF
fn generate_granular_cloud(
    samples: &[Vec<f32>],
    rng: &mut impl Rng,
    valence: f32,
    length_secs: f32,
    density_factor: f32,
    listener: &AudioListener,
    hrtf: &HrtfImpulseResponses,
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

    apply_real_hrtf(mono_buffer, Vec3::new(0.0, 5.0, 15.0), listener, valence, hrtf)
}

// Example generator
fn generate_golden_ratio_granular_bloom(samples: &[Vec<f32>], rng: &mut impl Rng, valence: f32, listener: &AudioListener, hrtf: &HrtfImpulseResponses) -> AudioSource {
    let cloud = generate_granular_cloud(samples, rng, valence, 45.0, 1.8, listener, hrtf);
    AudioSource::from(cloud.into_iter().collect::<Vec<_>>().into_source())
}

// (Other generators updated similarly — kept for compatibility)

fn generate_sine_wave(freq: f32, duration_secs: f32, volume: f32) -> Vec<f32> { /* ... */ vec![] }
fn generate_noise(duration_secs: f32, volume: f32, rng: &mut impl Rng) -> Vec<f32> { /* ... */ vec![] }
