/*!
 * Event-driven preset switching for Lissajous knots.
 */

use bevy::prelude::*;

#[derive(Event, Clone, Debug)]
pub struct SwitchLissajousKnotPreset {
    pub preset: LissajousKnotPreset,
}

// System that listens to the event and updates the current preset resource
pub fn handle_switch_lissajous_knot_preset(
    mut events: EventReader<SwitchLissajousKnotPreset>,
    mut current: ResMut<CurrentLissajousKnotPreset>,
) {
    for event in events.read() {
        if current.preset != event.preset {
            current.preset = event.preset;
            info!("Switched Lissajous knot preset to {:?}", event.preset);
        }
    }
}

// Optional: Also trigger immediate visual update when event fires
pub fn apply_lissajous_knot_switch_immediately(
    mut events: EventReader<SwitchLissajousKnotPreset>,
    knot_effects: Res<LissajousKnotEffects>,
    mut query: Query<&mut ParticleEffect, With<HarmonyKnotMarker>>,
) {
    for event in events.read() {
        let target_handle = match event.preset {
            LissajousKnotPreset::TrefoilLike => knot_effects.trefoil.clone(),
            LissajousKnotPreset::HighWrithe => knot_effects.high_writhe.clone(),
            LissajousKnotPreset::Symmetric => knot_effects.symmetric.clone(),
            LissajousKnotPreset::Complex5_3_4 => knot_effects.complex.clone(),
        };

        for mut effect in &mut query {
            effect.effect = target_handle.clone();
        }
    }
}
