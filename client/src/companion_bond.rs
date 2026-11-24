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
        app.init_resource::<CompanionBond>()
            .add_systems(
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
        tf.translation.y = crate::living_ecology::DEER_STAND_Y + crate::terrain::shelf_offset(tf.translation.x, tf.translation.z);
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
    let press = keyboard.just_pressed(soft_play_bindings::INTERACT) || input.interact;
    if !press {
        return;
    }
    if claim.is_some_and(|c| c.blocks()) {
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

    #[test]
    fn absent_wood_claim_keeps_legacy_mount_and_held_claim_blocks() {
        use crate::input::PlayerInput;
        use crate::living_practice_loop::SoftPlayerRealm;
        use crate::wood_expedition::WoodClaim;

        fn bond_app(claim: bool) -> App {
            let mut app = App::new();
            app.add_plugins(bevy::MinimalPlugins)
                .init_resource::<CompanionBond>()
                .init_resource::<NearbyMercyNode>()
                .init_resource::<PlayerInput>()
                .init_resource::<SoftPlayerRealm>()
                .init_resource::<crate::human_presence::SoftPresence>()
                .init_resource::<WorldAnswer>()
                .init_resource::<ButtonInput<KeyCode>>()
                .add_plugins(CompanionBondPlugin);
            if claim {
                app.init_resource::<WoodClaim>();
            }
            let here = Vec3::new(0.0, 0.9, 0.0);
            app.world_mut()
                .resource_mut::<crate::human_presence::SoftPresence>()
                .position = here;
            app.world_mut().resource_mut::<SoftPlayerRealm>().current = Some(0);
            app.world_mut().spawn((
                Name::new("ResonantDeer"),
                Transform::from_translation(here + Vec3::new(0.4, 0.55, 0.4)),
            ));
            {
                let mut bond = app.world_mut().resource_mut::<CompanionBond>();
                bond.trust = 0.6;
                bond.nearby = true;
            }
            app
        }

        fn tap(app: &mut App) {
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(soft_play_bindings::INTERACT);
            app.world_mut().resource_mut::<PlayerInput>().interact = true;
            app.update();
        }

        let mut legacy = bond_app(false);
        tap(&mut legacy);
        assert!(
            legacy.world().resource::<CompanionBond>().mounted,
            "absent claim should still mount"
        );
        tap(&mut legacy);
        assert!(
            !legacy.world().resource::<CompanionBond>().mounted,
            "absent claim should still dismount"
        );

        let mut held = bond_app(true);
        held.world_mut().resource_mut::<WoodClaim>().set_held(true);
        let mounted_before = held.world().resource::<CompanionBond>().mounted;
        tap(&mut held);
        assert_eq!(
            held.world().resource::<CompanionBond>().mounted,
            mounted_before,
            "held wood claim should block mount toggle"
        );
        held.world_mut().resource_mut::<CompanionBond>().mounted = true;
        tap(&mut held);
        assert!(
            held.world().resource::<CompanionBond>().mounted,
            "held wood claim should block dismount"
        );
    }

    fn gs_deer(player: Vec3, deer: Vec3) -> App {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins).init_resource::<CompanionBond>().init_resource::<NearbyMercyNode>().init_resource::<PlayerInput>().init_resource::<SoftPlayerRealm>().init_resource::<crate::human_presence::SoftPresence>().init_resource::<WorldAnswer>().init_resource::<ButtonInput<KeyCode>>().insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::from_millis(16))).add_plugins(CompanionBondPlugin);
        app.finish(); app.cleanup(); crate::test_env::single_thread(&mut app);
        app.world_mut().resource_mut::<crate::human_presence::SoftPresence>().position = player;
        app.world_mut().resource_mut::<SoftPlayerRealm>().current = Some(0);
        app.world_mut().spawn((Name::new("ResonantDeer"), Transform::from_translation(deer)));
        { let mut b = app.world_mut().resource_mut::<CompanionBond>(); b.trust = 0.6; b.mounted = false; }; app
    }
    fn gs_feet(app: &mut App, plan: crate::terrain::TerrainPlan) -> (Vec3, Vec3) {
        app.update();
        let deer = app.world_mut().query::<(&Name, &Transform)>().iter(app.world()).find(|(n, _)| n.as_str() == "ResonantDeer").unwrap().1.translation;
        let player = app.world().resource::<crate::human_presence::SoftPresence>().position;
        if crate::terrain::seat_clear(deer.x, deer.z, 0.16) { let tread = crate::terrain::tread_y_mesh(deer.x, deer.z, plan); assert!((deer.y - 0.435 - tread).abs() <= 0.03, "deer {deer:?} tread {tread}"); }
        (deer, player)
    }
    #[test]
    fn ground_snap_1_deer_follows_onto_shelf_one_with_feet_on_tread() {
        use crate::terrain::{shelf_offset, terrain_plan}; use shared::local_settings::GraphicsPreset;
        let plan = terrain_plan(GraphicsPreset::Medium);
        let mut app = gs_deer(Vec3::new(45.0, 0.90 + shelf_offset(45.0, 0.0), 0.0), Vec3::new(44.0, 0.55, -3.0));
        let (deer, at) = (0..300).fold((Vec3::ZERO, Vec3::ZERO), |_, _| gs_feet(&mut app, plan));
        assert!(deer.distance(at) <= 2.4, "deer {deer:?} player {at:?}");
    }
    #[test]
    fn ground_snap_1_deer_follow_court_edge_and_shelf_two_points() {
        use crate::terrain::{shelf_offset, terrain_plan}; use shared::local_settings::GraphicsPreset;
        let plan = terrain_plan(GraphicsPreset::Medium);
        for (x, z) in [(40.0_f32, 0.0_f32), (90.0, 0.0)] {
            let mut app = gs_deer(Vec3::new(x, 0.90 + shelf_offset(x, z), z), Vec3::new(x, 0.55, z - 3.0));
            let (deer, at) = (0..300).fold((Vec3::ZERO, Vec3::ZERO), |_, _| gs_feet(&mut app, plan));
            assert!(deer.distance(at) <= 2.4, "({x},{z}) deer {deer:?} player {at:?}");
        }
    }
}
