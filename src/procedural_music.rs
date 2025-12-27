use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;

pub struct ProceduralMusicPlugin;

impl Plugin for ProceduralMusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, procedural_music_system);
    }
}

fn procedural_music_system(
    audio: Res<Audio>,
    lattice: Res<LatticeStats>,
    trust_query: Query<&TrustCredits>,
) {
    let trust_avg = trust_query.iter().map(|t| t.0).sum::<f32>() / trust_query.iter().count() as f32;
    let mercy_freq = 220.0 + (trust_avg / 10.0);  // Higher trust = higher pitch
    let complexity = (lattice.nodes as f32 / 100.0).min(8.0);  // Polyphony

    // Procedural FM tone
    let carrier = mercy_freq;
    let modulator = mercy_freq * 1.5 * (lattice.connections as f32 / 1000.0 + 1.0);
    let index = trust_avg / 50.0;

    // Play layered tones
    for i in 0..complexity as u32 {
        let freq = carrier * (i as f32 + 1.0);
        audio.play(procedural_tone(freq, modulator, index));
    }
}

fn procedural_tone(carrier: f32, modulator: f32, index: f32) -> AudioSource {
    // Stub — real: generate waveform via oscillator
    AudioSource::from_bytes(&[/* sine wave */])
        .with_pitch(carrier / 440.0)
        .with_volume(0.3)
}
