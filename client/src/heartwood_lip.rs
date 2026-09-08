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
use shared::hex_travel::PlaceId;
use shared::threshold_shelf::{
    ThresholdShelfState, ThresholdVerb, THRESHOLD_SHELF_CENTER, THRESHOLD_SHELF_SIZE,
    THRESHOLD_SHELF_USE_RADIUS,
};

use crate::hex_travel::HexTravelState;
use crate::human_presence::SoftPresence;
use crate::input::PlayerInput;

const STAND_HEIGHT: f32 = 0.90;

#[derive(Component)]
struct HeartwoodLipProp;

#[derive(Component)]
struct HeartwoodWater;

#[derive(Component)]
struct ThresholdShelf;

#[derive(Resource, Debug, Default)]
struct HeartwoodLipState {
    active: bool,
}

#[derive(Resource, Debug, Default)]
pub struct ThresholdShelfSession {
    pub shelf: ThresholdShelfState,
    pub last_line: String,
}

pub struct HeartwoodLipPlugin;

impl Plugin for HeartwoodLipPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HeartwoodLipState>()
            .init_resource::<ThresholdShelfSession>()
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

fn use_threshold_shelf(
    travel: Res<HexTravelState>,
    input: Res<PlayerInput>,
    presence: Res<SoftPresence>,
    mut session: ResMut<ThresholdShelfSession>,
) {
    if travel.current != PlaceId::Heartwood {
        return;
    }
    let shelf_xz = Vec2::new(THRESHOLD_SHELF_CENTER[0], THRESHOLD_SHELF_CENTER[2]);
    let body_xz = Vec2::new(presence.position.x, presence.position.z);
    if body_xz.distance(shelf_xz) > THRESHOLD_SHELF_USE_RADIUS {
        return;
    }
    if !session.shelf.looked {
        session.last_line = session.shelf.apply(ThresholdVerb::Look).into();
        info!(target: "powrush::threshold", "{}", session.last_line);
    }
    if input.interact {
        session.last_line = session.shelf.apply(ThresholdVerb::Tend).into();
        info!(target: "powrush::threshold", "{}", session.last_line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::heartwood_lamp::{
        heartwood_lip_is_valid, in_heartwood_lamp_disk, in_heartwood_water, HeartwoodLipKind,
        HEARTWOOD_BATH_RETURN,
    };
    use shared::hex_travel::{confirm_leave, places_eligible};
    use shared::stranger_loop_proof::hour_three_held_fixture;
    use shared::threshold_shelf::{
        threshold_shelf_is_valid, visit_threshold, ThresholdShelfState, ThresholdVerb,
    };

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
        let house = hour_three_held_fixture();
        let before = house.clone();
        let mut shelf = ThresholdShelfState::default();
        let _ = visit_threshold(&mut shelf, ThresholdVerb::Look, &house);
        let receipt = visit_threshold(&mut shelf, ThresholdVerb::Tend, &house);
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
