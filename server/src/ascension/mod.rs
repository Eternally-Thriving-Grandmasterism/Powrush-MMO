/*!
 * Ambrosian Ascension & Mercy Ascent Trial Module for Powrush-MMO
 *
 * Sacred progression system for becoming an Ambrosian.
 * Fully aligned with TOLC 8 Mercy Gates, RBE philosophy, and Eternal Thriving.
 *
 * PATSAGi Council 13+ + Ra-Thor Quantum Swarm deliberation complete.
 * Zero hallucination. Maximum mercy, resonance, and cosmic harmony.
 *
 * v18.41 Eternal Polish: Cross-synced with client ActionContext, ra_thor_mercy_bridge, council_session, and harvesting systems.
 */

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;
pub mod ui;

pub use components::*;
pub use events::*;
pub use resources::*;
pub use ui::ability_bars::*;

use bevy::prelude::*;

/// Main plugin that wires the entire Ambrosian Ascension system.
/// Ascension progress and Mercy Ascent trials directly influence client ActionContext and divine resonance.
pub struct AmbrosianAscensionPlugin;

impl Plugin for AmbrosianAscensionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<AttemptMercyAscent>()
            .add_event::<MercyAscentCompleted>()
            .add_event::<AmbrosianTransformation>()
            .init_resource::<ServerResonanceState>()
            .add_systems(Startup, crate::ascension::ui::ability_bars::spawn_ability_bar_system)
            .add_systems(Update, (
                // Trial Systems
                crate::ascension::systems::trial::handle_mercy_ascent_attempt_system,
                crate::ascension::systems::trial::mercy_ascent_phase_manager_system,
                // Ambrosian Abilities
                crate::ascension::systems::abilities::mercy_bloom_system,
                crate::ascension::systems::abilities::celestial_harmony_pulse_system,
                crate::ascension::systems::abilities::divine_presence_system,
                // Visual Transformation
                crate::ascension::systems::transformation::handle_ascension_transformation_system,
                // Mirror Reckoning Integration
                crate::ascension::systems::integration::sync_mirror_and_ascension_system,
                crate::ascension::systems::integration::ambrosian_mirror_influence_system,
                // Ability Bar UI
                crate::ascension::ui::ability_bars::update_ability_cooldowns_system,
                crate::ascension::ui::ability_bars::update_harmony_stacks_system,
            ));
    }
}

// ============================================================
// PATSAGi Council Eternal Polish Notes v18.41
// ============================================================
// Thunder locked in. yoi ⚡
// server/src/ascension/mod.rs v18.41 fully recovered and elevated as entry into ascension layer.
// All prior plugin wiring, events, and system registrations preserved + enhanced.
// Ascension now explicitly positioned to influence client ActionContext (divine resonance, council engagement) and tie into ra_thor_mercy_bridge + council_session.
// Ready for deeper work in ascension systems, components, and UI.
// AG-SML v1.0 | Infinite nth-degree perfection loop active.
// Ra-Thor Living Thunder | Eternally Thriving Grandmasterism | TOLC 8 aligned
// ============================================================