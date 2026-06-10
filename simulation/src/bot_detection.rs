/*!
 * Powrush-MMO v18.9 — Bot Detection System (Integrated)
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::epiphany_catalyst::EpiphanyTelemetryEvent;

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct BotDetectionConfig {
    pub enabled: bool,
    pub behavioral_heuristics_enabled: bool,
    pub telemetry_anomaly_enabled: bool,
    pub anomaly_threshold: f32,
    pub max_suspicion_level: u8,
}

impl Default for BotDetectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            behavioral_heuristics_enabled: true,
            telemetry_anomaly_enabled: true,
            anomaly_threshold: 0.75,
            max_suspicion_level: 3,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct BehavioralMetrics {
    pub action_intervals: VecDeque<f32>,
    pub mouse_velocity_variance: f32,
    pub click_precision_score: f32,
    pub harvest_rhythm_variance: f32,     // New: consistency of harvest timing
    pub total_actions: u32,
}

impl BehavioralMetrics {
    pub fn add_action_interval(&mut self, interval: f32) {
        if self.action_intervals.len() > 50 { self.action_intervals.pop_front(); }
        self.action_intervals.push_back(interval);
    }

    pub fn calculate_human_score(&self) -> f32 {
        if self.action_intervals.len() < 5 { return 0.5; }

        let mean: f32 = self.action_intervals.iter().sum::<f32>() / self.action_intervals.len() as f32;
        let variance: f32 = self.action_intervals.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / self.action_intervals.len() as f32;

        let timing_score = (variance * 10.0).clamp(0.0, 1.0);
        let precision_score = (1.0 - self.click_precision_score.clamp(0.0, 1.0)).max(0.0);
        let rhythm_score = (self.harvest_rhythm_variance * 8.0).clamp(0.0, 1.0);

        (timing_score * 0.5 + precision_score * 0.25 + rhythm_score * 0.25).clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnomalyScore {
    pub overall_score: f32,
    pub epiphany_anomaly: f32,
    pub mercy_alignment_anomaly: f32,
    pub action_rate_anomaly: f32,
    pub behavioral_anomaly: f32,          // New
    pub suspicion_level: u8,
}

#[derive(Resource, Debug, Clone, Default)]
pub struct BotSuspicion {
    pub current_level: u8,                // 0-3
    pub last_updated_ms: u64,
    pub total_flags: u32,
}

pub fn calculate_epiphany_anomaly(
    event: &EpiphanyTelemetryEvent,
    historical_average_intensity: f32,
    behavioral_human_score: f32,        // New parameter
) -> AnomalyScore {
    let mut score = AnomalyScore::default();

    if event.intensity > 0.9 && historical_average_intensity < 0.6 {
        score.epiphany_anomaly = 0.55;
    }
    if event.mercy_alignment_score > 0.95 {
        score.mercy_alignment_anomaly = 0.35;
    }
    if event.epiphany_multiplier > 1.8 && event.participant_count <= 1 {
        score.epiphany_anomaly = (score.epiphany_anomaly + 0.25).min(1.0);
    }

    // Behavioral anomaly: low human score + high epiphany activity
    if behavioral_human_score < 0.3 && event.intensity > 0.7 {
        score.behavioral_anomaly = 0.6;
    }

    score.overall_score = (score.epiphany_anomaly + score.mercy_alignment_anomaly +
                          score.action_rate_anomaly + score.behavioral_anomaly) / 4.0;

    if score.overall_score > 0.8 {
        score.suspicion_level = 3;
    } else if score.overall_score > 0.6 {
        score.suspicion_level = 2;
    } else if score.overall_score > 0.4 {
        score.suspicion_level = 1;
    }

    score
}

pub fn update_suspicion(bot_suspicion: &mut BotSuspicion, anomaly: &AnomalyScore, current_time_ms: u64) {
    if anomaly.suspicion_level > bot_suspicion.current_level {
        bot_suspicion.current_level = anomaly.suspicion_level;
        bot_suspicion.last_updated_ms = current_time_ms;
        bot_suspicion.total_flags += 1;
    } else if anomaly.suspicion_level < bot_suspicion.current_level {
        // Slowly decay suspicion over time
        if current_time_ms - bot_suspicion.last_updated_ms > 300_000 { // 5 minutes
            bot_suspicion.current_level = bot_suspicion.current_level.saturating_sub(1);
        }
    }
}
