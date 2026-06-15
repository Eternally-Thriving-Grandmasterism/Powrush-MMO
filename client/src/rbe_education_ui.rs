/*!
 * RBE Education UI — Powrush-MMO
 *
 * v18.39 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Dedicated UI components for displaying RBE education content
 * — Added debug logging for EpiphanyEvent flow verification
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::onboarding::OnboardingState;
use crate::epiphany_scenario_wiring::EpiphanyEvent;

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
            info!("[RBE Education] Showing onboarding panel: The Lattice (RBEPrimer)");
        }
        crate::onboarding::OnboardingStep::MercyContribution => {
            education_note.title = "Mercy as Multiplier".to_string();
            education_note.text = "Mercy is the true currency of the eternal Lattice. It does not diminish when given — it multiplies. Every act of presence and care strengthens the whole web.".to_string();
            education_note.visible = true;
            education_note.timer = 11.0;
            info!("[RBE Education] Showing onboarding panel: Mercy as Multiplier");
        }
        crate::onboarding::OnboardingStep::SovereignStart => {
            education_note.title = "Earned Abundance".to_string();
            education_note.text = "Abundance without extraction is not given. It is grown. The Lattice reveals its deeper gifts to those who have shown they can hold them with mercy.".to_string();
            education_note.visible = true;
            education_note.timer = 11.0;
            info!("[RBE Education] Showing onboarding panel: Earned Abundance (SovereignStart)");
        }
        _ => {}
    }
}

/// Show educational note when relevant RBE-aligned epiphanies trigger
/// Includes debug logging for verification
fn show_epiphany_rbe_education(
    mut epiphany_events: EventReader<EpiphanyEvent>,
    mut education_note: ResMut<ActiveRbeEducationNote>,
) {
    for event in epiphany_events.read() {
        info!(
            "[RBE Education] Received EpiphanyEvent | id={} | name={} | note_preview={:.60}...",
            event.scenario_id,
            event.name,
            event.educational_note.chars().take(60).collect::<String>()
        );

        let note_text = &event.educational_note;

        if note_text.contains("Lattice") || note_text.contains("sustainable") {
            education_note.title = "Sustainable Harmony / The Lattice".to_string();
            education_note.text = note_text.clone();
            education_note.visible = true;
            education_note.timer = 10.0;
            info!("[RBE Education] Displaying RBE note: Sustainable Harmony / The Lattice");
        } else if note_text.contains("Council") || note_text.contains("harmony") {
            education_note.title = "Council Harmony".to_string();
            education_note.text = note_text.clone();
            education_note.visible = true;
            education_note.timer = 11.0;
            info!("[RBE Education] Displaying RBE note: Council Harmony");
        } else if note_text.contains("Mercy") || note_text.contains("redemption") {
            education_note.title = "Graceful Redemption".to_string();
            education_note.text = note_text.clone();
            education_note.visible = true;
            education_note.timer = 10.0;
            info!("[RBE Education] Displaying RBE note: Graceful Redemption");
        } else {
            info!("[RBE Education] EpiphanyEvent received but no matching RBE education note.");
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

    education_note.timer -= time.delta_seconds();
    if education_note.timer <= 0.0 {
        education_note.visible = false;
        info!("[RBE Education] RBE education panel auto-hidden (timer expired)");
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
                info!("[RBE Education] RBE education panel dismissed by player");
            }
        });
}

// End of rbe_education_ui.rs v18.39 — Debug logging added for EpiphanyEvent flow verification.
// Thunder locked in. Yoi ⚡
