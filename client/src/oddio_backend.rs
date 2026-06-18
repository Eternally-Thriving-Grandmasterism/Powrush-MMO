/*!
 * Oddio Audio Backend for Powrush-MMO
 *
 * Option B: Real audio file support (WAV via hound).
 *
 * v18.97 — Added ability to play real audio files for music layers.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 */

use bevy::prelude::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::WavReader;
use oddio::{Gain, Mixer, Source, Stop};
use std::sync::{Arc, Mutex};

#[derive(Resource)]
pub struct OddioAudioBackend {
    pub mixer: Arc<Mutex<Mixer<[f32; 2]>>>,
    _stream: cpal::Stream,
}

impl OddioAudioBackend {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("No output device available");
        let config = device.default_output_config().expect("Failed to get default output config");

        let sample_rate = config.sample_rate().0;

        let mixer = Mixer::<[f32; 2]>::new();
        let mixer_handle = mixer.handle();

        let mixer = Arc::new(Mutex::new(mixer));
        let mixer_clone = mixer.clone();

        let err_fn = |err| eprintln!("Audio stream error: {}", err);

        let stream = device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _| {
                let mut mixer = mixer_clone.lock().unwrap();
                oddio::run(&mut *mixer, sample_rate, data);
            },
            err_fn,
            None,
        ).expect("Failed to build output stream");

        stream.play().expect("Failed to play audio stream");

        Self {
            mixer,
            _stream: stream,
        }
    }

    pub fn mixer_handle(&self) -> oddio::Handle<Mixer<[f32; 2]>> {
        self.mixer.lock().unwrap().handle()
    }

    /// Play a procedural layer (for development / fallback)
    pub fn play_procedural_layer(&self, frequency: f64, initial_volume: f32) -> oddio::Handle<Gain<f32, Stop<Box<dyn Source<Frame = [f32; 2]> + Send>>>> {
        use fundsp::prelude::*;

        let source = (sine_hz(frequency) * 0.6 + sine_hz(frequency * 2.0) * 0.25 + sine_hz(frequency * 3.0) * 0.15)
            >> split::<U2>()
            >> map(|(l, r)| [l as f32, r as f32]);

        let boxed: Box<dyn Source<Frame = [f32; 2]> + Send> = Box::new(source);
        let stopped = Stop::new(boxed);
        let gained = Gain::new(stopped, initial_volume);

        self.mixer_handle().play(gained)
    }

    /// Play a real WAV file as a music layer source.
    /// Returns a handle for volume control.
    pub fn play_audio_file(
        &self,
        path: &str,
        initial_volume: f32,
    ) -> Result<oddio::Handle<Gain<f32, Stop<Box<dyn Source<Frame = [f32; 2]> + Send>>>>, String> {
        let mut reader = WavReader::open(path).map_err(|e| format!("Failed to open WAV: {}", e))?;

        let spec = reader.spec();
        if spec.channels != 2 || spec.sample_rate != 44100 {
            return Err("Only 44.1kHz stereo WAV files are supported currently".to_string());
        }

        let samples: Vec<f32> = reader
            .samples::<i16>()
            .map(|s| s.unwrap() as f32 / 32768.0)
            .collect();

        // Convert interleaved stereo to frames
        let frames: Vec<[f32; 2]> = samples
            .chunks_exact(2)
            .map(|chunk| [chunk[0], chunk[1]])
            .collect();

        let source = oddio::SamplesSource::from_frames(frames);
        let boxed: Box<dyn Source<Frame = [f32; 2]> + Send> = Box::new(source);
        let stopped = Stop::new(boxed);
        let gained = Gain::new(stopped, initial_volume);

        Ok(self.mixer_handle().play(gained))
    }
}
