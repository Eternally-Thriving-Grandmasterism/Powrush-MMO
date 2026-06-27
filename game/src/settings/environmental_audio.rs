/*!
 * Environmental Audio - Deeper Weather Interactions
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::settings::audio_mixing::{AudioMixer, AudioCategory, DynamicAudio, Priority};
use crate::settings::environmental_audio::{WeatherState, ReverbState};

/// Deep weather interaction: Rain affects ambient layers and distant sounds
pub fn apply_deep_weather_effects(
    weather: Res<WeatherState>,
    mut dynamic_audio: Query<(&DynamicAudio, &mut AudioSink)>,
    mixer: Res<AudioMixer>,
) {
    let rain = weather.rain_intensity;

    for (audio, mut sink) in dynamic_audio.iter_mut() {
        let base = mixer.get_volume_for_category(audio.category);

        match audio.category {
            AudioCategory::Ambient => {
                // Rain boosts ambient layers
                let boosted = base * (1.0 + rain * 0.5);
                sink.set_volume(boosted.clamp(0.0, base * 1.8));
            }
            AudioCategory::Sfx => {
                // Heavy rain slightly muffles distant SFX
                if rain > 0.6 {
                    let muffled = base * 0.85;
                    sink.set_volume(muffled);
                }
            }
            _ => {}
        }
    }
}

/// Thunder temporarily boosts reverb (dramatic effect)
pub fn apply_thunder_reverb_boost(
    mut reverb_state: ResMut<ReverbState>,
    weather: Res<WeatherState>,
    time: Res<Time>,
    mut thunder_timer: Local<f32>,
) {
    if weather.thunder_probability > 0.7 {
        if rand::random::<f32>() < 0.015 {
            // Trigger a thunder reverb boost
            *thunder_timer = 3.5; // seconds
            reverb_state.wetness = (reverb_state.wetness + 0.6).clamp(0.2, 1.0);
            reverb_state.decay_time = (reverb_state.decay_time + 1.5).min(6.0);
        }
    }

    if *thunder_timer > 0.0 {
        *thunder_timer -= time.delta_seconds();

        // Slowly return reverb to normal
        if *thunder_timer <= 0.0 {
            reverb_state.wetness = (reverb_state.wetness * 0.7).max(0.15);
            reverb_state.decay_time = (reverb_state.decay_time * 0.8).max(1.2);
        }
    }
}
