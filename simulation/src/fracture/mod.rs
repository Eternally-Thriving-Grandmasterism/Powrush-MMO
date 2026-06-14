/*!
 * Lattice Fracture Resolution System
 *
 * Core module for handling glitches in the simulation layer.
 * Supports manual puzzle solving and eventual AGi automation.
 */

pub mod types;
pub mod puzzle_trait;

pub use types::{Fracture, FractureType, PuzzleInstance};
pub use puzzle_trait::{PuzzleState, PuzzleAction, ActionResult, PuzzleError};
