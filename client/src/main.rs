use crate::council_bloom_feedback::CouncilBloomFeedbackPlugin;

fn main() {
    App::new()
        ...
        .add_plugins(FactionReputationUIPlugin)

        // === Council Bloom Rich Feedback ===
        .add_plugins(CouncilBloomFeedbackPlugin)

        // === Live Egui Settings Panel ===
        .add_plugins(EguiSettingsPanelPlugin)
        ...
}