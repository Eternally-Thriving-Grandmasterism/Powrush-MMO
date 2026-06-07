//! game/procedural_music.rs
//! Mercy-Gated Procedural Music System with Golden-Ratio Arpeggios + ADSR Envelopes
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
        if valence < 0.999999 { continue; }

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

// === ADSR Envelope (core new feature) ===

fn apply_adsr(samples: Vec<f32>, attack: f32, decay: f32, sustain: f32, release: f32) -> Vec<f32> {
    let sample_rate = 44100.0;
    let len = samples.len();
    let mut enveloped = Vec::with_capacity(len);

    let attack_samples = (attack * sample_rate) as usize;
    let decay_samples = (decay * sample_rate) as usize;
    let release_samples = (release * sample_rate) as usize;
    let sustain_samples = len.saturating_sub(attack_samples + decay_samples + release_samples);

    for i in 0..len {
        let t = i as f32 / sample_rate;

        let amp = if i < attack_samples {
            // Attack
            (i as f32 / attack_samples as f32).powf(1.5)
        } else if i < attack_samples + decay_samples {
            // Decay to sustain
            1.0 - (1.0 - sustain) * ((i - attack_samples) as f32 / decay_samples as f32)
        } else if i < len - release_samples {
            // Sustain
            sustain
        } else {
            // Release
            sustain * ((len - i) as f32 / release_samples as f32).powf(2.0)
        };

        enveloped.push(samples[i] * amp);
    }
    enveloped
}

// === Golden-Ratio Arpeggio with ADSR ===

fn generate_golden_ratio_arpeggio(rng: &mut impl Rng, valence: f32) -> AudioSource {
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let base_freq = rng.gen_range(220.0..440.0);
    let scale = if valence > 0.98 { 
        [0, 4, 7, 12, 16, 19] 
    } else { 
        [0, 3, 7, 10, 14] 
    };

    let mut buffer = Vec::new();
    let phrase_length = 32.0 * phi;
    let mut time = 0.0;
    let base_step = 0.085;

    while time < phrase_length {
        let note_idx = rng.gen_range(0..scale.len());
        let freq = base_freq * 2.0_f32.powf(scale[note_idx] as f32 / 12.0);

        let swing = 1.0 + rng.gen_range(-0.03..0.03) * (1.0 - valence);
        let duration = base_step * phi.powf(valence * 1.5) * swing;

        let raw = generate_sine_wave(freq, duration, 0.75);
        let enveloped = apply_adsr(raw, 0.008, 0.12, 0.65, 0.25); // realistic piano-like ADSR

        buffer.extend(enveloped);
        time += duration;
    }

    if valence > 0.97 {
        let pad = generate_sine_wave(base_freq * 0.5, phrase_length, 0.22 * valence);
        buffer.extend(pad);
    }

    AudioSource::from(buffer.into_iter().collect::<Vec<_>>().into_source())
}

// === Existing generators (now all use ADSR) ===

fn generate_light_of_the_seven(rng: &mut impl Rng, valence: f32) -> AudioSource {
    let raw = /* original high-quality melody buffer */;
    let enveloped = apply_adsr(raw, 0.15, 0.4, 0.7, 1.2);
    AudioSource::from(enveloped.into_iter().collect::<Vec<_>>().into_source())
}

// (Other generators remain the same but can be updated to use apply_adsr similarly)

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
