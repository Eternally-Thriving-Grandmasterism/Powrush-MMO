/*!
 * RBE Education UI — Powrush-MMO
 *
 * v18.39 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Structured logging for EpiphanyEvent matching and RBE note display
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
            info!("[RBE Education] Onboarding panel shown | step=RBEPrimer | topic=The Lattice");
        }
        crate::onboarding::OnboardingStep::MercyContribution => {
            education_note.title = "Mercy as Multiplier".to_string();
            education_note.text = "Mercy is the true currency of the eternal Lattice. It does not diminish when given — it multiplies. Every act of presence and care strengthens the whole web.".to_string();
            education_note.visible = true;
            education_note.timer = 11.0;
            info!("[RBE Education] Onboarding panel shown | step=MercyContribution | topic=Mercy as Multiplier");
        }
        crate::onboarding::OnboardingStep::SovereignStart => {
            education_note.title = "Earned Abundance".to_string();
            education_note.text = "Abundance without extraction is not given. It is grown. The Lattice reveals its deeper gifts to those who have shown they can hold them with mercy.".to_string();
            education_note.visible = true;
            education_note.timer = 11.0;
            info!("[RBE Education] Onboarding panel shown | step=SovereignStart | topic=Earned Abundance");
        }
        _ => {}
    }
}

/// Structured logging for EpiphanyEvent reception and RBE note matching
fn show_epiphany_rbe_education(
    mut epiphany_events: EventReader<EpiphanyEvent>,
    mut education_note: ResMut<ActiveRbeEducationNote>,
) {
    for event in epiphany_events.read() {
        // Structured log on every received EpiphanyEvent
        info!(
            target: "rbe_education",
            event_id = %event.scenario_id,
            event_name = %event.name,
            note_length = event.educational_note.len(),
            "[RBE Education] EpiphanyEvent received"
        );

        let note_text = &event.educational_note;
        let mut matched = false;

        if note_text.contains("Lattice") || note_text.contains("sustainable") {
            education_note.title = "Sustainable Harmony / The Lattice".to_string();
            education_note.text = note_text.clone();
            education_note.visible = true;
            education_note.timer = 10.0;
            matched = true;

            info!(
                target: "rbe_education",
                matched_topic = "Sustainable Harmony / The Lattice",
                trigger = if note_text.contains("Lattice") { "Lattice" } else { "sustainable" },
                "[RBE Education] RBE note matched and displayed"
            );
        } else if note_text.contains("Council") || note_text.contains("harmony") {
            education_note.title = "Council Harmony".to_string();
            education_note.text = note_text.clone();
            education_note.visible = true;
            education_note.timer = 11.0;
            matched = true;

            info!(
                target: "rbe_education",
                matched_topic = "Council Harmony",
                "[RBE Education] RBE note matched and displayed"
            );
        } else if note_text.contains("Mercy") || note_text.contains("redemption") {
            education_note.title = "Graceful Redemption".to_string();
            education_note.text = note_text.clone();
            education_note.visible = true;
            education_note.timer = 10.0;
            matched = true;

            info!(
                target: "rbe_education",
                matched_topic = "Graceful Redemption",
                "[RBE Education] RBE note matched and displayed"
            );
        }

        if !matched {
            info!(
                target: "rbe_education",
                event_id = %event.scenario_id,
                note_preview = %note_text.chars().take(80).collect::<String>(),
                "[RBE Education] No RBE education match found for this EpiphanyEvent"
            );
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
        info!(target: "rbe_education", "[RBE Education] Panel auto-hidden (timer expired)");
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
                info!(target: "rbe_education", "[RBE Education] Panel dismissed by player");
            }
        });
}

// End of rbe_education_ui.rs v18.39 — Structured logging for EpiphanyEvent matching added.
// Thunder locked in. Yoi ⚡
