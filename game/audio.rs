// game/audio.rs
// Powrush-MMO — Rodio Audio Backend for Hit Sounds
// AG-SML v1.0 License

use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::game::hit_markers::HitSound;

pub struct AudioSystem {
    _stream: OutputStream,
    sink: Sink,
}

impl AudioSystem {
    pub fn new() -> Result<Self, String> {
        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| format!("Failed to create audio output stream: {}", e))?;

        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| format!("Failed to create audio sink: {}", e))?;

        Ok(Self {
            _stream: stream,
            sink,
        })
    }

    pub fn play_hit_sound(&self, sound: HitSound, sounds_dir: &str) {
        let filename = match sound {
            HitSound::Normal => "hit_normal.wav",
            HitSound::Critical => "hit_critical.wav",
            HitSound::Headshot => "hit_headshot.wav",
            HitSound::WeakHit => "hit_weak.wav",
        };

        let path = Path::new(sounds_dir).join(filename);

        if !path.exists() {
            eprintln!("Sound file not found: {:?}", path);
            return;
        }

        match File::open(&path) {
            Ok(file) => {
                let source = match Decoder::new(BufReader::new(file)) {
                    Ok(src) => src,
                    Err(e) => {
                        eprintln!("Failed to decode audio file: {}", e);
                        return;
                    }
                };

                self.sink.append(source);
            }
            Err(e) => {
                eprintln!("Failed to open sound file {:?}: {}", path, e);
            }
        }
    }

    pub fn stop_all(&self) {
        self.sink.stop();
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume.clamp(0.0, 1.0));
    }
}