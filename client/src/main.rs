use crate::council_bloom_feedback::CouncilBloomFeedbackPlugin;
use crate::GpuVisualMaterialsPlugin;

// ... other imports ...

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin { ... }))

        // === Core Infrastructure ===
        .add_plugins(NetworkingPlugin)
        .add_plugins(ReplicationPlugin)
        // ... other plugins ...

        // === Visuals & Experience ===
        .add_plugins(ParticlePlugin)
        .add_plugins(UiPlugin)
        .add_plugins(DivineWhispersPlugin)

        // GPU Visual Materials — RenderState-driven pipeline specialization
        // for EnergyBurst, ValenceHalo, MycelialWebGlow, ResourceNodeGlow effects
        .add_plugins(GpuVisualMaterialsPlugin)

        // === Council Bloom Rich Feedback (new) ===
        .add_plugins(CouncilBloomFeedbackPlugin)

        // === Cinematic Audio ===
        .add_plugins(AudioPlugin)
        .add_plugins(FundspAudioPlugin)
        .add_plugins(SpatialAudioPlugin)

        // ... rest of app ...
        .run();
}