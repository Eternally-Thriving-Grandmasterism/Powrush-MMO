/*!
 * Settings Plugin - Distance-Based Reverb
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use super::{persistence, GameSettings, editor, audio_mixing, environmental_audio};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettings>()
            .init_resource::<audio_mixing::AudioMixer>()
            .init_resource::<environmental_audio::ReverbState>()
            .add_systems(Startup, persistence::load_settings)
            .add_systems(Update, (
                persistence::save_settings,
                audio_mixing::apply_audio_settings,
                audio_mixing::update_dynamic_audio_volumes,
                environmental_audio::apply_occlusion_with_rapier,
                environmental_audio::apply_dynamic_occlusion_zones,
                environmental_audio::apply_portal_audio,
                environmental_audio::apply_acoustic_occlusion,
                environmental_audio::update_reverb_state,
                environmental_audio::apply_reverb_to_sounds,
                environmental_audio::apply_distance_based_reverb,
                editor::update_audio_value_texts,
                editor::update_graphics_value_texts,
                editor::update_controls_value_texts,
                editor::update_slider_bars,
                editor::handle_settings_input,
                editor::mark_editor_dirty,
                editor::handle_reset_to_defaults,
                editor::play_value_change_sound,
                editor::trigger_slider_pop,
                editor::animate_slider_bars,
                editor::trigger_haptic_feedback,
            ));
    }
}
