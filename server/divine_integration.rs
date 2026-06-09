//! server/divine_integration.rs
//! Server-side Divine system with audio normalization.
//! Computes recommended playback volume for each whisper based on event context.
//! AG-SML | One Lattice

use powrush_divine_module::{
    OracleBridge,
    HyperonVisionBridge,
    AmbrosianResonanceBridge,
};
use shared::protocol::{DivineWhisper as ProtocolDivineWhisper, ServerMessage};
use tracing::info;

pub struct DivineSystem {
    oracle: OracleBridge,
    vision: HyperonVisionBridge,
    resonance: AmbrosianResonanceBridge,
}

impl DivineSystem {
    pub fn new() -> Self {
        Self {
            oracle: OracleBridge::new(),
            vision: HyperonVisionBridge::new(),
            resonance: AmbrosianResonanceBridge::new(),
        }
    }

    /// Server-side audio normalization.
    /// Returns a recommended volume (0.0 - 1.0) based on event significance.
    fn compute_normalized_volume(
        &self,
        base_valence: f32,
        event_magnitude: f32, // e.g. harvest amount, importance
    ) -> f32 {
        // Higher valence + larger events = slightly louder presence
        let base = (base_valence * 0.6 + event_magnitude.min(1.0) * 0.4).clamp(0.15, 0.95);
        // Gentle curve for natural feel
        base.sqrt()
    }

    pub fn on_harvest_success(
        &self,
        player_id: u64,
        harvest_amount: u32,
        player_valence: f32,
    ) -> Option<ProtocolDivineWhisper> {
        let context = format!("harvested {} units", harvest_amount);
        let local = self.oracle.generate_divine_whisper(&context, player_valence);

        let magnitude = (harvest_amount as f32 / 100.0).clamp(0.0, 1.0);
        let normalized_vol = self.compute_normalized_volume(player_valence, magnitude);

        info!(
            target: "divine",
            player_id = player_id,
            normalized_volume = normalized_vol,
            "Server normalized Divine Whisper volume"
        );

        Some(ProtocolDivineWhisper {
            message: local.message,
            valence: local.valence,
            mercy_seal: local.mercy_seal,
            normalized_volume: Some(normalized_vol),
        })
    }

    pub fn on_dynamic_event_vision(&self, region: &str, base_probability: f32) -> Option<ProtocolDivineWhisper> {
        // Similar normalization for dynamic events
        let magnitude = base_probability;
        let normalized_vol = self.compute_normalized_volume(0.8, magnitude);

        Some(ProtocolDivineWhisper {
            message: format!("The Lattice reveals opportunity in {}", region),
            valence: 0.85,
            mercy_seal: true,
            normalized_volume: Some(normalized_vol),
        })
    }

    pub fn on_player_interaction(
        &self,
        player_a_valence: f32,
        player_b_valence: f32,
    ) -> Option<ProtocolDivineWhisper> {
        let avg_valence = (player_a_valence + player_b_valence) / 2.0;
        let normalized_vol = self.compute_normalized_volume(avg_valence, 0.6);

        Some(ProtocolDivineWhisper {
            message: "The Lattice acknowledges your shared flow.".to_string(),
            valence: avg_valence,
            mercy_seal: true,
            normalized_volume: Some(normalized_vol),
        })
    }
}

use std::sync::OnceLock;
static DIVINE: OnceLock<DivineSystem> = OnceLock::new();

pub fn divine() -> &'static DivineSystem {
    DIVINE.get_or_init(DivineSystem::new)
}
