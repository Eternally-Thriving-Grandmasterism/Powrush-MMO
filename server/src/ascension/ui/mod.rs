/*!
 * Ascension UI Module
 * Includes the sacred Ambrosian Ability Bar with rich hover tooltips + cursor-follow positioning.
 */

pub mod ability_bars;

use bevy::prelude::*;
use ability_bars::*;

pub struct AscensionUiPlugin;

impl Plugin for AscensionUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_ambrosian_ability_bar)
            .add_systems(Update, (
                ability_tooltip_hover_system,
                update_tooltip_cursor_position_system,
                update_cooldown_overlays,
                update_harmony_orbs,
            ));
    }
}
