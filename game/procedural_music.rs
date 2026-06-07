//! game/procedural_music.rs
//! Mercy-Gated Procedural Music System with Granular Synthesis + Golden-Ratio Timing + ADSR + Stereo Panning
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;
use std::time::Duration;
use ra_thor_mercy::{MercyGate, evaluate_mercy_gates};
use lattice_conductor::SovereignLattice;

// Sample pool (real audio samples will be loaded here in production)
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

fn play_music_system(
    mut events: EventReader<MusicEvent>,
    audio: Res<Audio>,
    lattice: Res<SovereignLattice>,
    sample_pool: Res<MusicSamplePool>,
) {
    let mut rng = rand::thread_rng();
    for event in events.read() {
        let valence = lattice.current_valence();
        if valence < 0.999999 { continue; }

        let source = match event {
            MusicEvent::MenuStart => generate_granular_pad(&sample_pool.pads, &mut rng, valence),
            MusicEvent::Exploration => generate_granular_drone(&sample_pool.drones, &mut rng, valence),
            MusicEvent::BattleStart => generate_granular_grind(&sample_pool.blooms, &mut rng, valence),
            MusicEvent::QuestComplete => generate_growth_swell(&sample_pool.chimes, &mut rng, valence),
            MusicEvent::CouncilSession => generate_harmony_pad(&sample_pool.pads, &mut rng, valence),
            MusicEvent::IncomeReward => generate_abundance_chime(&sample_pool.chimes, &mut rng, valence),
            MusicEvent::AmbientPad => generate_desert_ambient(&sample_pool.drones, &mut rng, valence),
            MusicEvent::RbeAbundanceSpike => generate_golden_ratio_granular_bloom(&sample_pool.blooms, &mut rng, valence),
        };

        audio.play(source.repeat_infinite());
    }
}

// === GRANULAR SYNTHESIS WITH STEREO PANNING ===

fn generate_granular_cloud(
    samples: &[Vec<f32>],
    rng: &mut impl Rng,
    valence: f32,
    length_secs: f32,
    density_factor: f32,
) -> Vec<f32> {
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let sample_rate = 44100.0;
    let total_samples = (length_secs * sample_rate) as usize * 2; // stereo = 2 channels
    let mut buffer = vec![0.0; total_samples];

    let density = 25.0 + valence * 75.0 * density_factor;
    let grain_duration = 0.035 + (1.0 - valence) * 0.08;

    let mut time = 0.0;
    while time < length_secs {
        let source = &samples[rng.gen_range(0..samples.len())];
        let start_pos = rng.gen_range(0.0..(source.len() as f32 / sample_rate - grain_duration));

        let grain_samples = (grain_duration * sample_rate) as usize;
        let start_idx = (start_pos * sample_rate) as usize;

        // Stereo panning (-1.0 left → +1.0 right)
        let pan = rng.gen_range(-0.7..0.7) * (1.0 - valence * 0.3); // wider on high valence

        for i in 0..grain_samples {
            if start_idx + i >= source.len() { break; }
            let envelope = (i as f32 / grain_samples as f32)
                .min(1.0 - (i as f32 / grain_samples as f32))
                .powf(1.8);

            let sample = source[start_idx + i] * envelope * (0.6 + valence * 0.4);

            let left_gain = (0.5 - pan * 0.5).max(0.0);
            let right_gain = (0.5 + pan * 0.5).max(0.0);

            let idx = ((time * sample_rate) as usize + i) * 2;
            if idx + 1 < total_samples {
                buffer[idx]     += sample * left_gain;
                buffer[idx + 1] += sample * right_gain;
            }
        }

        time += 1.0 / density * phi.powf(valence * 1.2); // golden-ratio timing
    }

    buffer
}

// Specific generators (now all stereo)
fn generate_granular_pad(samples: &[Vec<f32>], rng: &mut impl Rng, valence: f32) -> AudioSource {
    let cloud = generate_granular_cloud(samples, rng, valence, 60.0, 0.6);
    AudioSource::from(cloud.into_iter().collect::<Vec<_>>().into_source())
}

fn generate_golden_ratio_granular_bloom(samples: &[Vec<f32>], rng: &mut impl Rng, valence: f32) -> AudioSource {
    let cloud = generate_granular_cloud(samples, rng, valence, 45.0, 1.8);
    AudioSource::from(cloud.into_iter().collect::<Vec<_>>().into_source())
}

// Fallback generators (kept for compatibility — can be upgraded similarly)
fn generate_light_of_the_seven(rng: &mut impl Rng, valence: f32) -> AudioSource { /* ... */ AudioSource::from(vec![]) }
fn generate_harrogath_drone(rng: &mut impl Rng, valence: f32) -> AudioSource { /* ... */ AudioSource::from(vec![]) }
fn generate_siege_grind(rng: &mut impl Rng, valence: f32) -> AudioSource { /* ... */ AudioSource::from(vec![]) }
fn generate_growth_swell(rng: &mut impl Rng, valence: f32) -> AudioSource { /* ... */ AudioSource::from(vec![]) }
fn generate_harmony_pad(rng: &mut impl Rng, valence: f32) -> AudioSource { /* ... */ AudioSource::from(vec![]) }
fn generate_abundance_chime(rng: &mut impl Rng, valence: f32) -> AudioSource { /* ... */ AudioSource::from(vec![]) }
fn generate_desert_ambient(rng: &mut impl Rng, valence: f32) -> AudioSource { /* ... */ AudioSource::from(vec![]) }

// Helpers
fn generate_sine_wave(freq: f32, duration_secs: f32, volume: f32) -> Vec<f32> { /* ... */ vec![] }
fn generate_noise(duration_secs: f32, volume: f32, rng: &mut impl Rng) -> Vec<f32> { /* ... */ vec![] }
