//! Lived-hour infra witness — Slice 5 (v23.2.9) + pack (v23.2.29)
//!
//! Offline extractor + spill as readable evidence. Dies in Peace.
//! Does not teach attack.
//!
//! CARD FLESH-INFRA-SPILL — the existing spill slab may name the Place
//! (`HexTravelState::chip_name`) while charter skin is live. Absent travel
//! keeps the slab byte for byte. No second HUD. No gold.
//! Peak memory, cited: walked · tended · week was the bill · yard remembered.
//!
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::hour_two::HourTwoPack;
use shared::infra_spill::InfraWitness;

use crate::hour_sacred::{read_hour_two_json, HourSacred};
use crate::hud_anchor_registry::{HudSlab, SPILL};
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};

/// CARD FLESH-INFRA-SPILL — `{place} · {slab}` when a chip is present.
/// `None` returns the bare slab. One string. No second widget.
fn spill_slab_line(bare: &str, place: Option<&str>) -> String {
    match place {
        Some(place) => format!("{place} · {bare}"),
        None => bare.to_string(),
    }
}

#[derive(Resource, Debug, Clone)]
pub struct EvidenceYard {
    pub witness: InfraWitness,
}

impl Default for EvidenceYard {
    fn default() -> Self {
        if let Some(raw) = read_hour_two_json() {
            return Self {
                witness: HourTwoPack::from_json(&raw).witness,
            };
        }
        Self {
            witness: InfraWitness::default(),
        }
    }
}

#[derive(Component)]
struct SpillSlabRoot;
#[derive(Component)]
struct SpillSlabText;

pub struct InfraSpillPlugin;

impl Plugin for InfraSpillPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EvidenceYard>()
            .add_systems(Startup, spawn_spill_slab)
            .add_systems(Update, (witness_offline, update_spill_slab));
    }
}

fn spawn_spill_slab(mut commands: Commands) {
    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: SPILL.top(),
                    left: SPILL.left(),
                    width: Val::Px(520.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG.with_alpha(1.0)),
                BorderColor::all(TITLE_BORDER.with_alpha(1.0)),
                Visibility::Hidden,
            ),
            SpillSlabRoot,
            HudSlab(SPILL.id),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(""),
TextFont { font_size: FontSize::Px(14.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                SpillSlabText,
            ));
        });
}

/// After Q founds the House, I2 / Offline extractor spill is the witness.
/// Peace and visitor-without-charter stay dark.
pub fn try_witness_house_spill(hour: &HourSacred, yard: &mut EvidenceYard) -> bool {
    if !hour.charter_skin_live() {
        return false;
    }
    yard.witness.ensure_offline_extractor();
    if yard.witness.seen {
        return false;
    }
    yard.witness.seen = true;
    true
}

fn witness_offline(
    hour: Res<HourSacred>,
    mut yard: ResMut<EvidenceYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if try_witness_house_spill(&hour, &mut yard) {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstSpillWitness,
            time.elapsed_secs_f64(),
        );
    }
}

fn update_spill_slab(
    hour: Res<HourSacred>,
    yard: Res<EvidenceYard>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut root: Query<&mut Visibility, With<SpillSlabRoot>>,
    mut text_q: Query<&mut Text, With<SpillSlabText>>,
) {
    let show = yard.witness.visible_on(hour.hex()) && hour.charter_skin_live();
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if !show {
        return;
    }
    let bare = yard.witness.slab_line();
    let line = spill_slab_line(&bare, travel.as_ref().map(|state| state.chip_name()));
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

    /// CARD HUD-ANCHOR-REGISTRY-2B — Spill position is the registry, Style is the coded literal.
    #[test]
    fn spill_style_byte_identical_to_coded_place() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_spill_slab);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<&Node, With<SpillSlabRoot>>();
        let style = q.single(app.world()).unwrap().clone();
        let coded = Node {
            position_type: PositionType::Absolute,
            top: Val::Px(52.0),
            left: Val::Px(16.0),
            width: Val::Px(520.0),
            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
            justify_content: JustifyContent::FlexStart,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };
        assert_eq!(style, coded);
        assert_eq!(style.top, SPILL.top());
        assert_eq!(style.left, SPILL.left());
        assert_eq!(style.margin, UiRect::default());
        assert_eq!(style.width, Val::Px(SPILL.width));
    }

    /// CARD VP-SLABS-REGAL-1 — the infra spill slab rests on the title palette: opaque
    /// TITLE_PLATE_BG plate, TITLE_BORDER rim at alpha 1, TITLE_TEXT_PRIMARY text.
    #[test]
    fn slabs_regal_spill_slab_rests_on_title_palette() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_spill_slab);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<(&BorderColor, &BackgroundColor), With<SpillSlabRoot>>();
        let (border, bg) = q.single(app.world()).unwrap();
        let (border, bg) = (({
            assert_eq!(border.top, border.right, "border edges");
            assert_eq!(border.top, border.bottom, "border edges");
            assert_eq!(border.top, border.left, "border edges");
            border.top
        }).to_srgba(), bg.0.to_srgba());
        let mut t = app.world_mut().query_filtered::<&TextColor, With<SpillSlabText>>();
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
    fn peace_hides_spill() {
        let hour = HourSacred {
            session: shared::space_law::SpaceSession::default(),
            complete: false,
            hour_three_complete: false,
        };
        assert_eq!(hour.hex(), HexFlag::Peace);
        let mut yard = EvidenceYard {
            witness: InfraWitness::default(),
        };
        yard.witness.ensure_offline_extractor();
        assert!(!yard.witness.visible_on(hour.hex()));
        assert!(!try_witness_house_spill(&hour, &mut yard));
    }

    /// Playtest H2-Q: Q off Peace plants House then I2 / Offline extractor spill.
    #[test]
    fn q_off_peace_plants_house_and_i2_spill() {
        let mut hour = HourSacred {
            session: shared::space_law::SpaceSession::default(),
            complete: false,
            hour_three_complete: false,
        };
        assert!(crate::hour_sacred::try_ridge_tab(&mut hour, true));
        let mut factory = shared::vertical_factory::VerticalFactory::default();
        assert!(crate::hour_sacred::try_plant_house(&mut hour, &mut factory));
        assert!(hour.charter_skin_live());

        let mut yard = EvidenceYard {
            witness: InfraWitness::default(),
        };
        assert!(try_witness_house_spill(&hour, &mut yard));
        assert!(yard.witness.seen);
        let pack = yard.witness.pack.as_ref().expect("I2 pack");
        assert_eq!(pack.code, shared::infra_spill::OffenseCode::I2);
        assert!(pack.spill);
        assert!(yard.witness.visible_on(hour.hex()));
        let line = yard.witness.slab_line();
        assert!(line.contains("Extractor"));
        assert!(line.contains("I2") || line.contains("spill"));
        assert!(!try_witness_house_spill(&hour, &mut yard), "spill is once");
    }

    /// CARD FLESH-INFRA-SPILL — Place prefixes the spill slab; no travel keeps it exact.
    #[test]
    fn flesh_infra_spill_names_chip_or_keeps_slab() {
        use shared::hex_travel::PlaceId;

        let mut yard = EvidenceYard {
            witness: InfraWitness::default(),
        };
        yard.witness.ensure_offline_extractor();
        let bare = yard.witness.slab_line();
        assert!(bare.starts_with("Extractor Offline · spill on the ground · "));
        assert_eq!(spill_slab_line(&bare, None), bare);

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
                spill_slab_line(&bare, Some(travel.chip_name())),
                format!("{name} · {bare}")
            );
        }

        let mut peace = HourSacred {
            session: shared::space_law::SpaceSession::default(),
            complete: false,
            hour_three_complete: false,
        };
        assert!(!peace.charter_skin_live());
        assert!(crate::hour_sacred::try_ridge_tab(&mut peace, true));
        assert!(crate::hour_sacred::try_plant_house(
            &mut peace,
            &mut shared::vertical_factory::VerticalFactory::default(),
        ));
        assert!(peace.charter_skin_live());

        for sample in [
            spill_slab_line(&bare, Some(PlaceId::Sanctuary.chip_name())),
            spill_slab_line(&bare, Some(PlaceId::Heartwood.chip_name())),
            spill_slab_line(&bare, Some(PlaceId::Depths.chip_name())),
            spill_slab_line(&bare, None),
        ] {
            let low = sample.to_lowercase();
            assert!(!low.contains("gold"), "{sample}");
            assert!(!low.contains("market"), "{sample}");
            assert!(!low.contains("hud"), "{sample}");
            assert!(!low.contains("online"), "{sample}");
            assert!(!sample.contains("Threshold"), "{sample}");
        }
    }

    /// STATUS-VIS-NEQ-1 SV1–SV4. Last tape of (Visibility, changed) on SpillSlabRoot; settle 2 then clear.
    #[derive(Resource, Default)] struct SvTape(Vec<(Visibility, bool)>);
    fn sv_tape(q: Query<Ref<Visibility>, With<SpillSlabRoot>>, mut t: ResMut<SvTape>) { t.0.extend(q.iter().map(|v| (*v, v.is_changed()))); }
    fn sv_run(app: &mut App, n: usize) -> Vec<(Visibility, bool)> { for _ in 0..n { app.update(); } std::mem::take(&mut app.world_mut().resource_mut::<SvTape>().0) }
    fn sv_settle(mut app: App) -> App { app.update(); app.update(); app.world_mut().resource_mut::<SvTape>().0.clear(); app }
    fn sv_hide(app: &mut App) { let e = app.world_mut().query_filtered::<Entity, With<SpillSlabRoot>>().single(app.world()).unwrap(); *app.world_mut().get_mut::<Visibility>(e).unwrap() = Visibility::Hidden; }
    fn sv_yielded(app: &App, id: &'static str) -> bool { app.world().resource::<crate::hud_presets::HudYielded>().0.contains(&id) }
    const SV: Visibility = Visibility::Visible;
    const SH: Visibility = Visibility::Hidden;
    /// Peace by default; `live` plants the House off Peace (ridge tab, plant house) and witnesses the I2 spill, as q_off_peace_plants_house_and_i2_spill.
    fn sv_live() -> (HourSacred, EvidenceYard) {
        let (mut hour, mut yard, mut factory) = (HourSacred::default(), EvidenceYard { witness: InfraWitness::default() }, shared::vertical_factory::VerticalFactory::default());
        assert!(crate::hour_sacred::try_ridge_tab(&mut hour, true) && crate::hour_sacred::try_plant_house(&mut hour, &mut factory) && try_witness_house_spill(&hour, &mut yard));
        (hour, yard)
    }
    fn sv_app(live: bool, hud: Option<(crate::hud_presets::HudPresetId, Option<crate::title_screen::LaunchDoor>)>) -> App {
        let (hour, yard) = if live { sv_live() } else { (HourSacred::default(), EvidenceYard { witness: InfraWitness::default() }) };
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).insert_resource(hour).insert_resource(yard).init_resource::<SvTape>().add_systems(Startup, spawn_spill_slab).add_systems(Update, update_spill_slab).add_systems(Last, sv_tape);
        if let Some((id, door)) = hud { app.add_plugins(crate::hud_presets::HudLayoutPlugin).insert_resource(crate::hud_presets::ActiveHudPreset { id: Some(id) }).init_resource::<crate::hex_travel::PlacesPlate>(); if let Some(door) = door { app.insert_resource(door); } }
        sv_settle(app)
    }
    #[test] fn status_vis_neq_1_sv1_spill_peace_steady_hidden_unmarked() { let mut app = sv_app(false, None); assert_eq!(sv_run(&mut app, 3), vec![(SH, false); 3], "SV1 spill Peace steady hidden"); }
    #[test] fn status_vis_neq_1_sv2_spill_live_then_peace_once_each() {
        let mut app = sv_app(false, None); let (hour, yard) = sv_live(); app.insert_resource(hour).insert_resource(yard);
        assert_eq!(sv_run(&mut app, 2), vec![(SV, true), (SV, false)], "SV2 spill goes live: Visible once, then quiet");
        app.insert_resource(HourSacred::default());
        assert_eq!(sv_run(&mut app, 2), vec![(SH, true), (SH, false)], "SV2 spill back to Peace: Hidden once, then quiet");
    }
    #[test] fn status_vis_neq_1_sv3_spill_direct_hidden_restored_once() { let mut app = sv_app(true, None); sv_hide(&mut app); assert_eq!(sv_run(&mut app, 2), vec![(SV, true), (SV, false)], "SV3 spill direct Hidden write: restored once, then quiet"); }
    #[test] fn status_vis_neq_1_sv4_spill_real_hud_layout() {
        let mut app = sv_app(true, Some((crate::hud_presets::HudPresetId::Classic, Some(crate::title_screen::LaunchDoor::InYard))));
        assert_eq!(sv_run(&mut app, 2), vec![(SH, true); 2], "SV4 Classic InYard live: clean-hidden, changed every frame on main and branch alike");
        let mut app = sv_app(true, Some((crate::hud_presets::HudPresetId::Management, Some(crate::title_screen::LaunchDoor::InYard))));
        assert_eq!(sv_run(&mut app, 2), vec![(SV, false); 2], "SV4 Management live, no Factory or Fab: steady Visible, quiet");
        app.world_mut().spawn((Node::default(), Visibility::Visible, HudSlab(crate::hud_anchor_registry::ID_FACTORY))); let t = sv_run(&mut app, 2);
        assert_eq!((t.last().unwrap().0, sv_yielded(&app, SPILL.id)), (SH, true), "SV4 Management with a Factory sentinel: Spill yields by R2 into HudYielded");
    }
}
