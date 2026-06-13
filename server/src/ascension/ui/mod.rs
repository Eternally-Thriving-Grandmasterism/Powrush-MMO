/*!
 * Ascension UI Module
 * Sacred interfaces for Ambrosian Ability Bars + Mercy Ascent Trial.
 */

pub mod ability_bars;
pub mod mercy_ascent_trial_ui;

use bevy::prelude::*;
use ability_bars::*;
use mercy_ascent_trial_ui::*;

pub struct AscensionUiPlugin;

impl Plugin for AscensionUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                spawn_ambrosian_ability_bar,
                spawn_mercy_ascent_trial_ui,
            ))
            .add_systems(Update, (
                ability_tooltip_hover_system,
                update_tooltip_cursor_position_system,
                update_cooldown_overlays,
                update_harmony_orbs,
                update_mercy_ascent_trial_ui,
            ));
    }
}
