/*!
 * PuzzleState trait and related types for the Lattice Fracture system.
 */

use std::fmt::Debug;

#[derive(Debug, Clone, thiserror::Error)]
pub enum PuzzleError {
    #[error("Invalid action for this puzzle type")]
    InvalidActionForPuzzleType,

    #[error("Gate is locked")]
    GateLocked,

    #[error("No mercy charges remaining")]
    NoMercyCharges,

    #[error("Puzzle is already solved")]
    AlreadySolved,

    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Clone)]
pub enum ActionResult {
    Success { message: Option<String> },
    Failure { reason: String },
    PartialProgress { progress: f32 },
}

#[derive(Debug, Clone)]
pub enum PuzzleAction {
    Reset,
    RequestHint,
    RotateGate { gate_index: usize, amount: i32 },
    AdjustFlow { connection_id: u32, delta: f32 },
    ReorderEvent { from_index: usize, to_index: usize },
    RemoveCorruptedData { indices: Vec<usize> },
}

pub trait PuzzleState: Send + Sync + Debug {
    fn is_solved(&self) -> bool;

    /// Returns true if the puzzle has at least one valid solution.
    /// Default implementation falls back to a simple check.
    fn is_solvable(&self) -> bool {
        // Default conservative implementation.
        // Concrete types should override with better logic.
        !self.is_solved() || true
    }

    /// Attempts to find a sequence of actions that solves the puzzle.
    /// Returns None if no solution is found within search limits.
    fn find_solution(&self) -> Option<Vec<PuzzleAction>>;

    fn apply_action(&mut self, action: PuzzleAction) -> Result<ActionResult, PuzzleError>;

    fn get_progress(&self) -> f32;

    fn get_hints(&self) -> Vec<String>;

    fn get_current_state_summary(&self) -> String;

    fn clone_box(&self) -> Box<dyn PuzzleState>;
}

impl Clone for Box<dyn PuzzleState> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
