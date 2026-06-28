/*!
 * AdaptiveAudioConfig - Dynamic Bias & Stacking parameters
 */

use bevy::prelude::*;
use crate::settings::audio_mixing::AudioMixer;

// Extend AdaptiveAudioConfig with dynamic bias & stacking fields
#[derive(Resource, Clone)]
pub struct AdaptiveAudioConfig {
    // ... existing fields ...

    // === Dynamic Bias & Stacking (live tunable) ===
    pub music_ducking_bias: f32,
    pub ambient_ducking_bias: f32,
    pub sfx_ducking_bias: f32,
    pub ui_ducking_bias: f32,

    pub stacking_critical_per_sound: f32,
    pub stacking_high_per_sound: f32,
    pub max_stacking_reduction: f32,
}

impl Default for AdaptiveAudioConfig {
    fn default() -> Self {
        Self {
            // ... existing defaults ...

            music_ducking_bias: 1.15,
            ambient_ducking_bias: 1.0,
            sfx_ducking_bias: 0.9,
            ui_ducking_bias: 0.6,

            stacking_critical_per_sound: 0.08,
            stacking_high_per_sound: 0.06,
            max_stacking_reduction: 0.35,
        }
    }
}

// Update apply and hot reload functions to sync these to AudioMixer
pub fn hot_reload_adaptive_audio_config_system(
    mut asset_events: EventReader<AssetEvent<AdaptiveAudioConfigAsset>>,
    assets: Res<Assets<AdaptiveAudioConfigAsset>>,
    handle: Res<AdaptiveAudioConfigHandle>,
    mut config: ResMut<AdaptiveAudioConfig>,
    mut mixer: ResMut<AudioMixer>,
) {
    for event in asset_events.read() {
        if let AssetEvent::LoadedWithDependencies { id } | AssetEvent::Modified { id } = event {
            if let Some(h) = &handle.0 {
                if h.id() == *id {
                    if let Some(loaded) = assets.get(h) {
                        // ... copy other fields ...

                        // Sync dynamic bias & stacking
                        mixer.music_ducking_bias = loaded.music_ducking_bias;
                        mixer.ambient_ducking_bias = loaded.ambient_ducking_bias;
                        mixer.sfx_ducking_bias = loaded.sfx_ducking_bias;
                        mixer.ui_ducking_bias = loaded.ui_ducking_bias;

                        mixer.stacking_critical_per_sound = loaded.stacking_critical_per_sound;
                        mixer.stacking_high_per_sound = loaded.stacking_high_per_sound;
                        mixer.max_stacking_reduction = loaded.max_stacking_reduction;

                        info!("[HotReload] Dynamic Bias & Stacking updated from RON");
                    }
                }
            }
        }
    }
}
