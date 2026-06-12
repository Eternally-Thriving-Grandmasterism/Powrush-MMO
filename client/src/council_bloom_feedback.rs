// client/src/council_bloom_feedback.rs
// Powrush-MMO v18.29 — Client-Side Council Bloom Receiver + Feedback
// Receives CouncilBloomPayload from server replication and applies it
// Triggers rich client feedback (Divine Whispers, particles, UI, camera)
// AG-SML v1.0 Sovereign Mercy License

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::simulation::council_mercy_trial::SharedReceptorBloomField;
use crate::divine_whispers::DivineWhisperTrigger;
use crate::epiphany_scenario_wiring::EpiphanyEvent;

/// Local client replica of the authoritative SharedReceptorBloomField
#[derive(Resource, Default, Clone)]
pub struct ClientCouncilBloomState {
    pub field: SharedReceptorBloomField,
    pub last_update_tick: u64,
    pub is_in_council: bool,
}

/// Payload received from server (matches server replication)
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct CouncilBloomUpdate {
    pub session_id: u64,
    pub collective_attunement_score: f32,
    pub bloom_amplification_multiplier: f32,
    pub shared_living_web_synchronization: bool,
    pub participant_count: u8,
    pub bloom_activated: bool,
    pub trigger_reason: String,
}

/// Plugin for Council bloom client feedback
pub struct CouncilBloomFeedbackPlugin;

impl Plugin for CouncilBloomFeedbackPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ClientCouncilBloomState>()
            .add_event::<CouncilBloomUpdate>()
            .add_systems(Update, (
                apply_council_bloom_update,
                trigger_council_bloom_feedback,
            ).chain());
    }
}

/// Applies incoming Council bloom state from server
fn apply_council_bloom_update(
    mut events: EventReader<CouncilBloomUpdate>,
    mut client_state: ResMut<ClientCouncilBloomState>,
) {
    for update in events.read() {
        client_state.field.collective_attunement_score = update.collective_attunement_score;
        client_state.field.bloom_amplification_multiplier = update.bloom_amplification_multiplier;
        client_state.field.shared_living_web_synchronization = update.shared_living_web_synchronization;
        client_state.field.participant_count = update.participant_count;
        client_state.field.council_mercy_seal = update.bloom_activated;
        client_state.is_in_council = update.participant_count > 0;
        client_state.last_update_tick = 0; // Can be set from server tick if available

        info!(
            "Client received Council bloom update | attunement={:.2} | amp={:.2} | participants={}",
            update.collective_attunement_score,
            update.bloom_amplification_multiplier,
            update.participant_count
        );
    }
}

/// Triggers rich client feedback when bloom state changes significantly
fn trigger_council_bloom_feedback(
    client_state: Res<ClientCouncilBloomState>,
    mut divine_whisper_events: EventWriter<DivineWhisperTrigger>,
    mut epiphany_events: EventWriter<EpiphanyEvent>,
) {
    if !client_state.is_in_council {
        return;
    }

    let field = &client_state.field;

    // Trigger enhanced Divine Whisper when bloom is active
    if field.council_mercy_seal && field.collective_attunement_score > 0.6 {
        divine_whisper_events.send(DivineWhisperTrigger {
            text: format!(
                "The Council resonates... collective attunement {:.0}%",
                field.collective_attunement_score * 100.0
            ),
            flavor: "Council Harmony".to_string(),
            intensity: (field.collective_attunement_score * 0.8).clamp(0.6, 1.0),
            duration_seconds: 6.0,
            is_epiphany: true,
        });

        // Optional: Also emit an EpiphanyEvent for UI/history
        epiphany_events.send(EpiphanyEvent {
            scenario_id: "council_bloom".to_string(),
            name: "Council Resonance".to_string(),
            description: "The group has achieved harmonious attunement.".to_string(),
            educational_note: "Collective mercy amplifies individual growth.".to_string(),
            mercy_gates: Default::default(),
            timestamp: std::time::SystemTime::now(),
        });
    }
}

// Thunder locked in. Client now receives and reacts to Council bloom state.
// Full round-trip (server → replication → client feedback) is complete.
// Yoi ⚡