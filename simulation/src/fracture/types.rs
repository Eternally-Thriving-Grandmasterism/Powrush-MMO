/*!
 * Core data types for the Lattice Fracture Resolution system.
 */

use serde::{Deserialize, Serialize};
use crate::fracture::puzzle_trait::PuzzleState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FractureType {
    TOLCGateAlignment,
    ResourceFlowBalancing,
    CausalChainReconstruction,
    PatternPurification,
    SpatialIntegrityRepair,
    ConsensusAlignment,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum GenerationError {
    #[error("Unsupported fracture type")]
    UnsupportedType,

    #[error("Failed to generate solvable puzzle")]
    UnsolvablePuzzle,

    #[error("Other generation error: {0}")]
    Other(String),
}

#[derive(Debug, Clone)]
pub struct Fracture {
    pub id: u64,
    pub fracture_type: FractureType,
    pub difficulty: f32,
    pub context_tags: Vec<String>,
    pub puzzle_seed: u64,
    pub resolved: bool,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub struct PuzzleInstance {
    pub fracture_id: u64,
    pub puzzle_type: FractureType,
    pub state: Box<dyn PuzzleState>,
    pub time_remaining: Option<f32>,
    pub attempts: u32,
    pub max_attempts: Option<u32>,
}
