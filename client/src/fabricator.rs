//! Lived-hour fabricator — Slice 7 (v23.2.11) + P2 week answer
//!
//! After Hour two held and the crate arrives, Q plants a fabricator then runs MendSpool and LaneCrate.
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
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::fabricator::Fabricator;
use shared::hex_travel::PlaceId;
use shared::hour_two::HourTwoPack;
use shared::space_law::HexFlag;

use crate::hex_travel::HexTravelState;
use crate::hud_anchor_registry::FAB;
use crate::hour_sacred::{read_hour_two_json, HourSacred};
use crate::human_presence::SoftPresence;
use crate::lived_hour_bind::LivedHourBind;
use crate::soft_play_bindings;
use crate::input::{InputMapSet, PlayerInput};
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};
use crate::vertical_factory::FactoryYard;
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};

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
struct FabSlabText;

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
            NodeBundle {
                style: Style {
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
                background_color: TITLE_PLATE_BG.with_alpha(1.0).into(),
                border_color: TITLE_BORDER.with_alpha(1.0).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            FabSlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: TITLE_TEXT_PRIMARY,
                        ..default()
                    },
                ),
                FabSlabText,
            ));
        });
}

/// Q after Hour two held + crate arrival plants/crafts the Proof Pack. Peace no-op.
pub fn try_craft_proof_pack(
    hour: &HourSacred,
    tutorial_complete: bool,
    fab: &mut Fabricator,
) -> Option<&'static str> {
    if hour.hex() == HexFlag::Peace || !hour.charter_skin_live() || !hour.complete {
        return None;
    }
    if !tutorial_complete {
        return None;
    }
    Some(fab.craft_next())
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
            time.elapsed_seconds_f64(),
        );
    }
}

fn tick_bench_glow(time: Res<Time>, mut yard: ResMut<FabricatorYard>) {
    if yard.bench_glow > 0.0 {
        yard.bench_glow = (yard.bench_glow - time.delta_seconds() * 0.55).max(0.0);
    }
}

fn update_fab_slab(
    hour: Res<HourSacred>,
    factory: Res<FactoryYard>,
    yard: Res<FabricatorYard>,
    travel: Option<Res<HexTravelState>>,
    presence: Option<Res<SoftPresence>>,
    mut root: Query<(&mut Visibility, &mut BorderColor, &mut BackgroundColor), With<FabSlabRoot>>,
    mut text_q: Query<&mut Text, With<FabSlabText>>,
) {
    let show = hour.complete && hour.charter_skin_live() && factory.factory.tutorial_complete();
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
    let line = fabricator_slab_at_place(&yard.fab, place);
    for mut text in &mut text_q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != line {
                s.value = line.clone();
            }
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
            .query_filtered::<&Style, With<FabSlabRoot>>();
        let style = q.single(app.world()).clone();
        let coded = Style {
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
        let (border, bg) = q.single(app.world());
        let (border, bg) = (border.0.to_srgba(), bg.0.to_srgba());
        let mut t = app.world_mut().query_filtered::<&Text, With<FabSlabText>>();
        let txt = t.single(app.world()).sections[0].style.color.to_srgba();
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
        let (border, bg, vis) = q.single(app.world());
        (border.0.to_srgba(), bg.0.to_srgba(), *vis)
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
}
