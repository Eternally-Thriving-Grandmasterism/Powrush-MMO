use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, procedural_sfx_system);
    }
}

fn procedural_sfx_system(
    mut audio: ResMut<Audio>,
    lattice: Res<LatticeStats>,
    trust_query: Query<&TrustCredits>,
    events: EventReader<MercyEvent>,
) {
    let trust_avg = trust_query.iter().map(|t| t.0).sum::<f32>() / trust_query.iter().count() as f32;
    let mercy_freq = 440.0 + (trust_avg / 10.0).min(1000.0);

    // Mercy burst (FM synth tone)
    for _ in events.read() {
        let carrier = mercy_freq;
        let modulator = mercy_freq * 1.4;
        let index = 5.0 + (lattice.nodes as f32 / 100.0);
        let duration = 0.3;

        // Procedural FM wave (bytebeat style)
        let sample_rate = 44100;
        let samples = (duration * sample_rate as f32) as usize;
        let mut buffer = Vec::with_capacity(samples);

        for t in 0..samples {
            let time = t as f32 / sample_rate as f32;
            let mod_phase = (time * modulator * 2.0 * std::f32::consts::PI).sin() * index;
            let carrier_phase = (time * carrier * 2.0 * std::f32::consts::PI + mod_phase).sin();
            buffer.push(carrier_phase as i16);
        }

        let sound = AudioSource::new(buffer, sample_rate);
        audio.play(sound)
            .with_volume(0.5)
            .with_pitch(mercy_freq / 440.0);
    }

    // Lattice growth chime (bytebeat: t*(t>>11&t>>8&123&t>>7))
    if lattice.connections % 10 == 0 {
        let sample_rate = 22050;
        let samples = 4410;  // 0.2s
        let mut buffer = Vec::with_capacity(samples);

        for t in 0..samples {
            let bytebeat = (t as u32 * (t >> 11) & (t >> 8) & 123) as i16 as u32 >> 7;
            buffer.push(bytebeat as i16 - 32768);
        }

        let sound = AudioSource::new(buffer, sample_rate);
        audio.play(sound).with_volume(0.3);
    }
}
