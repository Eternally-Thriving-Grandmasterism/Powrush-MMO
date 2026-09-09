//! U8 — Heartwood Wards dress. Spawns only on Heartwood. Does not persist.
//! Stranger-loop notice: near posts speak on the existing climate slab; E tends.

use bevy::prelude::*;

use shared::heartwood_wards::{
    WardDress, WardVerb, WARD_POST_CENTERS, WARD_POST_SIZE, WARD_USE_RADIUS,
};
use shared::hex_travel::PlaceId;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hex_travel::HexTravelState;
use crate::human_presence::SoftPresence;
use crate::input::PlayerInput;

#[derive(Component)]
struct WardPost;

#[derive(Resource, Debug, Default)]
pub struct WardSession {
    pub dress: WardDress,
    pub near: bool,
    pub last_line: String,
}

pub struct HeartwoodWardsPlugin;

impl Plugin for HeartwoodWardsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WardSession>()
            .add_systems(PreUpdate, mark_wards_near)
            .add_systems(
                Update,
                (
                    sync_heartwood_wards,
                    use_heartwood_wards,
                    breathe_ward_posts,
                )
                    .chain(),
            );
    }
}

fn wards_use_in_reach(place: PlaceId, x: f32, z: f32) -> bool {
    if place != PlaceId::Heartwood {
        return false;
    }
    let body = Vec2::new(x, z);
    WARD_POST_CENTERS
        .iter()
        .any(|c| body.distance(Vec2::new(c[0], c[2])) <= WARD_USE_RADIUS)
}

/// Reach test plus harvest hand-off. PreUpdate so Use at the posts is dress,
/// not a Take / stock credit.
fn mark_wards_near(
    travel: Res<HexTravelState>,
    presence: Res<SoftPresence>,
    mut session: ResMut<WardSession>,
    epiphany: Option<ResMut<FirstHarvestEpiphany>>,
) {
    let in_reach = wards_use_in_reach(travel.current, presence.position.x, presence.position.z);
    session.near = in_reach;
    if let Some(mut epiphany) = epiphany {
        if epiphany.wards_near != in_reach {
            epiphany.wards_near = in_reach;
        }
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
        session.near = false;
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
        // Soft notice juice — readable next to Threshold orb, not a HUD.
        emissive: LinearRgba::new(0.08, 0.14, 0.06, 1.0),
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

fn use_heartwood_wards(input: Res<PlayerInput>, mut session: ResMut<WardSession>) {
    if !session.near {
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

fn breathe_ward_posts(
    session: Res<WardSession>,
    time: Res<Time>,
    posts: Query<&Handle<StandardMaterial>, With<WardPost>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if posts.is_empty() {
        return;
    }
    let pulse = if session.near {
        0.10 + (time.elapsed_seconds() * 2.2).sin().abs() * 0.10
    } else {
        0.08
    };
    for handle in &posts {
        if let Some(mat) = materials.get_mut(handle) {
            mat.emissive = LinearRgba::new(pulse, pulse * 1.5, pulse * 0.7, 1.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::MinimalPlugins;

    use shared::heartwood_wards::WARD_POST_CENTERS;

    const STAND_HEIGHT: f32 = 0.90;

    #[test]
    fn near_posts_mark_session_and_teach_e_tend() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(HexTravelState {
            current: PlaceId::Heartwood,
        });
        let post = WARD_POST_CENTERS[0];
        app.insert_resource(SoftPresence {
            position: Vec3::new(post[0], STAND_HEIGHT, post[2]),
            velocity: Vec3::ZERO,
            grounded: true,
        });
        app.init_resource::<PlayerInput>();
        app.init_resource::<WardSession>();
        app.init_resource::<FirstHarvestEpiphany>();
        app.add_systems(PreUpdate, mark_wards_near);
        app.add_systems(Update, use_heartwood_wards);
        app.update();
        let session = app.world().resource::<WardSession>();
        assert!(session.near);
        assert!(session.dress.looked);
        assert!(session.last_line.contains("E tend"));
        assert!(app.world().resource::<FirstHarvestEpiphany>().wards_near);
    }

    #[test]
    fn sanctuary_does_not_notice_wards() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(HexTravelState {
            current: PlaceId::Sanctuary,
        });
        let post = WARD_POST_CENTERS[0];
        app.insert_resource(SoftPresence {
            position: Vec3::new(post[0], STAND_HEIGHT, post[2]),
            velocity: Vec3::ZERO,
            grounded: true,
        });
        app.init_resource::<WardSession>();
        app.init_resource::<FirstHarvestEpiphany>();
        app.add_systems(PreUpdate, mark_wards_near);
        app.update();
        assert!(!app.world().resource::<WardSession>().near);
        assert!(!app.world().resource::<FirstHarvestEpiphany>().wards_near);
    }
}
