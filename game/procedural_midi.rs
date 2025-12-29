use rodio::{OutputStream, Sink, Source};
use std::f32::consts::PI;
use std::time::Duration;

pub struct ProceduralMidi {
    sink: Sink,
}

impl ProceduralMidi {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Self { sink }
    }

    pub fn play_event_music(&self, event_type: &str) {
        let source = match event_type {
            "quest_complete" => self.generate_growth_chime(),
            "council_vote" => self.generate_harmony_swell(),
            "income_mint" => self.generate_abundance_wave(),
            "exploration" => self.generate_calm_ambient(),
            _ => self.generate_default_pad(),
        };
        self.sink.append(source.take_duration(Duration::from_secs(60)).repeat_infinite());
        self.sink.play();
        println!("Playing procedural MIDI for event: {}", event_type);
    }

    fn generate_growth_chime() -> impl Source<Item = f32> + Send {
        // Uplifting chime (positive emotion boost)
        SineWave::new(440.0).amplify(0.4) // A4 base
            .chain(SineWave::new(880.0).amplify(0.3)) // Octave harmony
            .periodic(Duration::from_millis(500)) // Gentle chime
    }

    fn generate_harmony_swell() -> impl Source<Item = f32> + Send {
        // Swelling harmony (council unity)
        SineWave::new(523.25).amplify(0.5) // C5
            .chain(SineWave::new(659.25).amplify(0.4)) // E5
            .chain(SineWave::new(783.99).amplify(0.3)) // G5 major chord
            .fade_in(Duration::from_secs(2))
    }

    fn generate_abundance_wave() -> impl Source<Item = f32> + Send {
        // Wavy abundance (wealth positive vibe)
        SineWave::new(392.0).amplify(0.4) // G4
            .chain(SineWave::new(440.0).amplify(0.3)) // A4
            .modulate(Duration::from_millis(200)) // Wave effect
    }

    fn generate_calm_ambient() -> impl Source<Item = f32> + Send {
        // Calm pads for exploration
        SineWave::new(110.0).amplify(0.3) // Low A2 drone
            .chain(SineWave::new(220.0).amplify(0.2)) // A3
            .periodic(Duration::from_secs(5)) // Slow pulse
    }

    fn generate_default_pad() -> impl Source<Item = f32> + Send {
        SineWave::new(196.0).amplify(0.35) // G3 pad
    }

    pub fn stop(&self) {
        self.sink.stop();
    }
}

// Integration example
pub fn init_midi() {
    let midi = ProceduralMidi::new();
    midi.play_event_music("exploration"); // Calm ambient start
}
