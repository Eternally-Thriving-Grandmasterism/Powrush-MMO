/*!
 * World Answer — v22.2.0
 *
 * Allocate spends a satchel stack. Sky, fog, ambient, node pulse answer.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::pbr::FogSettings;
use bevy::prelude::*;

use crate::harvest_feel::SoftRbePool;
use crate::mercy_harvest_nodes::MercyHarvestNode;
use crate::rbe_allocate_choice::{AllocatePath, RbeAllocateChoice};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnswerKind {
    #[default]
    Idle,
    Take,
    Tend,
    Flow,
    Reserve,
}

#[derive(Resource, Debug)]
pub struct WorldAnswer {
    pub kind: AnswerKind,
    pub until: f64,
    pub warmth: f32,
    pub last_line: String,
}

impl Default for WorldAnswer {
    fn default() -> Self {
        Self {
            kind: AnswerKind::Idle,
            until: 0.0,
            warmth: 0.0,
            last_line: String::new(),
        }
    }
}

impl WorldAnswer {
    pub fn fire(&mut self, kind: AnswerKind, now: f64, line: impl Into<String>) {
        self.kind = kind;
        self.until = now
            + match kind {
                AnswerKind::Flow | AnswerKind::Reserve => 3.4,
                AnswerKind::Tend => 2.2,
                AnswerKind::Take => 1.4,
                AnswerKind::Idle => 0.0,
            };
        self.warmth = match kind {
            AnswerKind::Flow => 0.85,
            AnswerKind::Reserve => 0.45,
            AnswerKind::Tend => 0.55,
            AnswerKind::Take => 0.22,
            AnswerKind::Idle => 0.0,
        };
        self.last_line = line.into();
    }

    pub fn live(&self, now: f64) -> bool {
        now < self.until && self.kind != AnswerKind::Idle
    }
}

pub fn fire_world_answer(answer: &mut WorldAnswer, kind: AnswerKind, now: f64, line: impl Into<String>) {
    answer.fire(kind, now, line);
}

/// R+2 confirm. Never "Reserve −0.0 harmony" — that was the named AMBER.
pub fn reserve_world_line(banked: f32) -> String {
    if banked > 0.05 {
        format!("Reserve {banked:.1} · repair-rights held")
    } else {
        "Reserve not banked".to_string()
    }
}

pub struct WorldAnswerPlugin;

impl Plugin for WorldAnswerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldAnswer>()
            .add_systems(
                Update,
                ((notice_allocate, note_place_answers).chain(), paint_world_answer),
            );
    }
}

fn notice_allocate(
    allocate: Res<RbeAllocateChoice>,
    mut pool: ResMut<SoftRbePool>,
    mut answer: ResMut<WorldAnswer>,
    time: Res<Time>,
    mut last: Local<u32>,
) {
    if allocate.choices_made == *last {
        return;
    }
    *last = allocate.choices_made;
    let now = time.elapsed_seconds_f64();
    let Some(path) = allocate.last_choice else {
        return;
    };
    match path {
        AllocatePath::FlowOutward => {
            let spent = pool.spend_allocate(path, 1.0);
            fire_world_answer(
                &mut answer,
                AnswerKind::Flow,
                now,
                format!("Flow −{spent:.1} vitality — the climate brightens"),
            );
            info!(target: "powrush::answer", ?path, spent, "allocate spent into the climate");
        }
        AllocatePath::StewardReserve => {
            let _ = pool.spend_allocate(path, 1.0);
            let line = reserve_world_line(allocate.reserve_total);
            fire_world_answer(&mut answer, AnswerKind::Reserve, now, line.clone());
            info!(target: "powrush::answer", ?path, line, "allocate reserve banked");
        }
    }
}

/// `{place} · the climate brightens` on Flow, `{place} · repair-rights held` on Reserve.
fn world_answer_place_line(kind: AnswerKind, place: &str) -> Option<String> {
    match kind {
        AnswerKind::Flow => Some(format!("{place} · the climate brightens")),
        AnswerKind::Reserve => Some(format!("{place} · repair-rights held")),
        AnswerKind::Tend | AnswerKind::Take | AnswerKind::Idle => None,
    }
}

/// After allocate, name the Place once in the Abundance Journey feed.
/// Tend, Take, Idle, and an unbanked reserve push nothing.
/// No travel or no echo means no line.
fn note_place_answers(
    answer: Res<WorldAnswer>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut echo: Option<ResMut<crate::abundance_journey_echo::AbundanceJourneyEcho>>,
) {
    if !answer.is_changed() {
        return;
    }
    if answer.kind != AnswerKind::Flow && answer.kind != AnswerKind::Reserve {
        return;
    }
    if answer.kind == AnswerKind::Reserve && answer.last_line == reserve_world_line(0.0) {
        return;
    }
    let Some(place) = travel.as_ref().map(|state| state.chip_name()) else {
        return;
    };
    let Some(echo) = echo.as_mut() else {
        return;
    };
    let Some(text) = world_answer_place_line(answer.kind, place) else {
        return;
    };
    if echo.lines.iter().any(|existing| existing.text == text) {
        return;
    }
    echo.push(crate::abundance_journey_echo::JourneyKind::Note, text);
}

fn paint_world_answer(
    answer: Res<WorldAnswer>,
    time: Res<Time>,
    mut ambient: ResMut<AmbientLight>,
    mut fogs: Query<&mut FogSettings>,
    mut nodes: Query<&mut MercyHarvestNode>,
    mut last_kind: Local<AnswerKind>,
) {
    let now = time.elapsed_seconds_f64();
    let live = answer.live(now);
    let fade = if live {
        ((answer.until - now) / 3.4).clamp(0.0, 1.0) as f32
    } else {
        0.0
    };
    let extra = answer.warmth * fade;
    ambient.brightness = 280.0 + extra * 220.0;

    for mut fog in &mut fogs {
        if live && matches!(answer.kind, AnswerKind::Flow) {
            fog.falloff = bevy::pbr::FogFalloff::Linear {
                start: 14.0,
                end: 52.0,
            };
        } else if live && matches!(answer.kind, AnswerKind::Reserve) {
            fog.falloff = bevy::pbr::FogFalloff::Linear {
                start: 8.0,
                end: 34.0,
            };
        }
    }

    if live && *last_kind != answer.kind {
        for mut node in &mut nodes {
            node.pulse = (node.pulse + 0.35).min(1.0);
            if matches!(answer.kind, AnswerKind::Tend | AnswerKind::Flow) {
                node.vitality = (node.vitality + 0.04).min(1.0);
            }
        }
    }
    *last_kind = if live {
        answer.kind
    } else {
        AnswerKind::Idle
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserve_confirm_is_a_bank_not_zero_spend() {
        let banked = reserve_world_line(1.0);
        assert!(banked.contains("Reserve 1.0"));
        assert!(banked.contains("repair-rights"));
        assert!(!banked.contains("−"));
        assert!(!banked.contains("-0.0"));
        assert!(!banked.contains("0.0 harmony"));
        assert_eq!(reserve_world_line(0.0), "Reserve not banked");
    }

    #[test]
    fn world_answer_place_line_names_the_place() {
        assert_eq!(
            world_answer_place_line(AnswerKind::Flow, "Heartwood"),
            Some("Heartwood · the climate brightens".to_string())
        );
        assert_eq!(
            world_answer_place_line(AnswerKind::Reserve, "Heartwood"),
            Some("Heartwood · repair-rights held".to_string())
        );
    }

    #[test]
    fn world_answer_place_line_is_none_for_tend_take_idle() {
        assert_eq!(world_answer_place_line(AnswerKind::Tend, "Heartwood"), None);
        assert_eq!(world_answer_place_line(AnswerKind::Take, "Heartwood"), None);
        assert_eq!(world_answer_place_line(AnswerKind::Idle, "Heartwood"), None);
    }

    #[test]
    fn world_answer_place_copy_skips_digits_market_words_and_peace_well_words() {
        for line in [
            world_answer_place_line(AnswerKind::Flow, "Heartwood").expect("flow"),
            world_answer_place_line(AnswerKind::Reserve, "Heartwood").expect("reserve"),
        ] {
            assert!(
                !line.chars().any(|c| c.is_ascii_digit()),
                "ascii digit in {line}"
            );
            let low = line.to_lowercase();
            for banned in ["threshold", "gold", "market", "xp", "level"] {
                assert!(!low.contains(banned), "{banned} in {line}");
            }
            // Peace well slab words (`skirmish_well_word`): Idle, Glowing, Tended, Resting, Stressed.
            for well in ["Idle", "Glowing", "Tended", "Resting", "Stressed"] {
                assert!(!low.contains(&well.to_lowercase()), "{well} in {line}");
            }
        }
    }
}
