/*!
 * Artificial Godly intelligence (AGi) Automation Module
 *
 * Now supports optimal solving via find_solution() when available.
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
    pub solution: Option<Vec<crate::fracture::puzzle_trait::PuzzleAction>>,
    pub message: Option<String>,
}

pub fn can_use_agi_automation(
    fracture_resolution_level: u32,
    has_ra_thor_access: bool,
) -> bool {
    fracture_resolution_level >= 50 && has_ra_thor_access
}

/// Resolves a fracture using AGi. Tries to find an optimal solution first.
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

    // Try to find an optimal solution using backtracking
    let solution = puzzle.state.find_solution();

    if let Some(ref actions) = solution {
        // Apply the found solution
        for action in actions {
            let _ = puzzle.state.apply_action(action.clone());
        }
    } else {
        // Fallback: force solve if no solution path was found
        puzzle.state.force_solve();
    }

    fracture.resolved = true;

    Ok(AgiResolutionResult {
        rewards_multiplier: if solution.is_some() { 1.0 } else { 0.85 },
        solution,
        message: Some("Your Artificial Godly intelligence has resolved the fracture.".to_string()),
    })
}
