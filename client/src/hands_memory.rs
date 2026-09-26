/*!
 * Hands Memory — v22.12.0
 *
 * Tend the same climate and the hands get surer. Not XP.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::harvest_feel::SoftRbePool;
use crate::living_practice_loop::SoftPlayerRealm;
use crate::world_answer::{AnswerKind, WorldAnswer};

/// Tend count where the hands are as sure as this hour allows.
const HANDS_CAP: u32 = 8;

#[derive(Resource, Debug)]
pub struct HandsMemory {
    pub tends: [u32; 5],
}

impl Default for HandsMemory {
    fn default() -> Self {
        Self { tends: [0; 5] }
    }
}

impl HandsMemory {
    pub fn idx(realm: Option<u8>) -> usize {
        realm.unwrap_or(0).min(4) as usize
    }

    pub fn mul(&self, realm: Option<u8>) -> f32 {
        let n = self.tends[Self::idx(realm)].min(HANDS_CAP);
        1.0 + n as f32 * 0.05
    }

    pub fn note_tend(&mut self, realm: Option<u8>) {
        let i = Self::idx(realm);
        self.tends[i] = self.tends[i].saturating_add(1);
    }
}

pub struct HandsMemoryPlugin;

impl Plugin for HandsMemoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HandsMemory>()
            .add_systems(
                Update,
                (note_skilled_tend, apply_skill_bonus, note_hands_remember).chain(),
            );
    }
}

fn note_skilled_tend(
    answer: Res<WorldAnswer>,
    realm: Res<SoftPlayerRealm>,
    mut hands: ResMut<HandsMemory>,
) {
    if answer.is_changed() && answer.kind == AnswerKind::Tend {
        hands.note_tend(realm.current);
    }
}

fn apply_skill_bonus(
    answer: Res<WorldAnswer>,
    realm: Res<SoftPlayerRealm>,
    hands: Res<HandsMemory>,
    mut pool: ResMut<SoftRbePool>,
) {
    if !answer.is_changed() || answer.kind != AnswerKind::Tend {
        return;
    }
    let extra = (hands.mul(realm.current) - 1.0) * 0.15;
    if extra > 0.0 {
        pool.harmony += extra;
        pool.joy += extra * 0.4;
    }
}

/// `{place} · the hands remember` — one line for the existing journey feed.
fn hands_remember_line(place: &str) -> String {
    format!("{place} · the hands remember")
}

/// True only when this climate's tend count has landed on the cap.
fn hands_remember_at_cap(count: u32) -> bool {
    count == HANDS_CAP
}

/// After a Tend, if this climate's hands just reached the cap, name the Place
/// once in the Abundance Journey feed. No travel or no echo means no line.
fn note_hands_remember(
    answer: Res<WorldAnswer>,
    realm: Res<SoftPlayerRealm>,
    hands: Res<HandsMemory>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut echo: Option<ResMut<crate::abundance_journey_echo::AbundanceJourneyEcho>>,
) {
    if !answer.is_changed() || answer.kind != AnswerKind::Tend {
        return;
    }
    let count = hands.tends[HandsMemory::idx(realm.current)];
    if !hands_remember_at_cap(count) {
        return;
    }
    let Some(place) = travel.as_ref().map(|state| state.chip_name()) else {
        return;
    };
    let Some(echo) = echo.as_mut() else {
        return;
    };
    let text = hands_remember_line(place);
    if echo.lines.iter().any(|existing| existing.text == text) {
        return;
    }
    echo.push(crate::abundance_journey_echo::JourneyKind::Note, text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hands_remember_line_names_the_place() {
        assert_eq!(
            hands_remember_line("Heartwood"),
            "Heartwood · the hands remember"
        );
    }

    #[test]
    fn hands_remember_predicate_is_true_only_at_the_cap() {
        assert!(!hands_remember_at_cap(7));
        assert!(hands_remember_at_cap(8));
        assert!(!hands_remember_at_cap(9));
        assert_eq!(HANDS_CAP, 8);
    }

    #[test]
    fn hands_remember_copy_skips_market_words_and_peace_well_words() {
        let line = hands_remember_line("Heartwood");
        let low = line.to_lowercase();
        for banned in ["threshold", "gold", "market", "xp", "level"] {
            assert!(!low.contains(banned), "{banned}");
        }
        // Peace well slab words (`skirmish_well_word`): Idle, Glowing, Tended, Resting, Stressed.
        for well in ["Idle", "Glowing", "Tended", "Resting", "Stressed"] {
            assert!(!low.contains(&well.to_lowercase()), "{well}");
        }
    }
}
