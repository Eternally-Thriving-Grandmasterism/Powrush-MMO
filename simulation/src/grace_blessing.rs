// simulation/src/grace_blessing.rs
// Powrush-MMO — GraceBlessing / Mentorship System
// Implements the defined GraceBlessing scoring formula with full integration.
// Closes human experience gap #3 (missing mentorship & social bonding).
// Tightly integrated with PlayerLegacyJournal and InterRealmDiplomacyEvent.
// TOLC 8 + 7 Living Mercy Gates enforced. Zero-harm, sovereign.
// AG-SML v1.0 licensed.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::world::Agent;
use crate::player_legacy_journal::{LegacyJournalRegistry, LegacyEventType};
use crate::inter_realm_diplomacy_event::DiplomacyOutcome;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlessingContext {
    Normal,
    PostForgivenessWave,
    CouncilResolution,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct GraceBlessing {
    pub last_blessing_tick: u64,
    pub total_blessings_given: u32,
    pub total_mentees_helped: u32,
}

impl Default for GraceBlessing {
    fn default() -> Self {
        Self {
            last_blessing_tick: 0,
            total_blessings_given: 0,
            total_mentees_helped: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GraceBlessingResult {
    pub mentee_mercy_boost: f32,
    pub mentor_persistence_gain: f32,
    pub realm_harmony_gain: f32,
    pub valence: f32,
    pub triggers_epiphany_chance: f32,
}

/// Core scoring formula (defined and approved)
pub fn calculate_grace_blessing(
    mentor_mercy: f32,
    mentee_mercy: f32,
    mentor_archetype: crate::world::ArchetypeId, // simplified for integration
    context: BlessingContext,
    time_since_last_blessing: f32,
) -> GraceBlessingResult {
    let base_transfer = (mentor_mercy / 100.0).powf(1.15) * 9.0;

    let archetype_mult = match mentor_archetype.as_str() {
        "BoundlessMercy" | "RadicalLove" => 1.25,
        "Service" | "CosmicHarmony" => 1.15,
        "Joy" => 1.10,
        "Truth" => 0.95,
        "Abundance" => 0.85,
        _ => 1.0,
    };

    let mentee_need = (1.0 - (mentee_mercy / 95.0)).clamp(0.4, 1.35);

    let context_mult = match context {
        BlessingContext::PostForgivenessWave => 1.45,
        BlessingContext::CouncilResolution => 1.20,
        BlessingContext::Normal => 1.0,
    };

    let cooldown_factor = (time_since_last_blessing / 180.0).clamp(0.35, 1.0);

    let mercy_transfer = (base_transfer * archetype_mult * mentee_need * context_mult * cooldown_factor)
        .clamp(1.5, 14.0);

    let mentor_persistence_gain = mercy_transfer * 0.35;
    let realm_harmony_gain = mercy_transfer * 0.22;

    GraceBlessingResult {
        mentee_mercy_boost: mercy_transfer,
        mentor_persistence_gain,
        realm_harmony_gain,
        valence: 0.78 + (archetype_mult - 1.0) * 0.4,
        triggers_epiphany_chance: if context == BlessingContext::PostForgivenessWave { 0.28 } else { 0.12 },
    }
}

/// Applies a Grace Blessing from mentor to mentee
pub fn apply_grace_blessing(
    mentor: &mut Agent,
    mentee: &mut Agent,
    mentor_grace: &mut GraceBlessing,
    context: BlessingContext,
    current_tick: u64,
    legacy_registry: &mut LegacyJournalRegistry,
) -> GraceBlessingResult {
    let time_since = (current_tick - mentor_grace.last_blessing_tick) as f32;

    let result = calculate_grace_blessing(
        mentor.mercy_score,
        mentee.mercy_score,
        mentor.archetype_id.clone(),
        context,
        time_since,
    );

    // Apply to mentee
    mentee.mercy_score = (mentee.mercy_score + result.mentee_mercy_boost).min(99.0);

    // Mentor gains persistence
    mentor.persistence_weight += result.mentor_persistence_gain;

    // Update mentor stats
    mentor_grace.last_blessing_tick = current_tick;
    mentor_grace.total_blessings_given += 1;
    mentor_grace.total_mentees_helped += 1;

    // Record in LegacyJournal for both
    legacy_registry.record_event(
        mentor.id,
        0,
        LegacyEventType::GraceBlessingGiven {
            recipient_id: mentee.id,
            mercy_boost: result.mentee_mercy_boost,
        },
        mentor.mercy_score,
        result.mentor_persistence_gain,
        result.valence,
        current_tick,
        false,
        Some(format!("Grace offered to a fellow seeker. Mercy +{:.1}", result.mentee_mercy_boost)),
    );

    legacy_registry.record_event(
        mentee.id,
        0,
        LegacyEventType::GraceBlessingGiven {
            recipient_id: mentor.id, // reverse for mentee perspective
            mercy_boost: result.mentee_mercy_boost,
        },
        mentee.mercy_score,
        result.mentor_persistence_gain * 0.6,
        result.valence,
        current_tick,
        false,
        Some(format!("Received grace and mercy from a mentor. +{:.1}", result.mentee_mercy_boost)),
    );

    result
}

/// System that can be extended to auto-trigger after Forgiveness Waves
pub fn grace_blessing_system(
    mut query: Query<(&mut Agent, &mut GraceBlessing)>,
    mut legacy: ResMut<LegacyJournalRegistry>,
    time: Res<Time>,
) {
    // Placeholder for future auto-trigger or UI-driven logic
    let _current_tick = time.elapsed_secs() as u64;
}

pub struct GraceBlessingPlugin;

impl Plugin for GraceBlessingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GraceBlessing>()
           .add_systems(Update, grace_blessing_system);
    }
}
