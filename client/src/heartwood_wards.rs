//! U8 — Heartwood Wards dress. Spawns only on Heartwood. The dress itself does not persist; only the journey note may appear in the Journey feed.

use bevy::prelude::*;

use crate::hex_travel::HexTravelState;
use crate::human_presence::SoftPresence;
use crate::input::PlayerInput;
use shared::heartwood_wards::{
    WardDress, WardVerb, WARD_POST_CENTERS, WARD_POST_SIZE, WARD_SEALS, WARD_USE_RADIUS,
};
use shared::hex_travel::PlaceId;

#[derive(Component)]
struct WardPost;

#[derive(Resource, Debug, Default)]
pub struct WardSession {
    pub dress: WardDress,
    pub last_line: String,
    pub near: bool,
}

pub struct HeartwoodWardsPlugin;

impl Plugin for HeartwoodWardsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WardSession>()
            .add_systems(Update, (sync_heartwood_wards, use_heartwood_wards).chain());
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

/// `{place} · Well · Grove · Ember tended` from the existing ward seal words.
fn ward_place_line(place: &str) -> String {
    format!("{place} · {} tended", WARD_SEALS.join(" · "))
}

fn use_heartwood_wards(
    travel: Res<HexTravelState>,
    input: Res<PlayerInput>,
    presence: Res<SoftPresence>,
    mut session: ResMut<WardSession>,
    mut epiphany: Option<ResMut<crate::first_harvest_epiphany::FirstHarvestEpiphany>>,
    mut echo: Option<ResMut<crate::abundance_journey_echo::AbundanceJourneyEcho>>,
) {
    if travel.current != PlaceId::Heartwood {
        session.near = false;
        if let Some(epiphany) = epiphany.as_deref_mut() {
            epiphany.wards_near = false;
        }
        return;
    }
    let body = Vec2::new(presence.position.x, presence.position.z);
    let near = near_ward_post(body);
    session.near = near;
    if let Some(epiphany) = epiphany.as_deref_mut() {
        epiphany.wards_near = near;
    }
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
        let Some(echo) = echo.as_mut() else {
            return;
        };
        let text = ward_place_line(travel.chip_name());
        if echo.lines.iter().any(|existing| existing.text == text) {
            return;
        }
        echo.push(crate::abundance_journey_echo::JourneyKind::Note, text);
    }
}

fn near_ward_post(body: Vec2) -> bool {
    WARD_POST_CENTERS
        .iter()
        .any(|c| body.distance(Vec2::new(c[0], c[2])) <= WARD_USE_RADIUS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_radius_belongs_to_posts_only() {
        let post = WARD_POST_CENTERS[0];
        assert!(near_ward_post(Vec2::new(post[0], post[2])));
        assert!(!near_ward_post(Vec2::ZERO));
    }

    #[test]
    fn ward_place_line_names_heartwood_seals() {
        assert_eq!(
            ward_place_line("Heartwood"),
            "Heartwood · Well · Grove · Ember tended"
        );
    }

    fn tend_app(place: PlaceId, interact: bool, with_echo: bool) -> App {
        let post = WARD_POST_CENTERS[0];
        let mut app = App::new();
        app.insert_resource(HexTravelState { current: place });
        app.insert_resource(SoftPresence {
            position: Vec3::new(post[0], post[1], post[2]),
            velocity: Vec3::ZERO,
            grounded: true,
        });
        app.insert_resource(PlayerInput {
            interact,
            ..default()
        });
        app.init_resource::<WardSession>();
        if with_echo {
            app.init_resource::<crate::abundance_journey_echo::AbundanceJourneyEcho>();
        }
        app.add_systems(Update, use_heartwood_wards);
        app
    }

    #[test]
    fn tend_near_a_post_notes_the_place_once() {
        let mut app = tend_app(PlaceId::Heartwood, true, true);
        app.update();
        {
            let echo = app
                .world()
                .resource::<crate::abundance_journey_echo::AbundanceJourneyEcho>();
            assert_eq!(echo.lines.len(), 1);
            assert_eq!(echo.lines[0].text, ward_place_line("Heartwood"));
            assert_eq!(
                echo.lines[0].kind,
                crate::abundance_journey_echo::JourneyKind::Note
            );
        }
        app.update();
        let echo = app
            .world()
            .resource::<crate::abundance_journey_echo::AbundanceJourneyEcho>();
        assert_eq!(echo.lines.len(), 1);
        assert_eq!(
            echo.lines[0].text,
            "Heartwood · Well · Grove · Ember tended"
        );
    }

    #[test]
    fn look_and_leaving_and_missing_echo_push_nothing() {
        let mut look = tend_app(PlaceId::Heartwood, false, true);
        look.update();
        assert!(look
            .world()
            .resource::<crate::abundance_journey_echo::AbundanceJourneyEcho>()
            .lines
            .is_empty());
        assert!(look.world().resource::<WardSession>().dress.looked);

        let mut away = tend_app(PlaceId::Sanctuary, true, true);
        away.update();
        assert!(away
            .world()
            .resource::<crate::abundance_journey_echo::AbundanceJourneyEcho>()
            .lines
            .is_empty());

        let mut bare = tend_app(PlaceId::Heartwood, true, false);
        bare.update();
        assert_eq!(
            bare.world().resource::<WardSession>().last_line,
            shared::heartwood_wards::WARDS_NOTICE
        );
    }
}
