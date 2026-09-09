//! U8 — Heartwood Wards dress. Spawns only on Heartwood. Does not persist.

use bevy::prelude::*;

use shared::heartwood_wards::{
    WardDress, WardVerb, WARD_POST_CENTERS, WARD_POST_SIZE, WARD_USE_RADIUS,
};
use shared::hex_travel::PlaceId;
use crate::hex_travel::HexTravelState;
use crate::human_presence::SoftPresence;
use crate::input::PlayerInput;

#[derive(Component)]
struct WardPost;

#[derive(Resource, Debug, Default)]
pub struct WardSession {
    pub dress: WardDress,
    pub last_line: String,
}

pub struct HeartwoodWardsPlugin;

impl Plugin for HeartwoodWardsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WardSession>().add_systems(
            Update,
            (sync_heartwood_wards, use_heartwood_wards).chain(),
        );
    }
}

fn sync_heartwood_wards(
    mut commands: Commands,
    travel: Res<HexTravelState>,
    mut session: ResMut<WardSession>,
    existing: Query<Entity, With<WardPost>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let heartwood = travel.current == PlaceId::Heartwood;
    if !heartwood {
        for entity in &existing {
            commands.entity(entity).despawn_recursive();
        }
        session.dress = WardDress::default();
        session.last_line.clear();
        return;
    }
    if !existing.is_empty() {
        return;
    }
    let mesh = meshes.add(Cuboid::new(
        WARD_POST_SIZE[0],
        WARD_POST_SIZE[1],
        WARD_POST_SIZE[2],
    ));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.62, 0.48),
        emissive: LinearRgba::new(0.02, 0.03, 0.012, 1.0),
        perceptual_roughness: 0.8,
        ..default()
    });
    for (i, center) in WARD_POST_CENTERS.iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_translation(Vec3::from_array(*center)),
                ..default()
            },
            WardPost,
            Name::new(format!("WardPost{i}")),
        ));
    }
}

fn use_heartwood_wards(
    travel: Res<HexTravelState>,
    input: Res<PlayerInput>,
    presence: Res<SoftPresence>,
    mut session: ResMut<WardSession>,
) {
    if travel.current != PlaceId::Heartwood {
        return;
    }
    let body = Vec2::new(presence.position.x, presence.position.z);
    let near = WARD_POST_CENTERS.iter().any(|c| {
        body.distance(Vec2::new(c[0], c[2])) <= WARD_USE_RADIUS
    });
    if !near {
        return;
    }
    // Session dress only. Do not load or write the house pack from this hook.
    if !session.dress.looked {
        session.last_line = session.dress.apply(WardVerb::Look).into();
        info!(target: "powrush::wards", "{}", session.last_line);
    }
    if input.interact {
        session.last_line = session.dress.apply(WardVerb::Tend).into();
        info!(target: "powrush::wards", "{}", session.last_line);
    }
}
