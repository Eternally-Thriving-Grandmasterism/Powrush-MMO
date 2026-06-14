/*!
 * Lattice Fracture Resolution System
 */

pub mod agi;
pub mod generation;
pub mod puzzles;
pub mod puzzle_trait;
pub mod types;

pub use agi::{can_use_agi_automation, resolve_fracture_with_agi, AgiError, AgiResolutionResult};
pub use generation::{generate_fracture, GenerationParams};
pub use types::{Fracture, FractureType, PuzzleInstance};
pub use puzzle_trait::{PuzzleState, PuzzleAction, ActionResult, PuzzleError};
