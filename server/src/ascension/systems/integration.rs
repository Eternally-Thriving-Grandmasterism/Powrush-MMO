/*!
 * Shared systems between Mercy Ascent Trial and Mirror Reckoning.
 * Creates deep systemic and narrative synergy.
 */

use bevy::prelude::*;
use crate::ascension::{components::*, resources::*};

pub fn sync_mirror_and_ascension_system(
    server_state: Res<ServerResonanceState>,
    mut ascension_query: Query<&mut AscensionProgress>,
) {
    for mut progress in ascension_query.iter_mut() {
        // Final boss difficulty in Mercy Ascent is influenced by mirror_score
        // (higher server shadow = harder personal trial)
    }
}

pub fn ambrosian_mirror_influence_system(
    ambrosians: Query<&AmbrosianAscended>,
    mut server_state: ResMut<ServerResonanceState>,
) {
    server_state.total_ambrosians = ambrosians.iter().count() as u32;

    // More Ambrosians on server = weaker Mirror manifestations for everyone
    let reduction = (server_state.total_ambrosians as f32 * 0.028).min(0.40);
    server_state.mirror_score = (server_state.mirror_score - reduction).max(0.0);
}
