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
        let n = self.tends[Self::idx(realm)].min(8);
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
            .add_systems(Update, (note_skilled_tend, apply_skill_bonus).chain());
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
