/*!
 * Shared Resources for Ascension + Mirror Reckoning synergy.
 */

use bevy::prelude::*;

/// Server-wide resonance state. Used by both Mercy Ascent Trial and Mirror Reckoning.
#[derive(Resource, Default)]
pub struct ServerResonanceState {
    /// Higher = stronger collective shadow (makes Mercy Ascent harder).
    pub mirror_score: f32,
    pub average_mercy_alignment: f32,
    pub total_ambrosians: u32,
    pub recent_epiphany_quality: f32,
    pub last_mirror_reckoning_tick: u64,
}
