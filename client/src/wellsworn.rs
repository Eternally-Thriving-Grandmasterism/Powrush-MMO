//! WELLSWORN-1 — Draw From the Well. Sanctuary plaque, oath book and Stavehold gate.
//! Draw at the Verdant well, pour on the ninth stave, Swear: Drawn to Sworn. Rank saved.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use shared::{heartwood_lamp::dist_xz, hex_travel::PlaceId, local_settings::{LocalSettings, PeaceKey}};

use crate::{first_harvest_epiphany::FirstHarvestEpiphany, hex_travel::HexTravelState, human_presence::SoftPresence};
use crate::{local_settings::LocalSettingsState, mercy_harvest_nodes::NearbyMercyNode, soft_play_bindings::peace_key_code};

/// COUNCIL-RULING-2026-10-30-NIGHT §5 Concept 1, CUT 6539749807. In-game only.
pub const OATH: [&str; 6] = [
    "By the Verdant well's water and the gate I keep,",
    "I stand where Sanctuary ends and the Contested ground begins.",
    "What hungers to copy us will find no copy here.",
    "I hold the line so others may hold their rest.",
    "I watch for those still lost, and for any who come home.",
    "I am the ninth stave. I do not break.",
];
pub const PLAQUE_TEXT: &str = "Stavehold gate. Nine staves stand where Sanctuary meets the Contested ground; the ninth is uncarved.";
pub const QUEST_TEXT: &str = "Draw From the Well: draw at the Verdant well, carry the flask to the gate, pour it on the ninth stave.";
pub const SAVE_NAME: &str = "powrush_wellsworn.json";
/// Sanctuary ground (x, z), south of the three wells.
pub const PLAQUE_SPOT: [f32; 2] = [-5.0, -10.0];
pub const BOOK_SPOT: [f32; 2] = [5.0, -10.0];
pub const STAVEHOLD_GATE_SPOT: [f32; 2] = [0.0, -11.0];
pub const SPOT_REACH: f32 = 2.0;
const WELL_NAME: &str = "Verdant well";

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WellswornRank { Drawn, Sworn, Stave, Wellkeeper }
/// Session only: the flask is not saved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawStep { #[default] Empty, Flask, Poured }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spot { Plaque, Book, Gate }
#[derive(Resource, Debug, Default)]
pub struct WellswornState { pub rank: Option<WellswornRank>, pub step: DrawStep, pub near: Option<Spot>, pub open: bool, seen: (u32, u32) }

impl WellswornState {
    /// Rise in the harvest/tend counters at the Verdant well fills the flask.
    pub fn observe(&mut self, counts: (u32, u32), at_well: bool) {
        let rose = counts.0 > self.seen.0 || counts.1 > self.seen.1;
        self.seen = counts;
        if rose && at_well && self.step == DrawStep::Empty && matches!(self.rank, None | Some(WellswornRank::Drawn)) {
            self.step = DrawStep::Flask;
            if self.rank.is_none() { self.rank = Some(WellswornRank::Drawn); }
        }
    }
    pub fn can_swear(&self) -> bool { self.open && self.step == DrawStep::Poured && self.near != Some(Spot::Plaque) }
    pub fn press(&mut self) {
        match self.near {
            Some(Spot::Gate) if self.step == DrawStep::Flask => { self.step = DrawStep::Poured; self.open = true; }
            Some(_) if self.can_swear() && matches!(self.rank, None | Some(WellswornRank::Drawn)) => { self.rank = Some(WellswornRank::Sworn); self.step = DrawStep::Empty; }
            Some(_) => self.open = !self.open,
            None => {}
        }
    }
    pub fn page(&self, key: &str) -> String {
        let rank = self.rank.map(|r| format!("\n\nRank: {r:?}")).unwrap_or_default();
        match self.near {
            Some(Spot::Plaque) => format!("{PLAQUE_TEXT}\n\n{QUEST_TEXT}{rank}"),
            _ if self.can_swear() => format!("{}\n\n[{key}] Swear", OATH.join("\n")),
            _ if matches!(self.rank, Some(WellswornRank::Sworn | WellswornRank::Stave | WellswornRank::Wellkeeper)) => {
                format!("{}\n\n{VOWS_LINE}\n{NINTH_STAVE_LINE}{rank}\n\n{ECHO_LINE}", OATH.join("\n"))
            }
            _ => format!("{}{rank}", OATH.join("\n")),
        }
    }
}
/// WELLSWORN-ECHO-1, CUT 6548784036. Sworn page only, in-game only.
pub const VOWS_LINE: &str = "The eight staves: Truth, Mercy, Dawn, Shelter, Ground, Return, Rest, Watch.";
pub const NINTH_STAVE_LINE: &str = "You are the ninth stave.";
pub const ECHO_LINE: &str = "Watch is my vow. Rest is yours.";
pub fn spot_at(place: PlaceId, x: f32, z: f32) -> Option<Spot> {
    let spots = [(PLAQUE_SPOT, Spot::Plaque), (BOOK_SPOT, Spot::Book), (STAVEHOLD_GATE_SPOT, Spot::Gate)];
    spots.into_iter().find(|(s, _)| place == PlaceId::Sanctuary && dist_xz(x, z, s[0], s[1]) <= SPOT_REACH).map(|(_, spot)| spot)
}
pub fn decode_rank(raw: Option<&str>) -> Option<WellswornRank> { raw.and_then(|s| serde_json::from_str(s).ok()).flatten() }
pub fn encode_rank(rank: Option<WellswornRank>) -> String { serde_json::to_string(&rank).unwrap_or_default() }
pub fn load_rank() -> Option<WellswornRank> { decode_rank(shared::user_persist::read_named(SAVE_NAME).ok().as_deref()) }
pub fn save_rank(rank: Option<WellswornRank>) { let _ = shared::user_persist::write_named(SAVE_NAME, encode_rank(rank)); }
fn use_key(cfg: Option<&LocalSettingsState>) -> PeaceKey { cfg.map(|s| s.inner.key_use).unwrap_or_else(|| LocalSettings::default().key_use) }
#[derive(Component)]
struct WellswornSlab;
pub struct WellswornPlugin;
impl Plugin for WellswornPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WellswornState>().add_systems(Startup, (load_state, spawn_slab)).add_systems(PreUpdate, mark_near)
            .add_systems(Update, (track_draw, handle_use, update_slab).chain());
    }
}
fn load_state(mut state: ResMut<WellswornState>) { state.rank = load_rank(); }
fn spawn_slab(mut commands: Commands) {
    let node = Node { position_type: PositionType::Absolute, top: Val::Px(164.0), left: Val::Px(16.0), width: Val::Px(420.0), padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)), border: UiRect::all(Val::Px(1.0)), ..default() };
    commands.spawn((node, BackgroundColor(Color::srgba(0.06, 0.08, 0.10, 0.92)), BorderColor::all(Color::srgba(0.52, 0.66, 0.78, 0.55)), Visibility::Hidden, WellswornSlab))
        .with_children(|p| { p.spawn((Text::new(""), TextFont { font_size: FontSize::Px(14.0 / 1.2), ..default() }, TextColor(Color::srgb(0.88, 0.92, 0.96)))); });
}
/// PreUpdate so the harvest tap sees `wellsworn_near` on the same Use edge. Writes only on change.
fn mark_near(travel: Res<HexTravelState>, presence: Res<SoftPresence>, mut state: ResMut<WellswornState>, epiphany: Option<ResMut<FirstHarvestEpiphany>>) {
    let near = spot_at(travel.current, presence.position.x, presence.position.z);
    if state.near != near { state.near = near; if near.is_none() { state.open = false; } }
    if let Some(mut epi) = epiphany.filter(|e| e.wellsworn_near != near.is_some()) { epi.wellsworn_near = near.is_some(); }
}
fn track_draw(epi: Option<Res<FirstHarvestEpiphany>>, node: Option<Res<NearbyMercyNode>>, mut state: ResMut<WellswornState>) {
    let (Some(epi), Some(node)) = (epi, node) else { return };
    let before = state.rank;
    state.observe((epi.harvests_this_session, epi.tends_this_session), node.in_range && node.name == Some(WELL_NAME));
    if state.rank != before { save_rank(state.rank); }
}
fn handle_use(keys: Res<ButtonInput<KeyCode>>, settings: Option<Res<LocalSettingsState>>, mut state: ResMut<WellswornState>) {
    if !keys.just_pressed(peace_key_code(use_key(settings.as_deref()))) { return; }
    let before = state.rank;
    state.press();
    if state.rank != before { save_rank(state.rank); }
}
fn update_slab(state: Res<WellswornState>, settings: Option<Res<LocalSettingsState>>, mut root: Query<(&mut Visibility, &Children), With<WellswornSlab>>, mut text: Query<&mut Text>) {
    let want = if state.open && state.near.is_some() { Visibility::Visible } else { Visibility::Hidden };
    for (mut vis, kids) in &mut root {
        if *vis != want { *vis = want; }
        let page = state.page(use_key(settings.as_deref()).display_label());
        for k in kids.iter() { if let Ok(mut t) = text.get_mut(k) { if t.0 != page { t.0 = page.clone(); } } }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::skirmish_well::{CONTEST_REACH, WELL_ANCHORS};
    fn app_at(place: PlaceId, s: [f32; 2]) -> App {
        let mut app = App::new();
        app.insert_resource(HexTravelState { current: place }).insert_resource(SoftPresence { position: Vec3::new(s[0], 0.9, s[1]), ..Default::default() });
        app.init_resource::<FirstHarvestEpiphany>().init_resource::<WellswornState>().add_systems(PreUpdate, mark_near);
        app.update(); app
    }
    fn claimed(app: &App) -> bool { app.world().resource::<FirstHarvestEpiphany>().harvest_use_is_claimed() }
    fn at(spot: Spot) -> WellswornState { WellswornState { near: Some(spot), ..Default::default() } }
    #[test]
    fn use_near_plaque_book_or_gate_does_not_harvest() {
        for s in [PLAQUE_SPOT, BOOK_SPOT, STAVEHOLD_GATE_SPOT] { assert!(claimed(&app_at(PlaceId::Sanctuary, s))); }
        assert!(!claimed(&app_at(PlaceId::Heartwood, PLAQUE_SPOT)), "outside Sanctuary");
        let mut app = app_at(PlaceId::Sanctuary, BOOK_SPOT);
        app.world_mut().resource_mut::<SoftPresence>().position.x += SPOT_REACH + 1.0;
        app.update();
        assert!(!claimed(&app), "cleared outside reach");
    }
    #[test]
    fn draw_carry_pour_swear_is_sworn_and_survives_save() {
        let mut s = WellswornState::default();
        s.observe((1, 0), false);
        assert_eq!((s.step, s.rank), (DrawStep::Empty, None), "other node does not draw");
        s.observe((1, 1), true);
        assert_eq!((s.step, s.rank), (DrawStep::Flask, Some(WellswornRank::Drawn)));
        s.near = Some(Spot::Gate); s.press();
        assert!(s.can_swear() && s.page("F").ends_with("[F] Swear"));
        s.press();
        assert_eq!(s.rank, Some(WellswornRank::Sworn));
        let dir = std::env::temp_dir().join(format!("powrush-wellsworn-{}", std::process::id()));
        let _guard = crate::test_env::lock();
        let (env, prev) = (shared::user_persist::USER_DIR_OVERRIDE_ENV, std::env::var(shared::user_persist::USER_DIR_OVERRIDE_ENV).ok());
        std::env::set_var(env, &dir);
        save_rank(s.rank); let back = load_rank();
        match prev { Some(v) => std::env::set_var(env, v), None => std::env::remove_var(env) }
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(back, Some(WellswornRank::Sworn));
    }
    #[test]
    fn swear_needs_a_poured_flask() {
        for spot in [Spot::Book, Spot::Gate] {
            let mut s = at(spot);
            s.press();
            assert!(s.open && !s.can_swear() && !s.page("F").contains("Swear"));
            s.press(); assert_eq!(s.rank, None);
        }
        let mut s = WellswornState { step: DrawStep::Flask, ..at(Spot::Book) };
        (0..2).for_each(|_| s.press());
        assert_eq!((s.step, s.rank), (DrawStep::Flask, None), "pour only at the gate");
    }
    #[test]
    fn spots_are_clear_of_the_wells() {
        let reach = crate::mercy_harvest_nodes::HARVEST_REACH;
        for &(x, _, z) in WELL_ANCHORS.iter() {
            for s in [PLAQUE_SPOT, BOOK_SPOT] { assert!(dist_xz(s[0], s[1], x, z) > reach); }
            assert!(dist_xz(STAVEHOLD_GATE_SPOT[0], STAVEHOLD_GATE_SPOT[1], x, z) > reach + CONTEST_REACH);
        }
    }
    #[test]
    fn oath_is_the_six_cut_lines() {
        assert_eq!(OATH.join("\n"), "By the Verdant well's water and the gate I keep,\n\
         I stand where Sanctuary ends and the Contested ground begins.\n\
         What hungers to copy us will find no copy here.\n\
         I hold the line so others may hold their rest.\n\
         I watch for those still lost, and for any who come home.\n\
         I am the ninth stave. I do not break.");
    }
    #[test]
    fn save_missing_junk_drawn_sworn() {
        for junk in [None, Some("not json"), Some("\"Oathbreaker\"")] { assert_eq!(decode_rank(junk), None, "missing or junk"); }
        for r in [None, Some(WellswornRank::Drawn), Some(WellswornRank::Sworn)] { assert_eq!(decode_rank(Some(&encode_rank(r))), r); }
    }
    #[test]
    fn sworn_page_has_vows_and_echo_last() {
        let s = WellswornState { rank: Some(WellswornRank::Sworn), ..at(Spot::Book) };
        let page = s.page("F");
        assert_eq!(page, "By the Verdant well's water and the gate I keep,\n\
         I stand where Sanctuary ends and the Contested ground begins.\n\
         What hungers to copy us will find no copy here.\n\
         I hold the line so others may hold their rest.\n\
         I watch for those still lost, and for any who come home.\n\
         I am the ninth stave. I do not break.\n\
         \n\
         The eight staves: Truth, Mercy, Dawn, Shelter, Ground, Return, Rest, Watch.\n\
         You are the ninth stave.\n\
         \n\
         Rank: Sworn\n\
         \n\
         Watch is my vow. Rest is yours.");
        assert!(page.ends_with("\nWatch is my vow. Rest is yours."));
        assert_eq!(page.lines().last(), Some(ECHO_LINE));
    }
    #[test]
    fn vows_are_the_eight_staves_in_canon_order() {
        assert_eq!(VOWS_LINE, "The eight staves: Truth, Mercy, Dawn, Shelter, Ground, Return, Rest, Watch.");
    }
    #[test]
    fn drawn_and_unranked_pages_are_unchanged() {
        for (rank, suffix) in [(None, ""), (Some(WellswornRank::Drawn), "\n\nRank: Drawn")] {
            let page = WellswornState { rank, ..at(Spot::Book) }.page("F");
            assert_eq!(page, format!("{}{suffix}", OATH.join("\n")));
            assert!(!page.contains(ECHO_LINE) && !page.contains("The eight staves"));
        }
    }
    #[test]
    fn stave_and_wellkeeper_keep_rank_through_draw_pour_and_book() {
        for r in [WellswornRank::Stave, WellswornRank::Wellkeeper] {
            let mut s = WellswornState { rank: Some(r), ..Default::default() };
            s.observe((1, 1), true);
            assert_eq!((s.step, s.rank), (DrawStep::Empty, Some(r)), "no flask for {r:?}");
            assert!(!s.page("F").contains("Swear"));
            for spot in [Spot::Gate, Spot::Book] {
                s.near = Some(spot); s.press();
                assert_eq!((s.step, s.rank), (DrawStep::Empty, Some(r)), "press at {spot:?}");
                assert!(!s.page("F").contains("Swear") && s.page("F").ends_with(ECHO_LINE));
            }
            let mut p = WellswornState { rank: Some(r), step: DrawStep::Poured, open: true, ..at(Spot::Book) };
            p.press();
            assert_eq!(p.rank, Some(r), "press never lowers {r:?}");
        }
    }
    #[test]
    fn drawn_press_still_swears() {
        let mut s = WellswornState { rank: Some(WellswornRank::Drawn), ..Default::default() };
        s.observe((0, 1), true);
        assert_eq!(s.step, DrawStep::Flask);
        s.near = Some(Spot::Gate); s.press(); s.press();
        assert_eq!((s.step, s.rank), (DrawStep::Empty, Some(WellswornRank::Sworn)));
    }
    #[test]
    fn stave_and_wellkeeper_survive_reload_and_press() {
        let dir = std::env::temp_dir().join(format!("powrush-wellsworn-marks-{}", std::process::id()));
        let _guard = crate::test_env::lock();
        let (env, prev) = (shared::user_persist::USER_DIR_OVERRIDE_ENV, std::env::var(shared::user_persist::USER_DIR_OVERRIDE_ENV).ok());
        std::env::set_var(env, &dir);
        let mut back = Vec::new();
        for r in [WellswornRank::Stave, WellswornRank::Wellkeeper] {
            save_rank(Some(r));
            let mut s = WellswornState { rank: load_rank(), ..at(Spot::Gate) };
            s.observe((1, 1), true); s.press(); s.press();
            back.push(s.rank);
        }
        match prev { Some(v) => std::env::set_var(env, v), None => std::env::remove_var(env) }
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(back, vec![Some(WellswornRank::Stave), Some(WellswornRank::Wellkeeper)]);
    }
}
