//! Lived-hour fabricator — Slice 7 (v23.2.11) + P2 week answer
//!
//! After House arrival, Q plants a fabricator then runs MendSpool and LaneCrate.
//! Mend/Lane refresh the week audit slab (tons + restored). Soft bench light (S+). Dies in Peace.
//!
//! CARD FLESH-FABRICATOR-LINE — the existing planted / Proof Pack / MendSpool /
//! LaneCrate line may name the Place stood in (Sanctuary / Heartwood /
//! Threshold-near / Depths). Threshold-near is Heartwood plus shelf reach.
//! `None` keeps the exact slab. Civic proof of place, not gear. No gold.
//! No Market. One string, one slab. Online grey. Bevy pin 0.14.
//!
//! CARD TEMPER-VISIBLE-1 — a stocked MendSpool names Digit1 on this slab.
//! Empty spool does not name a craft. No new recipe. No gold.
//!
//! CARD WOOD-USE-1 — once Q is idle after Proof Pack, Q refines the carried wood
//! unit into one stocked LaneCrate. No `on_lane` / `on_mend`: tons are the haul leg's.
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::fabricator::Fabricator;
use shared::hex_travel::PlaceId;
use shared::hour_two::HourTwoPack;
use shared::space_law::HexFlag;

use crate::hex_travel::HexTravelState;
use crate::hud_anchor_registry::{HudSlab, FAB};
use crate::hour_sacred::{read_hour_two_json, HourSacred};
use crate::human_presence::SoftPresence;
use crate::lived_hour_bind::LivedHourBind;
use crate::soft_play_bindings;
use crate::input::{InputMapSet, PlayerInput};
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};
use crate::vertical_factory::FactoryYard;
use crate::title_screen::{LaunchDoor, TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};
use crate::wood_expedition::WoodHub;

#[derive(Resource, Debug, Clone)]
pub struct FabricatorYard {
    pub fab: Fabricator,
    /// Soft bench light after MendSpool / LaneCrate (S+). Not a HUD.
    pub bench_glow: f32,
}

impl Default for FabricatorYard {
    fn default() -> Self {
        if let Some(raw) = read_hour_two_json() {
            return Self {
                fab: HourTwoPack::from_json(&raw).fabricator,
                bench_glow: 0.0,
            };
        }
        Self {
            fab: Fabricator::default(),
            bench_glow: 0.0,
        }
    }
}

#[derive(Component)]
struct FabSlabRoot;
#[derive(Component)]
pub(crate) struct FabSlabText;

pub struct FabricatorPlugin;

impl Plugin for FabricatorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FabricatorYard>()
            .add_systems(Startup, spawn_fab_slab)
            .add_systems(Update, (handle_fab_q, tick_bench_glow, update_fab_slab).after(InputMapSet));
    }
}

fn spawn_fab_slab(mut commands: Commands) {
    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: FAB.top(),
                    left: FAB.left(),
                    width: Val::Px(520.0),
                    margin: FAB.margin(),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG.with_alpha(1.0)),
                BorderColor::all(TITLE_BORDER.with_alpha(1.0)),
                Visibility::Hidden,
            ),
            FabSlabRoot,
            HudSlab(FAB.id),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(""),
TextFont { font_size: FontSize::Px(14.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                FabSlabText,
            ));
        });
}

/// Q after House arrival plants/crafts the Proof Pack. Peace no-op.
pub fn try_craft_proof_pack(
    hour: &HourSacred,
    tutorial_complete: bool,
    fab: &mut Fabricator,
) -> Option<&'static str> {
    if hour.hex() == HexFlag::Peace || !hour.charter_skin_live() {
        return None;
    }
    if !tutorial_complete {
        return None;
    }
    Some(fab.craft_next())
}

/// Wood is taken only after `craft_next` reports idle, so the civic first LaneCrate never needs it.
fn refine_wood_when_idle(step: &str, fab: &mut Fabricator, wood: Option<&mut WoodHub>) -> bool {
    match wood {
        Some(wood) if step == "idle" && fab.planted && fab.pack.unlocked() => {
            fab.refine_wood(wood.take_unit()) == "refined"
        }
        _ => false,
    }
}

/// CARD FLESH-FABRICATOR-LINE — spoken room while standing.
/// Threshold-near is Heartwood plus existing shelf reach, not a PlaceId.
fn fabricator_stood_place_name(place: PlaceId, threshold_near: bool) -> &'static str {
    match place {
        PlaceId::Sanctuary => "Sanctuary",
        PlaceId::Depths => "Depths",
        PlaceId::Heartwood if threshold_near => "Threshold-near",
        PlaceId::Heartwood => "Heartwood",
    }
}

/// Place label when travel is already in the world. None until that state exists.
fn fabricator_stood_place_label(
    travel: Option<&HexTravelState>,
    presence: Option<&SoftPresence>,
) -> Option<&'static str> {
    let travel = travel?;
    let near = presence.is_some_and(|body| {
        shared::threshold_shelf::threshold_use_in_reach(
            travel.current,
            body.position.x,
            body.position.z,
        )
    });
    Some(fabricator_stood_place_name(travel.current, near))
}

/// Planted / Proof Pack / MendSpool / LaneCrate copy — not Tend Hook, not a thin Reserve.
fn fabricator_line_may_name_place(line: &str) -> bool {
    if line.contains("too thin") {
        return false;
    }
    line.contains("Fabricator live")
        || line.contains("MendSpool ran")
        || line.contains("LaneCrate ran")
        || line.contains("Proof Pack")
        || line.contains("Q plant a fabricator")
}

fn fabricator_action_line(line: &str) -> bool {
    line.contains("Fabricator live")
        || line.contains("MendSpool ran")
        || line.contains("LaneCrate ran")
        || line.starts_with("Proof Pack")
}

/// Steady plate, or the short civic sentence after plant / MendSpool / LaneCrate / Proof Pack.
/// No Place yet → the exact `slab_line` (legacy).
fn fabricator_spoken_body(fab: &Fabricator, place: Option<&str>) -> String {
    if place.is_some() && fabricator_action_line(&fab.last_line) {
        fab.last_line.clone()
    } else {
        fab.slab_line()
    }
}

/// CARD FLESH-FABRICATOR-LINE — planted / Proof Pack / MendSpool / LaneCrate
/// may prefix the stood Place. `None` returns the body unchanged.
/// One string, one slab. Civic proof, not gear.
pub fn fabricator_line_at_place(line: &str, place: Option<&str>) -> String {
    match place {
        Some(place) if fabricator_line_may_name_place(line) => format!("{place} · {line}"),
        _ => line.to_string(),
    }
}

pub fn fabricator_slab_at_place(fab: &Fabricator, place: Option<&str>) -> String {
    let body = fabricator_spoken_body(fab, place);
    let line = fabricator_line_at_place(&body, place);
    append_digit1_when_mend_spool_stocked(fab, line)
}

/// CARD TEMPER-VISIBLE-1 — Digit1 is the Tend Hook key already on the bench.
/// Name it only while a MendSpool is stocked and the temper recipe is visible.
/// Empty spool keeps the civic sentence and does not name a craft.
fn append_digit1_when_mend_spool_stocked(fab: &Fabricator, line: String) -> String {
    if !mend_spool_names_digit1(fab) || line.contains("Digit1") {
        return line;
    }
    format!("{line} · Digit1")
}

fn mend_spool_names_digit1(fab: &Fabricator) -> bool {
    fab.planted && fab.spool_stock > 0 && shared::fabricator::tend_hook_recipe_visible()
}

fn handle_fab_q(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_input: Res<PlayerInput>,
    hour: Res<HourSacred>,
    factory: Res<FactoryYard>,
    mut yard: ResMut<FabricatorYard>,
    mut bind: ResMut<LivedHourBind>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
    mut wood: Option<ResMut<WoodHub>>,
) {
    // Q / pad West (same sheet verb).
    if !(keyboard.just_pressed(soft_play_bindings::BUILD_WHEEL) || player_input.sheet_q) {
        return;
    }
    let had_repair = yard.fab.pack.repair;
    let had_logi = yard.fab.pack.logi;
    let Some(step) = try_craft_proof_pack(&hour, factory.factory.tutorial_complete(), &mut yard.fab) else {
        return;
    };
    if refine_wood_when_idle(step, &mut yard.fab, wood.as_deref_mut()) {
        yard.bench_glow = 1.0;
    }
    if yard.fab.pack.repair && !had_repair {
        yard.bench_glow = 1.0;
        bind.climate.on_mend();
        bind.standing.on_mend();
        // P2: prefer week line after restored↑ — do not mute with standing-only clause.
        bind.refresh_climate_slab();
        bind.persist();
    }
    if yard.fab.pack.logi && !had_logi {
        yard.bench_glow = 1.0;
        bind.climate.on_lane();
        bind.standing.on_lane();
        // P2: prefer week line after tons↑ — stranger path hears the audit.
        bind.refresh_climate_slab();
        bind.persist();
    }
    if step == "unlocked" {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstProofPack,
            time.elapsed_secs_f64(),
        );
    }
}

fn tick_bench_glow(time: Res<Time>, mut yard: ResMut<FabricatorYard>) {
    if yard.bench_glow > 0.0 {
        yard.bench_glow = (yard.bench_glow - time.delta_secs() * 0.55).max(0.0);
    }
}

/// LOOP-TEACH-1b — Sanctuary yard, stocked crate, Places live.
pub const LOOP_HAUL_CUE: &str = "Esc · Places · haul the crate";

#[rustfmt::skip]
fn haul_cue_live(crate_stock: u32, complete: bool, hour_three_complete: bool, place: Option<PlaceId>, door: Option<LaunchDoor>) -> bool {
    crate_stock > 0 && shared::hex_travel::places_eligible(complete, hour_three_complete) && place == Some(PlaceId::Sanctuary) && door == Some(LaunchDoor::InYard)
}

#[allow(clippy::too_many_arguments)] // LaunchDoor is the yard gate for the haul cue
fn update_fab_slab(
    hour: Res<HourSacred>,
    factory: Res<FactoryYard>,
    yard: Res<FabricatorYard>,
    travel: Option<Res<HexTravelState>>,
    presence: Option<Res<SoftPresence>>,
    door: Option<Res<LaunchDoor>>,
    mut root: Query<(&mut Visibility, &mut BorderColor, &mut BackgroundColor), With<FabSlabRoot>>,
    mut text_q: Query<&mut Text, With<FabSlabText>>,
) {
    let show = hour.charter_skin_live() && factory.factory.tutorial_complete();
    let glow = yard.bench_glow;
    for (mut vis, mut border, mut bg) in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        if show {
            // VP-SLABS-REGAL-1 (#643 pattern): rest on the title palette; the
            // bench_glow lift rides in rgb (capped per channel), rim alpha 1.0.
            let rim = TITLE_BORDER.to_srgba();
            let plate = TITLE_PLATE_BG.to_srgba();
            *border = Color::srgba(
                (rim.red + glow * 0.18).min(1.0),
                (rim.green + glow * 0.22).min(1.0),
                rim.blue, // no blue lift on the rim, as before
                1.0,
            )
            .into();
            *bg = Color::srgba(
                (plate.red + glow * 0.10).min(1.0),
                (plate.green + glow * 0.08).min(1.0),
                (plate.blue + glow * 0.12).min(1.0),
                1.0,
            )
            .into();
        }
    }
    if !show {
        return;
    }
    let place = fabricator_stood_place_label(travel.as_deref(), presence.as_deref());
    let mut line = fabricator_slab_at_place(&yard.fab, place);
    if haul_cue_live(yard.fab.crate_stock, hour.complete, hour.hour_three_complete, travel.as_deref().map(|t| t.current), door.as_deref().copied()) { line = format!("{line} · {LOOP_HAUL_CUE}"); }
    for mut text in &mut text_q {
        if text.as_str() != line {
            **text = line.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// CARD HUD-ANCHOR-REGISTRY-2B — Fab position is the registry, Style is the coded literal.
    #[test]
    fn fab_style_byte_identical_to_coded_place() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_fab_slab);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<&Node, With<FabSlabRoot>>();
        let style = q.single(app.world()).unwrap().clone();
        let coded = Node {
            position_type: PositionType::Absolute,
            top: Val::Px(88.0),
            left: Val::Percent(50.0),
            width: Val::Px(520.0),
            margin: UiRect::left(Val::Px(-260.0)),
            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
            justify_content: JustifyContent::Center,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };
        assert_eq!(style, coded);
        assert_eq!(style.top, FAB.top());
        assert_eq!(style.left, FAB.left());
        assert_eq!(style.margin, FAB.margin());
        assert_eq!(style.width, Val::Px(FAB.width));
    }

    /// CARD VP-SLABS-REGAL-1 — the fabricator slab rests on the title palette: opaque
    /// TITLE_PLATE_BG plate, TITLE_BORDER rim at alpha 1, TITLE_TEXT_PRIMARY text.
    #[test]
    fn slabs_regal_fab_slab_rests_on_title_palette() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_fab_slab);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<(&BorderColor, &BackgroundColor), With<FabSlabRoot>>();
        let (border, bg) = q.single(app.world()).unwrap();
        let (border, bg) = (({
            assert_eq!(border.top, border.right, "border edges");
            assert_eq!(border.top, border.bottom, "border edges");
            assert_eq!(border.top, border.left, "border edges");
            border.top
        }).to_srgba(), bg.0.to_srgba());
        let mut t = app.world_mut().query_filtered::<&TextColor, With<FabSlabText>>();
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

    /// Fab slab app with the slab showing (Hour two held, charter skin live,
    /// factory tutorial complete) at the given bench_glow.
    fn slabs_regal_fab_app(glow: f32) -> App {
        use crate::hour_sacred::{try_mark_hour_two_held, try_plant_house, try_ridge_tab};
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
        while !factory.tutorial_complete() {
            if factory.advance() == "unfounded" {
                break;
            }
        }
        assert!(hour.complete && hour.charter_skin_live() && factory.tutorial_complete());
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .insert_resource(hour)
            .insert_resource(FactoryYard { factory })
            .insert_resource(FabricatorYard {
                fab: Fabricator::default(),
                bench_glow: glow,
            })
            .add_systems(Startup, spawn_fab_slab)
            .add_systems(Update, update_fab_slab);
        app.update();
        app
    }

    fn slabs_regal_fab_colors(app: &mut App) -> (Srgba, Srgba, Visibility) {
        let mut q = app
            .world_mut()
            .query_filtered::<(&BorderColor, &BackgroundColor, &Visibility), With<FabSlabRoot>>();
        let (border, bg, vis) = q.single(app.world()).unwrap();
        (({
            assert_eq!(border.top, border.right, "border edges");
            assert_eq!(border.top, border.bottom, "border edges");
            assert_eq!(border.top, border.left, "border edges");
            border.top
        }).to_srgba(), bg.0.to_srgba(), *vis)
    }

    /// CARD VP-SLABS-REGAL-1 — the bench_glow pulse rides in rgb on the gold
    /// rest base: at rest the shown slab is the palette, at peak the rim rgb
    /// differs (capped at 1.0) and rim and plate alpha stay 1.0.
    #[test]
    fn slabs_regal_fab_pulse_peak_rim_rgb_differs_from_rest() {
        let mut app = slabs_regal_fab_app(0.0);
        let (rest, rest_bg, vis) = slabs_regal_fab_colors(&mut app);
        assert_eq!(vis, Visibility::Visible);
        let rim = TITLE_BORDER.to_srgba();
        let plate = TITLE_PLATE_BG.to_srgba();
        assert!((rest.red - rim.red).abs() < 1e-6);
        assert!((rest.green - rim.green).abs() < 1e-6);
        assert!((rest.blue - rim.blue).abs() < 1e-6);
        assert!((rest_bg.red - plate.red).abs() < 1e-6);
        assert!((rest_bg.green - plate.green).abs() < 1e-6);
        assert!((rest_bg.blue - plate.blue).abs() < 1e-6);
        assert_eq!(rest.alpha, 1.0);
        assert_eq!(rest_bg.alpha, 1.0);

        let mut app = slabs_regal_fab_app(1.0);
        let (peak, peak_bg, _) = slabs_regal_fab_colors(&mut app);
        let moved = (peak.red - rest.red).abs()
            + (peak.green - rest.green).abs()
            + (peak.blue - rest.blue).abs();
        assert!(moved > 1e-3, "peak rim rgb must differ from rest");
        assert!((peak.red - (rim.red + 0.18).min(1.0)).abs() < 1e-6);
        assert!((peak.green - (rim.green + 0.22).min(1.0)).abs() < 1e-6);
        assert!(peak.red <= 1.0 && peak.green <= 1.0 && peak.blue <= 1.0);
        assert_eq!(peak.alpha, 1.0, "rim alpha stays 1.0 under the pulse");
        assert_eq!(peak_bg.alpha, 1.0, "plate alpha stays 1.0 under the pulse");
    }

    #[test]
    fn peace_does_not_plant() {
        let hour = HourSacred::default();
        assert_eq!(hour.hex(), HexFlag::Peace);
        let yard = FabricatorYard::default();
        assert!(!yard.fab.planted);
        let mut fab = yard.fab.clone();
        assert!(try_craft_proof_pack(&hour, true, &mut fab).is_none());
        assert!(!fab.planted);
    }

    /// Playtest H3-FAB: after Hour two held + crate arrival, Q plants then Proof Pack.
    #[test]
    fn h3_fab_q_after_arrival_unlocks_proof_pack() {
        use crate::hour_sacred::{try_mark_hour_two_held, try_plant_house, try_ridge_tab};
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
        assert!(
            try_craft_proof_pack(&hour, false, &mut fab).is_none(),
            "crate must arrive first"
        );
        while !factory.tutorial_complete() {
            let step = factory.advance();
            if step == "unfounded" {
                break;
            }
        }
        assert!(factory.tutorial_complete());
        assert_eq!(try_craft_proof_pack(&hour, true, &mut fab), Some("planted"));
        assert_eq!(try_craft_proof_pack(&hour, true, &mut fab), Some("crafted"));
        assert_eq!(try_craft_proof_pack(&hour, true, &mut fab), Some("unlocked"));
        assert!(fab.pack.unlocked());
        let line = fab.pack.line();
        assert!(line.contains("Proof Pack"));
        assert!(!line.to_lowercase().contains("loot"));
        assert!(!line.to_lowercase().contains("gold"));
    }

    /// CARD FLESH-FABRICATOR-LINE — planted / Proof Pack / MendSpool / LaneCrate
    /// may name the stood Place. None keeps the exact slab. Civic proof, not gear.
    /// Idle / Glowing / Tended / Resting / Stressed stay well words. No new PlaceId.
    #[test]
    fn flesh_fabricator_line_names_place_legacy_when_none() {
        use shared::climate_node::NodeState;
        use shared::fabricator::manufacture_copy_is_honest;
        use shared::persona::STEWARD_ONLINE_YES;

        let mut fab = Fabricator::default();
        let plant_prompt = fab.slab_line();
        assert!(plant_prompt.contains("plant a fabricator"));
        assert!(fabricator_stood_place_label(None, None).is_none());
        assert_eq!(fabricator_slab_at_place(&fab, None), plant_prompt);
        assert_eq!(fabricator_line_at_place(&plant_prompt, None), plant_prompt);

        assert_eq!(fab.craft_next(), "planted");
        let planted = fab.last_line.clone();
        assert!(planted.contains("Fabricator live"));
        assert!(planted.contains("MendSpool"));
        assert!(planted.contains("LaneCrate"));
        assert_eq!(fabricator_slab_at_place(&fab, None), fab.slab_line());
        let named_plant = fabricator_slab_at_place(&fab, Some("Sanctuary"));
        assert_eq!(named_plant, format!("Sanctuary · {planted}"));
        assert_eq!(named_plant.matches('\n').count(), 0);

        assert_eq!(fab.craft_next(), "crafted");
        let mend = fab.last_line.clone();
        assert!(mend.contains("MendSpool ran"));
        assert_eq!(
            fabricator_slab_at_place(&fab, None),
            format!("{} · Digit1", fab.slab_line())
        );
        let named_mend = fabricator_line_at_place(&mend, Some("Heartwood"));
        assert_eq!(named_mend, format!("Heartwood · {mend}"));

        assert_eq!(fab.craft_next(), "unlocked");
        let proof = fab.last_line.clone();
        assert!(proof.contains("Proof Pack"));
        assert!(fab.pack.unlocked());
        assert_eq!(
            fabricator_slab_at_place(&fab, None),
            format!("{} · Digit1", fab.slab_line())
        );
        let named_proof = fabricator_slab_at_place(&fab, Some("Depths"));
        assert_eq!(named_proof, format!("Depths · {proof} · Digit1"));

        let lane = "LaneCrate ran — the lane holds a crate";
        let named_lane = fabricator_line_at_place(lane, Some("Threshold-near"));
        assert_eq!(named_lane, format!("Threshold-near · {lane}"));

        for place in ["Sanctuary", "Heartwood", "Threshold-near", "Depths"] {
            for sample in [&planted, &mend, &proof, &plant_prompt] {
                let named = fabricator_line_at_place(sample, Some(place));
                assert!(named.starts_with(place));
                assert_eq!(named.matches(place).count(), 1);
                assert!(manufacture_copy_is_honest(&named), "{named}");
                let low = named.to_lowercase();
                assert!(!low.contains("gold"));
                assert!(!low.contains("market"));
                assert!(!low.contains("loot"));
                assert!(!low.contains("gear"));
            }
        }

        assert_eq!(
            fabricator_line_at_place("Tend Hook crafted · care · Temper +0", Some("Sanctuary")),
            "Tend Hook crafted · care · Temper +0"
        );
        assert_eq!(
            fabricator_line_at_place(
                "Repair-rights Reserve too thin to plant a fabricator",
                Some("Sanctuary")
            ),
            "Repair-rights Reserve too thin to plant a fabricator"
        );

        assert_eq!(
            fabricator_stood_place_name(PlaceId::Sanctuary, false),
            "Sanctuary"
        );
        assert_eq!(
            fabricator_stood_place_name(PlaceId::Heartwood, false),
            "Heartwood"
        );
        assert_eq!(
            fabricator_stood_place_name(PlaceId::Heartwood, true),
            "Threshold-near"
        );
        assert_eq!(fabricator_stood_place_name(PlaceId::Depths, false), "Depths");
        assert_eq!(fabricator_stood_place_name(PlaceId::Depths, true), "Depths");

        for (state, word) in [
            (NodeState::Idle, "Idle"),
            (NodeState::Glowing, "Glowing"),
            (NodeState::Tended, "Tended"),
            (NodeState::Resting, "Resting"),
            (NodeState::Stressed, "Stressed"),
        ] {
            assert_eq!(state.label(), word);
            assert_ne!(planted, word);
            assert_ne!(mend, word);
            assert_ne!(proof, word);
        }

        for id in [PlaceId::Sanctuary, PlaceId::Heartwood, PlaceId::Depths] {
            assert_ne!(id.display_name(), "Threshold-near");
            let _ = match id {
                PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => 1,
            };
        }
        assert!(!STEWARD_ONLINE_YES);
    }

    /// CARD TEMPER-VISIBLE-1 — stocked MendSpool names Digit1.
    /// Empty spool does not name a craft. Digit1 still refuses without a spool.
    #[test]
    fn temper_visible_digit1_names_stocked_spool_and_refuses_empty() {
        use shared::fabricator::{manufacture_copy_is_honest, Recipe};
        use shared::temper::ToolTier;

        let mut fab = Fabricator::default();
        let empty = fabricator_slab_at_place(&fab, None);
        assert!(!slab_names_a_craft(&empty), "{empty}");
        assert_eq!(fab.craft_tend_hook(1, "hands"), Err("unplanted"));

        assert_eq!(fab.plant(), "planted");
        let planted_empty = fabricator_slab_at_place(&fab, Some("Sanctuary"));
        assert!(planted_empty.starts_with("Sanctuary · "));
        assert!(!slab_names_a_craft(&planted_empty), "{planted_empty}");
        assert_eq!(fab.craft_tend_hook(1, "hands"), Err("missing_spool"));
        assert!(fab.last_tempered.is_none());
        assert_eq!(fab.spool_stock, 0);
        let still_empty = fabricator_slab_at_place(&fab, None);
        assert!(!slab_names_a_craft(&still_empty), "{still_empty}");

        assert_eq!(fab.craft(Recipe::MendSpool), "crafted");
        assert_eq!(fab.spool_stock, 1);
        let stocked = fabricator_slab_at_place(&fab, None);
        assert!(stocked.contains("Digit1"), "{stocked}");
        assert_eq!(stocked.matches("Digit1").count(), 1);
        assert!(stocked.starts_with(&fab.slab_line()));
        assert!(manufacture_copy_is_honest(&stocked), "{stocked}");
        assert!(!stocked.to_lowercase().contains("gold"));
        let named = fabricator_slab_at_place(&fab, Some("Heartwood"));
        assert_eq!(named, format!("Heartwood · {} · Digit1", fab.last_line));
        assert_eq!(named.matches('\n').count(), 0);
        assert_eq!(named.matches("Digit1").count(), 1);
        assert!(manufacture_copy_is_honest(&named), "{named}");

        let item = fab.craft_tend_hook(1, "hands").expect("hook");
        assert_eq!(item.tier, ToolTier::TendHook);
        assert_eq!(fab.spool_stock, 0);
        let spent = fabricator_slab_at_place(&fab, Some("Heartwood"));
        assert!(!slab_names_a_craft(&spent), "{spent}");
        assert_eq!(fab.craft_tend_hook(2, "hands"), Err("missing_spool"));
    }

    fn slab_names_a_craft(line: &str) -> bool {
        line.contains("Digit1")
            || line.contains("Tend Hook")
            || line
                .split(|c: char| !c.is_ascii_alphanumeric())
                .any(|word| word.eq_ignore_ascii_case("craft"))
    }

    use crate::wood_expedition::{tests::carried_hub, WoodHub};
    /// `shared/shard_climate.rs:170` legacy climate bytes.
    const CLIMATE_V0: &str = r#"{"hex_id":"local-hex","harmony":0.55,"stress":0.15,"regen":0.08,"reserve_pool":0,"restored_count":0,"tons_moved":0,"updated_at":1}"#;
    fn crate_stock(fab: &Fabricator) -> u64 {
        serde_json::to_value(fab).unwrap()["crate_stock"].as_u64().unwrap_or(0)
    }

    /// H1: MinimalPlugins with only `handle_fab_q`, the legacy climate, and the wood hub.
    fn q_app(hold: bool, steps: usize, hub: WoodHub) -> (App, Fabricator) {
        use crate::hour_sacred::{try_mark_hour_two_held, try_plant_house, try_ridge_tab};
        use shared::vertical_factory::VerticalFactory;
        let mut hour = HourSacred::default();
        let mut factory = VerticalFactory::default();
        if hold {
            assert!(try_ridge_tab(&mut hour, true) && try_plant_house(&mut hour, &mut factory));
            assert!(try_mark_hour_two_held(&mut hour, true, true));
        }
        while !factory.tutorial_complete() && factory.advance() != "unfounded" {}
        let mut fab = Fabricator::default();
        for _ in 0..steps { fab.craft_next(); }
        let mut bind = crate::loot_drops::tests::in_user_dir("wu5-bind", |_| LivedHourBind::default());
        bind.climate = shared::shard_climate::ShardClimate::from_json(CLIMATE_V0).unwrap();
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<PlayerInput>()
            .init_resource::<ThrivingMoments>()
            .insert_resource(hour)
            .insert_resource(FactoryYard { factory })
            .insert_resource(FabricatorYard { fab: fab.clone(), bench_glow: 0.0 })
            .insert_resource(bind)
            .insert_resource(hub)
            .add_systems(Update, handle_fab_q);
        (app, fab)
    }

    fn press_q(app: &mut App) {
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(soft_play_bindings::BUILD_WHEEL);
        app.insert_resource(keys).update();
    }

    /// WU5 (H1 + H3) — Q after Proof Pack, not Peace, with wood unit 1 refines one LaneCrate and
    /// takes the unit. No `on_lane` / `on_mend`: climate, standing and week stay as loaded. Peace refines nothing.
    #[test]
    fn wu5_q_after_proof_pack_refines_wood_without_tons() {
        let (hub, live) = carried_hub();
        let (mut app, fab) = q_app(true, 3, hub);
        let bind = app.world().resource::<LivedHourBind>().clone();
        press_q(&mut app);
        let yard = &app.world().resource::<FabricatorYard>().fab;
        assert_eq!((crate_stock(yard), yard.last_line.as_str()), (1, "LaneCrate ran — the lane holds a crate"));
        assert_eq!((yard.pack.clone(), yard.spool_stock, yard.reserve), (fab.pack.clone(), fab.spool_stock, fab.reserve));
        assert!(fabricator_slab_at_place(yard, None).ends_with(" · Digit1"));
        let pack = HourTwoPack { fabricator: yard.clone(), ..Default::default() };
        assert_eq!(crate_stock(&HourTwoPack::from_json(&serde_json::to_string(&pack).unwrap()).fabricator), 1);
        assert_eq!(app.world().resource::<WoodHub>().memory_unit(), Some(0));
        let now = app.world().resource::<LivedHourBind>();
        assert_eq!((&now.climate, &now.standing, &now.week), (&bind.climate, &bind.standing, &bind.week));
        assert_eq!((now.climate.tons_moved, now.climate.restored_count), (0, 0));
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
        while live() == crate::wood_expedition::tests::HARVESTED && std::time::Instant::now() < deadline {
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
        let disk: serde_json::Value = serde_json::from_slice(&live()).unwrap();
        assert_eq!((&disk["schema"], &disk["stand"], &disk["unit"], &disk["charge"]), (&1.into(), &"Depleted".into(), &0.into(), &"Spent".into()));
        press_q(&mut app);
        assert_eq!(crate_stock(&app.world().resource::<FabricatorYard>().fab), 1, "no wood, no second crate");
        app.world_mut().resource_mut::<WoodHub>().shutdown();

        let (mut peace, _) = q_app(false, 3, carried_hub().0);
        press_q(&mut peace);
        assert_eq!(crate_stock(&peace.world().resource::<FabricatorYard>().fab), 0);
        assert_eq!(peace.world().resource::<WoodHub>().memory_unit(), Some(1));
        peace.world_mut().resource_mut::<WoodHub>().shutdown();
    }

    /// LT2 — refine cue on card, real Q, then cue clears.
    #[test]
    fn loop_teach_lt2_refine_cue_real_q() {
        use crate::first_session_guidance::{card_line, spawn_guidance_strip, update_guidance_text, FirstSessionGuidance, FirstSessionGuidanceText, GuidanceObjective, LOOP_REFINE_CUE, Signpost};
        use crate::wood_expedition::WoodView;
        use bevy::ecs::system::RunSystemOnce;
        crate::loot_drops::tests::in_user_dir("lt2", |dir| {
            let (hub, _) = carried_hub();
            let (mut app, _) = q_app(true, 3, hub);
            app.init_resource::<FirstSessionGuidance>().init_resource::<Signpost>().init_resource::<WoodView>()
                .insert_resource(HexTravelState { current: PlaceId::Sanctuary });
            app.world_mut().resource_mut::<FirstSessionGuidance>().objective = GuidanceObjective::PlantFabricator;
            app.world_mut().run_system_once(spawn_guidance_strip).unwrap();
            *app.world_mut().resource_mut::<WoodView>() = WoodView { unit: 1, standing: false };
            app.world_mut().run_system_once(update_guidance_text).unwrap();
            let goal = card_line(GuidanceObjective::PlantFabricator.prompt());
            let card = |a: &mut App| a.world_mut().query_filtered::<&bevy::prelude::Text, With<FirstSessionGuidanceText>>().single(a.world()).unwrap().as_str().to_string();
            assert!(card(&mut app).contains(LOOP_REFINE_CUE));
            press_q(&mut app);
            *app.world_mut().resource_mut::<WoodView>() = WoodView { unit: 0, standing: false };
            assert_eq!((crate_stock(&app.world().resource::<FabricatorYard>().fab), app.world().resource::<WoodHub>().memory_unit()), (1, Some(0)));
            app.world_mut().run_system_once(update_guidance_text).unwrap();
            assert_eq!(card(&mut app), goal);
            press_q(&mut app);
            assert_eq!(crate_stock(&app.world().resource::<FabricatorYard>().fab), 1);
            app.world_mut().resource_mut::<WoodHub>().shutdown();
            assert!(dir.starts_with(std::env::temp_dir()));
        });
    }

    /// WU5 — the first civic LaneCrate still runs on Q with wood carried, still writes tons, and leaves the wood.
    #[test]
    fn wu5_first_lane_crate_keeps_on_lane_and_leaves_wood() {
        let (mut app, _) = q_app(true, 2, carried_hub().0);
        press_q(&mut app);
        let yard = &app.world().resource::<FabricatorYard>().fab;
        assert_eq!((yard.pack.unlocked(), crate_stock(yard)), (true, 0));
        assert_eq!(app.world().resource::<LivedHourBind>().climate.tons_moved, 1);
        assert_eq!(app.world().resource::<WoodHub>().memory_unit(), Some(1));
        app.world_mut().resource_mut::<WoodHub>().shutdown();
    }

    use crate::first_session_guidance::{card_line, loop_cue, spawn_guidance_strip, update_guidance_text, FirstSessionGuidance, FirstSessionGuidanceText, GuidanceObjective, LOOP_REFINE_CUE, Signpost};
    use crate::hour_sacred::{try_mark_hour_three_held, try_plant_house, try_ridge_tab};
    use crate::input::InputMapSet;
    use crate::wood_expedition::WoodView;
    use bevy::ecs::system::RunSystemOnce;
    use shared::vertical_factory::VerticalFactory;
    fn house_not_held(arrive: bool) -> (HourSacred, VerticalFactory) {
        let (mut hour, mut factory) = (HourSacred::default(), VerticalFactory::default());
        assert!(try_ridge_tab(&mut hour, true) && try_plant_house(&mut hour, &mut factory) && !hour.complete);
        if arrive { while !factory.tutorial_complete() && factory.advance() != "unfounded" {} }
        assert_eq!(factory.tutorial_complete(), arrive); (hour, factory)
    }
    /// `VerticalFactoryPlugin` schedules `handle_factory_q` with `handle_fab_q`, `update_fab_slab`, and `update_guidance_text`.
    fn gate_app(hour: HourSacred, factory: VerticalFactory, hub: WoodHub) -> App {
        let mut bind = LivedHourBind::default(); bind.climate = shared::shard_climate::ShardClimate::from_json(CLIMATE_V0).unwrap();
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).configure_sets(Update, InputMapSet).edit_schedule(Update, |schedule| { schedule.set_executor(bevy::ecs::schedule::SingleThreadedExecutor::new()); }).insert_resource(hour).insert_resource(FactoryYard { factory }).insert_resource(FabricatorYard { fab: Fabricator::default(), bench_glow: 0.0 }).insert_resource(bind).insert_resource(hub).insert_resource(HexTravelState { current: PlaceId::Sanctuary }).init_resource::<ButtonInput<KeyCode>>().init_resource::<PlayerInput>().init_resource::<ThrivingMoments>().init_resource::<FirstSessionGuidance>().init_resource::<Signpost>().init_resource::<WoodView>().add_plugins(crate::vertical_factory::VerticalFactoryPlugin).add_systems(Startup, (spawn_fab_slab, spawn_guidance_strip)).add_systems(Update, (handle_fab_q, update_fab_slab, update_guidance_text).chain().after(InputMapSet));
        app
    }
    struct GateShut(App);
    impl Drop for GateShut { fn drop(&mut self) { self.0.world_mut().resource_mut::<WoodHub>().shutdown(); } }
    fn with_gate(tag: &str, hour: HourSacred, factory: VerticalFactory, body: impl FnOnce(&mut App)) {
        let _lock = crate::test_env::lock();
        crate::loot_drops::tests::in_user_dir(tag, |_| { let mut shut = GateShut(gate_app(hour, factory, carried_hub().0)); body(&mut shut.0); });
    }
    fn card_text(app: &mut App) -> String { app.world_mut().query_filtered::<&Text, With<FirstSessionGuidanceText>>().single(app.world()).unwrap().as_str().to_string() }
    fn refine_cue(app: &mut App) -> Option<&'static str> { app.world_mut().run_system_once(|g: Res<FirstSessionGuidance>, w: Res<WoodView>, y: Res<FabricatorYard>, f: Res<FactoryYard>, h: Res<HourSacred>| loop_cue(&g, Some(&w), Some(&y), Some(&f), Some(&h), Some(PlaceId::Sanctuary), None)).unwrap() }
    #[test]
    fn loop_gate_lg1_q_refines_before_hour_two() {
        let (hour, factory) = house_not_held(true);
        with_gate("lg1", hour, factory, |app| {
            for _ in 0..3 { press_q(app); } let kept = app.world().resource::<WoodHub>().memory_unit();
            press_q(app);
            let fab = app.world().resource::<FabricatorYard>().fab.clone(); let open = !app.world().resource::<HourSacred>().complete;
            assert_eq!((kept, open, crate_stock(&fab), fab.last_line.as_str(), app.world().resource::<WoodHub>().memory_unit()), (Some(1), true, 1, "LaneCrate ran — the lane holds a crate", Some(0)));
        });
    }
    #[test]
    fn loop_gate_lg2_slab_visible_before_hour_two() {
        let (hour, factory) = house_not_held(true);
        with_gate("lg2", hour, factory, |app| {
            app.update();
            let vis = *app.world_mut().query_filtered::<&Visibility, With<FabSlabRoot>>().single(app.world()).unwrap();
            let text = app.world_mut().query_filtered::<&Text, With<FabSlabText>>().single(app.world()).unwrap().as_str().to_string();
            assert_eq!((vis, text), (Visibility::Visible, fabricator_slab_at_place(&app.world().resource::<FabricatorYard>().fab, Some("Sanctuary"))));
        });
    }
    #[test]
    fn loop_gate_lg3_refine_cue_exact_then_q_clears() {
        let (hour, factory) = house_not_held(true);
        with_gate("lg3", hour, factory, |app| {
            app.world_mut().resource_mut::<FirstSessionGuidance>().objective = GuidanceObjective::PlantFabricator;
            *app.world_mut().resource_mut::<WoodView>() = WoodView { unit: 1, standing: false };
            for _ in 0..3 { press_q(app); }
            let goal = card_line(GuidanceObjective::PlantFabricator.prompt());
            assert_eq!((refine_cue(app), card_text(app)), (Some(LOOP_REFINE_CUE), format!("{goal}\n{LOOP_REFINE_CUE}")));
            press_q(app);
            assert_eq!((refine_cue(app), card_text(app)), (None, goal));
        });
    }
    #[test]
    fn loop_gate_lg4_peace_pre_house_and_before_arrival_keep_wood() {
        let mut pre = HourSacred::default();
        assert!(try_ridge_tab(&mut pre, true) && !pre.charter_skin_live());
        let (house, founded) = house_not_held(false);
        let cases = [("lg4p", HourSacred::default(), VerticalFactory::default()), ("lg4r", pre, VerticalFactory::default()), ("lg4b", house, founded)];
        for (tag, hour, factory) in cases {
            let (mut sim_h, mut sim_f) = (hour.clone(), factory.clone());
            if !(try_plant_house(&mut sim_h, &mut sim_f) || sim_h.hex() == HexFlag::Peace || !sim_h.charter_skin_live() || sim_f.tutorial_complete()) { let _ = sim_f.advance(); }
            let want = (serde_json::to_string(&sim_f).unwrap(), serde_json::to_string(&sim_h.session).unwrap());
            with_gate(tag, hour, factory, |app| {
                press_q(app);
                let got_f = serde_json::to_string(&app.world().resource::<FactoryYard>().factory).unwrap();
                let got_s = serde_json::to_string(&app.world().resource::<HourSacred>().session).unwrap();
                let fab = app.world().resource::<FabricatorYard>().fab.clone();
                assert_eq!((got_f, got_s, fab.planted, crate_stock(&fab), app.world().resource::<WoodHub>().memory_unit()), (want.0, want.1, false, 0, Some(1)));
            });
        }
    }
    #[test]
    fn loop_gate_lg5_embassy_book_and_boot_refuse_before_hour_two() {
        let (hour, factory) = house_not_held(true);
        with_gate("lg5", hour, factory, |app| {
            for _ in 0..3 { let _ = app.world_mut().resource_mut::<FabricatorYard>().fab.craft_next(); }
            let hour = app.world().resource::<HourSacred>().clone(); let fab = app.world().resource::<FabricatorYard>().fab.clone();
            let mut embassy = shared::embassy::Embassy::default();
            assert!(fab.pack.unlocked() && !hour.complete && !hour.hour_three_complete && crate::embassy::try_request_embassy_seat(&hour, &fab.pack, &mut embassy).is_none() && !embassy.seated);
            let mut held = hour.clone();
            assert!(!try_mark_hour_three_held(&mut held, fab.pack.unlocked(), embassy.seated) && !held.hour_three_complete);
            let mut book = shared::hour_two::HourTwoPack { session: hour.session.clone(), fabricator: fab, embassy, complete: false, ..Default::default() };
            book.mark_hour_three();
            assert!(!book.hour_three_complete && book.line(true) != "Hour three held · the book is yours" && !shared::hex_travel::may_enter(PlaceId::Heartwood, false) && shared::hex_travel::boot_place(shared::hex_travel::BootKind::Continue, false, Some(PlaceId::Heartwood)) == PlaceId::Sanctuary);
        });
    }
    #[test]
    fn loop_gate_lg6_civic_mend_lane_matches_existing_bind() {
        let (hour, factory) = house_not_held(true);
        with_gate("lg6", hour, factory, |app| {
            let mut expect = app.world().resource::<LivedHourBind>().clone();
            expect.climate.on_mend(); expect.standing.on_mend(); expect.refresh_climate_slab();
            expect.climate.on_lane(); expect.standing.on_lane(); expect.refresh_climate_slab();
            for _ in 0..3 { press_q(app); } let now = app.world().resource::<LivedHourBind>().clone();
            assert!(!app.world().resource::<HourSacred>().complete); assert!(app.world().resource::<FabricatorYard>().fab.pack.unlocked());
            assert_eq!((now.climate.restored_count, now.climate.tons_moved, now.climate.updated_at, now.standing.updated_at, now.week.tons_moved, now.week.restored_count, now.week.updated_at), (1, 1, 3, 2, 1, 1, 3));
            assert_eq!((&now.climate, &now.standing, &now.week), (&expect.climate, &expect.standing, &expect.week));
            assert_eq!((shared::user_persist::read_named(crate::lived_hour_bind::SHARD_CLIMATE_PATH).unwrap(), shared::user_persist::read_named(crate::lived_hour_bind::SHARD_STANDING_PATH).unwrap(), shared::user_persist::read_named(crate::lived_hour_bind::WEEK_AUDIT_PATH).unwrap()), (now.climate.to_json().unwrap(), now.standing.to_json().unwrap(), now.week.to_json().unwrap()));
        });
    }

    fn haul_gate(tag: &str, booked: bool, body: impl FnOnce(&mut App)) {
        let (mut hour, factory) = house_not_held(true);
        if booked { hour.complete = true; hour.hour_three_complete = true; }
        with_gate(tag, hour, factory, |app| { app.insert_resource(LaunchDoor::InYard); body(app); });
    }
    fn q_edge(app: &mut App) { press_q(app); app.insert_resource(ButtonInput::<KeyCode>::default()); app.update(); }
    fn key_edge(app: &mut App, key: KeyCode) {
        let mut keys = ButtonInput::<KeyCode>::default(); keys.press(key); app.insert_resource(keys); app.update(); app.insert_resource(ButtonInput::<KeyCode>::default()); app.update();
    }
    fn fab_text(app: &mut App) -> String {
        app.world_mut().query_filtered::<&Text, With<FabSlabText>>().single(app.world()).unwrap().as_str().to_string()
    }
    fn assert_bare(app: &mut App) {
        app.update();
        app.update();
        let fab = app.world().resource::<FabricatorYard>().fab.clone();
        let travel = app.world().resource::<HexTravelState>().clone();
        assert_eq!(fab_text(app), fabricator_slab_at_place(&fab, fabricator_stood_place_label(Some(&travel), None)));
    }
    fn q4(app: &mut App) { for _ in 0..4 { q_edge(app); } }

    /// HB1 — charter booked, real Q x4, the haul cue once on one line.
    #[test]
    fn loop_teach_hb1_exact_cue() {
        haul_gate("hb1", true, |app| {
            q4(app);
            let fab = app.world().resource::<FabricatorYard>().fab.clone();
            assert_eq!((crate_stock(&fab), fab.last_line.as_str()), (1, "LaneCrate ran — the lane holds a crate"));
            let text = fab_text(app);
            assert_eq!(text, format!("{} · {LOOP_HAUL_CUE}", fabricator_slab_at_place(&fab, Some("Sanctuary"))));
            assert_eq!(text.matches(LOOP_HAUL_CUE).count(), 1);
            assert!(!text.contains('\n'));
        });
    }

    /// HB2 — one gate off at a time is the bare slab. Pre-book refine is heavy and quiet.
    #[test]
    fn loop_teach_hb2_honesty_table() {
        haul_gate("hb2", true, |app| {
            q4(app);
            assert!(fab_text(app).contains(LOOP_HAUL_CUE));
            app.world_mut().resource_mut::<FabricatorYard>().fab.crate_stock = 0;
            assert_bare(app);
            app.world_mut().resource_mut::<FabricatorYard>().fab.crate_stock = 1;
            app.world_mut().resource_mut::<HourSacred>().complete = false;
            assert_bare(app);
            app.world_mut().resource_mut::<HourSacred>().complete = true;
            app.world_mut().resource_mut::<HourSacred>().hour_three_complete = false;
            assert_bare(app);
            app.world_mut().resource_mut::<HourSacred>().hour_three_complete = true;
            app.world_mut().resource_mut::<HexTravelState>().current = PlaceId::Heartwood;
            assert_bare(app);
            app.world_mut().resource_mut::<HexTravelState>().current = PlaceId::Depths;
            assert_bare(app);
            app.world_mut().resource_mut::<HexTravelState>().current = PlaceId::Sanctuary;
            app.insert_resource(LaunchDoor::Title);
            assert_bare(app);
        });
        haul_gate("hb2pre", false, |app| {
            app.add_plugins(crate::living_body::LivingBodyPlugin).init_resource::<crate::living_practice_loop::SoftPlayerRealm>().init_resource::<crate::harvest_feel::SoftRbePool>();
            q4(app);
            let fab = app.world().resource::<FabricatorYard>().fab.clone();
            assert_eq!(crate_stock(&fab), 1);
            assert!(app.world().resource::<crate::living_body::LivingBody>().heavy);
            assert_eq!(fab_text(app), fabricator_slab_at_place(&fab, Some("Sanctuary")));
        });
    }

    /// HB4 — Digit1 stays before the haul cue, each once, civic copy.
    #[test]
    fn loop_teach_hb4_digit1_order() {
        haul_gate("hb4", true, |app| {
            q4(app);
            let fab = app.world().resource::<FabricatorYard>().fab.clone();
            assert!(fab.spool_stock > 0 && fab.crate_stock > 0);
            let text = fab_text(app);
            let want = format!("{} · {LOOP_HAUL_CUE}", fabricator_slab_at_place(&fab, Some("Sanctuary")));
            assert_eq!(text, want);
            assert!(want.contains(&format!(" · Digit1 · {LOOP_HAUL_CUE}")));
            assert_eq!((text.matches("Digit1").count(), text.matches(LOOP_HAUL_CUE).count()), (1, 1));
            assert!(shared::fabricator::manufacture_copy_is_honest(&text) && shared::fabricator::manufacture_copy_is_honest(LOOP_HAUL_CUE));
            assert!(!text.contains('\n'));
        });
    }

    /// HB5 — quiet frames do not rewrite the slab; the stock frame does.
    #[test]
    fn loop_teach_hb5_quiet_path() {
        #[derive(Resource)]
        struct SlabWrites(u32);
        fn count(q: Query<(), (Changed<Text>, With<FabSlabText>)>, mut n: ResMut<SlabWrites>) {
            n.0 += q.iter().count() as u32;
        }
        haul_gate("hb5", true, |app| {
            app.insert_resource(SlabWrites(0)).add_systems(Update, count.after(update_fab_slab));
            q4(app);
            app.world_mut().resource_mut::<SlabWrites>().0 = 0;
            app.update();
            app.update();
            assert_eq!(app.world().resource::<SlabWrites>().0, 0);
            app.world_mut().resource_mut::<FabricatorYard>().fab.crate_stock = 0;
            app.update();
            assert!(app.world().resource::<SlabWrites>().0 > 0);
        });
    }

    /// HB6 — Q, E, and Tab leave the crate, the place, and the tons alone.
    #[test]
    fn loop_teach_hb6_q_e_tab_leave_stock_travel_tons() {
        haul_gate("hb6", true, |app| {
            q4(app);
            let snap = |app: &App| { let w = app.world(); (w.resource::<FabricatorYard>().fab.crate_stock, w.resource::<HexTravelState>().current, w.resource::<LivedHourBind>().climate.tons_moved, w.resource::<LivedHourBind>().week.tons_moved) };
            let before = snap(app);
            assert!(fab_text(app).contains(LOOP_HAUL_CUE));
            let mut sim = app.world().resource::<FabricatorYard>().fab.clone();
            let hour = app.world().resource::<HourSacred>().clone();
            let done = app.world().resource::<FactoryYard>().factory.tutorial_complete();
            let step = try_craft_proof_pack(&hour, done, &mut sim).unwrap();
            assert!(!refine_wood_when_idle(step, &mut sim, None));
            q_edge(app);
            assert_eq!((app.world().resource::<FabricatorYard>().fab.clone(), snap(app)), (sim.clone(), before));
            for key in [soft_play_bindings::INTERACT, soft_play_bindings::CHART] {
                key_edge(app, key);
                assert_eq!((app.world().resource::<FabricatorYard>().fab.clone(), snap(app)), (sim.clone(), before));
            }
        });
    }
}
