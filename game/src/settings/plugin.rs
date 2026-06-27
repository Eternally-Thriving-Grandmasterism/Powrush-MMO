/*!
 * Settings Plugin - Wires GameSettings, Editor, and Live UI Updates
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use super::{persistence, GameSettings, editor};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettings>()
            .add_systems(Startup, persistence::load_settings)
            .add_systems(Update, (
                persistence::save_settings,
                editor::update_setting_value_texts,
                editor::mark_editor_dirty,
            ));
    }
}
