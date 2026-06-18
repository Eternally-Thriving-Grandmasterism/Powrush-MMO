/*!
 * Oddio Audio Backend for Powrush-MMO
 *
 * Hybrid system refinements: Caching + improved error handling.
 *
 * v18.98 — Added audio frame caching and better diagnostics.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 */

use bevy::prelude::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::WavReader;
use oddio::{Gain, Mixer, Source, Stop};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Resource)]
pub struct OddioAudioBackend {
    pub mixer: Arc<Mutex<Mixer<[f32; 2]>>>,
    _stream: cpal::Stream,
    /// Cache of decoded audio frames (keyed by file path)
    audio_cache: Arc<Mutex<HashMap<String, Vec<[f32; 2]>>>>,
}

impl OddioAudioBackend {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("No output device available");
        let config = device.default_output_config().expect("Failed to get default output config");

        let sample_rate = config.sample_rate().0;

        let mixer = Mixer::<[f32; 2]>::new();
        let mixer_clone = mixer.clone();

        let mixer = Arc::new(Mutex::new(mixer));
        let audio_cache = Arc::new(Mutex::new(HashMap::new()));

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
            audio_cache,
        }
    }

    pub fn mixer_handle(&self) -> oddio::Handle<Mixer<[f32; 2]>> {
        self.mixer.lock().unwrap().handle()
    }

    /// Play a procedural layer (development / fallback)
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

    /// Play a real WAV file. Uses internal cache to avoid repeated decoding.
    pub fn play_audio_file(
        &self,
        path: &str,
        initial_volume: f32,
    ) -> Result<oddio::Handle<Gain<f32, Stop<Box<dyn Source<Frame = [f32; 2]> + Send>>>>, String> {
        // Check cache first
        {
            let cache = self.audio_cache.lock().unwrap();
            if let Some(frames) = cache.get(path) {
                let source = oddio::SamplesSource::from_frames(frames.clone());
                let boxed: Box<dyn Source<Frame = [f32; 2]> + Send> = Box::new(source);
                let stopped = Stop::new(boxed);
                let gained = Gain::new(stopped, initial_volume);
                return Ok(self.mixer_handle().play(gained));
            }
        }

        // Not cached — decode from disk
        let mut reader = WavReader::open(path)
            .map_err(|e| format!("[Audio] Failed to open WAV '{}': {}", path, e))?;

        let spec = reader.spec();
        if spec.channels != 2 {
            return Err(format!("[Audio] Only stereo WAV files are supported (file: {})", path));
        }
        if spec.sample_rate != 44100 {
            return Err(format!("[Audio] Only 44.1kHz WAV files are supported (file: {})", path));
        }

        let samples: Vec<f32> = reader
            .samples::<i16>()
            .map(|s| s.map(|v| v as f32 / 32768.0).unwrap_or(0.0))
            .collect();

        let frames: Vec<[f32; 2]> = samples
            .chunks_exact(2)
            .map(|chunk| [chunk[0], chunk[1]])
            .collect();

        if frames.is_empty() {
            return Err(format!("[Audio] WAV file is empty: {}", path));
        }

        // Store in cache
        {
            let mut cache = self.audio_cache.lock().unwrap();
            cache.insert(path.to_string(), frames.clone());
        }

        let source = oddio::SamplesSource::from_frames(frames);
        let boxed: Box<dyn Source<Frame = [f32; 2]> + Send> = Box::new(source);
        let stopped = Stop::new(boxed);
        let gained = Gain::new(stopped, initial_volume);

        info!("[Audio] Loaded and cached: {}", path);
        Ok(self.mixer_handle().play(gained))
    }
}
