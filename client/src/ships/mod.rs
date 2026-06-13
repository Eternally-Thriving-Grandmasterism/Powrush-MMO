/*!
 * Ships Module — Expanded with Hybrid Protocol + Instability + Mirror Contribution
 */ 

use bevy::prelude::*;
use crate::world_simulation::{WorldSimulationState, MirrorReckoningState};

// ... (previous ShipClass, ShipVisualState, ActiveHybrid, calculate_hybrid_stability, etc. remain)

/// Connects ShipVisualState instability to Mirror Score contribution
pub fn ship_instability_to_mirror_contribution_system(
    query: Query<(&ShipVisualState, &ActiveHybrid)>,
    mut world_sim: ResMut<WorldSimulationState>,
) {
    for (visual, hybrid) in query.iter() {
        if hybrid.instability_level > 0.1 {
            world_sim.apply_ship_instability_contribution(
                hybrid.instability_level,
                visual.moral_alignment,
            );
        }
    }
}

// Add to plugin or app registration:
// app.add_systems(Update, ship_instability_to_mirror_contribution_system);
