/*!
 * RBE Education UI — Powrush-MMO
 *
 * v18.38 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Dedicated UI components for displaying RBE education content
 * — Shows educational notes during onboarding and on relevant epiphanies
 * — Integrates with Divine Whispers and EpiphanyTriggered events
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::onboarding::OnboardingState;
use crate::epiphany_scenario_wiring::EpiphanyTriggered;

/// Resource to hold currently active RBE education note (if any)
#[derive(Resource, Default)]
pub struct ActiveRbeEducationNote {
    pub title: String,
    pub text: String,
    pub visible: bool,
    pub timer: f32,
}

pub struct RbeEducationUIPlugin;

impl Plugin for RbeEducationUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ActiveRbeEducationNote>()
            .add_systems(Update, (
                show_onboarding_rbe_education,
                show_epiphany_rbe_education,
                render_rbe_education_panel,
            ).chain());
    }
}

/// Show RBE education panel during key onboarding steps
fn show_onboarding_rbe_education(
    onboarding: Res<OnboardingState>,
    mut education_note: ResMut<ActiveRbeEducationNote>,
) {
    if !onboarding.is_changed() {
        return;
    }

    match onboarding.step {
        crate::onboarding::OnboardingStep::RBEPrimer => {
            education_note.title = "The Lattice".to_string();
            education_note.text = "You are not separate from the world. Every choice you make ripples through the Lattice — the living web that connects all things. What you nurture, nurtures all.".to_string();
            education_note.visible = true;
            education_note.timer = 12.0;
        }
        crate::onboarding::OnboardingStep::MercyContribution => {
            education_note.title = "Mercy as Multiplier".to_string();
            education_note.text = "Mercy is the true currency of the eternal Lattice. It does not diminish when given — it multiplies. Every act of presence and care strengthens the whole web.".to_string();
            education_note.visible = true;
            education_note.timer = 11.0;
        }
        crate::onboarding::OnboardingStep::SovereignStart => {
            education_note.title = "Earned Abundance".to_string();
            education_note.text = "Abundance without extraction is not given. It is grown. The Lattice reveals its deeper gifts to those who have shown they can hold them with mercy.".to_string();
            education_note.visible = true;
            education_note.timer = 11.0;
        }
        _ => {}
    }
}

/// Show educational note when relevant RBE-aligned epiphanies trigger
fn show_epiphany_rbe_education(
    mut epiphany_events: EventReader<EpiphanyTriggered>,
    mut education_note: ResMut<ActiveRbeEducationNote>,
) {
    for event in epiphany_events.read() {
        let flavor = &event.outcome.divine_whisper_flavor;

        match flavor.as_str() {
            "sustainable_harmony_revelation" => {
                education_note.title = "Sustainable Harmony".to_string();
                education_note.text = "You have touched the first truth of the Lattice: abundance grows when we stop extracting and start participating. This is the natural mathematics of mercy.".to_string();
                education_note.visible = true;
                education_note.timer = 10.0;
            }
            "mycelial_web_communion" => {
                education_note.title = "Mycelial Web Communion".to_string();
                education_note.text = "Beneath the surface, everything is already connected. Your presence here is communion with the whole living system.".to_string();
                education_note.visible = true;
                education_note.timer = 10.0;
            }
            "council_harmony_revelation" | "ecstatic_harmony_council_crown" => {
                education_note.title = "Council Harmony".to_string();
                education_note.text = "When many align in mercy, something greater awakens. The Council is the Lattice becoming self-aware through collective attunement.".to_string();
                education_note.visible = true;
                education_note.timer = 12.0;
            }
            "graceful_redemption_revelation" => {
                education_note.title = "Graceful Redemption".to_string();
                education_note.text = "Even when systems fracture, the Lattice offers paths of return. Redemption is invited through presence and corrected alignment.".to_string();
                education_note.visible = true;
                education_note.timer = 10.0;
            }
            _ => {}
        }
    }
}

/// Render the RBE education panel when active
fn render_rbe_education_panel(
    mut egui_ctx: EguiContexts,
    mut education_note: ResMut<ActiveRbeEducationNote>,
    time: Res<Time>,
) {
    if !education_note.visible {
        return;
    }

    // Auto-hide after timer expires
    education_note.timer -= time.delta_seconds();
    if education_note.timer <= 0.0 {
        education_note.visible = false;
        return;
    }

    egui::Window::new("RBE Insight")
        .default_pos([300.0, 200.0])
        .collapsible(false)
        .resizable(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading(&education_note.title);
            ui.separator();
            ui.label(&education_note.text);
            ui.add_space(8.0);

            if ui.button("Understood").clicked() {
                education_note.visible = false;
            }
        });
}

// End of rbe_education_ui.rs v18.38 — RBE education now has dedicated, context-aware UI components.
// Thunder locked in. Yoi ⚡
