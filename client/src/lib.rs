//! Powrush-MMO lived first hour.
//!
//! One human, one machine. Network, simulation crate, GPU-test spheres, and
//! Steam-as-blocker stay off this graph. Contact: info@Rathor.ai

use bevy::prelude::*;

pub mod lived_hour_support;
pub mod hour_sacred;
pub mod vertical_factory;
pub mod coop_voice;
pub mod infra_spill;
pub mod ledger_bind;
pub mod fabricator;
pub mod input;
pub mod soft_play_bindings;
pub mod first_session_guidance;
pub mod first_hour_camera;
pub mod harvest_feel;
pub mod first_harvest_epiphany;
pub mod mercy_harvest_nodes;
pub mod human_inventory;
pub mod local_session_persist;
pub mod lived_sim_bridge;
pub mod local_sovereign_session;
pub mod living_practice_loop;
pub mod rbe_allocate_choice;
pub mod thriving_moments;
pub mod human_soft_panels;
pub mod human_presence;
pub mod first_whisper;
pub mod local_human_sim;
pub mod player_lineage;
pub mod living_ecology;
pub mod living_body;
pub mod living_freshness;
pub mod living_day;
pub mod companion_bond;
pub mod flow_weather;
pub mod abundance_journey_echo;
pub mod world_answer;
pub mod lattice_flow_share;
pub mod climate_plane;
pub mod hands_memory;
pub mod foundation_lattice;
pub mod resonance_flavors;
pub mod mercy_transporters;
pub mod steam_abundance_mirror;

pub use first_session_guidance::{FirstSessionGuidancePlugin, FirstSessionGuidance};
pub use input::InputPlugin;
pub use lived_hour_support::LivedHourEconomyPlugin;

/// Default lived-hour plugin graph. This is the player door.
pub struct PowrushClientBundle;

impl Default for PowrushClientBundle {
    fn default() -> Self {
        Self
    }
}

impl PowrushClientBundle {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for PowrushClientBundle {
    fn build(&self, app: &mut App) {
        app.add_plugins(hour_sacred::HourSacredPlugin);
        app.add_plugins(vertical_factory::VerticalFactoryPlugin);
        app.add_plugins(coop_voice::CoopVoicePlugin);
        app.add_plugins(infra_spill::InfraSpillPlugin);
        app.add_plugins(ledger_bind::LedgerBindPlugin);
        app.add_plugins(fabricator::FabricatorPlugin);
        app.add_plugins(LivedHourEconomyPlugin);
        app.add_plugins(InputPlugin);
        app.add_plugins(first_session_guidance::FirstSessionGuidancePlugin);
        app.add_plugins(thriving_moments::ThrivingMomentsPlugin);
        app.add_plugins(living_practice_loop::LivingPracticeLoopPlugin);
        app.add_plugins(rbe_allocate_choice::RbeAllocateChoicePlugin);
        app.add_plugins(abundance_journey_echo::AbundanceJourneyEchoPlugin);
        app.add_plugins(mercy_harvest_nodes::MercyHarvestNodesPlugin);
        app.add_plugins(climate_plane::ClimatePlanePlugin);
        app.add_plugins(living_ecology::LivingEcologyPlugin);
        app.add_plugins(living_freshness::LivingFreshnessPlugin);
        app.add_plugins(living_body::LivingBodyPlugin);
        app.add_plugins(living_day::LivingDayPlugin);
        app.add_plugins(hands_memory::HandsMemoryPlugin);
        app.add_plugins(companion_bond::CompanionBondPlugin);
        app.add_plugins(harvest_feel::HarvestFeelPlugin);
        app.add_plugins(world_answer::WorldAnswerPlugin);
        app.add_plugins(first_harvest_epiphany::FirstHarvestEpiphanyPlugin);
        app.add_plugins(human_inventory::HumanInventoryPlugin);
        app.add_plugins(local_session_persist::LocalSessionPersistPlugin);
        app.add_plugins(first_whisper::FirstWhisperPlugin);
        app.add_plugins(first_hour_camera::FirstHourCameraPlugin);
        app.add_plugins(human_soft_panels::HumanSoftPanelsPlugin);
        app.add_plugins(lattice_flow_share::LatticeFlowSharePlugin);
        app.add_plugins(steam_abundance_mirror::SteamAbundanceMirrorPlugin);
        app.add_plugins(foundation_lattice::FoundationLatticePlugin);
        app.add_plugins(resonance_flavors::ResonanceFlavorsPlugin);
        app.add_plugins(mercy_transporters::MercyTransportersPlugin);
        app.add_plugins(human_presence::HumanPresencePlugin);
        app.add_plugins(flow_weather::FlowWeatherPlugin);
        app.add_plugins(local_human_sim::LocalHumanSimPlugin);
        app.add_plugins(player_lineage::PlayerLineagePlugin);
        app.add_plugins(lived_sim_bridge::LivedSimBridgePlugin);
        app.add_plugins(local_sovereign_session::LocalSovereignSessionPlugin);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundle_is_a_plugin() {
        let _ = PowrushClientBundle::new();
    }

    #[test]
    fn first_session_starts_on_move() {
        let g = first_session_guidance::FirstSessionGuidance::default();
        assert!(g.active);
        assert!(!g.dismissed);
    }
}
