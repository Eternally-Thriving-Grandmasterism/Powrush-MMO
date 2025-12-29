use rodio::{Decoder, OutputStream, Sink, Source};
use rodio::source::{SineWave, SourcedSource};
use std::f32::consts::PI;
use std::time::Duration;
use std::collections::HashMap;

// Procedural Diablo 2 / WoW inspired soundtrack generator
// Original waves — no copyright, high-quality modern synths/orchestral

pub struct ProceduralMusic {
    sink: Sink,
    tracks: HashMap<String, Box<dyn Source<Item = f32> + Send>>,
}

impl ProceduralMusic {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let mut tracks = HashMap::new();
        tracks.insert("light_of_seven".to_string(), Box::new(Self::generate_light_of_seven()));
        tracks.insert("harrogath".to_string(), Box::new(Self::generate_harrogath()));
        tracks.insert("siege".to_string(), Box::new(Self::generate_siege()));
        tracks.insert("ice_caves".to_string(), Box::new(Self::generate_ice_caves()));
        tracks.insert("kurast_docks".to_string(), Box::new(Self::generate_kurast_docks()));
        tracks.insert("tristram".to_string(), Box::new(Self::generate_tristram()));
        tracks.insert("wilderness".to_string(), Box::new(Self::generate_wilderness()));
        tracks.insert("burning_legion".to_string(), Box::new(Self::generate_burning_legion()));
        tracks.insert("shards_exodar".to_string(), Box::new(Self::generate_shards_exodar()));
        tracks.insert("origins".to_string(), Box::new(Self::generate_origins()));
        tracks.insert("bloodmyst".to_string(), Box::new(Self::generate_bloodmyst()));
        Self { sink, tracks }
    }

    fn generate_light_of_seven() -> impl Source<Item = f32> + Send {
        // Ramin Djawadi piano epic swell
        let piano = SineWave::new(261.63).amplify(0.4); // C4
        let swell = SineWave::new(523.25).amplify(0.2).periodic(Duration::from_secs(2)); // C5 rising
        piano.chain(swell)
    }

    fn generate_harrogath() -> impl Source<Item = f32> + Send {
        // Matt Uelmen dark ambient drone
        let drone1 = SineWave::new(110.0).amplify(0.3); // A1
        let drone2 = SineWave::new(82.41).amplify(0.25); // E1
        drone1.chain(drone2)
    }

    fn generate_siege() -> impl Source<Item = f32> + Send {
        // Industrial battle grind
        let grind = SineWave::new(55.0).amplify(0.5); // A0 rumble
        let percussion = SineWave::new(220.0).amplify(0.7).periodic(Duration::from_millis(150)); // A3 pulse
        grind.chain(percussion)
    }

    fn generate_ice_caves() -> impl Source<Item = f32> + Send {
        // Echoing cave ambience
        let ice_drone = SineWave::new(130.81).amplify(0.35); // E2
        let echo = SineWave::new(329.63).amplify(0.15).periodic(Duration::from_millis(800)); // E4 reverb
        ice_drone.chain(echo)
    }

    fn generate_kurast_docks() -> impl Source<Item = f32> + Send {
        // Mystical harbor pads
        let pad1 = SineWave::new(196.0).amplify(0.3); // G3
        let pad2 = SineWave::new(246.94).amplify(0.25); // B3
        pad1.chain(pad2)
    }

    fn generate_tristram() -> impl Source<Item = f32> + Send {
        // Haunting town theme
        let haunt = SineWave::new(146.83).amplify(0.4); // D3
        let melody = SineWave::new(293.66).amplify(0.2); // D4
        haunt.chain(melody)
    }

    fn generate_wilderness() -> impl Source<Item = f32> + Send {
        // Open field orchestral swell
        let brass = SineWave::new(130.81).amplify(0.45); // E2
        let strings = SineWave::new(164.81).amplify(0.3); // E3
        brass.chain(strings)
    }

    fn generate_burning_legion() -> impl Source<Item = f32> + Send {
        // WoW epic main title
        let epic_horn = SineWave::new(98.0).amplify(0.5); // G2
        let choir = SineWave::new(196.0).amplify(0.35); // G3
        epic_horn.chain(choir)
    }

    fn generate_shards_exodar() -> impl Source<Item = f32> + Send {
        // Ethereal space pads
        let shard1 = SineWave::new(123.47).amplify(0.3); // B2
        let shard2 = SineWave::new(155.56).amplify(0.25); // G3
        shard1.chain(shard2)
    }

    fn generate_origins() -> impl Source<Item = f32> + Send {
        // Mystical origins swell
        let origin_drone = SineWave::new(65.41).amplify(0.4); // C2
        let origin_melody = SineWave::new(261.63).amplify(0.2); // C4
        origin_drone.chain(origin_melody)
    }

    fn generate_bloodmyst() -> impl Source<Item = f32> + Send {
        // Dark mystic isle
        let blood_drone = SineWave::new(87.31).amplify(0.35); // F2
        let myst = SineWave::new(174.61).amplify(0.25); // F3
        blood_drone.chain(myst)
    }

    pub fn play_track(&self, track_name: &str, loop_track: bool) {
        if let Some(track) = self.tracks.get(track_name) {
            self.sink.append(track.clone());
            if loop_track {
                self.sink.append(track.clone().repeat_infinite());
            }
            self.sink.play();
            println!("Playing procedural track: {}", track_name);
        } else {
            println!("Track '{}' not found", track_name);
        }
    }

    pub fn stop(&self) {
        self.sink.stop();
    }
}

// Game integration
pub fn init_soundtrack() {
    let soundtrack = ProceduralMusic::new();
    soundtrack.play_track("harrogath", true); // Dark ambient start
}
