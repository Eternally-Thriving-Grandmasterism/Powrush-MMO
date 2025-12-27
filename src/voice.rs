use bevy::prelude::*;
use bevy_replicon::prelude::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use opus::{Encoder, Decoder};
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Component, Replicated)]
pub struct VoicePacket {
    pub player_id: u64,
    pub data: Vec<u8>,  // Opus frame
    pub volume: f32,    // Trust-modulated
}

pub struct VoicePlugin;

impl Plugin for VoicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (capture_system, playback_system));
    }
}

// Mic capture
fn capture_system(
    mut events: EventWriter<VoicePacket>,
    player_query: Query<&TrustCredits>,
) {
    let host = cpal::default_host();
    let device = host.default_input_device().unwrap();
    let config = device.default_input_config().unwrap();

    let (tx, mut rx) = mpsc::channel(1024);

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let encoder = Encoder::new(48000, opus::Channels::Mono, opus::Application::Voip).unwrap();
            let encoded = encoder.encode_vec_f32(data, 960, vec![0; 1024]).unwrap();
            tx.blocking_send(encoded).unwrap();
        },
        |err| eprintln!("Capture error: {:?}", err),
        None,
    ).unwrap();

    stream.play().unwrap();

    // Send packets
    while let Some(encoded) = rx.recv().await {
        let trust = player_query.single().0;
        events.send(VoicePacket {
            player_id: 1,
            data: encoded,
            volume: trust / 10.0,
        });
    }
}

// Playback
fn playback_system(
    audio: Res<Audio>,
    packets: EventReader<VoicePacket>,
) {
    for packet in packets.read() {
        let decoder = Decoder::new(48000, opus::Channels::Mono).unwrap();
        let mut decoded = vec![0f32; 960];
        decoder.decode_float(&packet.data, &mut decoded, false).unwrap();
        audio.play_from_cursor(AudioSource::from(decoded));
    }
}
