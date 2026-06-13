/*!
 * Core Components for Ambrosian Ascension System
 * TOLC 8 Mercy Gates enforced. Non-bypassable Layer 0.
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Tracks a player's progress toward becoming an Ambrosian across multiple pillars.
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct AscensionProgress {
    pub council_participations: u32,
    pub successful_council_blooms: u32,
    pub total_epiphanies: u32,
    pub average_epiphany_intensity: f32,
    pub total_abundance_contributed: f64,
    pub resonance_attunement: f32,
    pub mercy_alignment_score: f32,
    pub ascension_attempts: u32,
}

/// Marker component for players who have successfully ascended to Ambrosian.
#[derive(Component)]
pub struct AmbrosianAscended;

/// Active state while a player is inside the Mercy Ascent Trial.
#[derive(Component)]
pub struct InMercyAscentTrial {
    pub phase: TrialPhase,
    pub mercy_score: f32,
    pub start_tick: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrialPhase {
    Reckoning,
    Alignment,
    Bloom,
}

/// Tracks recent mercy-aligned behavior (core to TOLC 8 enforcement).
#[derive(Component)]
pub struct MercyAlignment {
    pub score: f32, // 0.0 – 1.0
    pub last_update_tick: u64,
}

/// Resonance attunement level (affects Epiphany quality and ability power).
#[derive(Component)]
pub struct ResonanceAttunement {
    pub value: f32,
}

// ============================================================
// ABILITY-SPECIFIC COMPONENTS (Detailed Ability Systems v1.0)
// ============================================================

/// Cooldown tracker for Mercy Bloom active ability.
#[derive(Component)]
pub struct MercyBloomCooldown {
    pub timer: Timer,
}

/// Cooldown tracker for Celestial Harmony Pulse ultimate ability.
#[derive(Component)]
pub struct CelestialHarmonyPulseCooldown {
    pub timer: Timer,
}

/// Temporary harmony stacks granted by Mercy Bloom, Divine Presence, etc.
/// Provides buffs to resonance, cooldown reduction, or healing received.
#[derive(Component)]
pub struct HarmonyStack {
    pub count: u32,
    pub duration_timer: Timer,
}

/// Marker indicating Divine Presence passive aura is active.
#[derive(Component)]
pub struct DivinePresence;
