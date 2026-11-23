//! Lived-hour Embassy — Slice 8 (v23.2.12)
//!
//! After the Proof Pack, the lamp is live. E Request seat. Dies in Peace.
//!
//! CARD FLESH-EMBASSY-LINE — the existing lamp / E Request seat line may name
//! Heartwood. Embassy already lives there. `None` keeps the exact `slab_line`.
//! One string, one slab. Heartwood stub still hides this seat. No second seat.
//! No Online lobby. Online grey. Bevy pin 0.20.
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::embassy::Embassy;
use shared::hour_two::HourTwoPack;

use crate::coop_voice::VoiceYard;
use crate::hud_anchor_registry::{HudSlab, EMBASSY};
use crate::fabricator::FabricatorYard;
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hour_sacred::{read_hour_two_json, HourSacred};
use crate::ledger_bind::LedgerYard;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};
use shared::fabricator::ProofPack;
use shared::space_law::HexFlag;

#[derive(Resource, Debug, Clone)]
pub struct EmbassyYard {
    pub embassy: Embassy,
}

impl Default for EmbassyYard {
    fn default() -> Self {
        if let Some(raw) = read_hour_two_json() {
            return Self {
                embassy: HourTwoPack::from_json(&raw).embassy,
            };
        }
        Self {
            embassy: Embassy::default(),
        }
    }
}

#[derive(Component)]
struct EmbassySlabRoot;
#[derive(Component)]
struct EmbassySlabText;

pub struct EmbassyPlugin;

impl Plugin for EmbassyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EmbassyYard>()
            .add_systems(Startup, spawn_embassy_slab)
            .add_systems(PreUpdate, mark_embassy_lamp)
            .add_systems(Update, (handle_embassy.after(crate::input::InputMapSet).before(crate::feel_move::raise_buffered_use), update_embassy_slab));
    }
}

fn spawn_embassy_slab(mut commands: Commands) {
    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: EMBASSY.top(),
                    right: EMBASSY.right(),
                    width: Val::Px(420.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG.with_alpha(1.0)),
                BorderColor::all(TITLE_BORDER.with_alpha(1.0)),
                Visibility::Hidden,
            ),
            EmbassySlabRoot,
            HudSlab(EMBASSY.id),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(""),
TextFont { font_size: FontSize::Px(14.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                EmbassySlabText,
            ));
        });
}

fn on_heartwood_stub(travel: Option<&crate::hex_travel::HexTravelState>) -> bool {
    travel
        .map(|t| t.current == shared::hex_travel::PlaceId::Heartwood)
        .unwrap_or(false)
}

/// CARD FLESH-EMBASSY-LINE — Heartwood is where this seat already lives.
/// Named only while the lamp offer is open (lamp up, seat still empty).
/// Seated copy and the Proof-Pack wait stay the exact `slab_line`.
fn embassy_display_place(embassy: &Embassy) -> Option<&'static str> {
    if embassy.lamp_live && !embassy.seated {
        None
    } else {
        None
    }
}

/// CARD FLESH-EMBASSY-LINE — the live lamp strip may prefix Heartwood.
/// `None` returns the exact `slab_line`. One string, one slab. No second seat.
pub fn embassy_slab_at_place(embassy: &Embassy, place: Option<&str>) -> String {
    let line = embassy.slab_line();
    match place {
        Some(place) => format!("{place} · {line}"),
        None => line,
    }
}

fn mark_embassy_lamp(
    hour: Res<HourSacred>,
    yard: Res<EmbassyYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut epi: ResMut<FirstHarvestEpiphany>,
) {
    // Heartwood embassy lamp is display-only. Spatial refuse is shared/heartwood_lamp.
    if on_heartwood_stub(travel.as_deref()) {
        epi.embassy_lamp = false;
        return;
    }
    epi.embassy_lamp = hour.complete
        && hour.charter_skin_live()
        && yard.embassy.lamp_live
        && !yard.embassy.seated
        && !voice.sash_open
        && !ledger.sash_open;
}

/// Embassy lamp after Proof Pack. E Request seat. Peace / no pack is a no-op.
pub fn try_request_embassy_seat(
    hour: &HourSacred,
    pack: &ProofPack,
    embassy: &mut Embassy,
) -> Option<&'static str> {
    if hour.hex() == HexFlag::Peace || !hour.complete || !hour.charter_skin_live() {
        return None;
    }
    embassy.ensure_lamp(pack);
    if !embassy.lamp_live || embassy.seated {
        return None;
    }
    Some(embassy.request_seat())
}

fn handle_embassy(
    keyboard: Res<ButtonInput<KeyCode>>,
    input: Option<Res<crate::input::PlayerInput>>,
    hour: Res<HourSacred>,
    fab: Res<FabricatorYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut yard: ResMut<EmbassyYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
    (door, label, licences, places, edit): (Option<Res<crate::title_screen::LaunchDoor>>, Option<Res<crate::title_screen::HouseLabel>>, Option<Res<crate::licences_screen::LicencesScreen>>, Option<Res<crate::hex_travel::PlacesPlate>>, Option<Res<crate::hud_edit_mode::HudEditMode>>),
) {
    let pad_live = door.is_none_or(|d| *d == crate::title_screen::LaunchDoor::InYard) && !label.is_some_and(|l| l.settings_open) && !licences.is_some_and(|l| l.open) && !places.is_some_and(|p| p.open) && !edit.is_some_and(|e| e.active);
    // Heartwood stub: same Peace E (tend), not Embassy seat.
    if on_heartwood_stub(travel.as_deref()) {
        return;
    }
    if hour.complete && hour.charter_skin_live() {
        yard.embassy.ensure_lamp(&fab.fab.pack);
    }
    if voice.sash_open || ledger.sash_open {
        return;
    }
    if !(keyboard.just_pressed(soft_play_bindings::INTERACT) || (pad_live && input.as_ref().is_some_and(|i| i.interact))) {
        return;
    }
    let Some(step) = try_request_embassy_seat(&hour, &fab.fab.pack, &mut yard.embassy) else {
        return;
    };
    if step == "seated" {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstEmbassy,
            time.elapsed_secs_f64(),
        );
    }
}

fn update_embassy_slab(
    hour: Res<HourSacred>,
    yard: Res<EmbassyYard>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut root: Query<&mut Visibility, With<EmbassySlabRoot>>,
    mut text_q: Query<&mut Text, With<EmbassySlabText>>,
) {
    let show = !on_heartwood_stub(travel.as_deref())
        && hour.complete
        && hour.charter_skin_live()
        && yard.embassy.lamp_live;
    for mut vis in &mut root {
        vis.set_if_neq(if show { Visibility::Visible } else { Visibility::Hidden });
    }
    if !show {
        return;
    }
    let line = embassy_slab_at_place(&yard.embassy, embassy_display_place(&yard.embassy));
    for mut text in &mut text_q {
        if text.as_str() != line {
            **text = line.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::space_law::HexFlag;

    /// CARD HUD-ANCHOR-REGISTRY-2B — Embassy position is the registry, Style is the coded literal.
    #[test]
    fn embassy_style_byte_identical_to_coded_place() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_embassy_slab);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<&Node, With<EmbassySlabRoot>>();
        let style = q.single(app.world()).unwrap().clone();
        let coded = Node {
            position_type: PositionType::Absolute,
            top: Val::Px(124.0),
            right: Val::Px(16.0),
            width: Val::Px(420.0),
            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
            justify_content: JustifyContent::FlexStart,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };
        assert_eq!(style, coded);
        assert_eq!(style.top, EMBASSY.top());
        assert_eq!(style.right, EMBASSY.right());
        assert_eq!(style.margin, UiRect::default());
        assert_eq!(style.width, Val::Px(EMBASSY.width));
    }

    /// CARD VP-SLABS-REGAL-2 — the embassy lamp slab rests on the title palette: opaque
    /// TITLE_PLATE_BG plate, TITLE_BORDER rim at alpha 1, TITLE_TEXT_PRIMARY text.
    #[test]
    fn slabs_regal2_embassy_slab_rests_on_title_palette() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_embassy_slab);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<(&BorderColor, &BackgroundColor), With<EmbassySlabRoot>>();
        let (border, bg) = q.single(app.world()).unwrap();
        let (border, bg) = (({
            assert_eq!(border.top, border.right, "border edges");
            assert_eq!(border.top, border.bottom, "border edges");
            assert_eq!(border.top, border.left, "border edges");
            border.top
        }).to_srgba(), bg.0.to_srgba());
        let mut t = app.world_mut().query_filtered::<&TextColor, With<EmbassySlabText>>();
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

    #[test]
    fn peace_hides_lamp() {
        let hour = HourSacred::default();
        assert_eq!(hour.hex(), HexFlag::Peace);
        let yard = EmbassyYard::default();
        assert!(!yard.embassy.lamp_live);
        let mut embassy = yard.embassy.clone();
        assert!(try_request_embassy_seat(&hour, &ProofPack::default(), &mut embassy).is_none());
    }

    /// Playtest H3-SEAT: lamp after Proof Pack, E Request seat, Hour three held.
    #[test]
    fn h3_seat_e_requests_book() {
        use crate::hour_sacred::{
            try_mark_hour_three_held, try_mark_hour_two_held, try_plant_house, try_ridge_tab,
        };
        use shared::fabricator::Fabricator;
        use shared::space_law::SpaceSession;
        use shared::vertical_factory::VerticalFactory;

        let mut hour = HourSacred {
            session: SpaceSession::default(),
            complete: false,
            hour_three_complete: false,
        };
        assert!(try_ridge_tab(&mut hour, true));
        let mut factory = VerticalFactory::default();
        assert!(try_plant_house(&mut hour, &mut factory));
        assert!(try_mark_hour_two_held(&mut hour, true, true));

        let mut fab = Fabricator::default();
        assert_eq!(fab.craft_next(), "planted");
        assert_eq!(fab.craft_next(), "crafted");
        assert_eq!(fab.craft_next(), "unlocked");
        assert!(fab.pack.unlocked());

        let mut embassy = Embassy::default();
        assert_eq!(
            try_request_embassy_seat(&hour, &fab.pack, &mut embassy),
            Some("seated")
        );
        assert!(embassy.lamp_live);
        assert!(embassy.seated);
        assert!(try_mark_hour_three_held(&mut hour, fab.pack.unlocked(), embassy.seated));
        assert!(hour.hour_three_complete);
        assert_eq!(
            shared::hour_two::HourTwoPack {
                complete: true,
                hour_three_complete: true,
                ..Default::default()
            }
            .line(true),
            "Hour three held · the book is yours"
        );
    }

    /// CARD FLESH-EMBASSY-LINE — lamp / E Request seat may name Heartwood.
    /// None keeps the exact slab line. Heartwood stub still hides the seat.
    /// One seat. No Online lobby. No new PlaceId.
    #[test]
    fn flesh_embassy_line_names_heartwood_legacy_when_none() {
        use crate::hex_travel::HexTravelState;
        use shared::embassy::embassy_copy_is_honest;
        use shared::fabricator::Fabricator;
        use shared::hex_travel::PlaceId;
        use shared::persona::{ONLINE_PICKER_ENABLED, STEWARD_ONLINE_YES};

        let mut waiting = Embassy::default();
        let wait = waiting.slab_line();
        assert_eq!(
            wait,
            "Embassy lamp waits on the Proof Pack · civic seat after manufacture"
        );
        assert!(embassy_display_place(&waiting).is_none());
        assert_eq!(embassy_slab_at_place(&waiting, None), wait);
        assert_eq!(
            embassy_slab_at_place(&waiting, embassy_display_place(&waiting)),
            wait
        );

        let mut fab = Fabricator::default();
        assert_eq!(fab.craft_next(), "planted");
        assert_eq!(fab.craft_next(), "crafted");
        assert_eq!(fab.craft_next(), "unlocked");
        waiting.ensure_lamp(&fab.pack);
        let request = waiting.slab_line();
        assert_eq!(
            request,
            "Embassy lamp · E Request seat · civic after manufacture proof"
        );
        assert_eq!(embassy_slab_at_place(&waiting, None), request);
        assert!(embassy_display_place(&waiting).is_none());
        let named = embassy_slab_at_place(&waiting, embassy_display_place(&waiting));
        assert_eq!(named, request);
        assert!(!named.contains("Heartwood ·"));
        assert_eq!(named.matches("E Request seat").count(), 1);
        assert_eq!(named.matches('\n').count(), 0);
        assert!(embassy_copy_is_honest(&named));
        assert!(!named.to_lowercase().contains("lobby"));
        assert!(!named.to_lowercase().contains("online"));

        assert_eq!(waiting.request_seat(), "seated");
        assert!(waiting.seated);
        let seated = waiting.slab_line();
        assert!(embassy_display_place(&waiting).is_none());
        assert_eq!(embassy_slab_at_place(&waiting, None), seated);
        assert_eq!(
            embassy_slab_at_place(&waiting, embassy_display_place(&waiting)),
            seated
        );
        assert!(seated.contains("one embassy seat"));
        assert_eq!(waiting.request_seat(), "idle", "one seat only");

        assert!(on_heartwood_stub(Some(&HexTravelState {
            current: PlaceId::Heartwood,
        })));
        assert!(!on_heartwood_stub(Some(&HexTravelState {
            current: PlaceId::Sanctuary,
        })));
        assert!(!on_heartwood_stub(Some(&HexTravelState {
            current: PlaceId::Depths,
        })));
        assert!(!on_heartwood_stub(None));

        for id in [PlaceId::Sanctuary, PlaceId::Heartwood, PlaceId::Depths] {
            let _ = match id {
                PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => 1,
            };
        }
        assert_eq!(PlaceId::Heartwood.display_name(), "Heartwood");
        assert!(!STEWARD_ONLINE_YES);
        assert!(!ONLINE_PICKER_ENABLED);
    }

    fn embassy_slab_1_bench() -> (HourSacred, FabricatorYard) {
        use crate::hour_sacred::{try_mark_hour_two_held, try_plant_house, try_ridge_tab};
        use shared::fabricator::Fabricator;
        use shared::space_law::SpaceSession;
        use shared::vertical_factory::VerticalFactory;
        let mut hour = HourSacred {
            session: SpaceSession::default(),
            complete: false,
            hour_three_complete: false,
        };
        assert!(try_ridge_tab(&mut hour, true));
        let mut vf = VerticalFactory::default();
        assert!(try_plant_house(&mut hour, &mut vf));
        assert!(try_mark_hour_two_held(&mut hour, true, true));
        let mut fab = Fabricator::default();
        for s in ["planted", "crafted", "unlocked"] {
            assert_eq!(fab.craft_next(), s);
        }
        (hour, FabricatorYard { fab, bench_glow: 0.0 })
    }

    #[test]
    fn embassy_slab_1_offer_on_sanctuary_has_no_heartwood_prefix_and_e_seats() {
        use crate::hex_travel::HexTravelState;
        use bevy::input::keyboard::{Key, KeyboardInput};
        use bevy::input::{ButtonState, InputPlugin as BevyInputPlugin};
        use shared::hex_travel::PlaceId;

        let request = "Embassy lamp · E Request seat · civic after manufacture proof";
        let (hour, fabricator) = embassy_slab_1_bench();
        for (place, want_vis, want_seat) in
            [(Some(PlaceId::Sanctuary), true, true), (Some(PlaceId::Heartwood), false, false), (None, true, true)]
        {
            let mut app = App::new();
            app.add_plugins((MinimalPlugins, BevyInputPlugin, EmbassyPlugin))
                .insert_resource(hour.clone())
                .insert_resource(fabricator.clone())
                .insert_resource(VoiceYard::default())
                .insert_resource(LedgerYard::default())
                .insert_resource(ThrivingMoments::default())
                .insert_resource(FirstHarvestEpiphany::default())
                .insert_resource(EmbassyYard::default());
            if let Some(p) = place {
                app.insert_resource(HexTravelState { current: p });
            }
            app.update();
            app.update();
            let mut vis_q = app.world_mut().query_filtered::<&Visibility, With<EmbassySlabRoot>>();
            let mut txt_q = app.world_mut().query_filtered::<&Text, With<EmbassySlabText>>();
            assert_eq!(matches!(*vis_q.single(app.world()).unwrap(), Visibility::Visible), want_vis);
            if want_vis {
                assert_eq!(txt_q.single(app.world()).unwrap().as_str(), request);
            }
            let w = Entity::PLACEHOLDER;
            for state in [ButtonState::Pressed, ButtonState::Released] {
                app.world_mut().write_message(KeyboardInput { key_code: soft_play_bindings::INTERACT, logical_key: Key::Character("e".into()), state, repeat: false, text: None, window: w });
                app.update();
                app.update();
            }
            assert_eq!(app.world().resource::<EmbassyYard>().embassy.seated, want_seat);
            assert_eq!(app.world().resource::<ThrivingMoments>().fired.contains(&ThrivingKind::FirstEmbassy), want_seat);
            if want_seat {
                let seated = app.world().resource::<EmbassyYard>().embassy.slab_line();
                let slab = txt_q.single(app.world()).unwrap().as_str();
                assert_eq!(slab, seated);
                assert!(!slab.contains("Heartwood ·"));
            }
        }
    }

    /// TOAST-VIS-NEQ-1 TV12–TV15. Last tape of (Visibility, changed) on EmbassySlabRoot; settle 2 then clear.
    #[derive(Resource, Default)] struct TvTape(Vec<(Visibility, bool)>);
    fn tv_tape(q: Query<Ref<Visibility>, With<EmbassySlabRoot>>, mut t: ResMut<TvTape>) { t.0.extend(q.iter().map(|v| (*v, v.is_changed()))); }
    fn tv_run(app: &mut App, n: usize) -> Vec<(Visibility, bool)> { for _ in 0..n { app.update(); } std::mem::take(&mut app.world_mut().resource_mut::<TvTape>().0) }
    fn tv_settle(mut app: App) -> App { app.update(); app.update(); app.world_mut().resource_mut::<TvTape>().0.clear(); app }
    fn tv_hide(app: &mut App) { let e = app.world_mut().query_filtered::<Entity, With<EmbassySlabRoot>>().single(app.world()).unwrap(); *app.world_mut().get_mut::<Visibility>(e).unwrap() = Visibility::Hidden; }
    const TV: Visibility = Visibility::Visible;
    const TH: Visibility = Visibility::Hidden;
    /// The embassy_slab_1 lamp offer on Sanctuary (bench + EmbassyPlugin); `hud` adds the real HudLayoutPlugin at InYard.
    fn tv_app(hud: Option<crate::hud_presets::HudPresetId>) -> App {
        let (hour, fabricator) = embassy_slab_1_bench();
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, bevy::input::InputPlugin, EmbassyPlugin)).insert_resource(hour).insert_resource(fabricator).insert_resource(VoiceYard::default()).insert_resource(LedgerYard::default()).insert_resource(ThrivingMoments::default()).insert_resource(FirstHarvestEpiphany::default()).insert_resource(EmbassyYard::default())
            .insert_resource(crate::hex_travel::HexTravelState { current: shared::hex_travel::PlaceId::Sanctuary }).init_resource::<TvTape>().add_systems(Last, tv_tape);
        if let Some(id) = hud { app.add_plugins(crate::hud_presets::HudLayoutPlugin).insert_resource(crate::hud_presets::ActiveHudPreset { id: Some(id) }).insert_resource(crate::title_screen::LaunchDoor::InYard).init_resource::<crate::hex_travel::PlacesPlate>(); }
        tv_settle(app)
    }
    fn tv_place(app: &mut App, current: shared::hex_travel::PlaceId) { app.insert_resource(crate::hex_travel::HexTravelState { current }); }
    #[test] fn toast_vis_neq_1_tv12_embassy_lamp_steady_visible_unmarked() { let mut app = tv_app(None); assert_eq!(tv_run(&mut app, 3), vec![(TV, false); 3], "TV12 embassy lamp steady visible"); }
    #[test] fn toast_vis_neq_1_tv13_embassy_heartwood_and_back_once_each() {
        let mut app = tv_app(None); tv_place(&mut app, shared::hex_travel::PlaceId::Heartwood);
        assert_eq!(tv_run(&mut app, 2), vec![(TH, true), (TH, false)], "TV13 embassy on Heartwood: Hidden once, then quiet");
        tv_place(&mut app, shared::hex_travel::PlaceId::Sanctuary);
        assert_eq!(tv_run(&mut app, 2), vec![(TV, true), (TV, false)], "TV13 embassy back on Sanctuary: Visible once, then quiet");
    }
    #[test] fn toast_vis_neq_1_tv14_embassy_direct_hidden_restored_once() { let mut app = tv_app(None); tv_hide(&mut app); assert_eq!(tv_run(&mut app, 2), vec![(TV, true), (TV, false)], "TV14 embassy direct Hidden write: restored once, then quiet"); }
    #[test] fn toast_vis_neq_1_tv15_embassy_real_hud_layout() {
        let mut app = tv_app(Some(crate::hud_presets::HudPresetId::Classic));
        assert_eq!(tv_run(&mut app, 3), vec![(TH, true); 3], "TV15 Classic InYard lamp live: clean-hidden, changed every frame on main and branch alike");
        let mut app = tv_app(Some(crate::hud_presets::HudPresetId::Management));
        assert_eq!(tv_run(&mut app, 3), vec![(TV, false); 3], "TV15 Management: steady Visible, quiet");
        let mut app = tv_app(Some(crate::hud_presets::HudPresetId::Minimal)); app.insert_resource(crate::title_screen::LaunchDoor::Title);
        assert!(tv_run(&mut app, 2).iter().all(|r| r.0 == TH), "TV15 Minimal off the yard door: R5 hides the band");
    }

    fn pad_app(peace: bool, hud: Option<crate::hud_presets::HudPresetId>, settle: bool) -> (App, Entity) {
        use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent};
        let (bench, fabricator) = embassy_slab_1_bench();
        let mut app = App::new();
        app.insert_resource(EmbassyYard { embassy: Embassy::default() }).insert_resource(if peace { HourSacred::default() } else { bench }).insert_resource(fabricator).insert_resource(VoiceYard::default()).insert_resource(LedgerYard { board: shared::ledger_bind::LedgerBoard::default(), sash_open: false }).insert_resource(ThrivingMoments::default()).insert_resource(FirstHarvestEpiphany::default()).insert_resource(crate::hex_travel::HexTravelState { current: shared::hex_travel::PlaceId::Sanctuary }).insert_resource(crate::title_screen::LaunchDoor::InYard).insert_resource(crate::local_settings::LocalSettingsState { inner: shared::local_settings::LocalSettings::default(), dirty: false }).add_plugins((MinimalPlugins, bevy::input::InputPlugin, crate::input::InputPlugin, EmbassyPlugin));
        if let Some(id) = hud { app.add_plugins(crate::hud_presets::HudLayoutPlugin).insert_resource(crate::hud_presets::ActiveHudPreset { id: Some(id) }).init_resource::<crate::hex_travel::PlacesPlate>(); }
        crate::test_env::single_thread(&mut app); let id = app.world_mut().spawn_empty().id();
        app.world_mut().write_message(GamepadConnectionEvent::new(id, GamepadConnection::Connected { name: "pad".into(), vendor_id: None, product_id: None })); if settle { app.update(); app.update(); } (app, id)
    }
    fn pad_btn(app: &mut App, id: Entity, button: GamepadButton, value: f32) {
        use bevy::input::gamepad::{RawGamepadButtonChangedEvent, RawGamepadEvent};
        app.world_mut().write_message(RawGamepadEvent::Button(RawGamepadButtonChangedEvent::new(id, button, value))); app.update(); app.update();
    }
    fn south_tap(app: &mut App, id: Entity) { pad_btn(app, id, GamepadButton::South, 1.0); pad_btn(app, id, GamepadButton::South, 0.0); }
    fn seated(app: &App) -> bool { app.world().resource::<EmbassyYard>().embassy.seated }
    fn fires(app: &App) -> usize { app.world().resource::<ThrivingMoments>().fired.iter().filter(|k| **k == ThrivingKind::FirstEmbassy).count() }
    fn slab(app: &mut App) -> String { app.world_mut().query_filtered::<&Text, With<EmbassySlabText>>().single(app.world()).unwrap().as_str().to_string() }
    fn vis(app: &mut App) -> Visibility { *app.world_mut().query_filtered::<&Visibility, With<EmbassySlabRoot>>().single(app.world()).unwrap() }
    fn block(name: &str, set: impl FnOnce(&mut App), clear: impl FnOnce(&mut App)) { let (mut app, id) = pad_app(false, None, true); set(&mut app); south_tap(&mut app, id); assert!(!seated(&app), "E6 {name}"); clear(&mut app); south_tap(&mut app, id); assert!(seated(&app), "E6 clear {name}"); }
    #[test] fn embassy_pad_1_e1_south_seats() { let (mut app, id) = pad_app(false, None, true); south_tap(&mut app, id); assert!(seated(&app) && fires(&app) == 1 && slab(&mut app).starts_with("Seated — one embassy seat · civic eligibility · "), "E1"); }
    #[test] fn embassy_pad_1_e2_repeat_south_fires_once() { let (mut app, id) = pad_app(false, None, true); south_tap(&mut app, id); south_tap(&mut app, id); south_tap(&mut app, id); assert!(seated(&app) && fires(&app) == 1, "E2"); }
    #[test] fn embassy_pad_1_e3_e_and_south_same_frame_once() {
        use bevy::input::{gamepad::{RawGamepadButtonChangedEvent, RawGamepadEvent}, keyboard::{Key, KeyboardInput}, ButtonState};
        let (mut app, id) = pad_app(false, None, true);
        app.world_mut().write_message(KeyboardInput { key_code: soft_play_bindings::INTERACT, logical_key: Key::Character("e".into()), state: ButtonState::Pressed, repeat: false, text: None, window: Entity::PLACEHOLDER });
        app.world_mut().write_message(RawGamepadEvent::Button(RawGamepadButtonChangedEvent::new(id, GamepadButton::South, 1.0))); app.update(); app.update();
        assert!(seated(&app) && fires(&app) == 1, "E3");
    }
    #[test] fn embassy_pad_1_e4_heartwood_south_does_not_seat() { let (mut app, id) = pad_app(false, None, true); app.insert_resource(crate::hex_travel::HexTravelState { current: shared::hex_travel::PlaceId::Heartwood }); south_tap(&mut app, id); assert!(!seated(&app), "E4"); }
    #[test] fn embassy_pad_1_e5_sash_blocks_until_clear() {
        let (mut app, id) = pad_app(false, None, true);
        app.world_mut().resource_mut::<VoiceYard>().sash_open = true; south_tap(&mut app, id); assert!(!seated(&app), "E5 voice");
        app.world_mut().resource_mut::<LedgerYard>().sash_open = true; south_tap(&mut app, id); assert!(!seated(&app), "E5 ledger");
        app.world_mut().resource_mut::<VoiceYard>().sash_open = false; app.world_mut().resource_mut::<LedgerYard>().sash_open = false; south_tap(&mut app, id); assert!(seated(&app), "E5 clear");
    }
    #[test] fn embassy_pad_1_e6_pad_live_blockers() {
        use crate::title_screen::LaunchDoor::*;
        for door in [NameHouse, HouseDress, Title] { let kept = door; block(&format!("{door:?}"), move |a| { a.insert_resource(kept); }, |a| { a.insert_resource(InYard); }); }
        block("settings", |a| { a.insert_resource(crate::title_screen::HouseLabel { house: shared::house_name::HouseName::default(), persist_present: false, hour_two_held: false, book_held: false, settings_open: true, draft: String::new(), naming_offered: false, seals_offered: false }); }, |a| { a.world_mut().resource_mut::<crate::title_screen::HouseLabel>().settings_open = false; });
        block("places", |a| { let mut p = crate::hex_travel::PlacesPlate::default(); p.open = true; a.insert_resource(p); }, |a| { a.world_mut().resource_mut::<crate::hex_travel::PlacesPlate>().open = false; });
        block("licences", |a| { let mut s = crate::licences_screen::LicencesScreen::default(); s.open = true; a.insert_resource(s); }, |a| { a.world_mut().resource_mut::<crate::licences_screen::LicencesScreen>().open = false; });
        block("edit", |a| { let mut edit = crate::hud_edit_mode::HudEditMode::default(); edit.active = true; a.insert_resource(edit); }, |a| a.world_mut().resource_mut::<crate::hud_edit_mode::HudEditMode>().active = false);
    }
    #[test] fn embassy_pad_1_e7_south_off_and_lb_do_not_seat() { let (mut app, id) = pad_app(false, None, true); app.world_mut().resource_mut::<crate::local_settings::LocalSettingsState>().inner.gamepad_south_use = false; south_tap(&mut app, id); assert!(!seated(&app), "E7 south"); pad_btn(&mut app, id, GamepadButton::LeftTrigger, 1.0); pad_btn(&mut app, id, GamepadButton::LeftTrigger, 0.0); assert!(!seated(&app), "E7 lb"); }
    #[test] fn embassy_pad_1_e8_other_buttons_do_not_seat() { let (mut app, id) = pad_app(false, None, true); for b in [GamepadButton::East, GamepadButton::West, GamepadButton::North, GamepadButton::Start, GamepadButton::Select, GamepadButton::DPadUp, GamepadButton::DPadDown, GamepadButton::DPadLeft, GamepadButton::DPadRight, GamepadButton::LeftTrigger, GamepadButton::RightTrigger, GamepadButton::RightTrigger2, GamepadButton::LeftThumb, GamepadButton::RightThumb] { pad_btn(&mut app, id, b, 1.0); pad_btn(&mut app, id, b, 0.0); assert!(!seated(&app), "E8 {b:?}"); } }
    #[test] fn embassy_pad_1_e9_peace_south_leaves_lamp_dark() { let (mut app, id) = pad_app(true, None, true); south_tap(&mut app, id); let e = &app.world().resource::<EmbassyYard>().embassy; assert!(!e.seated && !e.lamp_live, "E9"); }
    #[test] fn embassy_pad_1_e10_lamp_clears_the_update_after_seat() { let (mut app, id) = pad_app(false, None, true); assert!(app.world().resource::<FirstHarvestEpiphany>().embassy_lamp, "E10 before"); south_tap(&mut app, id); assert!(!app.world().resource::<FirstHarvestEpiphany>().embassy_lamp, "E10 after"); }
    #[test] fn embassy_pad_1_e11_after_input_before_raise() {
        use bevy::ecs::schedule::{IntoSystemSet, NodeId, Schedules};
        use bevy::ecs::system::{IntoSystem, System};
        let (app, _) = pad_app(false, None, false);
        let graph = app.world().resource::<Schedules>().get(Update).expect("Update").graph();
        let key = graph.systems.iter().find_map(|(k, s, _)| (System::system_type(&**s) == IntoSystem::system_type_id(&handle_embassy)).then_some(k)).expect("handle_embassy");
        let (map, raise) = (graph.system_sets.get_key(crate::input::InputMapSet.intern()), graph.system_sets.get_key(IntoSystemSet::into_system_set(crate::feel_move::raise_buffered_use).intern()));
        let dep = graph.dependency().graph();
        assert!(map.is_some_and(|set| dep.contains_edge(NodeId::Set(set), NodeId::System(key))), "E11 after");
        assert!(raise.is_some_and(|set| dep.contains_edge(NodeId::System(key), NodeId::Set(set))), "E11 before");
    }
    #[test] fn embassy_pad_1_e12_hud_parity() {
        use bevy::input::gamepad::{RawGamepadButtonChangedEvent, RawGamepadEvent};
        let (mut classic, id) = pad_app(false, Some(crate::hud_presets::HudPresetId::Classic), true);
        for v in [1.0f32, 0.0] { classic.world_mut().write_message(RawGamepadEvent::Button(RawGamepadButtonChangedEvent::new(id, GamepadButton::South, v))); classic.update(); assert_eq!(vis(&mut classic), Visibility::Hidden, "E12 classic"); classic.update(); assert_eq!(vis(&mut classic), Visibility::Hidden, "E12 classic"); }
        assert!(seated(&classic), "E12 classic seated");
        let (mut mgmt, id) = pad_app(false, Some(crate::hud_presets::HudPresetId::Management), true); south_tap(&mut mgmt, id);
        let line = slab(&mut mgmt); let show = vis(&mut mgmt); let want = mgmt.world().resource::<EmbassyYard>().embassy.slab_line();
        assert!(seated(&mgmt) && show == Visibility::Visible && line == want && line.starts_with("Seated — one embassy seat · civic eligibility · "), "E12 management {line}");
    }
}
