//! U6 — Heartwood Lip dress and water bath.
//!
//! The Lip is fixed, place-scoped geometry: two walkway capsules and hanging
//! ribs, all outside the shared lamp disk. A pond step only returns the local
//! body to dry Lip ground; it never invokes house/embassy persistence.
//! Contact: info@Rathor.ai. Independent of xAI.

use bevy::prelude::*;

use shared::heartwood_lamp::{
    heartwood_bath_return, HeartwoodLipKind, HEARTWOOD_LIP_INSTANCES, WATER_POND_CENTER,
    WATER_POND_RADIUS,
};
use shared::hex_travel::{write_hex_named, HexClimateFile, PlaceId};
use shared::threshold_shelf::{
    resolve_threshold_use, threshold_use_in_reach, ThresholdPeaceNode, ThresholdShelfState,
    ThresholdUse, ThresholdVerb, THRESHOLD_NODE_CENTER, THRESHOLD_NODE_RADIUS,
    THRESHOLD_SHELF_CENTER, THRESHOLD_SHELF_SIZE,
};

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hex_travel::HexTravelState;
use crate::human_presence::SoftPresence;
use crate::input::PlayerInput;
use crate::lived_hour_bind::LivedHourBind;

const STAND_HEIGHT: f32 = 0.90;

#[derive(Component)]
struct HeartwoodLipProp;

#[derive(Component)]
struct HeartwoodWater;

#[derive(Component)]
struct ThresholdShelf;

#[derive(Component)]
struct ThresholdPeaceOrb;

#[derive(Resource, Debug, Default)]
struct HeartwoodLipState {
    active: bool,
}

#[derive(Resource, Debug, Default)]
pub struct ThresholdShelfSession {
    pub shelf: ThresholdShelfState,
    pub node: ThresholdPeaceNode,
    pub near: bool,
    pub last_line: String,
}

pub struct HeartwoodLipPlugin;

impl Plugin for HeartwoodLipPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HeartwoodLipState>()
            .init_resource::<ThresholdShelfSession>()
            // Claim the Use before the harvest tap reads it (same PreUpdate
            // seat the well / crownstone / embassy doors already use).
            .add_systems(PreUpdate, mark_threshold_near)
            .add_systems(
                Update,
                (
                    sync_heartwood_lip,
                    apply_heartwood_water_bath,
                    use_threshold_shelf,
                )
                    .chain(),
            );
    }
}

fn sync_heartwood_lip(
    mut commands: Commands,
    travel: Res<HexTravelState>,
    mut state: ResMut<HeartwoodLipState>,
    mut threshold: ResMut<ThresholdShelfSession>,
    mut presence: ResMut<SoftPresence>,
    existing: Query<
        Entity,
        Or<(
            With<HeartwoodLipProp>,
            With<HeartwoodWater>,
            With<ThresholdShelf>,
            With<ThresholdPeaceOrb>,
        )>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let heartwood = travel.current == PlaceId::Heartwood;
    if heartwood == state.active {
        return;
    }

    for entity in &existing {
        commands.entity(entity).despawn_recursive();
    }
    state.active = heartwood;
    threshold.shelf = ThresholdShelfState::default();
    threshold.node = ThresholdPeaceNode::default();
    threshold.near = false;
    threshold.last_line.clear();
    if !heartwood {
        return;
    }

    let walkway_mesh = meshes.add(Capsule3d::new(0.32, 2.0));
    let rib_mesh = meshes.add(Cuboid::new(0.16, 0.16, 3.0));
    let walkway_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.34, 0.20, 0.10),
        perceptual_roughness: 0.88,
        ..default()
    });
    let rib_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.22, 0.12, 0.07),
        perceptual_roughness: 0.82,
        ..default()
    });

    for (index, instance) in HEARTWOOD_LIP_INSTANCES.iter().enumerate() {
        let (mesh, material, rotation, label) = match instance.kind {
            HeartwoodLipKind::WalkwayCapsule => (
                walkway_mesh.clone(),
                walkway_material.clone(),
                Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                "HeartwoodWalkwayCapsule",
            ),
            HeartwoodLipKind::HangingRib => (
                rib_mesh.clone(),
                rib_material.clone(),
                Quat::IDENTITY,
                "HeartwoodHangingRib",
            ),
        };
        commands.spawn((
            PbrBundle {
                mesh,
                material,
                transform: Transform {
                    translation: Vec3::from_array(instance.center),
                    rotation,
                    ..default()
                },
                ..default()
            },
            HeartwoodLipProp,
            Name::new(format!("{label}_{index}")),
        ));
    }

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(
                THRESHOLD_SHELF_SIZE[0],
                THRESHOLD_SHELF_SIZE[1],
                THRESHOLD_SHELF_SIZE[2],
            )),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.40, 0.25, 0.11),
                emissive: LinearRgba::new(0.025, 0.012, 0.003, 1.0),
                perceptual_roughness: 0.86,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::from_array(THRESHOLD_SHELF_CENTER)),
            ..default()
        },
        ThresholdShelf,
        Name::new("ThresholdShelfLookTend"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(THRESHOLD_NODE_RADIUS)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.58, 0.82, 0.64),
                emissive: LinearRgba::new(0.12, 0.26, 0.14, 1.0),
                perceptual_roughness: 0.42,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::from_array(THRESHOLD_NODE_CENTER)),
            ..default()
        },
        ThresholdPeaceOrb,
        Name::new("ThresholdPeaceNode"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder::new(WATER_POND_RADIUS, 0.04)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgba(0.10, 0.36, 0.40, 0.82),
                perceptual_roughness: 0.24,
                metallic: 0.05,
                ..default()
            }),
            transform: Transform::from_xyz(WATER_POND_CENTER[0], 0.01, WATER_POND_CENTER[1]),
            ..default()
        },
        HeartwoodWater,
        Name::new("HeartwoodBath"),
    ));

    set_presence_on_dry_lip(&mut presence);
}

fn set_presence_on_dry_lip(presence: &mut SoftPresence) {
    if let Some([x, z]) = heartwood_bath_return(
        PlaceId::Heartwood,
        WATER_POND_CENTER[0],
        WATER_POND_CENTER[1],
    ) {
        presence.position = Vec3::new(x, STAND_HEIGHT, z);
        presence.velocity = Vec3::ZERO;
        presence.grounded = true;
    }
}

pub fn apply_bath_to_presence(place: PlaceId, presence: &mut SoftPresence) -> bool {
    let Some([x, z]) = heartwood_bath_return(place, presence.position.x, presence.position.z)
    else {
        return false;
    };
    presence.position = Vec3::new(x, STAND_HEIGHT, z);
    presence.velocity = Vec3::ZERO;
    presence.grounded = true;
    true
}

fn apply_heartwood_water_bath(travel: Res<HexTravelState>, mut presence: ResMut<SoftPresence>) {
    let _ = apply_bath_to_presence(travel.current, &mut presence);
}

/// Reach test plus the harvest hand-off. Runs in PreUpdate so
/// `handle_interact_harvest` sees `threshold_near` on the same Use edge and
/// returns before any Take, thriving line, or stock credit.
fn mark_threshold_near(
    travel: Res<HexTravelState>,
    presence: Res<SoftPresence>,
    mut session: ResMut<ThresholdShelfSession>,
    epiphany: Option<ResMut<FirstHarvestEpiphany>>,
) {
    let in_reach = threshold_use_in_reach(travel.current, presence.position.x, presence.position.z);
    session.near = in_reach;
    if let Some(mut epiphany) = epiphany {
        if epiphany.threshold_near != in_reach {
            epiphany.threshold_near = in_reach;
        }
    }
}

fn use_threshold_shelf(
    input: Res<PlayerInput>,
    travel: Res<HexTravelState>,
    mut session: ResMut<ThresholdShelfSession>,
    mut bind: Option<ResMut<LivedHourBind>>,
) {
    if !session.near {
        return;
    }
    if !session.shelf.looked {
        let _ = session.shelf.apply(ThresholdVerb::Look);
        session.last_line = session.node.speech();
        info!(target: "powrush::threshold", "{}", session.last_line);
    }
    if !input.interact {
        return;
    }
    // At the pipe the Peace Use is a Tend, never a harvest Take.
    if resolve_threshold_use(session.near) != ThresholdUse::Tend {
        return;
    }
    let _ = session.shelf.apply(ThresholdVerb::Tend);
    session.last_line = session.node.tend();
    if let Some(bind) = bind.as_deref_mut() {
        ink_room_tend(travel.current, bind);
    }
    info!(target: "powrush::threshold", "{}", session.last_line);
}

/// The pipe Tend leaves restored ink on the room it happened in, then writes
/// that room's own hex file so the ink survives without waiting for a leave.
/// The hex_id guard means one room's numbers can never land in another's file.
pub fn ink_room_tend(place: PlaceId, bind: &mut LivedHourBind) {
    if bind.climate.hex_id != place.as_str() {
        return;
    }
    bind.room_tend();
    let file = HexClimateFile::from_parts(place, bind.climate.clone(), bind.standing.clone());
    let _ = write_hex_named(&file);
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::gamepad::GamepadRumbleRequest;
    use bevy::MinimalPlugins;

    use crate::abundance_journey_echo::AbundanceJourneyEcho;
    use crate::first_session_guidance::FirstSessionGuidance;
    use crate::harvest_feel::SoftRbePool;
    use crate::hour_sacred::HourSacred;
    use crate::lived_hour_support::RbeUiSync;
    use crate::living_practice_loop::{LivingPracticeLoop, SoftPlayerRealm};
    use crate::mercy_harvest_nodes::{MercyHarvestNode, NearbyMercyNode};
    use crate::soft_play_bindings;
    use crate::thriving_moments::{ThrivingKind, ThrivingMoments};
    use crate::world_answer::WorldAnswer;
    use shared::heartwood_lamp::{
        heartwood_lip_is_valid, in_heartwood_lamp_disk, in_heartwood_water, HeartwoodLipKind,
        HEARTWOOD_BATH_RETURN,
    };
    use shared::hex_travel::{confirm_leave, places_eligible};
    use shared::stranger_loop_proof::hour_three_held_fixture;
    use shared::threshold_shelf::{
        threshold_node_is_valid, threshold_shelf_is_valid, visit_threshold, ThresholdPeaceNode,
        ThresholdShelfState, ThresholdVerb, THRESHOLD_PEACE_VERBS, THRESHOLD_SHELF_CENTER,
    };

    /// Body standing at the existing shelf with the real harvest loop live and
    /// a glowing node in reach, so the Use edge is genuinely contested.
    fn pipe_app(place: PlaceId, x: f32, z: f32) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(HexTravelState { current: place });
        app.insert_resource(SoftPresence {
            position: Vec3::new(x, STAND_HEIGHT, z),
            velocity: Vec3::ZERO,
            grounded: true,
        });
        app.insert_resource(HourSacred {
            session: Default::default(),
            complete: true,
            hour_three_complete: true,
        });
        app.init_resource::<PlayerInput>();
        app.init_resource::<ThresholdShelfSession>();
        app.init_resource::<ButtonInput<KeyCode>>();
        app.init_resource::<Gamepads>();
        app.init_resource::<FirstSessionGuidance>();
        app.init_resource::<ThrivingMoments>();
        app.init_resource::<AbundanceJourneyEcho>();
        app.init_resource::<SoftRbePool>();
        app.init_resource::<WorldAnswer>();
        app.init_resource::<RbeUiSync>();
        app.init_resource::<SoftPlayerRealm>();
        app.add_event::<GamepadRumbleRequest>();

        let node = app
            .world_mut()
            .spawn(MercyHarvestNode {
                name: "Verdant well",
                climate_id: 2,
                vitality: 1.0,
                harvests: 0,
                pulse: 0.0,
            })
            .id();
        app.insert_resource(NearbyMercyNode {
            entity: Some(node),
            name: Some("Verdant well"),
            distance: 0.5,
            in_range: true,
            nodes_exist: true,
            last_harvested: None,
        });

        app.add_plugins(crate::first_harvest_epiphany::FirstHarvestEpiphanyPlugin);
        // A seated book leaves the practice strip live, and it credits a mercy
        // harvest straight off the E key, so the Use edge is contested twice.
        app.add_plugins(crate::living_practice_loop::LivingPracticeLoopPlugin);
        app.insert_resource(LivingPracticeLoop {
            active: true,
            ..default()
        });
        app.add_systems(PreUpdate, mark_threshold_near);
        app.add_systems(Update, use_threshold_shelf);
        app
    }

    /// One real E: the key edge and the derived `PlayerInput` edge together,
    /// the way `input::sync_player_input` hands them to the frame.
    fn press_use_once(app: &mut App) {
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(soft_play_bindings::INTERACT);
        app.world_mut().resource_mut::<PlayerInput>().interact = true;
        app.update();
        app.world_mut().resource_mut::<PlayerInput>().interact = false;
        let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        keys.release(soft_play_bindings::INTERACT);
        keys.clear();
    }

    fn harvest_voice_spoke(app: &App) -> bool {
        app.world()
            .resource::<ThrivingMoments>()
            .fired
            .contains(&ThrivingKind::FirstMercyHarvest)
    }

    /// Practice harvests counted so far, across any surface it has cleared.
    fn practice_credits(app: &App) -> u32 {
        let practice = app.world().resource::<LivingPracticeLoop>();
        practice.surfaces_cleared * practice.harvests_needed + practice.mercy_harvests_on_surface
    }

    fn spoken_line(app: &App) -> String {
        app.world()
            .resource::<ThresholdShelfSession>()
            .last_line
            .clone()
    }

    /// The re-walk fail: the pipe Tend spoke, but Heartwood's climate stayed
    /// 0/0, so the file written on leave was blank and the House bill could
    /// never see the second well. One E must leave restored ink on this room.
    #[test]
    fn one_e_at_the_pipe_inks_the_heartwood_room_climate() {
        let mut app = pipe_app(
            PlaceId::Heartwood,
            THRESHOLD_SHELF_CENTER[0],
            THRESHOLD_SHELF_CENTER[2],
        );
        let mut heartwood = shared::hex_travel::heartwood_stub_climate();
        heartwood.hex_id = PlaceId::Heartwood.as_str().into();
        app.insert_resource(LivedHourBind {
            hour: shared::climate_node::LivedHour::new_demo(),
            climate: heartwood,
            standing: shared::hex_travel::heartwood_stub_standing(),
            week: shared::week_audit::WeekAudit::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        });
        app.update();

        let before = app.world().resource::<LivedHourBind>().climate.clone();
        let satchel_before = app.world().resource::<LivedHourBind>().satchel_count();
        assert_eq!(before.restored_count, 0, "Heartwood starts blank");

        press_use_once(&mut app);

        let bind = app.world().resource::<LivedHourBind>();
        assert_eq!(bind.climate.restored_count, 1, "the tend inked this room");
        assert_eq!(bind.climate.tons_moved, 0, "a Tend is still not a haul");
        assert_eq!(bind.climate.hex_id, PlaceId::Heartwood.as_str());
        assert_eq!(
            bind.satchel_count(),
            satchel_before,
            "a Tend is not a Take: no stock moved"
        );
    }

    /// One room's numbers must never be stamped into another room's file.
    #[test]
    fn ink_refuses_a_room_whose_climate_is_not_loaded() {
        let mut bind = LivedHourBind {
            hour: shared::climate_node::LivedHour::new_demo(),
            climate: shared::hex_travel::sanctuary_fresh_climate(),
            standing: shared::hex_travel::sanctuary_fresh_standing(),
            week: shared::week_audit::WeekAudit::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        };

        // Standing "on Heartwood" while Sanctuary climate is loaded: refuse.
        ink_room_tend(PlaceId::Heartwood, &mut bind);

        assert_eq!(bind.climate.restored_count, 0);
        assert_eq!(bind.climate.hex_id, PlaceId::Sanctuary.as_str());
    }

    /// The steward walk: face the node, one E, the line leaves Idle.
    #[test]
    fn one_e_at_the_pipe_leaves_idle_and_does_not_take() {
        let mut app = pipe_app(
            PlaceId::Heartwood,
            THRESHOLD_SHELF_CENTER[0],
            THRESHOLD_SHELF_CENTER[2],
        );
        app.update();

        let looked = spoken_line(&app);
        assert!(looked.contains("Idle"), "look line reads Idle: {looked}");
        assert!(
            app.world()
                .resource::<FirstHarvestEpiphany>()
                .threshold_near,
            "the pipe claims the Use before the harvest tap reads it"
        );

        press_use_once(&mut app);

        let tended = spoken_line(&app);
        assert_ne!(tended, looked, "one E must change the spoken line");
        assert!(
            !tended.contains("Idle"),
            "Idle after one E is a fail: {tended}"
        );
        assert!(tended.contains("Tended"));
        for verb in THRESHOLD_PEACE_VERBS {
            assert!(tended.contains(verb));
        }

        // That E was a Tend: the harvest tap did not swallow it.
        let epiphany = app.world().resource::<FirstHarvestEpiphany>();
        assert_eq!(epiphany.harvests_this_session, 0, "E must not Take");
        assert!(!epiphany.first_harvest_lived);
        assert!(
            !epiphany.pulse_line.contains("Take"),
            "no harvest voice on this E: {}",
            epiphany.pulse_line
        );

        // No harvest voice anywhere on that press — not from the harvest tap,
        // and not from the practice strip listening to the same key.
        assert!(
            !harvest_voice_spoke(&app),
            "the harvest-taken line must not show for a Tend"
        );
        assert_eq!(
            practice_credits(&app),
            0,
            "a Tend at the pipe is not practice harvest credit"
        );

        // Stock did not drop as if Taken.
        let pool = app.world().resource::<SoftRbePool>();
        assert_eq!(pool.harvests, 0, "Tend must not credit a harvest");
        assert_eq!(pool.vitality, 0.0);
        assert!(
            app.world()
                .resource::<NearbyMercyNode>()
                .last_harvested
                .is_none(),
            "no node was harvested"
        );
        assert_eq!(
            app.world().resource::<ThresholdShelfSession>().node.tends,
            1
        );
    }

    /// The harvest voice has more than one mouth: the harvest tap speaks it,
    /// and the practice strip speaks it straight off the E key. Standing at the
    /// pipe and pressing again must not open either of them.
    #[test]
    fn repeated_e_at_the_pipe_never_speaks_the_harvest_voice() {
        let mut app = pipe_app(
            PlaceId::Heartwood,
            THRESHOLD_SHELF_CENTER[0],
            THRESHOLD_SHELF_CENTER[2],
        );
        app.update();

        for press in 1..=5 {
            press_use_once(&mut app);
            let line = spoken_line(&app);
            assert!(!line.contains("Idle"), "press {press} read Idle: {line}");
            assert!(
                !line.contains("Take") && !line.contains("harvest"),
                "press {press} spoke a Take: {line}"
            );
            assert!(
                !harvest_voice_spoke(&app),
                "press {press} let the harvest-taken line through"
            );
            assert_eq!(
                practice_credits(&app),
                0,
                "press {press} credited a harvest"
            );
            assert_eq!(
                app.world().resource::<SoftRbePool>().harvests,
                0,
                "press {press} moved stock"
            );
            assert_eq!(
                app.world()
                    .resource::<FirstHarvestEpiphany>()
                    .harvests_this_session,
                0,
                "press {press} took"
            );
            assert_eq!(
                app.world().resource::<ThresholdShelfSession>().node.tends,
                press,
                "every press is one Tend"
            );
        }
    }

    /// Away from the pipe the Use is released, so the global Take still lives.
    #[test]
    fn away_from_the_pipe_the_harvest_keeps_the_use() {
        for (place, x, z) in [
            (
                PlaceId::Sanctuary,
                THRESHOLD_SHELF_CENTER[0],
                THRESHOLD_SHELF_CENTER[2],
            ),
            (
                PlaceId::Heartwood,
                THRESHOLD_SHELF_CENTER[0],
                THRESHOLD_SHELF_CENTER[2] - 9.0,
            ),
        ] {
            let mut app = pipe_app(place, x, z);
            app.update();
            press_use_once(&mut app);
            assert!(
                !app.world()
                    .resource::<FirstHarvestEpiphany>()
                    .threshold_near,
                "{place:?}: the pipe must not claim the Use from here"
            );
            assert!(
                spoken_line(&app).is_empty(),
                "{place:?}: no Threshold speech away from the shelf"
            );
            assert_eq!(
                app.world().resource::<ThresholdShelfSession>().node.tends,
                0
            );
            // Take is still the choice out here — the guard is the only change.
            assert_eq!(
                app.world()
                    .resource::<FirstHarvestEpiphany>()
                    .harvests_this_session,
                1,
                "{place:?}: the global harvest Take still owns this Use"
            );
            assert_eq!(app.world().resource::<SoftRbePool>().harvests, 1);
            assert!(
                harvest_voice_spoke(&app),
                "{place:?}: the harvest voice still belongs to Take out here"
            );
            assert!(
                practice_credits(&app) >= 1,
                "{place:?}: practice credit still rides the ordinary Take"
            );
        }
    }

    #[test]
    fn lip_has_two_capsules_and_no_instance_reaches_lamp() {
        assert!(heartwood_lip_is_valid());
        assert_eq!(
            HEARTWOOD_LIP_INSTANCES
                .iter()
                .filter(|instance| instance.kind == HeartwoodLipKind::WalkwayCapsule)
                .count(),
            2
        );
        assert!(HEARTWOOD_LIP_INSTANCES
            .iter()
            .all(|instance| { !in_heartwood_lamp_disk(instance.center[0], instance.center[2]) }));
    }

    #[test]
    fn water_bath_keeps_book_embassy_and_places_return() {
        let house = hour_three_held_fixture();
        let before = house.clone();
        let mut presence = SoftPresence {
            position: Vec3::new(WATER_POND_CENTER[0], -0.2, WATER_POND_CENTER[1]),
            velocity: Vec3::new(0.0, -4.0, 0.0),
            grounded: false,
        };

        assert!(apply_bath_to_presence(PlaceId::Heartwood, &mut presence));
        assert_eq!(house, before, "bath must not write the house pack");
        assert!(house.hour_three_complete);
        assert!(house.embassy.seated);
        assert!(places_eligible(house.complete, house.hour_three_complete));
        assert_eq!(
            confirm_leave(
                house.complete,
                house.hour_three_complete,
                PlaceId::Heartwood,
                PlaceId::Sanctuary,
            ),
            Ok(PlaceId::Sanctuary)
        );
        assert_eq!(
            [presence.position.x, presence.position.z],
            HEARTWOOD_BATH_RETURN
        );
        assert!(!in_heartwood_lamp_disk(
            presence.position.x,
            presence.position.z
        ));
        assert!(!in_heartwood_water(
            presence.position.x,
            presence.position.z
        ));
    }

    #[test]
    fn sanctuary_never_applies_heartwood_bath() {
        let mut presence = SoftPresence {
            position: Vec3::new(WATER_POND_CENTER[0], STAND_HEIGHT, WATER_POND_CENTER[1]),
            velocity: Vec3::ZERO,
            grounded: true,
        };
        assert!(!apply_bath_to_presence(PlaceId::Sanctuary, &mut presence));
    }

    #[test]
    fn threshold_roof_look_tend_keeps_house_and_places() {
        assert!(threshold_shelf_is_valid());
        assert!(threshold_node_is_valid());
        let house = hour_three_held_fixture();
        let before = house.clone();
        let mut shelf = ThresholdShelfState::default();
        let mut node = ThresholdPeaceNode::default();
        let _ = visit_threshold(&mut shelf, ThresholdVerb::Look, &house);
        let receipt = visit_threshold(&mut shelf, ThresholdVerb::Tend, &house);
        let speech = node.tend();
        assert!(THRESHOLD_PEACE_VERBS
            .iter()
            .all(|verb| speech.contains(verb)));
        assert!(receipt.hour_three_complete);
        assert!(receipt.embassy_seated);
        assert_eq!(house, before);
        assert!(places_eligible(house.complete, house.hour_three_complete));
        assert_eq!(
            confirm_leave(
                house.complete,
                house.hour_three_complete,
                PlaceId::Heartwood,
                PlaceId::Sanctuary,
            ),
            Ok(PlaceId::Sanctuary)
        );
    }
}
