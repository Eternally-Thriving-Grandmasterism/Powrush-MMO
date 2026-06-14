/*!
 * Artificial Godly intelligence (AGi) Automation Module
 *
 * Handles instant resolution of Lattice Fractures once the player
 * has earned AGi access.
 */

use crate::fracture::puzzle_trait::PuzzleState;
use crate::fracture::types::{Fracture, PuzzleInstance};

#[derive(Debug, Clone, thiserror::Error)]
pub enum AgiError {
    #[error("Player does not have AGi access")]
    AccessDenied,

    #[error("Fracture is already resolved")]
    AlreadyResolved,

    #[error("AGi failed to resolve fracture: {0}")]
    ResolutionFailed(String),
}

#[derive(Debug, Clone)]
pub struct AgiResolutionResult {
    pub rewards_multiplier: f32,
    pub message: Option<String>,
}

/// Checks if the player meets the requirements to use AGi automation.
pub fn can_use_agi_automation(
    fracture_resolution_level: u32,
    has_ra_thor_access: bool,
) -> bool {
    fracture_resolution_level >= 50 && has_ra_thor_access
}

/// Attempts to instantly resolve a fracture using the player's AGi.
pub fn resolve_fracture_with_agi(
    fracture: &mut Fracture,
    puzzle: &mut PuzzleInstance,
    has_access: bool,
) -> Result<AgiResolutionResult, AgiError> {
    if !has_access {
        return Err(AgiError::AccessDenied);
    }

    if fracture.resolved {
        return Err(AgiError::AlreadyResolved);
    }

    // AGi forces the puzzle into a solved state
    // In a more advanced implementation, this could call an optimal solver.
    puzzle.state.force_solve(); // We need to add this method to the trait or handle per-type

    fracture.resolved = true;

    Ok(AgiResolutionResult {
        rewards_multiplier: 0.9, // Slightly reduced rewards for convenience
        message: Some("Your Artificial Godly intelligence has stabilized the fracture.".to_string()),
    })
}

// Temporary helper until we add force_solve to the trait
trait ForceSolve {
    fn force_solve(&mut self);
}

impl ForceSolve for Box<dyn PuzzleState> {
    fn force_solve(&mut self) {
        // For now, we mark it solved by setting internal state if possible.
        // In production, each puzzle type should implement proper optimal solving.
        // This is a placeholder.
        println!("[AGi] Force-solving puzzle (placeholder)");
    }
}
