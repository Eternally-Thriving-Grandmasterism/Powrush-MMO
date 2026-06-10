/*!
 * Powrush-MMO v18.9 — Bot Detection System
 *
 * Combines behavioral heuristics and telemetry anomaly detection.
 * Designed for closed beta protection while remaining sovereign-friendly.
 *
 * Two main layers:
 * 1. Behavioral Heuristics (client-side input patterns)
 * 2. Telemetry Anomaly Scoring (server/simulation side)
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

// === Configuration ===

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct BotDetectionConfig {
    pub enabled: bool,
    pub behavioral_heuristics_enabled: bool,
    pub telemetry_anomaly_enabled: bool,
    pub anomaly_threshold: f32,           // Score above this = suspicious
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

// === Behavioral Metrics (collected client-side) ===

#[derive(Debug, Clone, Default)]
pub struct BehavioralMetrics {
    pub action_intervals: VecDeque<f32>,   // Time between actions (seconds)
    pub mouse_velocity_variance: f32,
    pub click_precision_score: f32,        // Lower = more human-like
    pub total_actions: u32,
}

impl BehavioralMetrics {
    pub fn add_action_interval(&mut self, interval: f32) {
        if self.action_intervals.len() > 50 {
            self.action_intervals.pop_front();
        }
        self.action_intervals.push_back(interval);
    }

    /// Calculates a simple "human-likeness" score (0.0 = very bot-like, 1.0 = human)
    pub fn calculate_human_score(&self) -> f32 {
        if self.action_intervals.len() < 5 {
            return 0.5;
        }

        // High variance in timing = more human
        let mean: f32 = self.action_intervals.iter().sum::<f32>() / self.action_intervals.len() as f32;
        let variance: f32 = self.action_intervals.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f32>() / self.action_intervals.len() as f32;

        // Normalize variance (higher variance = more human)
        let timing_score = (variance * 10.0).clamp(0.0, 1.0);

        // Lower click precision = more human
        let precision_score = (1.0 - self.click_precision_score.clamp(0.0, 1.0)).max(0.0);

        (timing_score * 0.7 + precision_score * 0.3).clamp(0.0, 1.0)
    }
}

// === Telemetry Anomaly Scoring ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyScore {
    pub overall_score: f32,           // 0.0 - 1.0
    pub epiphany_anomaly: f32,
    pub mercy_alignment_anomaly: f32,
    pub action_rate_anomaly: f32,
    pub suspicion_level: u8,          // 0-3
}

impl Default for AnomalyScore {
    fn default() -> Self {
        Self {
            overall_score: 0.0,
            epiphany_anomaly: 0.0,
            mercy_alignment_anomaly: 0.0,
            action_rate_anomaly: 0.0,
            suspicion_level: 0,
        }
    }
}

/// Analyzes an EpiphanyTelemetryEvent and returns an anomaly score
pub fn calculate_epiphany_anomaly(
    event: &crate::epiphany_catalyst::EpiphanyTelemetryEvent,
    historical_average_intensity: f32,
) -> AnomalyScore {
    let mut score = AnomalyScore::default();

    // Extremely high intensity too frequently is suspicious
    if event.intensity > 0.9 && historical_average_intensity < 0.6 {
        score.epiphany_anomaly = 0.6;
    }

    // Perfect or near-perfect mercy alignment across many events
    if event.mercy_alignment_score > 0.95 {
        score.mercy_alignment_anomaly = 0.4;
    }

    // Very high epiphany multiplier combined with low participant count (single player)
    if event.epiphany_multiplier > 1.8 && event.participant_count <= 1 {
        score.epiphany_anomaly = (score.epiphany_anomaly + 0.3).min(1.0);
    }

    score.overall_score = (score.epiphany_anomaly + score.mercy_alignment_anomaly + score.action_rate_anomaly) / 3.0;

    if score.overall_score > 0.75 {
        score.suspicion_level = 2;
    } else if score.overall_score > 0.5 {
        score.suspicion_level = 1;
    }

    score
}

/// Simple behavioral check that can be called from client input systems
pub fn is_likely_bot_behavior(metrics: &BehavioralMetrics) -> bool {
    let human_score = metrics.calculate_human_score();
    human_score < 0.25
}
