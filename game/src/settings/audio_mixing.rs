/*!
 * Dynamic Audio Mixing - Real-time AudioSink + Broad Exposure
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::settings::editor::SettingsEditor;

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

/// Syncs SettingsEditor volumes into AudioMixer in real time
pub fn apply_audio_settings(
    editor: Option<Res<SettingsEditor>>,
    mut mixer: ResMut<AudioMixer>,
) {
    if let Some(editor) = editor {
        if editor.is_changed() || editor.dirty {
            mixer.master  = editor.audio.master_volume;
            mixer.music   = editor.audio.music_volume   * mixer.master;
            mixer.sfx     = editor.audio.sfx_volume     * mixer.master;
            mixer.ui      = editor.audio.navigation_volume.max(editor.audio.activation_volume) * mixer.master;
            mixer.voice   = mixer.master; // Can be expanded later
            mixer.ambient = mixer.master * 0.8; // Slight ducking for ambient
        }
    }
}

/// Updates volumes on all active DynamicAudio sinks
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

/// Recommended helper for spawning mixer-aware sounds
pub fn play_dynamic_sound(
    commands: &mut Commands,
    asset_server: &AssetServer,
    path: &str,
    category: AudioCategory,
    mixer: &AudioMixer,
    settings: PlaybackSettings,
) {
    let source = asset_server.load(path);
    let vol = match category {
        AudioCategory::Music   => mixer.music,
        AudioCategory::Sfx     => mixer.sfx,
        AudioCategory::Ui      => mixer.ui,
        AudioCategory::Voice   => mixer.voice,
        AudioCategory::Ambient => mixer.ambient,
    };
    commands.spawn((
        AudioBundle {
            source,
            settings: settings.with_volume(vol),
        },
        DynamicAudio { category },
    ));
}
