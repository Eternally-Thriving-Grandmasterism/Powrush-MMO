//! STORY-TEXT-1 — Heartwood plaque. E reads Sherif's Cydruid line; E again,
//! leaving reach, or leaving Heartwood hides it. Text only, no save field.

use bevy::prelude::*;
use shared::{heartwood_lamp::dist_xz, hex_travel::PlaceId};

use crate::{first_harvest_epiphany::FirstHarvestEpiphany, hex_travel::HexTravelState};
use crate::{human_presence::SoftPresence, soft_play_bindings};

/// docs/PLAYABLE_RACES.md:204 without the leading `> `.
pub const PLAQUE_TEXT: &str = "The Cydruids thrived in the hollows of the earth till they became virtually 1 with machines, and some of them opt to keep parts of their original human body; such as their face, brain and heart for biological emotions to continue after becoming cyborgs.";
pub const PLAQUE_BY: &str = "Sherif";
/// Heartwood ground (x, z), clear of lamp, pond, wards, shelf, Lip, nodes and teeth.
pub const PLAQUE_SPOT: [f32; 2] = [-8.0, -6.0];
pub const PLAQUE_REACH: f32 = 2.0;

#[derive(Resource, Debug, Default)]
pub struct PlaqueSession { pub near: bool, pub open: bool }

#[derive(Component)]
struct PlaqueSlabRoot;
pub struct HeartwoodPlaquePlugin;

impl Plugin for HeartwoodPlaquePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlaqueSession>().add_systems(Startup, spawn_plaque_slab).add_systems(PreUpdate, mark_plaque_near)
            .add_systems(Update, (handle_plaque, update_plaque_slab).chain());
    }
}

pub fn plaque_in_reach(place: PlaceId, x: f32, z: f32) -> bool {
    place == PlaceId::Heartwood && dist_xz(x, z, PLAQUE_SPOT[0], PLAQUE_SPOT[1]) <= PLAQUE_REACH
}

fn spawn_plaque_slab(mut commands: Commands) {
    let node = Node { position_type: PositionType::Absolute, top: Val::Px(164.0), left: Val::Px(16.0), width: Val::Px(420.0), padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)), justify_content: JustifyContent::FlexStart, border: UiRect::all(Val::Px(1.0)), ..default() };
    commands
        .spawn(((node, BackgroundColor(Color::srgba(0.06, 0.10, 0.07, 0.92)), BorderColor::all(Color::srgba(0.55, 0.78, 0.52, 0.55)), Visibility::Hidden), PlaqueSlabRoot))
        .with_children(|p| {
            p.spawn((
                Text::new(format!("{PLAQUE_TEXT}\n\n— {PLAQUE_BY}")),
                TextFont { font_size: FontSize::Px(14.0 / 1.2), ..default() },
                TextColor(Color::srgb(0.88, 0.96, 0.86)),
            ));
        });
}

/// PreUpdate so the harvest tap sees `plaque_near` on the same E edge.
/// Writes only on a change (heartwood_lip.rs:248-250), so saves stay quiet.
fn mark_plaque_near(
    travel: Res<HexTravelState>,
    presence: Res<SoftPresence>,
    mut session: ResMut<PlaqueSession>,
    epiphany: Option<ResMut<FirstHarvestEpiphany>>,
) {
    let near = plaque_in_reach(travel.current, presence.position.x, presence.position.z);
    if session.near != near { session.near = near; }
    if let Some(mut epi) = epiphany.filter(|e| e.plaque_near != near) {
        epi.plaque_near = near;
    }
}

fn handle_plaque(keyboard: Res<ButtonInput<KeyCode>>, mut session: ResMut<PlaqueSession>) {
    if !session.near {
        if session.open { session.open = false; }
    } else if keyboard.just_pressed(soft_play_bindings::INTERACT) {
        session.open = !session.open;
    }
}

fn update_plaque_slab(
    session: Res<PlaqueSession>,
    mut root: Query<&mut Visibility, With<PlaqueSlabRoot>>,
) {
    let want = if session.open && session.near { Visibility::Visible } else { Visibility::Hidden };
    for mut vis in &mut root {
        if *vis != want { *vis = want; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Resource, Default)]
    struct Seen(Vec<bool>);
    fn watch(epi: Res<FirstHarvestEpiphany>, mut seen: ResMut<Seen>) { seen.0.push(epi.is_changed()); }

    fn app_at(place: PlaceId, x: f32, z: f32) -> App {
        let mut app = App::new();
        app.insert_resource(HexTravelState { current: place });
        app.insert_resource(SoftPresence { position: Vec3::new(x, 0.9, z), ..Default::default() });
        app.init_resource::<ButtonInput<KeyCode>>().init_resource::<FirstHarvestEpiphany>();
        app.init_resource::<PlaqueSession>().init_resource::<Seen>();
        app.add_systems(PreUpdate, mark_plaque_near).add_systems(Update, (handle_plaque, watch).chain());
        app
    }
    fn at_plaque() -> App { app_at(PlaceId::Heartwood, PLAQUE_SPOT[0], PLAQUE_SPOT[1]) }
    fn press_e(app: &mut App) {
        let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        keys.release(soft_play_bindings::INTERACT);
        keys.clear();
        keys.press(soft_play_bindings::INTERACT);
        app.update();
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().clear();
    }
    fn open(app: &App) -> bool { app.world().resource::<PlaqueSession>().open }

    #[test]
    fn opens_near_not_far_and_e_toggles() {
        let mut near = at_plaque();
        press_e(&mut near);
        assert!(open(&near) && near.world().resource::<FirstHarvestEpiphany>().harvest_use_is_claimed());
        press_e(&mut near);
        assert!(!open(&near), "second E closes");
        let mut far = app_at(PlaceId::Heartwood, 0.0, 6.0);
        press_e(&mut far);
        assert!(!open(&far) && !far.world().resource::<FirstHarvestEpiphany>().harvest_use_is_claimed());
    }

    #[test]
    fn plaque_near_is_not_rewritten_each_frame() {
        let mut app = at_plaque();
        (0..3).for_each(|_| app.update());
        assert_eq!(app.world().resource::<Seen>().0, vec![true, false, false]);
    }

    #[test]
    fn leaving_reach_or_place_hides() {
        let mut app = at_plaque();
        press_e(&mut app);
        assert!(open(&app));
        app.world_mut().resource_mut::<SoftPresence>().position.x += PLAQUE_REACH + 1.0;
        app.update();
        assert!(!open(&app));
        let mut app = at_plaque();
        press_e(&mut app);
        app.world_mut().resource_mut::<HexTravelState>().current = PlaceId::Sanctuary;
        app.update();
        assert!(!open(&app));
        assert!(!app.world().resource::<FirstHarvestEpiphany>().plaque_near);
    }

    #[test]
    fn text_matches_doc_line_and_has_no_class_names() {
        let doc = include_str!("../../docs/PLAYABLE_RACES.md");
        assert!(doc.lines().any(|l| l.trim_end() == format!("> {PLAQUE_TEXT}")));
        for word in ["Legionnaire", "Arcblade", "Striker", "Resonator", "Mender", "Rebirth"] {
            assert!(!PLAQUE_TEXT.contains(word), "{word}");
        }
    }

    #[test]
    fn spot_is_clear_of_every_heartwood_thing() {
        use shared::heartwood_lamp::*;
        use shared::heartwood_wards::{WARD_POST_CENTERS, WARD_USE_RADIUS};
        use shared::threshold_shelf::{THRESHOLD_SHELF_CENTER as T, THRESHOLD_SHELF_USE_RADIUS};
        let reach = crate::mercy_harvest_nodes::HARVEST_REACH;
        let clear = |c: [f32; 2], r: f32| dist_xz(PLAQUE_SPOT[0], PLAQUE_SPOT[1], c[0], c[1]) > r + PLAQUE_REACH;
        // Node spawns at mercy_harvest_nodes.rs:323,330,337 equal shared WELL_ANCHORS.
        let well = reach.max(shared::skirmish_well::CONTEST_REACH);
        shared::skirmish_well::WELL_ANCHORS.iter().for_each(|&(x, _, z)| assert!(clear([x, z], well)));
        assert!(clear(LAMP_DISK_CENTER, LAMP_DISK_RADIUS));
        assert!(clear(WATER_POND_CENTER, WATER_POND_RADIUS));
        WARD_POST_CENTERS.iter().for_each(|p| assert!(clear([p[0], p[2]], WARD_USE_RADIUS)));
        assert!(clear([T[0], T[2]], THRESHOLD_SHELF_USE_RADIUS));
        HEARTWOOD_LIP_INSTANCES.iter().for_each(|l| assert!(clear([l.center[0], l.center[2]], l.footprint_radius)));
        assert!(clear(crate::combat_core::PREDATOR_HOME, reach));
    }
}
