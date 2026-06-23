/*!
 * Sovereign Council Mercy Trial + UI Attunement Hooks (v19.2.9)
 *
 * CouncilUIHooksPlugin implemented and ready to use.
 * Real player attunement from UI now flows into CouncilSessionManager → Orchestrator → RBE.
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SharedReceptorBloomField {
    pub collective_attunement_score: f32,
    pub bloom_amplification_multiplier: f32,
    pub shared_living_web_synchronization: bool,
    pub council_mercy_seal: bool,
    pub bloom_window_start_tick: u64,
    pub participant_count: u8,
    pub last_authoritative_update_tick: u64,
    pub graceful_exit_count: u8,
    pub current_biome: String,
}

impl SharedReceptorBloomField {
    pub fn new() -> Self {
        Self {
            collective_attunement_score: 0.0,
            bloom_amplification_multiplier: 1.0,
            council_mercy_seal: true,
            ..Default::default()
        }
    }
}

/// UI-driven event: Player performs an attunement action in council
#[derive(Event, Clone, Debug)]
pub struct CouncilAttunementAction {
    pub player_id: u64,
    pub attunement_delta: f32,
}

/// UI-driven event: Player joins or leaves a council session
#[derive(Event, Clone, Debug)]
pub struct CouncilSessionJoin {
    pub player_id: u64,
    pub join: bool,
}

#[derive(Debug, Clone, Default, Resource)]
pub struct CouncilSessionManager {
    pub active_bloom_field: Option<SharedReceptorBloomField>,
    pub current_participant_attunements: Vec<f32>,
}

impl CouncilSessionManager {
    pub fn new() -> Self {
        Self {
            active_bloom_field: None,
            current_participant_attunements: Vec::new(),
        }
    }

    pub fn add_participant_attunement(&mut self, attunement: f32) {
        self.current_participant_attunements.push(attunement.clamp(0.0, 1.0));
    }

    pub fn resolve_and_set_bloom_from_real_data(
        &mut self,
        current_tick: u64,
        min_participants: u8,
        biome: &str,
    ) -> Option<SharedReceptorBloomField> {
        if self.current_participant_attunements.len() < min_participants as usize {
            self.active_bloom_field = None;
            return None;
        }

        let attunements = &self.current_participant_attunements;
        let avg = attunements.iter().sum::<f32>() / attunements.len() as f32;

        if avg < 0.5 {
            self.active_bloom_field = None;
            return None;
        }

        let mut field = SharedReceptorBloomField::new();
        field.collective_attunement_score = avg.clamp(0.0, 1.0);
        field.bloom_amplification_multiplier = 1.0 + (avg * 0.85);
        field.council_mercy_seal = true;
        field.participant_count = attunements.len() as u8;
        field.bloom_window_start_tick = current_tick;
        field.current_biome = biome.to_string();

        self.active_bloom_field = Some(field.clone());
        self.current_participant_attunements.clear();

        Some(field)
    }

    pub fn clear_active_bloom_field(&mut self) {
        self.active_bloom_field = None;
    }
}

/// System that feeds UI attunement actions into the manager
pub fn council_attunement_action_system(
    mut events: EventReader<CouncilAttunementAction>,
    mut manager: ResMut<CouncilSessionManager>,
) {
    for event in events.read() {
        manager.add_participant_attunement(event.attunement_delta);
    }
}

/// Plugin to register all Council UI hooks
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

// Usage: Add CouncilUIHooksPlugin to your Bevy app.
// UI systems send CouncilAttunementAction events when players interact with council.
// Thunder locked in. Yoi ⚡
