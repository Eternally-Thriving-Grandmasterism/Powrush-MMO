use crate::council_bloom_feedback::CouncilBloomFeedbackPlugin;
use crate::GpuVisualMaterialsPlugin;
use bevy::log::LogPlugin;

// ... other imports ...

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin { ... })
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    // Enable debug spans + logs from our new RenderState visual materials
                    // while keeping noisy crates (wgpu, bevy_ecs, etc.) quieter.
                    // Override at runtime with: RUST_LOG="debug" or RUST_LOG="powrush_mmo=trace"
                    filter: "wgpu=error,bevy_ecs=warn,bevy=info,powrush_mmo=debug,example_gpu_material=debug".to_string(),
                    ..default()
                }),
        )

        // === Core Infrastructure ===
        .add_plugins(NetworkingPlugin)
        .add_plugins(ReplicationPlugin)
        // ... other plugins ...

        // === Visuals & Experience ===
        .add_plugins(ParticlePlugin)
        .add_plugins(UiPlugin)
        .add_plugins(DivineWhispersPlugin)

        // GPU Visual Materials — RenderState-driven effects
        // (EnergyBurst, ValenceHalo, MycelialWebGlow, ResourceNodeGlow)
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