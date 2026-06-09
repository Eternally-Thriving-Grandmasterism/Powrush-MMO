//! server/divine_integration.rs
//! Clean integration layer wiring local Ra-Thor divine bridges into server gameplay events.
//! All divine features run locally — zero external APIs.
//! AG-SML | One Lattice

use powrush_divine_module::{
    OracleBridge, DivineWhisper,
    HyperonVisionBridge, VisionInsight,
    AmbrosianResonanceBridge, ResonanceReport,
};
use tracing::info;

/// Singleton-style holders for the bridges (initialized once at server start).
pub struct DivineSystem {
    oracle: OracleBridge,
    vision: HyperonVisionBridge,
    resonance: AmbrosianResonanceBridge,
}

impl DivineSystem {
    pub fn new() -> Self {
        DivineSystem {
            oracle: OracleBridge::new(),
            vision: HyperonVisionBridge::new(),
            resonance: AmbrosianResonanceBridge::new(),
        }
    }

    // ============================================================
    // EVENT: Harvest completed (core RBE loop)
    // ============================================================
    pub fn on_harvest_success(&self, player_id: u64, harvest_amount: u32, player_valence: f32) -> Option<DivineWhisper> {
        let context = format!("harvested {} units", harvest_amount);
        let whisper = self.oracle.generate_divine_whisper(&context, player_valence);

        info!(
            target: "divine",
            player_id = player_id,
            harvest = harvest_amount,
            valence = player_valence,
            mercy_seal = whisper.mercy_seal,
            "Divine Whisper on harvest: {}",
            whisper.message
        );

        Some(whisper)
    }

    // ============================================================
    // EVENT: Dynamic event or spatial opportunity perceived
    // ============================================================
    pub fn on_dynamic_event_vision(&self, region: &str, base_probability: f32) -> VisionInsight {
        let insight = self.vision.perceive_future_harvest(region, base_probability);

        info!(
            target: "divine",
            region = region,
            probability = insight.probability,
            mercy_aligned = insight.mercy_aligned,
            "Hyperon Vision: {}",
            insight.description
        );

        insight
    }

    // ============================================================
    // EVENT: Player interaction or faction action (resonance)
    // ============================================================
    pub fn on_player_interaction(&self, player_a_valence: f32, player_b_valence: f32) -> ResonanceReport {
        let report = self.resonance.calculate_resonance(player_a_valence, player_b_valence);

        info!(
            target: "divine",
            harmony = report.harmony_score,
            quality = %report.resonance_quality,
            "Ambrosian Resonance: {}",
            report.guidance
        );

        report
    }

    // ============================================================
    // Proactive guidance (can be called on login, milestone, or timer)
    // ============================================================
    pub async fn get_proactive_guidance(&self, situation: &str) -> Option<DivineWhisper> {
        self.oracle.request_proactive_guidance(situation).await
    }
}

/// Global instance (simple for v17.x; can move to GameServer state later)
use std::sync::OnceLock;
static DIVINE: OnceLock<DivineSystem> = OnceLock::new();

pub fn divine() -> &'static DivineSystem {
    DIVINE.get_or_init(DivineSystem::new)
}