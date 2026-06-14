/*!
 * Bevy ECS Integration for Lattice Fracture Resolution
 *
 * Expanded with UI panel structure, show/hide logic, and visual patterns.
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

#[derive(Component)]
pub struct ActiveFracture {
    pub fracture: Fracture,
    pub puzzle: PuzzleInstance,
}

#[derive(Component)]
pub struct FractureInteractor;

#[derive(Component)]
pub struct ShowFractureUI;

#[derive(Component)]
pub struct FractureVisualHighlight;

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
    pub enable_visual_highlighting: bool,
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
                fracture_visual_highlighting_system,
                fracture_completion_system,
                fracture_debug_ui,
            ).chain());
    }
}

// =============================================================================
// SYSTEMS
// =============================================================================

pub fn fracture_discovery_system(
    mut commands: Commands,
    settings: Res<FractureSettings>,
) {
    if !settings.auto_discover_on_event {
        return;
    }
    // Real discovery logic here.
}

/// Input handling - connect to your real input system.
pub fn fracture_input_system(
    mut query: Query<(&mut ActiveFracture, Option<&ShowFractureUI>), With<FractureInteractor>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    skill: Res<FractureResolutionSkill>,
) {
    for (mut active, show_ui) in query.iter_mut() {
        if keyboard.just_pressed(KeyCode::Space) {
            // Send context-appropriate PuzzleAction
        }

        if keyboard.just_pressed(KeyCode::KeyG) {
            if can_use_agi_automation(skill.level, true) {
                let _ = resolve_fracture_with_agi(
                    &mut active.fracture,
                    &mut active.puzzle,
                    true,
                );
            }
        }

        if keyboard.just_pressed(KeyCode::KeyR) {
            let _ = active.puzzle.state.apply_action(PuzzleAction::Reset);
        }
    }
}

/// Updates puzzle-related UI elements.
/// This is the main place to sync your UI panels.
pub fn fracture_ui_update_system(
    query: Query<(&ActiveFracture, &ShowFractureUI)>,
) {
    for (active, _ui_marker) in query.iter() {
        let progress = active.puzzle.state.get_progress();
        let summary = active.puzzle.state.get_current_state_summary();

        // TODO: Update your actual UI here.
        // Recommended structure:
        // - Fracture status panel (always visible when ActiveFracture exists)
        // - Progress bar
        // - Current state summary text
        // - Puzzle-specific controls (depends on fracture type)
        // - AGi button (enabled only if player has access)
        //
        // Example with bevy_egui:
        // egui::Window::new("Fracture").show(ctx, |ui| {
        //     ui.add(egui::ProgressBar::new(progress));
        //     ui.label(summary);
        //     if ui.button("Use AGi (G)").clicked() { ... }
        // });
    }
}

/// Manages visual highlighting for fractured entities.
pub fn fracture_visual_highlighting_system(
    mut commands: Commands,
    query: Query<(Entity, Option<&FractureVisualHighlight>), &ActiveFracture>,
    settings: Res<FractureSettings>,
) {
    if !settings.enable_visual_highlighting {
        return;
    }

    for (entity, highlight) in query.iter() {
        if highlight.is_none() {
            commands.entity(entity).insert(FractureVisualHighlight);

            // TODO: Add real visual effect here:
            // - Outline / glow material
            // - Particle system
            // - Pulsing scale animation
            // - Special shader
        }
    }
}

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

            commands.entity(entity).remove::<FractureVisualHighlight>();
            commands.entity(entity).remove::<ActiveFracture>();
        }
    }
}

fn xp_required_for_level(level: u32) -> u64 {
    800 + (level as u64 * 400)
}

pub fn fracture_debug_ui(
    query: Query<&ActiveFracture>,
    settings: Res<FractureSettings>,
) {
    if !settings.show_debug_ui {
        return;
    }

    for active in query.iter() {
        let progress = active.puzzle.state.get_progress();
        if (progress * 100.0) as i32 % 20 == 0 {
            println!(
                "[Fracture] {:.1}% | {}",
                progress * 100.0,
                active.puzzle.state.get_current_state_summary()
            );
        }
    }
}

// =============================================================================
// PUBLIC HELPERS
// =============================================================================

pub fn try_agi_resolution(active: &mut ActiveFracture, skill_level: u32) -> bool {
    if can_use_agi_automation(skill_level, true) {
        return resolve_fracture_with_agi(&mut active.fracture, &mut active.puzzle, true).is_ok();
    }
    false
}

pub fn send_puzzle_action(active: &mut ActiveFracture, action: PuzzleAction) -> bool {
    active.puzzle.state.apply_action(action).is_ok()
}
