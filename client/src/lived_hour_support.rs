//! Lived-hour economy types.
//!
//! First hour does not link `simulation`, Ra-Thor crates, or the network RBE UI.
//! These resources are the local stand-ins first-harvest / practice already talk to.
//! Contact: info@Rathor.ai

use bevy::prelude::*;

#[derive(Resource, Default, Debug, Clone)]
pub struct RbeGlobalState {
    pub total_abundance: f32,
    pub global_harmony_score: f32,
}

#[derive(Resource, Debug)]
pub struct RbeUiSync {
    pub last_harvest_feedback: Option<String>,
    pub harvest_cooldown: Timer,
}

impl Default for RbeUiSync {
    fn default() -> Self {
        Self::new()
    }
}

impl RbeUiSync {
    pub fn new() -> Self {
        Self {
            last_harvest_feedback: None,
            harvest_cooldown: Timer::from_seconds(0.5, TimerMode::Once),
        }
    }
}

pub struct LivedHourEconomyPlugin;

impl Plugin for LivedHourEconomyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RbeGlobalState>()
            .init_resource::<RbeUiSync>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_take_credits_global() {
        let mut g = RbeGlobalState::default();
        g.total_abundance += 1.5;
        assert!(g.total_abundance > 0.0);
    }
}
