/*!
 * Local Sovereign Session — first hour is complete alone (v21.99.4)
 *
 * No dedicated servers. No other humans in the realm yet.
 * The nodes, journey, climates, and pool must still reward the person at the keys.
 * Multiplayer / Steam / peer files are future sockets — never first-hour gates.
 *
 * PATSAGi ruling: do not teach launch-ops during play.
 *
 * CARD FLESH-SOVEREIGN-HOUR — the existing banner may name the Place
 * (`HexTravelState::chip_name`). Absent travel keeps the banner byte for byte.
 * The log stays offline / single human. No second HUD. No Online.
 * Peak memory, cited: walked · tended · week was the bill · yard remembered.
 *
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hud_anchor_registry::{HudSlab, ID_SOVEREIGN, SOVEREIGN};
use crate::first_session_guidance::FirstSessionGuidance;
use crate::input::{InputMapSet, PlayerInput};
use crate::title_screen::{HouseLabel, LaunchDoor, TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};

const BANNER_SECS: f64 = 8.5;

/// Existing banner. Absent travel keeps this byte for byte.
const BANNER_LINE: &str = "This hour is yours alone · no servers · the nodes still answer";
/// Existing log. Stays offline / single human. Place does not enter this line.
const LOG_LINE: &str = "local first session — offline, single human, complete without peers";

/// CARD FLESH-SOVEREIGN-HOUR — `{place} · {banner}` when a chip is present.
/// `None` returns the bare banner. One string. No second widget.
fn sovereign_banner_line(place: Option<&str>) -> String {
    match place {
        Some(place) => format!("{place} · {BANNER_LINE}"),
        None => BANNER_LINE.to_string(),
    }
}

#[derive(Resource, Debug)]
pub struct LocalSovereignSession {
    /// SOVEREIGN-YARD-GATE-1 — the visible budget in seconds, not an app-time end.
    pub banner_until: f64,
    pub dismissed: bool,
    pub announced: bool,
    /// Visible seconds so far: counted only InYard, not yielded and not paused.
    pub shown: f64,
}

impl Default for LocalSovereignSession {
    fn default() -> Self {
        Self {
            banner_until: BANNER_SECS,
            dismissed: false,
            announced: false,
            shown: 0.0,
        }
    }
}

#[derive(Component)]
struct SovereignBannerRoot;
#[derive(Component)]
struct SovereignBannerText;

pub struct LocalSovereignSessionPlugin;

impl Plugin for LocalSovereignSessionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LocalSovereignSession>()
            .add_systems(Startup, spawn_banner)
            .add_systems(
                Update,
                (announce_once, dismiss_on_intent, update_banner).after(InputMapSet),
            );
    }
}

fn spawn_banner(
    mut commands: Commands,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
) {
    let line = sovereign_banner_line(travel.as_ref().map(|state| state.chip_name()));
    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: SOVEREIGN.top(),
                    left: SOVEREIGN.left(),
                    width: Val::Px(520.0),
                    margin: SOVEREIGN.margin(),
                    padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG.with_alpha(1.0)),
                BorderColor::all(TITLE_BORDER.with_alpha(1.0)),
                Visibility::Visible,
            ),
            SovereignBannerRoot,
            HudSlab(SOVEREIGN.id),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(line),
TextFont { font_size: FontSize::Px(14.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                SovereignBannerText,
            ));
        });
}

/// SOVEREIGN-YARD-GATE-1 — the clock counts InYard only (an absent door, tests only, counts as InYard), not yielded, not paused.
pub fn banner_counts(door: Option<LaunchDoor>, yielded: bool, paused: bool) -> bool {
    door.is_none_or(|d| d == LaunchDoor::InYard) && !yielded && !paused
}

fn announce_once(mut session: ResMut<LocalSovereignSession>) {
    if session.announced {
        return;
    }
    session.announced = true;
    info!(
        target: "powrush::sovereign",
        "{LOG_LINE}"
    );
}

fn dismiss_on_intent(
    keyboard: Res<ButtonInput<KeyCode>>,
    harvest: Res<FirstHarvestEpiphany>,
    guidance: Res<FirstSessionGuidance>,
    mut session: ResMut<LocalSovereignSession>,
    player_input: Option<Res<PlayerInput>>,
    door: Option<Res<LaunchDoor>>,
) {
    if session.dismissed {
        return;
    }
    if door.is_some_and(|d| *d != LaunchDoor::InYard) { return; } // SOVEREIGN-YARD-GATE-1: no intent before the yard
    // WASD stays the keyboard check. Arrows, the stick, and touch arrive as
    // this frame's `PlayerInput.movement` (deadzone stays in input.rs).
    let moving = keyboard.pressed(KeyCode::KeyW)
        || keyboard.pressed(KeyCode::KeyA)
        || keyboard.pressed(KeyCode::KeyS)
        || keyboard.pressed(KeyCode::KeyD)
        || player_input.is_some_and(|input| input.movement.length_squared() > 0.0);
    if moving || harvest.first_harvest_lived || guidance.dismissed {
        session.dismissed = true;
    }
}

fn update_banner(
    time: Res<Time>,
    mut session: ResMut<LocalSovereignSession>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut root: Query<&mut Visibility, With<SovereignBannerRoot>>,
    mut text_q: Query<&mut Text, With<SovereignBannerText>>,
    (door, house, yielded, places, persona, edit): (
        Option<Res<LaunchDoor>>, Option<Res<HouseLabel>>, Option<Res<crate::hud_presets::HudYielded>>,
        Option<Res<crate::hex_travel::PlacesPlate>>, Option<Res<crate::title_screen::PersonaCreatorState>>, Option<Res<crate::hud_edit_mode::HudEditMode>>,
    ),
) {
    let (dt, yielded) = (time.delta_secs_f64(), yielded.is_some_and(|y| y.0.contains(&ID_SOVEREIGN)));
    let counts = banner_counts(door.as_deref().copied(), yielded, house.is_some_and(|h| h.settings_open) || ((places.is_some_and(|p| p.open) || persona.is_some_and(|p| p.open)) && !edit.is_some_and(|e| e.active)));
    if counts && dt > 0.0 && !session.dismissed && session.shown < session.banner_until { session.shown += dt; }
    let show = !session.dismissed && session.shown < session.banner_until;
    for mut vis in &mut root {
        vis.set_if_neq(if show { Visibility::Visible } else { Visibility::Hidden });
    }
    if !show {
        return;
    }
    let line = sovereign_banner_line(travel.as_ref().map(|state| state.chip_name()));
    for mut text in &mut text_q {
        if text.as_str() != line {
            **text = line.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::hex_travel::PlaceId;

    /// CARD HUD-ANCHOR-REGISTRY-2B — Sovereign position is the registry, Style is the coded literal.
    #[test]
    fn sovereign_style_byte_identical_to_coded_place() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_banner);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<&Node, With<SovereignBannerRoot>>();
        let style = q.single(app.world()).unwrap().clone();
        let coded = Node {
            position_type: PositionType::Absolute,
            top: Val::Px(52.0),
            left: Val::Percent(50.0),
            width: Val::Px(520.0),
            margin: UiRect::left(Val::Px(-260.0)),
            padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };
        assert_eq!(style, coded);
        assert_eq!(style.top, SOVEREIGN.top());
        assert_eq!(style.left, SOVEREIGN.left());
        assert_eq!(style.margin, SOVEREIGN.margin());
        assert_eq!(style.width, Val::Px(SOVEREIGN.width));
    }

    /// CARD VP-SLABS-REGAL-1 — the sovereign session banner rests on the title palette: opaque
    /// TITLE_PLATE_BG plate, TITLE_BORDER rim at alpha 1, TITLE_TEXT_PRIMARY text.
    #[test]
    fn slabs_regal_sovereign_banner_rests_on_title_palette() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_banner);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<(&BorderColor, &BackgroundColor), With<SovereignBannerRoot>>();
        let (border, bg) = q.single(app.world()).unwrap();
        let (border, bg) = (({
            assert_eq!(border.top, border.right, "border edges");
            assert_eq!(border.top, border.bottom, "border edges");
            assert_eq!(border.top, border.left, "border edges");
            border.top
        }).to_srgba(), bg.0.to_srgba());
        let mut t = app.world_mut().query_filtered::<&TextColor, With<SovereignBannerText>>();
        let txt = t.single(app.world()).unwrap().0.to_srgba();
        for (got, want, what) in [
            (bg, TITLE_PLATE_BG.to_srgba(), "plate"),
            (border, TITLE_BORDER.to_srgba(), "rim"),
            (txt, TITLE_TEXT_PRIMARY.to_srgba(), "text"),
        ] {
            assert!((got.red - want.red).abs() < 1e-6, "{what} red");
            assert!((got.green - want.green).abs() < 1e-6, "{what} green");
            assert!((got.blue - want.blue).abs() < 1e-6, "{what} blue");
            assert_eq!(got.alpha, 1.0, "{what} alpha");
        }
    }

    /// CARD FLESH-SOVEREIGN-HOUR — Place prefixes the banner; no travel keeps it exact.
    #[test]
    fn flesh_sovereign_banner_names_chip_or_keeps_banner() {
        assert_eq!(sovereign_banner_line(None), BANNER_LINE);
        assert_eq!(
            sovereign_banner_line(None),
            "This hour is yours alone · no servers · the nodes still answer"
        );
        assert_eq!(
            LOG_LINE,
            "local first session — offline, single human, complete without peers"
        );
        assert!(LOG_LINE.contains("offline"));
        assert!(LOG_LINE.contains("single human"));
        assert!(!LOG_LINE.contains("Sanctuary"));
        assert!(!LOG_LINE.contains("Heartwood"));
        assert!(!LOG_LINE.contains("Depths"));

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
                sovereign_banner_line(Some(travel.chip_name())),
                format!("{name} · {BANNER_LINE}")
            );
        }

        for sample in [
            sovereign_banner_line(Some(PlaceId::Sanctuary.chip_name())),
            sovereign_banner_line(Some(PlaceId::Heartwood.chip_name())),
            sovereign_banner_line(Some(PlaceId::Depths.chip_name())),
            sovereign_banner_line(None),
        ] {
            let low = sample.to_lowercase();
            assert!(!low.contains("gold"), "{sample}");
            assert!(!low.contains("market"), "{sample}");
            assert!(!low.contains("xp"), "{sample}");
            assert!(!low.contains("hud"), "{sample}");
            assert!(!low.contains("online"), "{sample}");
            assert!(!sample.contains("Threshold"), "{sample}");
        }
    }

    /// Resources `dismiss_on_intent` already required, plus optional movement.
    fn intent_app(with_player_input: bool) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<FirstHarvestEpiphany>()
            .init_resource::<FirstSessionGuidance>()
            .init_resource::<LocalSovereignSession>()
            .add_systems(Update, dismiss_on_intent);
        if with_player_input {
            app.init_resource::<PlayerInput>();
        }
        assert!(!app.world().resource::<LocalSovereignSession>().dismissed);
        assert!(
            !app.world()
                .resource::<FirstHarvestEpiphany>()
                .first_harvest_lived
        );
        assert!(!app.world().resource::<FirstSessionGuidance>().dismissed);
        if with_player_input {
            assert_eq!(app.world().resource::<PlayerInput>().movement, Vec2::ZERO);
        } else {
            assert!(app.world().get_resource::<PlayerInput>().is_none());
        }
        app
    }

    /// CARD SOVEREIGN-BANNER-INTENT-1 — WASD still dismisses when mapped movement is zero.
    /// W+S is the input-map cancel (movement stays 0) and must keep today's keyboard path.
    #[test]
    fn sovereign_banner_wasd_dismisses_when_movement_is_zero() {
        for key in [KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD] {
            let mut app = intent_app(true);
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(key);
            assert_eq!(
                app.world().resource::<PlayerInput>().movement,
                Vec2::ZERO,
                "{key:?} is tested with movement still zero"
            );
            app.update();
            assert!(
                app.world().resource::<LocalSovereignSession>().dismissed,
                "{key:?} still dismisses"
            );
        }

        let mut cancel = intent_app(true);
        {
            let mut keys = cancel.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.press(KeyCode::KeyW);
            keys.press(KeyCode::KeyS);
        }
        cancel.world_mut().resource_mut::<PlayerInput>().movement = Vec2::ZERO;
        cancel.update();
        assert!(
            cancel.world().resource::<LocalSovereignSession>().dismissed,
            "W+S still dismisses when movement cancels to zero"
        );
    }

    /// CARD SOVEREIGN-BANNER-INTENT-1 — arrow axes and stick deflections share `movement`.
    /// A value under input.rs's 0.18 deadzone still counts: this reader does not deadzone again.
    #[test]
    fn sovereign_banner_nonzero_movement_dismisses_without_wasd() {
        for movement in [
            Vec2::new(0.0, 1.0),
            Vec2::new(-1.0, 0.0),
            Vec2::new(0.42, -0.25),
            Vec2::new(0.05, 0.0),
        ] {
            let mut app = intent_app(true);
            app.world_mut().resource_mut::<PlayerInput>().movement = movement;
            app.update();
            assert!(
                app.world().resource::<LocalSovereignSession>().dismissed,
                "{movement:?} dismisses with no WASD"
            );
            let keys = app.world().resource::<ButtonInput<KeyCode>>();
            assert!(!keys.pressed(KeyCode::KeyW));
            assert!(!keys.pressed(KeyCode::KeyA));
            assert!(!keys.pressed(KeyCode::KeyS));
            assert!(!keys.pressed(KeyCode::KeyD));
        }
    }

    /// CARD SOVEREIGN-BANNER-INTENT-1 — zero movement and no WASD leaves the banner up.
    /// Arrow keys are not a second keyboard check; they dismiss only through `movement`.
    #[test]
    fn sovereign_banner_zero_movement_with_no_wasd_does_not_dismiss() {
        let mut idle = intent_app(true);
        idle.world_mut().resource_mut::<PlayerInput>().movement = Vec2::ZERO;
        idle.update();
        assert!(!idle.world().resource::<LocalSovereignSession>().dismissed);

        let mut arrows = intent_app(true);
        {
            let mut keys = arrows.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.press(KeyCode::ArrowUp);
            keys.press(KeyCode::ArrowDown);
            keys.press(KeyCode::ArrowLeft);
            keys.press(KeyCode::ArrowRight);
        }
        assert_eq!(
            arrows.world().resource::<PlayerInput>().movement,
            Vec2::ZERO
        );
        arrows.update();
        assert!(
            !arrows.world().resource::<LocalSovereignSession>().dismissed,
            "arrow keys with zero movement do not dismiss"
        );

        let mut other = intent_app(true);
        {
            let mut keys = other.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.press(KeyCode::KeyE);
            keys.press(KeyCode::Space);
            keys.press(KeyCode::ShiftLeft);
        }
        other.update();
        assert!(
            !other.world().resource::<LocalSovereignSession>().dismissed,
            "non-move keys do not dismiss"
        );
    }

    /// CARD SOVEREIGN-BANNER-INTENT-1 — `dismiss_on_intent` is ordered after `InputMapSet`
    /// so it reads this frame's movement.
    #[test]
    fn sovereign_banner_intent_runs_after_input_map_set() {
        use bevy::ecs::schedule::{NodeId, Schedules};
        use bevy::ecs::system::{IntoSystem, System};

        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(LocalSovereignSessionPlugin);
        let graph = app
            .world()
            .resource::<Schedules>()
            .get(Update)
            .expect("Update schedule")
            .graph();
        let want = IntoSystem::system_type_id(&dismiss_on_intent);
        let system_key = graph
            .systems
            .iter()
            .find_map(|(key, scheduled, _)| {
                (System::system_type(&**scheduled) == want).then_some(key)
            })
            .expect("dismiss_on_intent is in Update");
        let set_key = graph
            .system_sets
            .get_key(InputMapSet.intern())
            .expect("InputMapSet is on the Update graph");
        assert!(
            graph
                .dependency()
                .graph()
                .contains_edge(NodeId::Set(set_key), NodeId::System(system_key),),
            "dismiss_on_intent is after InputMapSet"
        );
    }

    /// CARD SOVEREIGN-BANNER-INTENT-1 — no `PlayerInput` resource: no panic, WASD still dismisses.
    #[test]
    fn sovereign_banner_missing_player_input_does_not_panic_and_wasd_still_dismisses() {
        let mut idle = intent_app(false);
        idle.update();
        assert!(idle.world().get_resource::<PlayerInput>().is_none());
        assert!(!idle.world().resource::<LocalSovereignSession>().dismissed);

        let mut keyed = intent_app(false);
        keyed
            .world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        keyed.update();
        assert!(keyed.world().get_resource::<PlayerInput>().is_none());
        assert!(keyed.world().resource::<LocalSovereignSession>().dismissed);

        let mut plugin = App::new();
        plugin
            .add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<FirstHarvestEpiphany>()
            .init_resource::<FirstSessionGuidance>()
            .add_plugins(LocalSovereignSessionPlugin);
        assert!(plugin.world().get_resource::<PlayerInput>().is_none());
        plugin
            .world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyD);
        plugin.update();
        assert!(plugin.world().get_resource::<PlayerInput>().is_none());
        let session = plugin.world().resource::<LocalSovereignSession>();
        assert!(session.dismissed, "plugin path still sees WASD");
        assert!(session.announced);
    }

    /// CARD SOVEREIGN-BANNER-INTENT-1 — a dismissed banner stays dismissed on a quiet frame.
    #[test]
    fn sovereign_banner_already_dismissed_stays_dismissed() {
        let mut quiet = intent_app(true);
        {
            let mut session = quiet.world_mut().resource_mut::<LocalSovereignSession>();
            session.dismissed = true;
            assert_eq!(session.banner_until, BANNER_SECS);
        }
        quiet.update();
        {
            let session = quiet.world().resource::<LocalSovereignSession>();
            assert!(session.dismissed);
            assert_eq!(session.banner_until, BANNER_SECS);
            assert!(!session.announced);
        }

        let mut moving = intent_app(false);
        moving
            .world_mut()
            .resource_mut::<LocalSovereignSession>()
            .dismissed = true;
        moving
            .world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyA);
        moving.update();
        assert!(moving.world().resource::<LocalSovereignSession>().dismissed);
    }

    /// Harvest and guidance dismissal stay on the same predicate.
    #[test]
    fn sovereign_banner_harvest_and_guidance_still_dismiss_without_movement() {
        let mut harvest = intent_app(true);
        harvest
            .world_mut()
            .resource_mut::<FirstHarvestEpiphany>()
            .first_harvest_lived = true;
        harvest.update();
        assert!(
            harvest
                .world()
                .resource::<LocalSovereignSession>()
                .dismissed
        );

        let mut guidance = intent_app(false);
        guidance
            .world_mut()
            .resource_mut::<FirstSessionGuidance>()
            .dismissed = true;
        guidance.update();
        assert!(
            guidance
                .world()
                .resource::<LocalSovereignSession>()
                .dismissed
        );
    }

    /// SOVEREIGN-YARD-GATE-1 — one frame of the visible clock, the same rule as `update_banner`.
    fn step(s: &mut LocalSovereignSession, door: LaunchDoor, yielded: bool, paused: bool, dt: f64) -> bool {
        if banner_counts(Some(door), yielded, paused) && s.shown < s.banner_until { s.shown += dt; }
        !s.dismissed && s.shown < s.banner_until
    }

    #[test]
    fn sovereign_yard_gate_title_time_yield_and_pause_do_not_count() {
        let mut s = LocalSovereignSession::default();
        assert!((0..40).all(|_| step(&mut s, LaunchDoor::Title, false, false, 0.5)) && s.shown == 0.0, "20 s of title flow");
        assert!((0..24).all(|_| step(&mut s, LaunchDoor::InYard, true, false, 0.125) && step(&mut s, LaunchDoor::InYard, false, true, 0.125)));
        assert!(s.shown == 0.0 && banner_counts(None, false, false), "yield and pause hold the clock");
        assert!((0..16).all(|_| step(&mut s, LaunchDoor::InYard, false, false, 0.5)) && s.shown == 8.0);
        assert!(!step(&mut s, LaunchDoor::InYard, false, false, 0.5) && s.shown == 8.5, "8.5 s visible, then hidden");
    }

    #[test]
    fn sovereign_yard_gate_title_and_back_resumes_without_reset() {
        let mut s = LocalSovereignSession::default();
        (0..8).for_each(|_| { step(&mut s, LaunchDoor::InYard, false, false, 0.5); });
        (0..20).for_each(|_| { step(&mut s, LaunchDoor::Title, false, false, 0.5); });
        assert!(s.shown == 4.0 && s.banner_until - s.shown == 4.5 && s.banner_until == BANNER_SECS);
        assert!((0..8).all(|_| step(&mut s, LaunchDoor::InYard, false, false, 0.5)) && !step(&mut s, LaunchDoor::InYard, false, false, 0.5));
    }

    #[test]
    fn sovereign_yard_gate_intent_before_the_yard_does_not_dismiss() {
        let mut app = intent_app(false);
        app.insert_resource(LaunchDoor::Title);
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::KeyW);
        app.update();
        assert!(!app.world().resource::<LocalSovereignSession>().dismissed, "WASD on Title keeps the banner");
        app.insert_resource(LaunchDoor::InYard);
        app.update();
        assert!(app.world().resource::<LocalSovereignSession>().dismissed, "the same press InYard dismisses");
    }

    /// SOVEREIGN-MODAL-HOLD-1 — Places or Persona open holds the clock (R5 hides the banner), unless HUD edit mode is up.
    #[test]
    fn sovereign_modal_hold_places_and_persona_hold_the_clock() {
        let src = include_str!("local_sovereign_session.rs");
        assert!(src.contains(concat!("house.is_some_and(|h| h.settings_open) || ((places.is_some_and(|p| p.open) || persona.is_some_and(|p| p.open))", " && !edit.is_some_and(|e| e.active)));")));
        let modal = |places: bool, persona: bool, edit: bool| ((places || persona) && !edit);
        let mut s = LocalSovereignSession::default();
        let run = |s: &mut LocalSovereignSession, paused: bool| (0..8).for_each(|_| { step(s, LaunchDoor::InYard, false, paused, 0.125); });
        run(&mut s, modal(true, false, false));
        assert!(s.shown == 0.0, "Places open holds");
        run(&mut s, modal(false, true, false));
        assert!(s.shown == 0.0, "Persona open holds");
        run(&mut s, modal(true, false, true));
        assert!(s.shown == 1.0, "Places with edit mode counts");
        run(&mut s, modal(false, false, false));
        assert!(s.shown == 2.0 && s.banner_until == BANNER_SECS, "closing resumes, no reset");
    }

    /// STATUS-VIS-NEQ-1 SV13–SV16. Last tape of (Visibility, changed) on SovereignBannerRoot; settle 2 then clear; 125 ms manual steps.
    #[derive(Resource, Default)] struct SvTape(Vec<(Visibility, bool)>);
    fn sv_tape(q: Query<Ref<Visibility>, With<SovereignBannerRoot>>, mut t: ResMut<SvTape>) { t.0.extend(q.iter().map(|v| (*v, v.is_changed()))); }
    fn sv_run(app: &mut App, n: usize) -> Vec<(Visibility, bool)> { for _ in 0..n { app.update(); } std::mem::take(&mut app.world_mut().resource_mut::<SvTape>().0) }
    fn sv_settle(mut app: App) -> App { app.update(); app.update(); app.world_mut().resource_mut::<SvTape>().0.clear(); app }
    fn sv_hide(app: &mut App) { let e = app.world_mut().query_filtered::<Entity, With<SovereignBannerRoot>>().single(app.world()).unwrap(); *app.world_mut().get_mut::<Visibility>(e).unwrap() = Visibility::Hidden; }
    fn sv_yielded(app: &App, id: &'static str) -> bool { app.world().resource::<crate::hud_presets::HudYielded>().0.contains(&id) }
    const SV: Visibility = Visibility::Visible;
    const SH: Visibility = Visibility::Hidden;
    fn sv_app(dismissed: bool, hud: Option<(crate::hud_presets::HudPresetId, Option<crate::title_screen::LaunchDoor>)>) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).insert_resource(LocalSovereignSession { dismissed, ..Default::default() }).init_resource::<SvTape>().insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::from_millis(125))).add_systems(Startup, spawn_banner).add_systems(Update, update_banner).add_systems(Last, sv_tape);
        if let Some((id, door)) = hud { app.add_plugins(crate::hud_presets::HudLayoutPlugin).insert_resource(crate::hud_presets::ActiveHudPreset { id: Some(id) }).init_resource::<crate::hex_travel::PlacesPlate>(); if let Some(door) = door { app.insert_resource(door); } }
        sv_settle(app)
    }
    fn sv_shown(app: &App) -> f64 { app.world().resource::<LocalSovereignSession>().shown }
    #[test] fn status_vis_neq_1_sv13_sovereign_dismissed_steady_unmarked() { let mut app = sv_app(true, None); assert_eq!(sv_run(&mut app, 3), vec![(SH, false); 3], "SV13 sovereign dismissed steady hidden"); }
    #[test] fn status_vis_neq_1_sv14_sovereign_fresh_steady_then_one_change_at_banner_end() {
        let mut app = sv_app(false, None);
        assert_eq!(sv_run(&mut app, 3), vec![(SV, false); 3], "SV14 fresh banner frames 3-5: steady Visible, quiet");
        let t = sv_run(&mut app, 80); let k = t.iter().position(|r| r.0 == SH).expect("SV14 banner ends");
        assert_eq!(t, [vec![(SV, false); k], vec![(SH, true)], vec![(SH, false); 79 - k]].concat(), "SV14 banner end: Hidden once, then quiet");
        assert!(sv_shown(&app) >= BANNER_SECS, "SV14 the banner ends at shown {BANNER_SECS}");
    }
    #[test] fn status_vis_neq_1_sv15_sovereign_direct_hidden_restored_once() { let mut app = sv_app(false, None); sv_hide(&mut app); assert_eq!(sv_run(&mut app, 2), vec![(SV, true), (SV, false)], "SV15 sovereign direct Hidden write while live: restored once, then quiet"); }
    #[test] fn status_vis_neq_1_sv16_sovereign_real_hud_layout() {
        let mut app = sv_app(false, Some((crate::hud_presets::HudPresetId::Classic, Some(crate::title_screen::LaunchDoor::Title))));
        assert_eq!((sv_run(&mut app, 2), sv_shown(&app)), (vec![(SH, true); 2], 0.0), "SV16 Title door: R5 hides every frame on main and branch alike; the clock holds");
        let mut app = sv_app(false, Some((crate::hud_presets::HudPresetId::Classic, Some(crate::title_screen::LaunchDoor::InYard))));
        assert_eq!(sv_run(&mut app, 2), vec![(SH, true); 2], "SV16 Classic InYard: clean-hidden, changed every frame on main and branch alike");
        let mut app = sv_app(false, Some((crate::hud_presets::HudPresetId::Management, Some(crate::title_screen::LaunchDoor::InYard))));
        assert_eq!(sv_run(&mut app, 2), vec![(SV, false); 2], "SV16 Management InYard: steady Visible, quiet");
        app.world_mut().spawn((Node::default(), Visibility::Visible, HudSlab(crate::hud_anchor_registry::ID_PULSE))); sv_run(&mut app, 1); let held = sv_shown(&app); let t = sv_run(&mut app, 2);
        assert_eq!((t, sv_yielded(&app, ID_SOVEREIGN), sv_shown(&app)), (vec![(SH, true); 2], true, held), "SV16 Management with a Pulse sentinel: Sovereign yields by R2 into HudYielded (changed every frame on main and branch alike) and the clock holds");
    }
}
