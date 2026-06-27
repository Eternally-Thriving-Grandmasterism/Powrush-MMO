/*!
 * Dynamic Audio Mixing - Real-time AudioSink + Broad Exposure
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::settings::editor::SettingsEditor;

/// Marker component for audio entities that should respond to live mixer changes
#[derive(Component)]
pub struct DynamicAudio {
    pub category: AudioCategory,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AudioCategory {
    Music,
    Sfx,
    Ui,
    Voice,
    Ambient,
}

#[derive(Resource, Default, Clone)]
pub struct AudioMixer {
    pub master: f32,
    pub music: f32,
    pub sfx: f32,
    pub ui: f32,
    pub voice: f32,
    pub ambient: f32,
}

/// Applies current mixer volumes to all DynamicAudio entities in real time
pub fn update_dynamic_audio_volumes(
    mixer: Res<AudioMixer>,
    mut query: Query<(&DynamicAudio, &mut AudioSink)>,
) {
    for (dynamic, mut sink) in query.iter_mut() {
        let volume = match dynamic.category {
            AudioCategory::Music   => mixer.music,
            AudioCategory::Sfx     => mixer.sfx,
            AudioCategory::Ui      => mixer.ui,
            AudioCategory::Voice   => mixer.voice,
            AudioCategory::Ambient => mixer.ambient,
        };
        sink.set_volume(volume);
    }
}

/// Helper to play a sound that respects the dynamic mixer
pub fn play_dynamic_sound(
    commands: &mut Commands,
    asset_server: &AssetServer,
    path: &str,
    category: AudioCategory,
    mixer: &AudioMixer,
    settings: PlaybackSettings,
) {
    let source = asset_server.load(path);

    let effective_volume = match category {
        AudioCategory::Music   => mixer.music,
        AudioCategory::Sfx     => mixer.sfx,
        AudioCategory::Ui      => mixer.ui,
        AudioCategory::Voice   => mixer.voice,
        AudioCategory::Ambient => mixer.ambient,
    };

    commands.spawn((
        AudioBundle {
            source,
            settings: settings.with_volume(effective_volume),
        },
        DynamicAudio { category },
    ));
}
