use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;
use std::time::Duration;

/// Procedural Music Plugin — event-triggered original tracks
/// Inspired by attachment moods (Diablo 2/WoW/Light of the Seven)

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
}

fn play_music_system(
    mut events: EventReader<MusicEvent>,
    audio: Res<Audio>,
) {
    let mut rng = rand::thread_rng();
    for event in events.read() {
        let source = match event {
            MusicEvent::MenuStart => generate_light_of_the_seven(&mut rng),
            MusicEvent::Exploration => generate_harrogath_drone(&mut rng),
            MusicEvent::BattleStart => generate_siege_grind(&mut rng),
            MusicEvent::QuestComplete => generate_growth_swell(&mut rng),
            MusicEvent::CouncilSession => generate_harmony_pad(&mut rng),
            MusicEvent::IncomeReward => generate_abundance_chime(&mut rng),
            MusicEvent::AmbientPad => generate_desert_ambient(&mut rng),
        };
        audio.play(source.repeat_infinite());
    }
}

// Generators (inspired moods — original synthesis)
fn generate_light_of_the_seven(rng: &mut impl RngCore) -> AudioSource {
    // Balanced piano epic ascent
    let notes = [261.63, 293.66, 329.63, 349.23, 392.0, 440.0, 493.88, 523.25]; // C major
    let mut melody = Vec::new();
    for _ in 0..60 {
        let note = notes[rng.gen_range(0..notes.len())];
        melody.extend(generate_sine_wave(note, 1.0, 0.5));
        melody.extend(generate_sine_wave(note * 1.5, 0.5, 0.3)); // Harmony
    }
    AudioSource::from(melody.into_iter().collect::<Vec<_>>().into_source())
}

fn generate_harrogath_drone(rng: &mut impl RngCore) -> AudioSource {
    // Dark ambient drone
    let low = rng.gen_range(80.0..120.0);
    AudioSource::from(generate_sine_wave(low, 300.0, 0.4)
        .chain(generate_sine_wave(low * 1.5, 300.0, 0.3)))
}

fn generate_siege_grind(rng: &mut impl RngCore) -> AudioSource {
    // Industrial battle grind
    let rumble = rng.gen_range(40.0..80.0);
    AudioSource::from(generate_noise(120.0, 0.8, rng)
        .modulate(Duration::from_millis(150)))
}

fn generate_growth_swell(rng: &mut impl RngCore) -> AudioSource {
    // Quest complete swell
    let base = rng.gen_range(200.0..400.0);
    AudioSource::from(generate_sine_wave(base, 60.0, 0.5)
        .fade_in(Duration::from_secs(5)))
}

fn generate_harmony_pad(rng: &mut impl RngCore) -> AudioSource {
    // Council unity pad
    let chord = [523.25, 659.25, 783.99]; // C major
    AudioSource::from(generate_sine_wave(chord[0], 300.0, 0.4)
        .chain(generate_sine_wave(chord[1], 300.0, 0.3))
        .chain(generate_sine_wave(chord[2], 300.0, 0.2)))
}

fn generate_abundance_chime(rng: &mut impl RngCore) -> AudioSource {
    // Income reward chime
    let high = rng.gen_range(800.0..1200.0);
    AudioSource::from(generate_sine_wave(high, 30.0, 0.6))
}

fn generate_desert_ambient(rng: &mut impl RngCore) -> AudioSource {
    // Desert wind pads
    let wind = rng.gen_range(100.0..200.0);
    AudioSource::from(generate_noise(300.0, 0.35, rng))
}

// Helpers
fn generate_sine_wave(freq: f32, duration_secs: f32, volume: f32) -> impl AudioSource {
    SineWave::new(freq)
        .amplify(volume)
        .take_duration(Duration::from_secs_f32(duration_secs))
}

fn generate_noise(duration_secs: f32, volume: f32, rng: &mut impl RngCore) -> impl AudioSource {
    Noise::new(rng.gen())
        .amplify(volume)
        .take_duration(Duration::from_secs_f32(duration_secs))
}
