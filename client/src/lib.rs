//! client/src/lib.rs
//! Powrush-MMO Client Crate Root — Public API and module declarations
//! AG-SML v1.0 | TOLC 8 Mercy Gates | v21.90 end-user experience perfection
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

/// Soft first-session objective strip for perfect human first 5–15 minutes.
pub mod first_session_guidance;

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
        app.add_plugins(InputPlugin);
    }
}
