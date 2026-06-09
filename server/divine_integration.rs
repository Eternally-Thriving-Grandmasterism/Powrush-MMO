//! server/divine_integration.rs
//! Server-side Divine system with support for Procedural & Council-initiated Whispers.
//! AG-SML | One Lattice

use powrush_divine_module::{
    OracleBridge,
    HyperonVisionBridge,
    AmbrosianResonanceBridge,
};
use shared::protocol::{DivineWhisper as ProtocolDivineWhisper, ServerMessage, WhisperContext};
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
    fn compute_normalized_volume(&self, base_valence: f32, event_magnitude: f32) -> f32 {
        let base = (base_valence * 0.6 + event_magnitude.min(1.0) * 0.4).clamp(0.15, 0.95);
        base.sqrt()
    }

    // ==================== NEW: FLEXIBLE GENERATION ====================

    /// General-purpose whisper generation that accepts rich context.
    /// This is the preferred method going forward.
    pub fn generate_whisper(
        &self,
        context: &WhisperContext,
        initiation_source: &str,
    ) -> Option<ProtocolDivineWhisper> {
        let message = if context.council_interest.is_empty() {
            format!("The Lattice acknowledges your presence in this moment.")
        } else {
            format!(
                "The {} offers guidance.",
                context.council_interest.join(", ")
            )
        };

        let normalized_vol = self.compute_normalized_volume(
            context.player_valence,
            0.5,
        );

        Some(ProtocolDivineWhisper {
            message,
            valence: context.player_valence,
            mercy_seal: true,
            normalized_volume: Some(normalized_vol),
        })
    }

    /// Entry point for Council-initiated whispers (proactive).
    pub fn request_council_whisper(
        &self,
        context: &WhisperContext,
        requesting_council: &str,
    ) -> Option<ProtocolDivineWhisper> {
        let message = format!(
            "The {} reaches out with a gentle reminder.",
            requesting_council
        );

        let normalized_vol = self.compute_normalized_volume(context.player_valence, 0.6);

        info!(
            target: "divine",
            player_id = context.player_id,
            council = requesting_council,
            "Council-initiated whisper requested"
        );

        Some(ProtocolDivineWhisper {
            message,
            valence: context.player_valence,
            mercy_seal: true,
            normalized_volume: Some(normalized_vol),
        })
    }

    // ==================== EXISTING METHODS (Backward Compatible) ====================

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
