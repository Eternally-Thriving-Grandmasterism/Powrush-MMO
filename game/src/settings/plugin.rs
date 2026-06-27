/*!
 * Settings Plugin - Weather-Driven Audio
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
            .init_resource::<environmental_audio::WeatherState>()
            .add_systems(Startup, persistence::load_settings)
            .add_systems(Update, (
                persistence::save_settings,
                audio_mixing::apply_audio_settings,
                audio_mixing::update_dynamic_audio_volumes,
                environmental_audio::apply_unified_spatial_audio,
                environmental_audio::update_weather_ambients,
                environmental_audio::trigger_thunder,
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
