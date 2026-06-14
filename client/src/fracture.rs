/*!
 * Bevy ECS Integration for Lattice Fracture Resolution
 *
 * Allows the simulation-layer fracture system to be used inside the Bevy game.
 */

use bevy::prelude::*;
use simulation::fracture::{
    Fracture, PuzzleInstance, FractureType,
    generate_fracture, GenerationParams,
    resolve_fracture_with_agi, can_use_agi_automation,
};
use simulation::fracture::puzzle_trait::PuzzleState;

/// Component marking an entity that currently has an active fracture.
#[derive(Component)]
pub struct ActiveFracture {
    pub fracture: Fracture,
    pub puzzle: PuzzleInstance,
}

/// Resource tracking the player's Fracture Resolution skill in Bevy.
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

/// Plugin that registers fracture-related components and systems.
pub struct FracturePlugin;

impl Plugin for FracturePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FractureResolutionSkill>()
            .add_systems(Update, fracture_discovery_system)
            .add_systems(Update, fracture_completion_system);
    }
}

/// Example system that can discover fractures in the world.
/// In a real implementation this would be triggered by world events.
pub fn fracture_discovery_system(
    mut commands: Commands,
    skill: Res<FractureResolutionSkill>,
) {
    // This is a placeholder. Real discovery would come from world simulation events.
    // For now it demonstrates how to spawn an ActiveFracture.
}

/// System that checks for completed fractures and awards experience.
pub fn fracture_completion_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ActiveFracture)>,
    mut skill: ResMut<FractureResolutionSkill>,
) {
    for (entity, mut active) in query.iter_mut() {
        if active.puzzle.state.is_solved() {
            // Award experience
            let base_xp = (active.fracture.difficulty * 150.0) as u64;
            skill.experience += base_xp;

            // Level up check (simplified)
            while skill.experience >= xp_for_next_level(skill.level) {
                skill.level += 1;
            }

            // Remove the active fracture
            commands.entity(entity).remove::<ActiveFracture>();
        }
    }
}

fn xp_for_next_level(level: u32) -> u64 {
    800 + (level as u64 * 300)
}

/// Helper to trigger AGi resolution from a Bevy system.
pub fn try_resolve_with_agi_bevy(
    active: &mut ActiveFracture,
    has_agi_access: bool,
) -> bool {
    if can_use_agi_automation(active.fracture.difficulty as u32 * 10, has_agi_access) {
        if let Ok(_) = resolve_fracture_with_agi(
            &mut active.fracture,
            &mut active.puzzle,
            has_agi_access,
        ) {
            return true;
        }
    }
    false
}
