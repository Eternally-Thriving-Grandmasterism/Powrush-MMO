use crate::council_bloom_feedback::CouncilBloomFeedbackPlugin;

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
        // ...

        // === Council Bloom Rich Feedback (new) ===
        .add_plugins(CouncilBloomFeedbackPlugin)

        // === Cinematic Audio ===
        .add_plugins(AudioPlugin)
        .add_plugins(FundspAudioPlugin)
        .add_plugins(SpatialAudioPlugin)

        // ... rest of app ...
        .run();
}