/*!
 * Lattice Fracture Resolution System
 */

pub mod generation;
pub mod puzzles;
pub mod puzzle_trait;
pub mod types;

pub use generation::{generate_fracture, GenerationParams};
pub use types::{Fracture, FractureType, PuzzleInstance};
pub use puzzle_trait::{PuzzleState, PuzzleAction, ActionResult, PuzzleError};
