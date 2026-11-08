/*!
 * Companion Bond — v22.9.0
 *
 * ARK ride, without the club. Tend raises trust. Take lowers it.
 * When trust is enough and you are not harvesting, E mounts.
 *
 * CARD FLESH-COMPANION-LINE — the existing mount and feet lines may name
 * the Place (`HexTravelState::chip_name` / `PlaceId::chip_name`). Absent
 * travel keeps each bare line. No pet HUD. No XP. No new widget.
 * Peak memory, cited: walked · tended · week was the bill · yard remembered.
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

/// Existing dismount line. Absent travel keeps this byte for byte.
const FEET_LINE: &str = "feet on the ground";
/// Existing mount line. Absent travel keeps this byte for byte.
const RIDE_LINE: &str = "companion offered a ride";

/// CARD FLESH-COMPANION-LINE — `{place} · {bare}` when a chip is present.
/// `None` returns `bare` unchanged. One string. No second widget.
fn companion_bond_line(bare: &str, place: Option<&str>) -> String {
    match place {
        Some(place) => format!("{place} · {bare}"),
        None => bare.to_string(),
    }
}

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
        app.init_resource::<CompanionBond>().add_systems(
            Update,
            (
                note_care,
                follow_or_wait,
                try_mount.after(crate::wood_expedition::claim_wood_press),
            )
                .chain(),
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
    let dt = time.delta_secs();
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
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    claim: Option<Res<crate::wood_expedition::WoodClaim>>,
    mut bond: ResMut<CompanionBond>,
) {
    if nearby_node.in_range {
        return;
    }
    // Held wood owns this update, success or late reject. Absent claim stays legacy.
    if claim.is_some_and(|held| held.blocks()) {
        return;
    }
    let press = keyboard.just_pressed(soft_play_bindings::INTERACT) || input.interact;
    if !press {
        return;
    }
    let place = travel.as_ref().map(|state| state.chip_name());
    if bond.mounted {
        bond.mounted = false;
        let line = companion_bond_line(FEET_LINE, place);
        info!(target: "powrush::companion", "{line}");
        return;
    }
    if bond.trust >= MOUNT_TRUST && bond.nearby {
        bond.mounted = true;
        let line = companion_bond_line(RIDE_LINE, place);
        info!(target: "powrush::companion", trust = bond.trust, "{line}");
    }
}

#[cfg(test)]
pub(crate) fn same_frame_mount_id() -> std::any::TypeId {
    use bevy::ecs::system::IntoSystem;
    IntoSystem::system_type_id(&try_mount)
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::hex_travel::PlaceId;

    /// CARD FLESH-COMPANION-LINE — Place prefixes the bond line; no travel keeps it exact.
    #[test]
    fn flesh_companion_line_names_chip_or_keeps_bare_line() {
        assert_eq!(companion_bond_line(RIDE_LINE, None), RIDE_LINE);
        assert_eq!(companion_bond_line(FEET_LINE, None), FEET_LINE);
        assert_eq!(companion_bond_line(RIDE_LINE, None), "companion offered a ride");
        assert_eq!(companion_bond_line(FEET_LINE, None), "feet on the ground");

        let cases = [
            (PlaceId::Sanctuary, "Sanctuary Prime"),
            (PlaceId::Heartwood, "Heartwood"),
            (PlaceId::Depths, "Depths"),
        ];
        for (id, name) in cases {
            assert_eq!(id.chip_name(), name);
            let travel = crate::hex_travel::HexTravelState { current: id };
            assert_eq!(travel.chip_name(), id.chip_name());
            assert_eq!(
                companion_bond_line(RIDE_LINE, Some(travel.chip_name())),
                format!("{name} · companion offered a ride")
            );
            assert_eq!(
                companion_bond_line(FEET_LINE, Some(id.chip_name())),
                format!("{name} · feet on the ground")
            );
        }

        for sample in [
            companion_bond_line(RIDE_LINE, Some(PlaceId::Heartwood.chip_name())),
            companion_bond_line(FEET_LINE, Some(PlaceId::Depths.chip_name())),
            companion_bond_line(RIDE_LINE, None),
        ] {
            let low = sample.to_lowercase();
            assert!(!low.contains("gold"), "{sample}");
            assert!(!low.contains("market"), "{sample}");
            assert!(!low.contains("xp"), "{sample}");
            assert!(!low.contains("hud"), "{sample}");
            assert!(!low.contains("widget"), "{sample}");
            assert!(!sample.contains("Threshold"), "{sample}");
        }
    }

    /// Absent `WoodClaim` keeps care, follow, mount, and dismount. A held claim blocks both.
    #[test]
    fn absent_wood_claim_keeps_legacy_mount_and_held_claim_blocks() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(CompanionBondPlugin)
            .init_resource::<PlayerInput>()
            .init_resource::<NearbyMercyNode>()
            .init_resource::<WorldAnswer>()
            .init_resource::<SoftPresence>()
            .init_resource::<crate::living_practice_loop::SoftPlayerRealm>()
            .init_resource::<ButtonInput<KeyCode>>();
        app.world_mut()
            .resource_mut::<crate::living_practice_loop::SoftPlayerRealm>()
            .current = Some(0);
        app.world_mut().resource_mut::<WorldAnswer>().kind = AnswerKind::Tend;
        app.world_mut().spawn((
            Name::new("ResonantDeer"),
            Transform::from_xyz(0.4, 0.9, 0.0),
        ));
        app.update();
        {
            let bond = app.world().resource::<CompanionBond>();
            assert!((bond.trust - 0.28).abs() < 1e-5);
            assert!(bond.nearby && !bond.mounted);
        }
        app.world_mut().resource_mut::<CompanionBond>().trust = 1.0;
        let edge = |app: &mut App| {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.release(soft_play_bindings::INTERACT);
            keys.clear();
            keys.press(soft_play_bindings::INTERACT);
        };
        edge(&mut app);
        app.update();
        assert!(app.world().resource::<CompanionBond>().mounted);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .clear();
        app.update();
        assert!(app.world().resource::<CompanionBond>().mounted);
        edge(&mut app);
        app.update();
        assert!(!app.world().resource::<CompanionBond>().mounted);
        app.insert_resource(crate::wood_expedition::WoodClaim::default());
        app.world_mut()
            .resource_mut::<crate::wood_expedition::WoodClaim>()
            .set_held(true);
        edge(&mut app);
        app.update();
        assert!(!app.world().resource::<CompanionBond>().mounted);
        app.world_mut().resource_mut::<CompanionBond>().mounted = true;
        edge(&mut app);
        app.update();
        assert!(app.world().resource::<CompanionBond>().mounted);
        app.world_mut()
            .remove_resource::<crate::wood_expedition::WoodClaim>();
        edge(&mut app);
        app.update();
        assert!(!app.world().resource::<CompanionBond>().mounted);
    }
}
