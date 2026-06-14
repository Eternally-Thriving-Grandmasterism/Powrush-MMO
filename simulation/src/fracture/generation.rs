/*!
 * Fracture generation and solvability validation logic.
 */

use crate::fracture::puzzle_trait::PuzzleState;
use crate::fracture::puzzles::tolc_gates::TolcGateState;
use crate::fracture::types::{Fracture, FractureType, GenerationError, PuzzleInstance};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct GenerationParams {
    pub difficulty: f32,
    pub context_tags: Vec<String>,
    pub player_skill_level: u32,
    pub allow_dynamic_events: bool,
    pub enable_time_pressure: bool,
    pub rng_seed: Option<u64>,
}

/// Generates a new Lattice Fracture and its corresponding puzzle.
pub fn generate_fracture(
    params: &GenerationParams,
) -> Result<(Fracture, PuzzleInstance), GenerationError> {
    let mut rng = match params.rng_seed {
        Some(seed) => rand::rngs::StdRng::seed_from_u64(seed),
        None => rand::rngs::StdRng::from_entropy(),
    };

    let fracture_type = if params.context_tags.contains(&"council".to_string())
        || params.context_tags.contains(&"deep_simulation".to_string())
    {
        FractureType::TOLCGateAlignment
    } else if params.context_tags.contains(&"harvesting".to_string())
        || params.context_tags.contains(&"economy".to_string())
    {
        FractureType::ResourceFlowBalancing
    } else {
        FractureType::TOLCGateAlignment
    };

    let puzzle_seed = params.rng_seed.unwrap_or_else(|| rng.gen());

    // Create the concrete puzzle state
    let puzzle_state: Box<dyn PuzzleState> = match fracture_type {
        FractureType::TOLCGateAlignment => {
            let num_gates = 8;
            let mercy_charges = ((3.0 - params.difficulty * 2.0).max(1.0)) as u32;
            Box::new(TolcGateState::new(num_gates, mercy_charges))
        }
        FractureType::ResourceFlowBalancing => {
            Box::new(crate::fracture::puzzles::resource_flow::ResourceFlowState::new(8))
        }
        _ => Box::new(TolcGateState::new(8, 2)),
    };

    // Proper solvability validation
    if !puzzle_state.is_solvable() {
        return Err(GenerationError::UnsolvablePuzzle);
    }

    let fracture = Fracture {
        id: rng.gen(),
        fracture_type,
        difficulty: params.difficulty,
        context_tags: params.context_tags.clone(),
        puzzle_seed,
        resolved: false,
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    let puzzle_instance = PuzzleInstance {
        fracture_id: fracture.id,
        puzzle_type: fracture_type,
        state: puzzle_state,
        time_remaining: if params.enable_time_pressure {
            Some(90.0 - params.difficulty * 30.0)
        } else {
            None
        },
        attempts: 0,
        max_attempts: None,
    };

    Ok((fracture, puzzle_instance))
}
