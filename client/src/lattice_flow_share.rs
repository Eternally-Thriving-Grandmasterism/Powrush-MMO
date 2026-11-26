/*!
 * Soft Lattice Flow Share — local export + future peer socket (v21.99.4)
 *
 * First hour is solo. Own export is not dressed up as a peer.
 * Chip appears only if a *peer* file exists. U without a peer is honest, not an ops ticket.
 *
 * TOLC 8 · Contact: info@Rathor.ai · Yoi ⚡
 *
 * CARD FLESH-LATTICE-SHARE — the existing share lines may name the Place
 * (`HexTravelState::chip_name`). Absent travel keeps each bare line.
 * No share HUD. No Online. No sockets.
 * Peak memory, cited: walked · tended · week was the bill · yard remembered.
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::abundance_journey_echo::{AbundanceJourneyEcho, JourneyKind};
use crate::hud_anchor_registry::{HudSlab, PEER};
use crate::rbe_allocate_choice::RbeAllocateChoice;
use crate::soft_play_bindings;
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};

const SHARE_PATH: &str = "data/powrush_lattice_flow_share.json";
const PEER_PATH: &str = "data/powrush_lattice_flow_share_peer.json";
const AMBIENT_POLL: f32 = 4.0;
const ALONE_LINE: &str = "This hour is yours alone — other travelers will appear here later";

/// CARD FLESH-LATTICE-SHARE — `{place} · {bare}` when a chip is present.
/// `None` returns `bare`. Same chip, same journey note. No second HUD.
fn lattice_share_line(bare: &str, place: Option<&str>) -> String {
    match place {
        Some(place) => format!("{place} · {bare}"),
        None => bare.to_string(),
    }
}

fn stood_chip(travel: Option<&crate::hex_travel::HexTravelState>) -> Option<&'static str> {
    travel.map(|state| state.chip_name())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LatticeFlowShareEnvelope {
    pub schema: String,
    pub flow_total: f32,
    pub reserve_total: f32,
    pub choices_made: u32,
    pub last_path: Option<String>,
    pub mercy_note: String,
    pub exported_at_secs: f64,
}

#[derive(Resource, Debug, Default)]
pub struct LatticeFlowShare {
    pub last_exported_choices: u32,
    pub last_path: Option<PathBuf>,
    pub last_peer: Option<LatticeFlowShareEnvelope>,
    pub last_ingest_note: Option<String>,
    pub ambient_seen_choices: Option<u32>,
    pub poll_accum: f32,
    pub chip_visible: bool,
}

#[derive(Component)]
struct PeerPresenceRoot;
#[derive(Component)]
struct PeerPresenceText;

pub struct LatticeFlowSharePlugin;

impl Plugin for LatticeFlowSharePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LatticeFlowShare>()
            .add_systems(Startup, spawn_peer_presence_chip)
            .add_systems(
                Update,
                (
                    export_on_allocate_change,
                    ambient_peer_poll,
                    soft_peer_ingest,
                    update_peer_presence_chip,
                ),
            );
    }
}

fn spawn_peer_presence_chip(mut commands: Commands) {
    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    bottom: PEER.bottom(),
                    right: PEER.right(),
                    width: Val::Px(280.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG.with_alpha(1.0)),
                BorderColor::all(TITLE_BORDER.with_alpha(1.0)),
                Visibility::Hidden,
            ),
            PeerPresenceRoot,
            HudSlab(PEER.id),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(""),
TextFont { font_size: FontSize::Px(12.5 / 1.2), ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                PeerPresenceText,
            ));
        });
}

fn export_on_allocate_change(
    allocate: Res<RbeAllocateChoice>,
    mut share: ResMut<LatticeFlowShare>,
    time: Res<Time>,
) {
    if allocate.choices_made == 0 {
        return;
    }
    if allocate.choices_made <= share.last_exported_choices {
        return;
    }

    let last_path = allocate.last_choice.map(|p| p.title().to_string());
    let env = LatticeFlowShareEnvelope {
        schema: "powrush_lattice_flow_share_v1".into(),
        flow_total: allocate.flow_total,
        reserve_total: allocate.reserve_total,
        choices_made: allocate.choices_made,
        last_path,
        mercy_note: "Voluntary abundance direction — never scarcity".into(),
        exported_at_secs: time.elapsed_secs_f64(),
    };

    let path = shared::user_persist::persist_path(SHARE_PATH);
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    match serde_json::to_string_pretty(&env) {
        Ok(json) => {
            if let Err(e) = fs::write(&path, json) {
                warn!(target: "powrush::lattice", "flow share write failed: {e}");
            } else {
                share.last_exported_choices = allocate.choices_made;
                share.last_path = Some(path.clone());
            }
        }
        Err(e) => warn!(target: "powrush::lattice", "flow share serialize failed: {e}"),
    }
}

fn try_read_envelope(path: impl AsRef<std::path::Path>) -> Option<LatticeFlowShareEnvelope> {
    let bytes = fs::read(path.as_ref()).ok()?;
    let env: LatticeFlowShareEnvelope = serde_json::from_slice(&bytes).ok()?;
    if env.schema.starts_with("powrush_lattice_flow_share") {
        Some(env)
    } else {
        None
    }
}

fn ambient_peer_poll(mut share: ResMut<LatticeFlowShare>, time: Res<Time>) {
    share.poll_accum += time.delta_secs();
    if share.poll_accum < AMBIENT_POLL {
        return;
    }
    share.poll_accum = 0.0;
    // Solo first hour: only a *peer* file is presence. Own export stays local.
    match try_read_envelope(shared::user_persist::persist_path(PEER_PATH)) {
        Some(env) => {
            share.ambient_seen_choices = Some(env.choices_made);
            share.last_peer = Some(env);
            share.chip_visible = true;
        }
        None => {
            share.chip_visible = false;
        }
    }
}

fn soft_peer_ingest(
    keyboard: Res<ButtonInput<KeyCode>>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut share: ResMut<LatticeFlowShare>,
    mut echo: ResMut<AbundanceJourneyEcho>,
) {
    if !keyboard.just_pressed(soft_play_bindings::PEER_INGEST) {
        return;
    }
    let place = stood_chip(travel.as_deref());

    match try_read_envelope(shared::user_persist::persist_path(PEER_PATH)) {
        Some(env) => {
            let bare = format!(
                "A fellow traveler left flow {:.1} · reserve {:.1}",
                env.flow_total, env.reserve_total
            );
            let note = lattice_share_line(&bare, place);
            echo.push(JourneyKind::Note, note.clone());
            share.last_ingest_note = Some(note);
            share.last_peer = Some(env);
            share.chip_visible = true;
        }
        None => {
            let note = lattice_share_line(ALONE_LINE, place);
            echo.push(JourneyKind::Note, note.clone());
            share.last_ingest_note = Some(note);
        }
    }
}

fn update_peer_presence_chip(
    share: Res<LatticeFlowShare>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut root: Query<&mut Visibility, With<PeerPresenceRoot>>,
    mut text_q: Query<&mut Text, With<PeerPresenceText>>,
) {
    let show = share.chip_visible && share.last_peer.is_some();
    for mut vis in &mut root {
        vis.set_if_neq(if show { Visibility::Visible } else { Visibility::Hidden });
    }
    if let Some(env) = &share.last_peer {
        let bare = format!(
            "A traveler shares flow {:.0} · U to remember",
            env.flow_total
        );
        let line = lattice_share_line(&bare, stood_chip(travel.as_deref()));
        for mut text in &mut text_q {
            if text.as_str() != line {
            **text = line.clone();
        }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// CARD HUD-ANCHOR-REGISTRY-2B — Peer position is the registry, Style is the coded literal.
    #[test]
    fn peer_style_byte_identical_to_coded_place() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_peer_presence_chip);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<&Node, With<PeerPresenceRoot>>();
        let style = q.single(app.world()).unwrap().clone();
        let coded = Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(16.0),
            right: Val::Px(16.0),
            width: Val::Px(280.0),
            padding: UiRect::all(Val::Px(10.0)),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };
        assert_eq!(style, coded);
        assert_eq!(style.bottom, PEER.bottom());
        assert_eq!(style.right, PEER.right());
        assert_eq!(style.margin, UiRect::default());
        assert_eq!(style.width, Val::Px(PEER.width));
    }

    /// CARD VP-HUD-TOP-1 — the peer-presence chip rests on the title palette: opaque
    /// plate, gold rim at alpha 1, cream text (no blue-grey / dim chrome).
    #[test]
    fn hud_top_peer_presence_chip_rests_on_title_palette() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_peer_presence_chip);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<(&BorderColor, &BackgroundColor), With<PeerPresenceRoot>>();
        let (border, bg) = q.single(app.world()).unwrap();
        let (border, bg) = (({
            assert_eq!(border.top, border.right, "border edges");
            assert_eq!(border.top, border.bottom, "border edges");
            assert_eq!(border.top, border.left, "border edges");
            border.top
        }).to_srgba(), bg.0.to_srgba());
        let mut t = app.world_mut().query_filtered::<&TextColor, With<PeerPresenceText>>();
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
    fn envelope_roundtrip() {
        let env = LatticeFlowShareEnvelope {
            schema: "powrush_lattice_flow_share_v1".into(),
            flow_total: 3.0,
            reserve_total: 1.5,
            choices_made: 4,
            last_path: Some("Flow outward".into()),
            mercy_note: "Voluntary abundance direction — never scarcity".into(),
            exported_at_secs: 12.0,
        };
        let json = serde_json::to_string(&env).unwrap();
        let back: LatticeFlowShareEnvelope = serde_json::from_str(&json).unwrap();
        assert_eq!(back.flow_total, 3.0);
    }

    /// CARD FLESH-LATTICE-SHARE — Place prefixes the share lines; no travel keeps them exact.
    #[test]
    fn flesh_lattice_share_line_names_chip_or_keeps_bare_line() {
        use shared::hex_travel::PlaceId;

        let chip = format!("A traveler shares flow {:.0} · U to remember", 4.0);
        let left = format!(
            "A fellow traveler left flow {:.1} · reserve {:.1}",
            3.0, 1.5
        );
        assert_eq!(
            lattice_share_line(&chip, None),
            "A traveler shares flow 4 · U to remember"
        );
        assert_eq!(
            lattice_share_line(&left, None),
            "A fellow traveler left flow 3.0 · reserve 1.5"
        );
        assert_eq!(lattice_share_line(ALONE_LINE, None), ALONE_LINE);

        let cases = [
            (PlaceId::Sanctuary, "Sanctuary Prime"),
            (PlaceId::Heartwood, "Heartwood"),
            (PlaceId::Depths, "Depths"),
        ];
        for (id, name) in cases {
            assert_eq!(id.chip_name(), name);
            let travel = crate::hex_travel::HexTravelState { current: id };
            assert_eq!(stood_chip(Some(&travel)), Some(name));
            assert_eq!(
                lattice_share_line(&chip, stood_chip(Some(&travel))),
                format!("{name} · A traveler shares flow 4 · U to remember")
            );
            assert_eq!(
                lattice_share_line(&left, Some(id.chip_name())),
                format!("{name} · A fellow traveler left flow 3.0 · reserve 1.5")
            );
            assert_eq!(
                lattice_share_line(ALONE_LINE, Some(travel.chip_name())),
                format!("{name} · {ALONE_LINE}")
            );
        }

        for sample in [
            lattice_share_line(&chip, Some(PlaceId::Heartwood.chip_name())),
            lattice_share_line(ALONE_LINE, None),
        ] {
            let low = sample.to_lowercase();
            assert!(!low.contains("online"), "{sample}");
            assert!(!low.contains("socket"), "{sample}");
            assert!(!low.contains("gold"), "{sample}");
            assert!(!low.contains("market"), "{sample}");
            assert!(!low.contains("xp"), "{sample}");
            assert!(!low.contains("hud"), "{sample}");
            assert!(!sample.contains("Threshold"), "{sample}");
        }
    }

    /// STATUS-VIS-NEQ-1 SV5–SV8. Last tape of (Visibility, changed) on PeerPresenceRoot; settle 2 then clear. No file I/O (no ambient_peer_poll).
    #[derive(Resource, Default)] struct SvTape(Vec<(Visibility, bool)>);
    fn sv_tape(q: Query<Ref<Visibility>, With<PeerPresenceRoot>>, mut t: ResMut<SvTape>) { t.0.extend(q.iter().map(|v| (*v, v.is_changed()))); }
    fn sv_run(app: &mut App, n: usize) -> Vec<(Visibility, bool)> { for _ in 0..n { app.update(); } std::mem::take(&mut app.world_mut().resource_mut::<SvTape>().0) }
    fn sv_settle(mut app: App) -> App { app.update(); app.update(); app.world_mut().resource_mut::<SvTape>().0.clear(); app }
    fn sv_hide(app: &mut App) { let e = app.world_mut().query_filtered::<Entity, With<PeerPresenceRoot>>().single(app.world()).unwrap(); *app.world_mut().get_mut::<Visibility>(e).unwrap() = Visibility::Hidden; }
    fn sv_yielded(app: &App, id: &'static str) -> bool { app.world().resource::<crate::hud_presets::HudYielded>().0.contains(&id) }
    const SV: Visibility = Visibility::Visible;
    const SH: Visibility = Visibility::Hidden;
    fn sv_app(hud: Option<(crate::hud_presets::HudPresetId, Option<crate::title_screen::LaunchDoor>)>) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).init_resource::<LatticeFlowShare>().init_resource::<SvTape>().add_systems(Startup, spawn_peer_presence_chip).add_systems(Update, update_peer_presence_chip).add_systems(Last, sv_tape);
        if let Some((id, door)) = hud { app.add_plugins(crate::hud_presets::HudLayoutPlugin).insert_resource(crate::hud_presets::ActiveHudPreset { id: Some(id) }).init_resource::<crate::hex_travel::PlacesPlate>(); if let Some(door) = door { app.insert_resource(door); } }
        sv_settle(app)
    }
    /// The envelope_roundtrip peer, with the chip up or down.
    fn sv_peer(app: &mut App, chip_visible: bool) {
        let env = LatticeFlowShareEnvelope { schema: "powrush_lattice_flow_share_v1".into(), flow_total: 3.0, reserve_total: 1.5, choices_made: 4, last_path: Some("Flow outward".into()), mercy_note: "Voluntary abundance direction — never scarcity".into(), exported_at_secs: 12.0 };
        let mut share = app.world_mut().resource_mut::<LatticeFlowShare>(); share.chip_visible = chip_visible; share.last_peer = Some(env);
    }
    #[test] fn status_vis_neq_1_sv5_peer_steady_hidden_unmarked() { let mut app = sv_app(None); assert_eq!(sv_run(&mut app, 3), vec![(SH, false); 3], "SV5 peer steady hidden"); }
    #[test] fn status_vis_neq_1_sv6_peer_chip_flip_once_each_way() {
        let mut app = sv_app(None); sv_peer(&mut app, true);
        assert_eq!(sv_run(&mut app, 2), vec![(SV, true), (SV, false)], "SV6 peer chip up: Visible once, then quiet");
        app.world_mut().resource_mut::<LatticeFlowShare>().chip_visible = false;
        assert_eq!(sv_run(&mut app, 2), vec![(SH, true), (SH, false)], "SV6 peer chip down: Hidden once, then quiet");
    }
    #[test] fn status_vis_neq_1_sv7_peer_direct_hidden_restored_once() { let mut app = sv_app(None); sv_peer(&mut app, true); sv_run(&mut app, 2); sv_hide(&mut app); assert_eq!(sv_run(&mut app, 2), vec![(SV, true), (SV, false)], "SV7 peer direct Hidden write: restored once, then quiet"); }
    #[test] fn status_vis_neq_1_sv8_peer_real_hud_layout() {
        let mut app = sv_app(Some((crate::hud_presets::HudPresetId::Classic, Some(crate::title_screen::LaunchDoor::InYard)))); sv_peer(&mut app, true);
        assert_eq!(sv_run(&mut app, 2), vec![(SH, true); 2], "SV8 Classic InYard chip up: clean-hidden, changed every frame on main and branch alike");
        let mut app = sv_app(Some((crate::hud_presets::HudPresetId::Management, Some(crate::title_screen::LaunchDoor::InYard)))); sv_peer(&mut app, true); sv_run(&mut app, 1);
        assert_eq!(sv_run(&mut app, 2), vec![(SV, false); 2], "SV8 Management chip up: steady Visible, quiet");
        let mut app = sv_app(Some((crate::hud_presets::HudPresetId::Minimal, None))); sv_peer(&mut app, true); app.world_mut().spawn((Node::default(), Visibility::Visible, HudSlab(crate::hud_anchor_registry::ID_WATCH))); let t = sv_run(&mut app, 2);
        assert_eq!((t.last().unwrap().0, sv_yielded(&app, PEER.id)), (SH, true), "SV8 Minimal, no door, Watch sentinel: Peer yields by R2 into HudYielded");
    }

    /// CARD SAVE-FIXTURE-6 — the playable-preview tag's (11c577ef) own share export and its peer file
    /// (the practice writer) load through today's reader; the peer through `ambient_peer_poll`.
    /// `exported_at_secs` is the fixed fixture literal 600.0. No wall time: `Time` stays default (delta 0).
    #[test]
    fn save_fixture_6_playable_preview_share_and_peer_load() {
        use std::sync::atomic::{AtomicU64, Ordering};
        static N: AtomicU64 = AtomicU64::new(0);
        let share_raw = include_str!("../../tests/fixtures/save-preview/data/powrush_lattice_flow_share.json");
        let peer_raw = include_str!("../../tests/fixtures/save-preview/data/powrush_lattice_flow_share_peer.json");
        let dir = std::env::temp_dir().join(format!("powrush-sf6-{}-{}", std::process::id(), N.fetch_add(1, Ordering::Relaxed)));
        std::fs::create_dir_all(&dir).unwrap();
        // The tag's own logical names (tag lattice_flow_share.rs:19-20), not today's consts, so a rename is caught.
        for (logical, raw) in [("data/powrush_lattice_flow_share.json", share_raw), ("data/powrush_lattice_flow_share_peer.json", peer_raw)] {
            std::fs::write(dir.join(shared::user_persist::persist_file_name(logical)), raw).unwrap();
        }
        let key = shared::user_persist::USER_DIR_OVERRIDE_ENV;
        let _env = crate::test_env::lock();
        let prev = std::env::var(key).ok();
        std::env::set_var(key, &dir);
        let loaded = std::panic::catch_unwind(|| {
            let own = try_read_envelope(shared::user_persist::persist_path(SHARE_PATH));
            let mut app = App::new();
            app.init_resource::<Time>()
                .insert_resource(LatticeFlowShare { poll_accum: AMBIENT_POLL, ..Default::default() })
                .add_systems(Update, ambient_peer_poll);
            crate::test_env::single_thread(&mut app);
            app.update();
            (own, app.world_mut().remove_resource::<LatticeFlowShare>().unwrap())
        });
        match prev { Some(v) => std::env::set_var(key, v), None => std::env::remove_var(key) }
        let _ = std::fs::remove_dir_all(&dir);
        let (own, share) = loaded.unwrap();
        let same_text = |env: &LatticeFlowShareEnvelope, raw: &str| {
            let v: serde_json::Value = serde_json::from_str(raw).unwrap();
            assert_eq!(v["schema"].as_str(), Some(env.schema.as_str()));
            assert_eq!(Some(v["last_path"].as_str().expect("last_path saved")), env.last_path.as_deref());
            assert_eq!(v["mercy_note"].as_str(), Some(env.mercy_note.as_str()));
        };
        let own = own.expect("tag share export parses");
        same_text(&own, share_raw);
        assert_eq!((own.flow_total, own.reserve_total, own.choices_made, own.exported_at_secs), (1.0, 1.0, 2, 600.0));
        let peer = share.last_peer.as_ref().expect("ambient poll read the tag peer file");
        same_text(peer, peer_raw);
        assert_eq!((peer.flow_total, peer.reserve_total, peer.choices_made, peer.exported_at_secs), (2.0, 1.1, 34, 600.0));
        assert_eq!((share.chip_visible, share.ambient_seen_choices, share.poll_accum), (true, Some(34), 0.0));
    }
}
