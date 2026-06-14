/*!
 * PuzzleState trait and related types for the Lattice Fracture system.
 */

use std::fmt::Debug;

/// Errors that can occur during puzzle interaction.
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

/// Result of applying an action to a puzzle.
#[derive(Debug, Clone)]
pub enum ActionResult {
    Success { message: Option<String> },
    Failure { reason: String },
    PartialProgress { progress: f32 },
}

/// Actions the player (or AGi) can perform on a puzzle.
#[derive(Debug, Clone)]
pub enum PuzzleAction {
    // Universal actions
    Reset,
    RequestHint,

    // Type-specific actions (examples - will be expanded per puzzle type)
    RotateGate { gate_index: usize, amount: i32 },
    AdjustFlow { connection_id: u32, delta: f32 },
    ReorderEvent { from_index: usize, to_index: usize },
    RemoveCorruptedData { indices: Vec<usize> },
}

/// Core trait that all puzzle types must implement.
pub trait PuzzleState: Send + Sync + Debug {
    /// Returns true if the puzzle is currently solved.
    fn is_solved(&self) -> bool;

    /// Apply a player or AGi action to the puzzle.
    fn apply_action(&mut self, action: PuzzleAction) -> Result<ActionResult, PuzzleError>;

    /// Returns progress as a value between 0.0 and 1.0.
    fn get_progress(&self) -> f32;

    /// Returns a list of contextual hints.
    fn get_hints(&self) -> Vec<String>;

    /// Returns a short summary of the current state (useful for UI and AGi).
    fn get_current_state_summary(&self) -> String;

    /// Required for cloning trait objects.
    fn clone_box(&self) -> Box<dyn PuzzleState>;
}

// Enable cloning of Box<dyn PuzzleState>
impl Clone for Box<dyn PuzzleState> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
