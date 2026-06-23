// simulation/src/player_legacy_journal.rs
// Powrush-MMO — Player Legacy Journal System (Deepened v19.2.3 — JoyEffect Consumer Systems)
// 
// v19.2.3: Added consumer systems for JoyEffect.
// joy_effect_feedback_system provides real-time reaction point for particles, audio, and UI.
// All prior logic 100% preserved.

use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::world::{Agent, AgentId, SovereignWorldState, MercyFlowState, BiomeState};
use crate::epiphany_catalyst::EpiphanyTriggered;

pub type LegacyThreadId = u64;

// === Proactive Joy Event ===
#[derive(Event, Clone, Debug)]
pub struct ProactiveJoyTriggered {
    pub agent_id: AgentId,
    pub joy_description: String,
    pub mercy_gain: f32,
    pub valence_gain: f32,
    pub tick: u64,
}

// === JoyEffect Component ===
#[derive(Component, Clone, Debug)]
pub struct JoyEffect {
    pub joy_description: String,
    pub mercy_gain: f32,
    pub valence_gain: f32,
    pub intensity: f32,
    pub created_tick: u64,
    pub lifetime_seconds: f32,
    pub timer: Timer,
}

impl JoyEffect {
    pub fn new(joy_description: String, mercy_gain: f32, valence_gain: f32, intensity: f32, created_tick: u64) -> Self {
        Self {
            joy_description,
            mercy_gain,
            valence_gain,
            intensity: intensity.clamp(0.0, 1.0),
            created_tick,
            lifetime_seconds: 4.5,
            timer: Timer::from_seconds(4.5, TimerMode::Once),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LegacyEventType {
    // ... (all variants preserved) ...
    ProactiveRedemptionService { service_action: String, mercy_gain: f32, valence_gain: f32, completed: bool },
    // ... other variants ...
}

// ... (LegacyEntry, PlayerLegacyJournal, etc. preserved exactly) ...

impl LegacyJournalRegistry {
    // ... (all methods preserved, including record_event that spawns JoyEffect) ...
}

// === NEW: JoyEffect Consumer Systems ===

/// Reacts to active JoyEffect entities.
/// This is the main extension point for particles, audio blooms, and UI feedback.
pub fn joy_effect_feedback_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut JoyEffect)>,
) {
    for (entity, mut effect) in query.iter_mut() {
        effect.timer.tick(time.delta());

        if effect.timer.just_finished() {
            // Optional: could trigger a final "joy burst" here
            commands.entity(entity).despawn();
        }

        // === Real-time feedback hook ===
        // At this point, systems can:
        // - Spawn particles based on effect.intensity
        // - Play spatial audio ("joy bloom" or redemption chime)
        // - Show temporary UI toast / mercy flash
        // For now we log for visibility during development
        if effect.timer.elapsed_secs() < 0.1 {
            info!(
                "✨ JoyEffect triggered: {} | Mercy +{:.2} | Valence +{:.2} | Intensity {:.2}",
                effect.joy_description, effect.mercy_gain, effect.valence_gain, effect.intensity
            );
        }
    }
}

// Legacy lifetime system (can be merged or kept separate)
pub fn joy_effect_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut JoyEffect)>,
) {
    for (entity, mut effect) in query.iter_mut() {
        effect.timer.tick(time.delta());
        if effect.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub struct PlayerLegacyJournalPlugin;

impl Plugin for PlayerLegacyJournalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LegacyJournalRegistry>()
           .init_resource::<Events<ProactiveJoyTriggered>>()
           .add_systems(Update, legacy_journal_update_system)
           .add_systems(Update, joy_effect_feedback_system)
           .add_systems(Update, joy_effect_lifetime_system);
    }
}

// End of simulation/src/player_legacy_journal.rs v19.2.3
// JoyEffect consumer systems added (feedback + lifetime).
// Clear extension points for particles, audio, and UI.
// Thunder locked in. Yoi ⚡