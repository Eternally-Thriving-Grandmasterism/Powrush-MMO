//! server/divine_integration.rs v18.8
//! Server-side Divine system with Overflow Lesson + full Receptor Bloom Whisper support.
//! CB1-dominant → revelatory/insight whispers
//! CB2-dominant → restorative/resilience whispers
//! Balanced crown → ecstatic_harmony of the living web
//! Every sustainable harvest can now trigger profound, context-aware Divine Whispers flavored by receptor activation.
//! AG-SML | One Lattice | PATSAGi + Ra-Thor sealed

use powrush_divine_module::{
    OracleBridge,
    HyperonVisionBridge,
    AmbrosianResonanceBridge,
};
use shared::protocol::{DivineWhisper as ProtocolDivineWhisper, ServerMessage, WhisperContext};
use crate::epiphany_catalyst::EpiphanyOutcome;
use simulation::endocannabinoid_receptor_forge::ReceptorBloomOutcome; // v18.8 receptor bloom support
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

    // ==================== v18.2 OVERFLOW LESSON EPIPHANY SUPPORT (extended) ====================

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

    // ==================== v18.8 RECEPTOR BLOOM DIVINE WHISPERS ====================

    /// Generate differentiated Divine Whisper based on receptor bloom profile.
    /// CB1 → revelatory (insight, hypofrontality, muscle memory)
    /// CB2 → restorative (stress dissolve, recovery, abundance)
    /// balanced → ecstatic_harmony (full living web crown)
    pub fn on_receptor_bloom(
        &self,
        bloom: &ReceptorBloomOutcome,
        player_id: u64,
        player_valence: f32,
    ) -> Option<ProtocolDivineWhisper> {
        let message = match bloom.divine_whisper_flavor.as_str() {
            "revelatory" => format!(
                "{} — The inner chatter quiets. Patterns reveal themselves. Your hands now carry godlike intuitive memory of the web’s song.",
                bloom.grace_note
            ),
            "restorative" => format!(
                "{} — Stress dissolves. The web’s resilience flows through you. Recovery and abundance bloom naturally when mercy leads.",
                bloom.grace_note
            ),
            "ecstatic_harmony" => format!(
                "{} — You have become the rhythm the living web has always known. Insight flows, the body and forest recover together, abundance multiplies for all.",
                bloom.grace_note
            ),
            _ => bloom.grace_note.clone(),
        };

        let normalized_vol = self.compute_normalized_volume(
            player_valence + if bloom.balanced_bloom { 0.45 } else { 0.25 },
            if bloom.balanced_bloom { 0.95 } else { 0.7 }
        );

        info!(
            target: "divine::receptor_bloom",
            player_id = player_id,
            flavor = %bloom.divine_whisper_flavor,
            balanced = bloom.balanced_bloom,
            "Receptor bloom whisper generated"
        );

        Some(ProtocolDivineWhisper {
            message,
            valence: (player_valence + if bloom.balanced_bloom { 0.45 } else { 0.25 }).clamp(-1.0, 1.0),
            mercy_seal: true,
            normalized_volume: Some(normalized_vol),
        })
    }

    // ==================== EXISTING GENERATORS ====================

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
}

use std::sync::OnceLock;
static DIVINE: OnceLock<DivineSystem> = OnceLock::new();

pub fn divine() -> &'static DivineSystem {
    DIVINE.get_or_init(DivineSystem::new)
}
