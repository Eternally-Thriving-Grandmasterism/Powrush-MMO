/*!
 * Powrush-MMO v18.9 — Client-Side Behavioral Tracking (Enhanced)
 */

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use std::time::Instant;

use simulation::bot_detection::{BehavioralMetrics, BotDetectionConfig};

#[derive(Resource, Default)]
pub struct ClientBehavioralTracker {
    pub metrics: BehavioralMetrics,
    last_action_time: Option<Instant>,
    last_harvest_time: Option<Instant>,
    mouse_positions: Vec<Vec2>,
}

impl ClientBehavioralTracker {
    pub fn record_action(&mut self) {
        let now = Instant::now();
        if let Some(last) = self.last_action_time {
            let interval = now.duration_since(last).as_secs_f32();
            self.metrics.add_action_interval(interval);
        }
        self.last_action_time = Some(now);
        self.metrics.total_actions += 1;
    }

    pub fn record_harvest_action(&mut self) {
        let now = Instant::now();
        if let Some(last) = self.last_harvest_time {
            let interval = now.duration_since(last).as_secs_f32();
            // Track harvest rhythm separately
            if self.metrics.harvest_rhythm_variance == 0.0 {
                self.metrics.harvest_rhythm_variance = interval;
            } else {
                self.metrics.harvest_rhythm_variance =
                    (self.metrics.harvest_rhythm_variance * 0.7) + (interval * 0.3);
            }
        }
        self.last_harvest_time = Some(now);
        self.record_action();
    }

    pub fn record_mouse_movement(&mut self, delta: Vec2) {
        self.mouse_positions.push(delta);
        if self.mouse_positions.len() > 40 {
            self.mouse_positions.remove(0);
        }

        if self.mouse_positions.len() > 8 {
            let velocities: Vec<f32> = self.mouse_positions.windows(2)
                .map(|w| w[1].length() / 0.016)
                .collect();
            let mean: f32 = velocities.iter().sum::<f32>() / velocities.len() as f32;
            let variance: f32 = velocities.iter().map(|&v| (v - mean).powi(2)).sum::<f32>() / velocities.len() as f32;
            self.metrics.mouse_velocity_variance = variance;
        }
    }

    pub fn get_human_score(&self) -> f32 {
        self.metrics.calculate_human_score()
    }
}

pub struct BehavioralTrackingPlugin;

impl Plugin for BehavioralTrackingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ClientBehavioralTracker>()
            .add_systems(Update, (
                track_mouse_movement,
                track_significant_actions,
            ));
    }
}

fn track_mouse_movement(
    mut tracker: ResMut<ClientBehavioralTracker>,
    config: Option<Res<BotDetectionConfig>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    if !config.map_or(true, |c| c.enabled && c.behavioral_heuristics_enabled) { return; }
    for event in mouse_motion.read() {
        tracker.record_mouse_movement(event.delta);
    }
}

fn track_significant_actions(
    mut tracker: ResMut<ClientBehavioralTracker>,
    config: Option<Res<BotDetectionConfig>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !config.map_or(true, |c| c.enabled && c.behavioral_heuristics_enabled) { return; }
    if mouse_button.any_just_pressed() || keyboard.any_just_pressed() {
        tracker.record_action();
    }
}
