/*!
 * Core data types for the Lattice Fracture Resolution system.
 */

use serde::{Deserialize, Serialize};
use crate::fracture::puzzle_trait::PuzzleState;

/// Types of Lattice Fractures that can occur.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FractureType {
    TOLCGateAlignment,
    ResourceFlowBalancing,
    CausalChainReconstruction,
    PatternPurification,
    SpatialIntegrityRepair,
    ConsensusAlignment,
}

/// Represents a detected Lattice Fracture in the world.
#[derive(Debug, Clone)]
pub struct Fracture {
    pub id: u64,
    pub fracture_type: FractureType,
    pub difficulty: f32,           // 0.0 - 1.0
    pub context_tags: Vec<String>, // e.g. ["harvesting", "combat", "council"]
    pub puzzle_seed: u64,
    pub resolved: bool,
    pub created_at: u64,
}

/// Holds the runtime state of an active puzzle instance.
#[derive(Debug, Clone)]
pub struct PuzzleInstance {
    pub fracture_id: u64,
    pub puzzle_type: FractureType,
    pub state: Box<dyn PuzzleState>,
    pub time_remaining: Option<f32>,
    pub attempts: u32,
    pub max_attempts: Option<u32>,
}
