/*!
 * Oddio Audio Backend for Powrush-MMO
 *
 * Looping support for music layers.
 *
 * v18.99 — Added seamless looping for real audio and procedural sources.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 */

use bevy::prelude::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::WavReader;
use oddio::{Gain, Loop, Mixer, Source, Stop};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Resource)]
pub struct OddioAudioBackend {
    pub mixer: Arc<Mutex<Mixer<[f32; 2]>>>,
    _stream: cpal::Stream,
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

    /// Play a procedural source. Set `looping = true` for continuous music layers.
    pub fn play_procedural_layer(
        &self,
        frequency: f64,
        initial_volume: f32,
        looping: bool,
    ) -> oddio::Handle<Gain<f32, Stop<Box<dyn Source<Frame = [f32; 2]> + Send>>>> {
        use fundsp::prelude::*;

        let base_source = sine_hz(frequency) * 0.6 + sine_hz(frequency * 2.0) * 0.25 + sine_hz(frequency * 3.0) * 0.15;

        let source: Box<dyn Source<Frame = [f32; 2]> + Send> = if looping {
            Box::new(Loop::new(base_source >> split::<U2>() >> map(|(l, r)| [l as f32, r as f32])))
        } else {
            Box::new(base_source >> split::<U2>() >> map(|(l, r)| [l as f32, r as f32]))
        };

        let stopped = Stop::new(source);
        let gained = Gain::new(stopped, initial_volume);

        self.mixer_handle().play(gained)
    }

    /// Play a real WAV file. Set `looping = true` for music layers that should repeat seamlessly.
    pub fn play_audio_file(
        &self,
        path: &str,
        initial_volume: f32,
        looping: bool,
    ) -> Result<oddio::Handle<Gain<f32, Stop<Box<dyn Source<Frame = [f32; 2]> + Send>>>>, String> {
        // Check cache first
        {
            let cache = self.audio_cache.lock().unwrap();
            if let Some(frames) = cache.get(path) {
                let base_source = oddio::SamplesSource::from_frames(frames.clone());
                let source: Box<dyn Source<Frame = [f32; 2]> + Send> = if looping {
                    Box::new(Loop::new(base_source))
                } else {
                    Box::new(base_source)
                };

                let stopped = Stop::new(source);
                let gained = Gain::new(stopped, initial_volume);
                return Ok(self.mixer_handle().play(gained));
            }
        }

        // Decode from disk
        let mut reader = WavReader::open(path)
            .map_err(|e| format!("[Audio] Failed to open WAV '{}': {}", path, e))?;

        let spec = reader.spec();
        if spec.channels != 2 {
            return Err(format!("[Audio] Only stereo WAV supported (file: {})", path));
        }
        if spec.sample_rate != 44100 {
            return Err(format!("[Audio] Only 44.1kHz WAV supported (file: {})", path));
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

        // Cache the decoded frames
        {
            let mut cache = self.audio_cache.lock().unwrap();
            cache.insert(path.to_string(), frames.clone());
        }

        let base_source = oddio::SamplesSource::from_frames(frames);
        let source: Box<dyn Source<Frame = [f32; 2]> + Send> = if looping {
            Box::new(Loop::new(base_source))
        } else {
            Box::new(base_source)
        };

        let stopped = Stop::new(source);
        let gained = Gain::new(stopped, initial_volume);

        info!("[Audio] Loaded (looping={}): {}", looping, path);
        Ok(self.mixer_handle().play(gained))
    }
}
