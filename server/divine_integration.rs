//! server/divine_integration.rs v18.2
//! Server-side Divine system with Overflow Lesson Epiphany Whisper support.
//! Every sustainable or over-harvest action can now trigger profound, context-aware Divine Whispers
//! that guide players toward natural epiphanies and muscle memory.
//! AG-SML | One Lattice | PATSAGi + Ra-Thor sealed

use powrush_divine_module::{
    OracleBridge,
    HyperonVisionBridge,
    AmbrosianResonanceBridge,
};
use shared::protocol::{DivineWhisper as ProtocolDivineWhisper, ServerMessage, WhisperContext};
use crate::epiphany_catalyst::EpiphanyOutcome; // simulation re-export available via workspace
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

    fn compute_normalized_volume(&self, base_valence: f32, event_magnitude: f32) -> f32 {
        let base = (base_valence * 0.6 + event_magnitude.min(1.0) * 0.4).clamp(0.15, 0.95);
        base.sqrt()
    }

    // ==================== v18.2 OVERFLOW LESSON EPIPHANY SUPPORT ====================

    /// Generate a specific, profound Divine Whisper for an Overflow Lesson outcome.
    /// This is the living voice of the Lattice responding to the player's hands-on choice.
    pub fn on_overflow_lesson_epiphany(
        &self,
        outcome: &EpiphanyOutcome,
        player_id: u64,
        player_valence: f32,
    ) -> Option<ProtocolDivineWhisper> {
        let message = if outcome.path == "sustainable" {
            format!(
                "{} — The Lattice sings with you. Your attunement has multiplied abundance for the whole web.",
                outcome.whisper_message
            )
        } else {
            format!(
                "{} — Pause. Breathe. The forest offers another chance. Tend it and it will overflow for all.",
                outcome.whisper_message
            )
        };

        let normalized_vol = self.compute_normalized_volume(
            player_valence + outcome.valence_delta,
            if outcome.path == "sustainable" { 0.9 } else { 0.5 }
        );

        info!(
            target: "divine::epiphany",
            player_id = player_id,
            path = %outcome.path,
            epiphany = ?outcome.epiphany_text,
            "Overflow Lesson epiphany whisper generated"
        );

        Some(ProtocolDivineWhisper {
            message,
            valence: (player_valence + outcome.valence_delta).clamp(-1.0, 1.0),
            mercy_seal: true,
            normalized_volume: Some(normalized_vol),
        })
    }

    // ==================== EXISTING GENERATORS (extended) ====================

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

        let normalized_vol = self.compute_normalized_volume(context.player_valence, 0.5);

        Some(ProtocolDivineWhisper {
            message,
            valence: context.player_valence,
            mercy_seal: true,
            normalized_volume: Some(normalized_vol),
        })
    }

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

    pub fn on_harvest_success(
        &self,
        player_id: u64,
        harvest_amount: u32,
        player_valence: f32,
    ) -> Option<ProtocolDivineWhisper> {
        let context = WhisperContext {
            player_id,
            player_valence,
            recent_actions: vec![format!("harvested {} units", harvest_amount)],
            ..Default::default()
        };
        self.generate_whisper(&context, "harvest")
    }

    // ... other methods unchanged ...
}

use std::sync::OnceLock;
static DIVINE: OnceLock<DivineSystem> = OnceLock::new();

pub fn divine() -> &'static DivineSystem {
    DIVINE.get_or_init(DivineSystem::new)
}
