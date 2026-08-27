/*!
 * Player Lineage — v23.0.0
 *
 * Human, Cydruid, Quellorian, Draek, Ambrosian.
 * C cycles. Felt in tend / take / freshness / body tint.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::harvest_feel::SoftRbePool;
use crate::human_presence::HumanPresence;
use crate::living_freshness::LivingFreshness;
use crate::world_answer::{AnswerKind, WorldAnswer};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lineage {
    Human,
    Cydruid,
    Quellorian,
    Draek,
    Ambrosian,
}

impl Lineage {
    pub fn next(self) -> Self {
        match self {
            Self::Human => Self::Cydruid,
            Self::Cydruid => Self::Quellorian,
            Self::Quellorian => Self::Draek,
            Self::Draek => Self::Ambrosian,
            Self::Ambrosian => Self::Human,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Human => "Human",
            Self::Cydruid => "Cydruid",
            Self::Quellorian => "Quellorian",
            Self::Draek => "Draek",
            Self::Ambrosian => "Ambrosian",
        }
    }

    pub fn tint(self) -> Color {
        match self {
            Self::Human => Color::srgb(0.20, 0.26, 0.24),
            Self::Cydruid => Color::srgb(0.16, 0.42, 0.22),
            Self::Quellorian => Color::srgb(0.55, 0.78, 0.92),
            Self::Draek => Color::srgb(0.42, 0.16, 0.28),
            Self::Ambrosian => Color::srgb(0.78, 0.72, 0.88),
        }
    }
}

#[derive(Resource, Debug)]
pub struct PlayerLineage {
    pub current: Lineage,
}

impl Default for PlayerLineage {
    fn default() -> Self {
        Self {
            current: Lineage::Human,
        }
    }
}

pub struct PlayerLineagePlugin;

impl Plugin for PlayerLineagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerLineage>()
            .add_systems(Update, (cycle_lineage, paint_body, lineage_care));
    }
}

fn cycle_lineage(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut lineage: ResMut<PlayerLineage>,
    mut inv_line: Option<ResMut<crate::human_inventory::HumanInventory>>,
    time: Res<Time>,
) {
    if !keyboard.just_pressed(KeyCode::KeyC) {
        return;
    }
    lineage.current = lineage.current.next();
    if let Some(mut inv) = inv_line {
        inv.pickup_until = time.elapsed_seconds_f64() + 2.2;
        inv.pickup_line = format!("lineage · {}", lineage.current.name());
    }
    info!(target: "powrush::lineage", race = lineage.current.name(), "C cycled");
}

fn paint_body(
    lineage: Res<PlayerLineage>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q: Query<&Handle<StandardMaterial>, With<HumanPresence>>,
) {
    if !lineage.is_changed() {
        return;
    }
    let tint = lineage.current.tint();
    for handle in &q {
        if let Some(mat) = materials.get_mut(handle) {
            mat.base_color = tint;
        }
    }
}

fn lineage_care(
    lineage: Res<PlayerLineage>,
    answer: Res<WorldAnswer>,
    mut pool: ResMut<SoftRbePool>,
    mut fresh: Option<ResMut<LivingFreshness>>,
) {
    if !answer.is_changed() || answer.kind == AnswerKind::Idle {
        return;
    }
    match (lineage.current, answer.kind) {
        (Lineage::Cydruid, AnswerKind::Tend) => {
            pool.harmony += 0.12;
            pool.joy += 0.06;
        }
        (Lineage::Draek, AnswerKind::Take) => {
            pool.vitality += 0.18;
        }
        (Lineage::Draek, AnswerKind::Tend) => {
            pool.harmony += 0.16;
            pool.joy += 0.10;
        }
        (Lineage::Quellorian, AnswerKind::Tend | AnswerKind::Flow) => {
            pool.harmony += 0.08;
        }
        (Lineage::Ambrosian, _) => {
            if let Some(ref mut f) = fresh {
                f.age = (f.age * 0.72).max(0.0);
            }
        }
        (Lineage::Human, _) => {
            pool.joy += 0.03;
        }
        _ => {}
    }
}
