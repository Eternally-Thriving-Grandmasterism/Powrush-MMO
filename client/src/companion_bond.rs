/*!
 * Companion Bond — v22.9.0
 *
 * ARK ride, without the club. Tend raises trust. Take lowers it.
 * When trust is enough and you are not harvesting, E mounts.
 *
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::human_presence::SoftPresence;
use crate::input::PlayerInput;
use crate::living_practice_loop::SoftPlayerRealm;
use crate::mercy_harvest_nodes::NearbyMercyNode;
use crate::soft_play_bindings;
use crate::world_answer::{AnswerKind, WorldAnswer};

const FOLLOW_TRUST: f32 = 0.32;
const MOUNT_TRUST: f32 = 0.55;
const MOUNT_REACH: f32 = 2.15;

#[derive(Resource, Debug)]
pub struct CompanionBond {
    pub trust: f32,
    pub mounted: bool,
    pub nearby: bool,
}

impl Default for CompanionBond {
    fn default() -> Self {
        Self {
            trust: 0.18,
            mounted: false,
            nearby: false,
        }
    }
}

pub struct CompanionBondPlugin;

impl Plugin for CompanionBondPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CompanionBond>()
            .add_systems(
                Update,
                (note_care, follow_or_wait, try_mount).chain(),
            );
    }
}

fn note_care(answer: Res<WorldAnswer>, mut bond: ResMut<CompanionBond>) {
    if !answer.is_changed() || answer.kind == AnswerKind::Idle {
        return;
    }
    match answer.kind {
        AnswerKind::Tend | AnswerKind::Flow => {
            bond.trust = (bond.trust + 0.10).min(1.0);
        }
        AnswerKind::Take => {
            bond.trust = (bond.trust - 0.07).max(0.0);
            if bond.trust < MOUNT_TRUST {
                bond.mounted = false;
            }
        }
        _ => {}
    }
}

fn follow_or_wait(
    presence: Res<SoftPresence>,
    realm: Res<SoftPlayerRealm>,
    mut bond: ResMut<CompanionBond>,
    time: Res<Time>,
    mut deer: Query<(&Name, &mut Transform)>,
) {
    let id = realm.current.unwrap_or(0);
    if !matches!(id, 0 | 2) {
        bond.mounted = false;
        bond.nearby = false;
        return;
    }
    let dt = time.delta_seconds();
    let player = presence.position;
    for (name, mut tf) in &mut deer {
        if name.as_str() != "ResonantDeer" {
            continue;
        }
        let d = tf.translation.distance(player);
        bond.nearby = d <= MOUNT_REACH;
        if bond.mounted {
            let seat = player + Vec3::new(0.0, 0.15, 0.0);
            tf.translation = tf.translation.lerp(seat, (8.0 * dt).min(1.0));
            continue;
        }
        if bond.trust < FOLLOW_TRUST {
            continue;
        }
        let mut want = player + Vec3::new(1.1, 0.0, 1.0);
        want.y = 0.55;
        if d > 2.4 {
            tf.translation = tf.translation.lerp(want, (1.8 * dt).min(1.0));
        }
    }
}

fn try_mount(
    keyboard: Res<ButtonInput<KeyCode>>,
    input: Res<PlayerInput>,
    nearby_node: Res<NearbyMercyNode>,
    mut bond: ResMut<CompanionBond>,
) {
    if nearby_node.in_range {
        return;
    }
    let press = keyboard.just_pressed(soft_play_bindings::INTERACT) || input.interact;
    if !press {
        return;
    }
    if bond.mounted {
        bond.mounted = false;
        info!(target: "powrush::companion", "feet on the ground");
        return;
    }
    if bond.trust >= MOUNT_TRUST && bond.nearby {
        bond.mounted = true;
        info!(target: "powrush::companion", trust = bond.trust, "companion offered a ride");
    }
}
