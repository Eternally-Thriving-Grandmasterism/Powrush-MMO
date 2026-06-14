/*!
 * Bevy ECS Integration for Lattice Fracture Resolution
 *
 * Expanded version with input handling, progress tracking, and AGi/manual flow.
 */

use bevy::prelude::*;
use simulation::fracture::{
    Fracture, PuzzleInstance, FractureType,
    generate_fracture, GenerationParams,
    resolve_fracture_with_agi, can_use_agi_automation,
};
use simulation::fracture::puzzle_trait::{PuzzleState, PuzzleAction};

// =============================================================================
// COMPONENTS
// =============================================================================

/// Component attached to entities that currently have an active fracture.
#[derive(Component)]
pub struct ActiveFracture {
    pub fracture: Fracture,
    pub puzzle: PuzzleInstance,
}

/// Marker component for entities that can interact with fractures (e.g. player).
#[derive(Component)]
pub struct FractureInteractor;

// =============================================================================
// RESOURCES
// =============================================================================

/// Tracks the player's Fracture Resolution skill progression.
#[derive(Resource)]
pub struct FractureResolutionSkill {
    pub level: u32,
    pub experience: u64,
}

impl Default for FractureResolutionSkill {
    fn default() -> Self {
        Self {
            level: 1,
            experience: 0,
        }
    }
}

/// Global settings for fracture behavior in the game.
#[derive(Resource)]
pub struct FractureSettings {
    pub auto_discover_on_event: bool,
    pub show_debug_ui: bool,
}

impl Default for FractureSettings {
    fn default() -> Self {
        Self {
            auto_discover_on_event: true,
            show_debug_ui: false,
        }
    }
}

// =============================================================================
// PLUGIN
// =============================================================================

pub struct FracturePlugin;

impl Plugin for FracturePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FractureResolutionSkill>()
            .init_resource::<FractureSettings>()
            .add_systems(Update, fracture_discovery_system)
            .add_systems(Update, fracture_input_system)
            .add_systems(Update, fracture_completion_system)
            .add_systems(Update, fracture_debug_ui);
    }
}

// =============================================================================
// SYSTEMS
// =============================================================================

/// Placeholder discovery system.
/// In a real game this would be triggered by world events, exploration, or combat.
pub fn fracture_discovery_system(
    mut commands: Commands,
    settings: Res<FractureSettings>,
    skill: Res<FractureResolutionSkill>,
) {
    if !settings.auto_discover_on_event {
        return;
    }

    // Example: Occasionally spawn a test fracture (remove in real game)
    // This demonstrates how to create an ActiveFracture entity.
}

/// Handles player input for interacting with active fractures.
/// This is a placeholder. Connect it to your real input system (keyboard, UI buttons, etc.).
pub fn fracture_input_system(
    mut query: Query<&mut ActiveFracture, With<FractureInteractor>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    skill: Res<FractureResolutionSkill>,
) {
    for mut active in query.iter_mut() {
        // Example: Press SPACE to attempt manual progress on the puzzle
        if keyboard.just_pressed(KeyCode::Space) {
            // In a real implementation you would send a specific PuzzleAction here
            // based on player input or UI selection.
            // For now this is just a placeholder.
        }

        // Example: Press G to let AGi resolve the fracture (if unlocked)
        if keyboard.just_pressed(KeyCode::KeyG) {
            let has_access = can_use_agi_automation(skill.level, true); // placeholder
            if has_access {
                let _ = resolve_fracture_with_agi(
                    &mut active.fracture,
                    &mut active.puzzle,
                    true,
                );
            }
        }
    }
}

/// Awards experience and cleans up when a fracture is solved.
pub fn fracture_completion_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ActiveFracture)>,
    mut skill: ResMut<FractureResolutionSkill>,
) {
    for (entity, mut active) in query.iter_mut() {
        if active.puzzle.state.is_solved() {
            let base_xp = (active.fracture.difficulty * 180.0) as u64;
            skill.experience += base_xp;

            // Simple level-up logic
            while skill.experience >= xp_required_for_level(skill.level + 1) {
                skill.level += 1;
            }

            commands.entity(entity).remove::<ActiveFracture>();
        }
    }
}

fn xp_required_for_level(level: u32) -> u64 {
    800 + (level as u64 * 350)
}

/// Simple debug UI (text) showing fracture status.
/// Replace with proper UI (eg. using bevy_egui or your UI framework) in production.
pub fn fracture_debug_ui(
    query: Query<&ActiveFracture>,
    settings: Res<FractureSettings>,
) {
    if !settings.show_debug_ui {
        return;
    }

    for active in query.iter() {
        let progress = active.puzzle.state.get_progress();
        let summary = active.puzzle.state.get_current_state_summary();

        // In a real game you would render this nicely in the UI layer.
        // For now we just print occasionally.
        if (progress * 100.0) % 10.0 < 1.0 {
            println!("[Fracture] Progress: {:.1}% | {}", progress * 100.0, summary);
        }
    }
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/// Attempts to resolve the active fracture using the player's AGi (if unlocked).
pub fn try_agi_resolution(
    active: &mut ActiveFracture,
    skill_level: u32,
) -> bool {
    if can_use_agi_automation(skill_level, true) {
        if resolve_fracture_with_agi(&mut active.fracture, &mut active.puzzle, true).is_ok() {
            return true;
        }
    }
    false
}

/// Sends a manual PuzzleAction to the active fracture.
/// Call this from your input or UI systems.
pub fn send_puzzle_action(
    active: &mut ActiveFracture,
    action: PuzzleAction,
) -> bool {
    active.puzzle.state.apply_action(action).is_ok()
}
