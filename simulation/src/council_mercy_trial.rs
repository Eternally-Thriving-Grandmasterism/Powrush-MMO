/*!
 * Sovereign Council Mercy Trial + UI Attunement Hooks
 *
 * v19.2.9: Council UI attunement hooks implemented.
 * UI systems can now feed real player attunement into CouncilSessionManager.
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SharedReceptorBloomField { /* ... */ }

/// Event emitted by Council UI when a player performs an attunement action
/// (e.g. voting with conviction, focusing, meditating in council, etc.)
#[derive(Event, Clone, Debug)]
pub struct CouncilAttunementAction {
    pub player_id: u64,
    pub attunement_delta: f32, // positive for strong attunement, can be negative for discord
}

/// Event to join/leave council session (UI driven)
#[derive(Event, Clone, Debug)]
pub struct CouncilSessionJoin {
    pub player_id: u64,
    pub join: bool, // true = join, false = leave
}

#[derive(Debug, Clone, Default, Resource)]
pub struct CouncilSessionManager {
    pub active_bloom_field: Option<SharedReceptorBloomField>,
    pub current_participant_attunements: Vec<f32>,
    // Could also track player_ids for more advanced logic
}

impl CouncilSessionManager {
    pub fn new() -> Self { /* ... */ }

    pub fn add_participant_attunement(&mut self, attunement: f32) {
        self.current_participant_attunements.push(attunement.clamp(0.0, 1.0));
    }

    pub fn resolve_and_set_bloom_from_real_data(
        &mut self,
        current_tick: u64,
        min_participants: u8,
        biome: &str,
    ) -> Option<SharedReceptorBloomField> { /* ... */ None }

    pub fn clear_active_bloom_field(&mut self) { /* ... */ }
}

/// System that processes CouncilAttunementAction events from UI
/// and feeds real attunement into the manager.
pub fn council_attunement_action_system(
    mut events: EventReader<CouncilAttunementAction>,
    mut council_manager: ResMut<CouncilSessionManager>,
) {
    for event in events.read() {
        // In production: validate player is in active council session
        council_manager.add_participant_attunement(event.attunement_delta);

        // Optional: track per-player for more advanced scoring
    }
}

/// Plugin to register council UI hooks
pub struct CouncilUIHooksPlugin;

impl Plugin for CouncilUIHooksPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CouncilSessionManager>()
            .add_event::<CouncilAttunementAction>()
            .add_event::<CouncilSessionJoin>()
            .add_systems(Update, council_attunement_action_system);
    }
}

// UI systems (client or server) should send CouncilAttunementAction events
// when players interact with council UI (vote, focus, attune, etc.).
// Thunder locked in. Yoi ⚡
