//! client/src/divine_whispers.rs
//! Divine Whispers — Mercy-gated narrative guidance + Dynamic Localization
//! Now powered by the professional Localization resource (v18.9)

use bevy::prelude::*;
use crate::localization::Localization;

#[derive(Component, Debug, Clone, Default)]
pub struct DivineWhisper {
    pub text: String,
    pub valence: f32,
    pub timestamp: f64,
    pub priority: WhisperPriority,
}

#[derive(Resource, Default, Debug)]
pub struct WhisperQueue {
    pub whispers: Vec<DivineWhisper>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WhisperPriority { #[default] Normal, High, Critical }

pub struct DivineWhispersPlugin;

impl Plugin for DivineWhispersPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WhisperQueue::default())
           .add_systems(Update, process_divine_whispers);
    }
}

fn process_divine_whispers(
    mut queue: ResMut<WhisperQueue>,
    time: Res<Time>,
    loc: Res<Localization>,
) {
    let now = time.elapsed_seconds_f64();
    queue.whispers.retain(|w| now - w.timestamp < 18.0);

    // Example: dynamically localized whisper can be injected from onboarding or systems
    // using loc.t("onboarding_welcome")
}

/// Convenience helper that respects current player language
pub fn get_localized_whisper(loc: &Localization, key: &str) -> String {
    loc.t(key)
}
