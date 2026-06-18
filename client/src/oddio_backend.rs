/*!
 * Oddio Audio Backend for Powrush-MMO
 *
 * Provides the low-level audio mixing and output for the dynamic music system.
 * Phase 3: Support for controllable music layers.
 *
 * v18.95 — Extended to support dynamic music layer playback with volume control.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 */

use bevy::prelude::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use oddio::{Gain, Mixer, Source, Stop};
use std::sync::{Arc, Mutex};

/// Resource that owns the oddio mixer and audio output
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

    /// Returns a handle to add sources to the mixer
    pub fn mixer_handle(&self) -> oddio::Handle<Mixer<[f32; 2]>> {
        self.mixer.lock().unwrap().handle()
    }

    /// Play a music layer source with initial volume control.
    /// Returns a handle that can be used to adjust gain later.
    pub fn play_music_layer<S>(&self, source: S, initial_volume: f32) -> oddio::Handle<Gain<f32, Stop<S>>>
    where
        S: Source<Frame = [f32; 2]> + Send + 'static,
    {
        let handle = self.mixer_handle();
        let stopped = Stop::new(source);
        let gained = Gain::new(stopped, initial_volume);
        handle.play(gained)
    }
}
