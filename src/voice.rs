use bevy::prelude::*;
use bevy_replicon::prelude::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use opus::{Encoder, Decoder};
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Component, Replicated)]
pub struct VoicePacket {
    pub player_id: u64,
    pub data: Vec<u8>,  // Opus compressed frame
    pub volume: f32,    // Trust-modulated
}

pub struct VoicePlugin;

impl Plugin for VoicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (capture_system, playback_system));
    }
}

// Mic capture + Opus encode
fn capture_system(
    mut events: EventWriter<VoicePacket>,
    player_query: Query<&TrustCredits>,
) {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("No mic");
    let config = device.default_input_config().expect("No config");

    let (tx, mut rx) = mpsc::channel(1024);

    let mut encoder = Encoder::new(config.sample_rate().0, opus::Channels::Mono, opus::Application::Voip).unwrap();

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut encoded = vec![0; 1024];
            let len = encoder.encode_float(data, &mut encoded).unwrap();
            encoded.truncate(len);
            tx.blocking_send(encoded).unwrap();
        },
        |err| eprintln!("Capture error: {:?}", err),
        None,
    ).unwrap();

    stream.play().unwrap();

    let trust = player_query.single().0;

    while let Ok(encoded) = rx.try_recv() {
        events.send(VoicePacket {
            player_id: 1,
            data: encoded,
            volume: trust / 10.0,
        });
    }
}

// Playback + decode
fn playback_system(
    mut audio: ResMut<Audio>,
    packets: EventReader<VoicePacket>,
) {
    let mut decoder = Decoder::new(48000, opus::Channels::Mono).unwrap();

    for packet in packets.read() {
        let mut decoded = vec![0f32; 960];
        decoder.decode_float(&packet.data, &mut decoded, false).unwrap();
        audio.play(AudioSource::from(decoded)).with_volume(packet.volume);
    }
}
