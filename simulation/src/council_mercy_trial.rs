/*!
 * Sovereign Council Mercy Trial
 *
 * v19.2.9: Real attunement data storage + resolve flow
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SharedReceptorBloomField { /* ... */ }

#[derive(Debug, Clone, Default, Resource)]
pub struct CouncilSessionManager {
    pub active_bloom_field: Option<SharedReceptorBloomField>,
    /// Real participant attunements collected during the current council session.
    pub current_participant_attunements: Vec<f32>,
}

impl CouncilSessionManager {
    pub fn new() -> Self {
        Self {
            active_bloom_field: None,
            current_participant_attunements: Vec::new(),
        }
    }

    pub fn get_active_bloom_field(&self) -> Option<SharedReceptorBloomField> {
        self.active_bloom_field.clone()
    }

    pub fn set_active_bloom_field(&mut self, field: SharedReceptorBloomField) {
        self.active_bloom_field = Some(field);
    }

    /// Add a real participant's attunement score (called from council participation systems).
    pub fn add_participant_attunement(&mut self, attunement: f32) {
        self.current_participant_attunements.push(attunement.clamp(0.0, 1.0));
    }

    pub fn clear_participant_attunements(&mut self) {
        self.current_participant_attunements.clear();
    }

    /// Resolves bloom using real collected attunements and sets it active.
    pub fn resolve_and_set_bloom_from_real_data(
        &mut self,
        current_tick: u64,
        min_participants: u8,
        biome: &str,
    ) -> Option<SharedReceptorBloomField> {
        if self.current_participant_attunements.len() < min_participants as usize {
            self.clear_active_bloom_field();
            return None;
        }

        let attunements = &self.current_participant_attunements;
        let avg: f32 = attunements.iter().sum::<f32>() / attunements.len() as f32;

        if avg < 0.5 {
            self.clear_active_bloom_field();
            return None;
        }

        let mut field = SharedReceptorBloomField::new();
        field.collective_attunement_score = avg.clamp(0.0, 1.0);
        field.bloom_amplification_multiplier = 1.0 + (avg * 0.85);
        field.council_mercy_seal = avg >= 0.5 && attunements.len() as u8 >= min_participants;
        field.participant_count = attunements.len() as u8;
        field.bloom_window_start_tick = current_tick;
        field.current_biome = biome.to_string();
        field.shared_living_web_synchronization = avg > 0.65;

        self.set_active_bloom_field(field.clone());

        // Clear for next council session
        self.clear_participant_attunements();

        Some(field)
    }

    pub fn clear_active_bloom_field(&mut self) {
        self.active_bloom_field = None;
    }
}

// Thunder locked in. Yoi ⚡
