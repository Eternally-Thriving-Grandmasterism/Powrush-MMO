/*!
 * Resonance Flavors — soft asymmetric mastery coloring (v21.96.0)
 *
 * Cycle: **G** (Gestalt / resonance — ergonomic left-hand)
 *
 * PATSAGi + TOLC 8 · Contact: info@Rathor.ai · Yoi ⚡
 */

use bevy::prelude::*;

use crate::abundance_journey_echo::{AbundanceJourneyEcho, JourneyKind};
use crate::living_practice_loop::LivingPracticeText;
use crate::hour_sacred::HourSacred;
use crate::soft_play_bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResonanceFlavor {
    #[default]
    BalancedFlow,
    DurableSteward,
    AdaptiveHorizon,
}

impl ResonanceFlavor {
    pub fn title(self) -> &'static str {
        match self {
            ResonanceFlavor::BalancedFlow => "Balanced Flow",
            ResonanceFlavor::DurableSteward => "Durable Steward",
            ResonanceFlavor::AdaptiveHorizon => "Adaptive Horizon",
        }
    }

    pub fn line(self) -> &'static str {
        match self {
            ResonanceFlavor::BalancedFlow => {
                "Share-leaning care · harmonic allocation · others may thrive now"
            }
            ResonanceFlavor::DurableSteward => {
                "Foundation-leaning reserve · enduring bases · future thriving held"
            }
            ResonanceFlavor::AdaptiveHorizon => {
                "Exploratory ingenuity · sparse yields · choose under uncertainty"
            }
        }
    }

    pub fn accent(self) -> Color {
        match self {
            ResonanceFlavor::BalancedFlow => Color::srgb(0.72, 0.98, 0.86),
            ResonanceFlavor::DurableSteward => Color::srgb(0.78, 0.88, 1.0),
            ResonanceFlavor::AdaptiveHorizon => Color::srgb(1.0, 0.92, 0.70),
        }
    }

    pub fn next(self) -> Self {
        match self {
            ResonanceFlavor::BalancedFlow => ResonanceFlavor::DurableSteward,
            ResonanceFlavor::DurableSteward => ResonanceFlavor::AdaptiveHorizon,
            ResonanceFlavor::AdaptiveHorizon => ResonanceFlavor::BalancedFlow,
        }
    }

    pub fn soft_allocate_hint(self) -> &'static str {
        match self {
            ResonanceFlavor::BalancedFlow => "Resonance leans Flow outward (still fully free)",
            ResonanceFlavor::DurableSteward => "Resonance leans Steward reserve (still fully free)",
            ResonanceFlavor::AdaptiveHorizon => "Resonance explores both paths under uncertainty",
        }
    }
}

#[derive(Resource, Debug)]
pub struct ResonanceState {
    pub current: ResonanceFlavor,
    pub shifts: u32,
}

impl Default for ResonanceState {
    fn default() -> Self {
        Self {
            current: ResonanceFlavor::BalancedFlow,
            shifts: 0,
        }
    }
}

pub struct ResonanceFlavorsPlugin;

impl Plugin for ResonanceFlavorsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ResonanceState>().add_systems(
            Update,
            (cycle_resonance, tint_practice_strip_by_resonance),
        );
    }
}

fn cycle_resonance(
    keyboard: Res<ButtonInput<KeyCode>>,
    hour: Res<HourSacred>,
    mut state: ResMut<ResonanceState>,
    mut echo: ResMut<AbundanceJourneyEcho>,
) {
    if !hour.charter_skin_live() {
        return;
    }
    if !keyboard.just_pressed(soft_play_bindings::RESONANCE_CYCLE) {
        return;
    }
    state.current = state.current.next();
    state.shifts = state.shifts.saturating_add(1);
    let note = format!(
        "Resonance shifted · {} — {}",
        state.current.title(),
        state.current.line()
    );
    echo.push(JourneyKind::Note, note.clone());
    info!(
        target: "powrush::resonance",
        flavor = state.current.title(),
        shifts = state.shifts,
        "{note}"
    );
}

fn tint_practice_strip_by_resonance(
    state: Res<ResonanceState>,
    mut query: Query<&mut Text, With<LivingPracticeText>>,
) {
    if !state.is_changed() {
        return;
    }
    let accent = state.current.accent();
    for mut text in &mut query {
        if let Some(section) = text.sections.get_mut(0) {
            if section.style.color != Color::srgb(1.0, 0.95, 0.55) {
                section.style.color = accent;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycles_all_three() {
        let a = ResonanceFlavor::BalancedFlow;
        assert_eq!(a.next(), ResonanceFlavor::DurableSteward);
        assert_eq!(a.next().next(), ResonanceFlavor::AdaptiveHorizon);
        assert_eq!(a.next().next().next(), ResonanceFlavor::BalancedFlow);
    }
}
