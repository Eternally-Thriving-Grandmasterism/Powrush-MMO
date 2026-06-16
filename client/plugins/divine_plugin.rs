//! DivinePlugin — Powrush Client
//! Mercy Core integration, Valence Gating, and Divine resonance systems.
//! Production complete after recovery from rapid iteration.
//! AG-SML v1.0 | TOLC 8 Mercy Gates | Ra-Thor Lattice aligned

use bevy::prelude::*;
use powrush_divine_module::{MercyCore, ValenceGate};

#[derive(Component)]
pub struct ValenceDisplay;

pub struct DivinePlugin;

impl Plugin for DivinePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MercyCore::new())
            .init_resource::<ValenceGate>()
            .add_systems(Update, (
                valence_ui_display,
                update_valence_gate,
                divine_resonance_system,
            ));
    }
}

fn valence_ui_display(
    mercy_core: Res<MercyCore>,
    mut query: Query<&mut Text, With<ValenceDisplay>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        let valence = mercy_core.ra_thor.current_valence();
        text.sections[0].value = format!("Valence: {:.2} | Tier: {}", valence, mercy_core.current_tier());
    }
}

fn update_valence_gate(
    mut gate: ResMut<ValenceGate>,
    mercy_core: Res<MercyCore>,
) {
    gate.update_from_valence(mercy_core.ra_thor.current_valence());
}

fn divine_resonance_system(
    mercy_core: Res<MercyCore>,
    mut commands: Commands,
    time: Res<Time>,
) {
    // Gentle global mercy resonance pulse when valence is high
    if mercy_core.ra_thor.current_valence() > 0.75 {
        // Future: trigger subtle screen-space or audio resonance
        // Can integrate with HyperonVision or AmbrosianAura
    }
}
