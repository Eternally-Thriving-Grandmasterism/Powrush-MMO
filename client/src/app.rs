/*!
 * App setup — Updated with WorldSimulation + Mirror + Data Hooks + Ship systems
 */ 

use bevy::prelude::*;
use crate::world_simulation::{setup_world_simulation, register_data_collection_hooks};
use crate::ships::{ship_instability_to_mirror_contribution_system /*, other ship systems */};

pub fn setup_app(app: &mut App) {
    // ... existing setup
    setup_world_simulation(app);
    register_data_collection_hooks(app);
    app.add_systems(Update, ship_instability_to_mirror_contribution_system);
    // Future: add Hybrid instability detection, mitigation, etc.
}
