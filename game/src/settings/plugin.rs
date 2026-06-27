/*!
 * Settings Plugin - Dynamic Audio Mixing + Real-time Sink Updates
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use super::{persistence, GameSettings, editor, audio_mixing};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettings>()
            .init_resource::<audio_mixing::AudioMixer>()
            .add_systems(Startup, persistence::load_settings)
            .add_systems(Update, (
                persistence::save_settings,
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
                audio_mixing::update_dynamic_audio_volumes,
            ));
    }
}
