/*!
 * Bevy ECS Integration for Lattice Fracture Resolution
 *
 * Expanded with better input/UI separation and practical patterns.
 */

use bevy::prelude::*;
use simulation::fracture::{
    Fracture, PuzzleInstance,
    resolve_fracture_with_agi, can_use_agi_automation,
};
use simulation::fracture::puzzle_trait::{PuzzleState, PuzzleAction};

// =============================================================================
// COMPONENTS
// =============================================================================

/// Attached to any entity that currently has an active fracture.
#[derive(Component)]
pub struct ActiveFracture {
    pub fracture: Fracture,
    pub puzzle: PuzzleInstance,
}

/// Marker for entities that can interact with fractures (usually the player).
#[derive(Component)]
pub struct FractureInteractor;

/// Optional marker to show fracture-related UI for this entity.
#[derive(Component)]
pub struct ShowFractureUI;

// =============================================================================
// RESOURCES
// =============================================================================

#[derive(Resource, Default)]
pub struct FractureResolutionSkill {
    pub level: u32,
    pub experience: u64,
}

#[derive(Resource, Default)]
pub struct FractureSettings {
    pub auto_discover_on_event: bool,
    pub show_debug_ui: bool,
}

// =============================================================================
// PLUGIN
// =============================================================================

pub struct FracturePlugin;

impl Plugin for FracturePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FractureResolutionSkill>()
            .init_resource::<FractureSettings>()
            .add_systems(Update, (
                fracture_discovery_system,
                fracture_input_system,
                fracture_ui_update_system,
                fracture_completion_system,
                fracture_debug_ui,
            ).chain());
    }
}

// =============================================================================
// SYSTEMS
// =============================================================================

/// Discovery system (placeholder).
pub fn fracture_discovery_system(
    mut commands: Commands,
    settings: Res<FractureSettings>,
) {
    if !settings.auto_discover_on_event {
        return;
    }
    // Real implementation would spawn ActiveFracture based on world events.
}

/// Input system - handles player actions on active fractures.
/// Replace the keyboard checks with your real input mapping.
pub fn fracture_input_system(
    mut query: Query<(&mut ActiveFracture, Option<&ShowFractureUI>), With<FractureInteractor>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    skill: Res<FractureResolutionSkill>,
) {
    for (mut active, show_ui) in query.iter_mut() {
        // Example: Press SPACE to send a generic "progress" action
        if keyboard.just_pressed(KeyCode::Space) {
            // In a real game you would map this to a specific PuzzleAction
            // based on the current puzzle type and selected option.
        }

        // Press G to attempt AGi resolution (if unlocked)
        if keyboard.just_pressed(KeyCode::KeyG) {
            if can_use_agi_automation(skill.level, true) {
                let _ = resolve_fracture_with_agi(
                    &mut active.fracture,
                    &mut active.puzzle,
                    true,
                );
            }
        }

        // Example: Press R to reset the current puzzle attempt
        if keyboard.just_pressed(KeyCode::KeyR) {
            let _ = active.puzzle.state.apply_action(PuzzleAction::Reset);
        }
    }
}

/// Updates any UI elements related to the active fracture.
/// This is where you would sync Bevy UI text, progress bars, etc.
pub fn fracture_ui_update_system(
    query: Query<(&ActiveFracture, &ShowFractureUI)>,
) {
    for (active, _ui_marker) in query.iter() {
        let progress = active.puzzle.state.get_progress();
        let summary = active.puzzle.state.get_current_state_summary();

        // TODO: Update your actual UI components here.
        // Example with bevy_egui or your custom UI:
        // ui_text_section.set_text(format!("Fracture Progress: {:.0}%", progress * 100.0));
        // ui_summary.set_text(summary);
    }
}

/// Awards experience and removes solved fractures.
pub fn fracture_completion_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ActiveFracture)>,
    mut skill: ResMut<FractureResolutionSkill>,
) {
    for (entity, mut active) in query.iter_mut() {
        if active.puzzle.state.is_solved() {
            let xp = (active.fracture.difficulty * 200.0) as u64;
            skill.experience += xp;

            while skill.experience >= xp_required_for_level(skill.level + 1) {
                skill.level += 1;
            }

            commands.entity(entity).remove::<ActiveFracture>();
        }
    }
}

fn xp_required_for_level(level: u32) -> u64 {
    800 + (level as u64 * 400)
}

/// Simple debug output (replace with real UI).
pub fn fracture_debug_ui(
    query: Query<&ActiveFracture>,
    settings: Res<FractureSettings>,
) {
    if !settings.show_debug_ui {
        return;
    }

    for active in query.iter() {
        let progress = active.puzzle.state.get_progress();
        if (progress * 100.0) as i32 % 15 == 0 {
            println!(
                "[Fracture] {:.1}% | {}",
                progress * 100.0,
                active.puzzle.state.get_current_state_summary()
            );
        }
    }
}

// =============================================================================
// PUBLIC API / HELPERS
// =============================================================================

/// Try to resolve the fracture using AGi (if the player has access).
pub fn try_agi_resolution(active: &mut ActiveFracture, skill_level: u32) -> bool {
    if can_use_agi_automation(skill_level, true) {
        return resolve_fracture_with_agi(&mut active.fracture, &mut active.puzzle, true).is_ok();
    }
    false
}

/// Send a specific PuzzleAction to the active fracture.
/// Use this from your input or UI button handlers.
pub fn send_puzzle_action(active: &mut ActiveFracture, action: PuzzleAction) -> bool {
    active.puzzle.state.apply_action(action).is_ok()
}
