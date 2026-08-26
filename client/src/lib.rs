//! client/src/lib.rs
//! Powrush-MMO Client Crate Root — Public API and module declarations
//! AG-SML v1.0 | TOLC 8 Mercy Gates | v21.95.2 Mercy Transporters
//! Contact: info@Rathor.ai

pub mod networking;
pub mod replication;
pub mod prediction;
pub mod delta_compression;
pub mod rbe_client_sync;
pub mod rbe;
pub mod rbe_engine;
pub mod particles;
pub mod ui;
pub mod divine_whispers;
pub mod input;
pub mod bevy_ecs_scheduling;
pub mod ships;
pub mod world_simulation;

pub mod ambisonics_engine;
pub mod binaural_ambisonics_decoder;
pub mod higher_order_ambisonics;
pub mod rbe_client_ui_sync;
pub mod rbe_ui_feedback;
pub mod webxr_bootstrap;

pub mod example_gpu_material;

pub mod first_session_guidance;
pub mod living_practice_loop;
pub mod thriving_moments;
pub mod rbe_allocate_choice;
pub mod abundance_journey_echo;
pub mod lattice_flow_share;
pub mod steam_abundance_mirror;
pub mod foundation_lattice;
pub mod resonance_flavors;
pub mod mercy_transporters;

// Re-exports
pub use networking::NetworkingPlugin;
pub use replication::ReplicationPlugin;
pub use prediction::{PredictionPlugin, PredictedPosition, PredictedAbility, RollbackState};
pub use delta_compression::DeltaCompressionPlugin;
pub use rbe_client_sync::RbeClientSyncPlugin;
pub use rbe::{RbePlugin, RbeResource, RbeInventory, RbeResourceType};
pub use rbe_engine::RbeEnginePlugin;
pub use particles::ParticlePlugin;
pub use ui::UiPlugin;
pub use divine_whispers::DivineWhispersPlugin;
pub use input::InputPlugin;
pub use bevy_ecs_scheduling::ClientSchedulingPlugin;
pub use world_simulation::{WorldSimulationState, setup_world_simulation};

pub use ambisonics_engine::{AmbisonicsEnginePlugin, AmbisonicEmitter, AmbisonicField, SoundType};
pub use binaural_ambisonics_decoder::{BinauralAmbisonicsDecoderPlugin, BinauralAmbisonicsDecoder};
pub use higher_order_ambisonics::{HigherOrderAmbisonicsDecoderPlugin, HoaField};
pub use rbe_client_ui_sync::{RbeUiSyncPlugin, RbeUiSync, RbeClientLoopExt};
pub use rbe_ui_feedback::{RbeUiFeedbackPlugin, HarvestFeedbackText};
pub use webxr_bootstrap::PowrushWebXrClient;

pub use example_gpu_material::GpuVisualMaterialsPlugin;
pub use first_session_guidance::{FirstSessionGuidancePlugin, FirstSessionGuidance, credit_harvest, credit_epiphany};
pub use living_practice_loop::{
    LivingPracticeLoopPlugin, LivingPracticeLoop, PracticeSurface, SoftPlayerRealm,
    credit_practice_mercy_harvest,
};
pub use thriving_moments::{ThrivingMomentsPlugin, ThrivingMoments, ThrivingKind, fire_thriving};
pub use rbe_allocate_choice::{RbeAllocateChoicePlugin, RbeAllocateChoice, AllocatePath};
pub use abundance_journey_echo::{AbundanceJourneyEchoPlugin, AbundanceJourneyEcho, JourneyKind};
pub use lattice_flow_share::{LatticeFlowSharePlugin, LatticeFlowShare, LatticeFlowShareEnvelope};
pub use steam_abundance_mirror::{
    SteamAbundanceMirrorPlugin, SteamAbundanceMirror, ABUNDANCE_SUBDIR, REMOTE_JOURNEY, REMOTE_LATTICE,
    preferred_abundance_stage_root,
};
pub use foundation_lattice::{FoundationLatticePlugin, FoundationLattice};
pub use resonance_flavors::{ResonanceFlavorsPlugin, ResonanceState, ResonanceFlavor};
pub use mercy_transporters::{MercyTransportersPlugin, MercyTransporters};

pub struct PowrushClientBundle;

impl PowrushClientBundle {
    pub fn new() -> Self { Self }
}

impl Plugin for PowrushClientBundle {
    fn build(&self, app: &mut App) {
        app.add_plugins(ClientSchedulingPlugin);
        app.add_plugins(AmbisonicsEnginePlugin);
        app.add_plugins(BinauralAmbisonicsDecoderPlugin);
        app.add_plugins(HigherOrderAmbisonicsDecoderPlugin);
        app.add_plugins(RbeUiSyncPlugin);
        app.add_plugins(RbeUiFeedbackPlugin);
        app.add_plugins(GpuVisualMaterialsPlugin);
        app.add_plugins(FirstSessionGuidancePlugin);
        app.add_plugins(ThrivingMomentsPlugin);
        app.add_plugins(LivingPracticeLoopPlugin);
        app.add_plugins(RbeAllocateChoicePlugin);
        app.add_plugins(AbundanceJourneyEchoPlugin);
        app.add_plugins(LatticeFlowSharePlugin);
        app.add_plugins(SteamAbundanceMirrorPlugin);
        app.add_plugins(FoundationLatticePlugin);
        app.add_plugins(ResonanceFlavorsPlugin);
        app.add_plugins(MercyTransportersPlugin);
        app.add_plugins(InputPlugin);
    }
}
