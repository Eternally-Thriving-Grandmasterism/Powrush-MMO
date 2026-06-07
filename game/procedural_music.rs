//! game/procedural_music.rs
//! Mercy-Gated Procedural Music System with Refined Golden-Ratio Arpeggios
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;
use std::time::Duration;
use ra_thor_mercy::{MercyGate, evaluate_mercy_gates};
use lattice_conductor::SovereignLattice;

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
) {
    let mut rng = rand::thread_rng();
    for event in events.read() {
        let valence = lattice.current_valence();
        if valence < 0.999999 { continue; } // mercy refinement

        let source = match event {
            MusicEvent::MenuStart => generate_light_of_the_seven(&mut rng, valence),
            MusicEvent::Exploration => generate_harrogath_drone(&mut rng, valence),
            MusicEvent::BattleStart => generate_siege_grind(&mut rng, valence),
            MusicEvent::QuestComplete => generate_growth_swell(&mut rng, valence),
            MusicEvent::CouncilSession => generate_harmony_pad(&mut rng, valence),
            MusicEvent::IncomeReward => generate_abundance_chime(&mut rng, valence),
            MusicEvent::AmbientPad => generate_desert_ambient(&mut rng, valence),
            MusicEvent::RbeAbundanceSpike => generate_golden_ratio_arpeggio(&mut rng, valence),
        };

        audio.play(source.repeat_infinite());
    }
}

// === REFINED GOLDEN-RATIO ARPEGGIO (main new feature) ===

fn generate_golden_ratio_arpeggio(rng: &mut impl Rng, valence: f32) -> AudioSource {
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let base_freq = rng.gen_range(220.0..440.0);
    let scale = if valence > 0.98 { 
        [0, 4, 7, 12, 16, 19] // bright major extensions 
    } else { 
        [0, 3, 7, 10, 14] // softer minor/pentatonic 
    };

    let mut buffer = Vec::new();
    let phrase_length = 32.0 * phi; // golden-ratio phrase
    let mut time = 0.0;
    let base_step = 0.085; // ~16th note feel

    while time < phrase_length {
        let note_idx = rng.gen_range(0..scale.len());
        let freq = base_freq * 2.0_f32.powf(scale[note_idx] as f32 / 12.0);

        // Humanized timing + swing
        let swing = 1.0 + rng.gen_range(-0.03..0.03) * (1.0 - valence);
        let duration = base_step * phi.powf(valence * 1.5) * swing;

        let velocity = 0.6 + valence * 0.4 + rng.gen_range(-0.08..0.08);
        buffer.extend(generate_sine_wave(freq, duration, velocity));

        time += duration;
    }

    // Gentle pad layer for high valence
    if valence > 0.97 {
        let pad_freq = base_freq * 0.5;
        buffer.extend(generate_sine_wave(pad_freq, phrase_length, 0.25 * valence));
    }

    AudioSource::from(buffer.into_iter().collect::<Vec<_>>().into_source())
}

// === Existing generators (valence-scaled, kept high quality) ===
fn generate_light_of_the_seven(rng: &mut impl Rng, valence: f32) -> AudioSource { /* ... same as before ... */ AudioSource::from(/* buffer */) }
fn generate_harrogath_drone(rng: &mut impl Rng, valence: f32) -> AudioSource { /* ... */ AudioSource::from(/* buffer */) }
fn generate_siege_grind(rng: &mut impl Rng, valence: f32) -> AudioSource { /* ... */ AudioSource::from(/* buffer */) }
fn generate_growth_swell(rng: &mut impl Rng, valence: f32) -> AudioSource { /* ... */ AudioSource::from(/* buffer */) }
fn generate_harmony_pad(rng: &mut impl Rng, valence: f32) -> AudioSource { /* ... */ AudioSource::from(/* buffer */) }
fn generate_abundance_chime(rng: &mut impl Rng, valence: f32) -> AudioSource { /* ... */ AudioSource::from(/* buffer */) }
fn generate_desert_ambient(rng: &mut impl Rng, valence: f32) -> AudioSource { /* ... */ AudioSource::from(/* buffer */) }

// Helpers (unchanged but now used by arpeggio)
fn generate_sine_wave(freq: f32, duration_secs: f32, volume: f32) -> Vec<f32> {
    let sample_rate = 44100.0;
    let num_samples = (duration_secs * sample_rate) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate;
        samples.push((2.0 * std::f32::consts::PI * freq * t).sin() * volume);
    }
    samples
}

fn generate_noise(duration_secs: f32, volume: f32, rng: &mut impl Rng) -> Vec<f32> {
    let sample_rate = 44100.0;
    let num_samples = (duration_secs * sample_rate) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    for _ in 0..num_samples {
        samples.push(rng.gen_range(-1.0..1.0) * volume);
    }
    samples
}
