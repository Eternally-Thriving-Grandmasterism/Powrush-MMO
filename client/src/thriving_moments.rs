/*!
 * Thriving Moments — soft, non-extractive joy feedback
 *
 * Celebrates meaningful firsts without achievement-hunting pressure.
 * Moments are presence markers, not leaderboard fuel.
 *
 * PATSAGi + TOLC 8: Joy gate without scarcity framing.
 * AG-SML v1.0 | Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;
use crate::hud_anchor_registry::{HudSlab, THRIVING};
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThrivingKind {
    FirstMercyHarvest,
    SurfaceCleared,
    PrincipleSealed,
    CouncilInvite,
    FirstInventoryOpen,
    FirstShare,
    FirstArrival,
    FirstVoice,
    FirstSpillWitness,
    FirstBind,
    FirstProofPack,
    FirstEmbassy,
    FirstWarWeek,
    FirstCrownstone,
    FirstRedemption,
    FirstHybrid,
    FirstCompass,
    FirstWell,
}

impl ThrivingKind {
    pub fn line(&self) -> &'static str {
        match self {
            ThrivingKind::FirstMercyHarvest => {
                "Thriving moment · A harvest tended with restraint — the node still glows"
            }
            ThrivingKind::SurfaceCleared => {
                "Thriving moment · One climate practiced — the principle travels with you"
            }
            ThrivingKind::PrincipleSealed => {
                "Thriving moment · Caps Across Climates sealed — same truth, three faces"
            }
            ThrivingKind::CouncilInvite => {
                "Invitation · A soft Council seat is open when you are ready"
            }
            ThrivingKind::FirstInventoryOpen => {
                "Thriving moment · Inventory open — abundance is held, not hoarded"
            }
            ThrivingKind::FirstShare => {
                "Thriving moment · Surplus shared — the yard remembered"
            }
            ThrivingKind::FirstArrival => {
                "The machine exists — a crate arrived"
            }
            ThrivingKind::FirstVoice => {
                "The card carried — the yard voted"
            }
            ThrivingKind::FirstSpillWitness => {
                "Spill is the witness — the pack is readable"
            }
            ThrivingKind::FirstBind => {
                "Bind, not a corpse — escort delivered"
            }
            ThrivingKind::FirstProofPack => {
                "The graph unlocked — repair and logi"
            }
            ThrivingKind::FirstEmbassy => {
                "Seated at the lamp — the book is yours"
            }
            ThrivingKind::FirstWarWeek => {
                "Hex gone green — tons plus restored"
            }
            ThrivingKind::FirstCrownstone => {
                "The stone is seen — path waits"
            }
            ThrivingKind::FirstRedemption => {
                "A tend offered — the grove answers"
            }
            ThrivingKind::FirstHybrid => {
                "Double vision — the ledger still holds"
            }
            ThrivingKind::FirstCompass => {
                "The air shifted — a cited wind"
            }
            ThrivingKind::FirstWell => {
                "You walked to the well — Mira stepped back"
            }
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct ThrivingMoments {
    pub fired: Vec<ThrivingKind>,
    pub queue: Vec<(ThrivingKind, f64)>,
    pub display_until: f64,
    pub current: Option<ThrivingKind>,
}

impl ThrivingMoments {
    pub fn try_fire(&mut self, kind: ThrivingKind, now: f64) {
        if self.fired.contains(&kind) {
            return;
        }
        self.fired.push(kind);
        self.queue.push((kind, now));
    }

    pub fn tick(&mut self, now: f64) {
        if self.current.is_some() && now < self.display_until {
            return;
        }
        self.current = None;
        if let Some((kind, _)) = self.queue.first().copied() {
            self.queue.remove(0);
            self.current = Some(kind);
            self.display_until = now + 5.0;
        }
    }
    /// HUD-TOP-YIELD-1 — the clock pauses only while yielded: hidden time moves `display_until`, so 5.0 s stay visible.
    pub fn hold(&mut self, yielded: bool, dt: f64) {
        if yielded && self.current.is_some() { self.display_until += dt; }
    }
}

#[derive(Component)]
pub struct ThrivingToastRoot;

#[derive(Component)]
pub struct ThrivingToastText;

pub struct ThrivingMomentsPlugin;

impl Plugin for ThrivingMomentsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ThrivingMoments>()
            .add_systems(Startup, spawn_toast)
            .add_systems(
                Update,
                (tick_moments, update_toast_ui, soft_inventory_moment),
            );
    }
}

fn spawn_toast(mut commands: Commands) {
    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: THRIVING.top(),
                    left: THRIVING.left(),
                    width: Val::Px(620.0),
                    margin: THRIVING.margin(),
                    padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG.with_alpha(1.0)),
                BorderColor::all(TITLE_BORDER.with_alpha(1.0)),
                Visibility::Hidden,
            ),
            ThrivingToastRoot,
            HudSlab(THRIVING.id),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(""),
TextFont { font_size: FontSize::Px(15.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                ThrivingToastText,
            ));
        });
}

fn tick_moments(time: Res<Time>, mut moments: ResMut<ThrivingMoments>, calm: Option<Res<crate::first_harvest_epiphany::CalmQueue>>, yielded: Option<Res<crate::hud_presets::HudYielded>>) {
    let now = time.elapsed_secs_f64(); // UI-CALM-1: the toast waits for its step; a pause keeps its clock (CL-U1)
    let left = match calm.as_deref().filter(|c| !c.done(now)) { Some(c) if !c.live(crate::first_harvest_epiphany::CalmStep::Thriving, now) => return, c => c.map(|c| c.left(now)) };
    if let (Some(l), Some(ThrivingKind::FirstMercyHarvest)) = (left, moments.current) { moments.display_until = now + l; }
    if left.is_none() && moments.current.is_some() && yielded.is_some_and(|y| y.0.contains(&crate::hud_anchor_registry::ID_THRIVING)) { moments.hold(true, time.delta_secs_f64()); } // CL-Y3
    moments.tick(now);
    if let (Some(l), Some(ThrivingKind::FirstMercyHarvest)) = (left, moments.current) { moments.display_until = now + l; }
}

fn update_toast_ui(
    moments: Res<ThrivingMoments>,
    mut root: Query<&mut Visibility, With<ThrivingToastRoot>>,
    mut text_q: Query<&mut Text, With<ThrivingToastText>>,
    (time, calm): (Option<Res<Time>>, Option<Res<crate::first_harvest_epiphany::CalmQueue>>),
) {
    let now = time.map_or(0.0, |t| t.elapsed_secs_f64());
    let show = moments.current.is_some() && calm.as_deref().filter(|c| !c.done(now)).map_or(true, |c| c.live(crate::first_harvest_epiphany::CalmStep::Thriving, now));
    for mut vis in &mut root {
        vis.set_if_neq(if show { Visibility::Visible } else { Visibility::Hidden });
    }
    if let Some(kind) = moments.current {
        for mut text in &mut text_q {
            **text = kind.line().to_string();
        }
    }
}

fn soft_inventory_moment(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        moments.try_fire(ThrivingKind::FirstInventoryOpen, time.elapsed_secs_f64());
    }
}

/// Fire from practice loop / harvest paths.
pub fn fire_thriving(moments: &mut ThrivingMoments, kind: ThrivingKind, now: f64) {
    moments.try_fire(kind, now);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// CARD HUD-ANCHOR-REGISTRY-2B — Thriving position is the registry, Style is the coded literal.
    #[test]
    fn thriving_style_byte_identical_to_coded_place() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_toast);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<&Node, With<ThrivingToastRoot>>();
        let style = q.single(app.world()).unwrap().clone();
        let coded = Node {
            position_type: PositionType::Absolute,
            top: Val::Px(48.0),
            left: Val::Percent(50.0),
            width: Val::Px(620.0),
            margin: UiRect::left(Val::Px(-310.0)),
            padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };
        assert_eq!(style, coded);
        assert_eq!(style.top, THRIVING.top());
        assert_eq!(style.left, THRIVING.left());
        assert_eq!(style.margin, THRIVING.margin());
        assert_eq!(style.width, Val::Px(THRIVING.width));
    }

    /// CARD VP-SLABS-REGAL-2 — the thriving moments toast rests on the title palette: opaque
    /// TITLE_PLATE_BG plate, TITLE_BORDER rim at alpha 1, TITLE_TEXT_PRIMARY text.
    #[test]
    fn slabs_regal2_thriving_toast_rests_on_title_palette() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_toast);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<(&BorderColor, &BackgroundColor), With<ThrivingToastRoot>>();
        let (border, bg) = q.single(app.world()).unwrap();
        let (border, bg) = (({
            assert_eq!(border.top, border.right, "border edges");
            assert_eq!(border.top, border.bottom, "border edges");
            assert_eq!(border.top, border.left, "border edges");
            border.top
        }).to_srgba(), bg.0.to_srgba());
        let mut t = app.world_mut().query_filtered::<&TextColor, With<ThrivingToastText>>();
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
    fn fires_once() {
        let mut m = ThrivingMoments::default();
        m.try_fire(ThrivingKind::FirstMercyHarvest, 1.0);
        m.try_fire(ThrivingKind::FirstMercyHarvest, 2.0);
        assert_eq!(m.fired.len(), 1);
        assert_eq!(m.queue.len(), 1);
    }

    #[test]
    fn council_invite_drops_c_hint_and_locked_lines_hold() {
        let council = ThrivingKind::CouncilInvite.line();
        assert!(!council.contains("(C"), "{council}");

        let changed = [
            ThrivingKind::CouncilInvite.line(),
            ThrivingKind::FirstMercyHarvest.line(),
            ThrivingKind::FirstShare.line(),
            ThrivingKind::FirstWell.line(),
        ];
        for line in changed {
            assert!(!line.contains("XP"), "{line}");
            assert!(!line.chars().any(|c| c.is_ascii_digit()), "{line}");
        }

        assert_one_peak_phrase(ThrivingKind::FirstMercyHarvest.line(), "tended");
        assert_one_peak_phrase(ThrivingKind::FirstShare.line(), "yard remembered");
        assert_one_peak_phrase(ThrivingKind::FirstWell.line(), "walked");

        assert_eq!(
            ThrivingKind::FirstVoice.line(),
            "The card carried — the yard voted"
        );
        assert_eq!(
            ThrivingKind::FirstWarWeek.line(),
            "Hex gone green — tons plus restored"
        );
        assert_eq!(
            ThrivingKind::FirstCrownstone.line(),
            "The stone is seen — path waits"
        );
        assert_eq!(
            ThrivingKind::FirstRedemption.line(),
            "A tend offered — the grove answers"
        );
        assert_eq!(
            ThrivingKind::FirstHybrid.line(),
            "Double vision — the ledger still holds"
        );
    }

    fn assert_one_peak_phrase(line: &str, phrase: &str) {
        const PHRASES: [&str; 4] = [
            "walked",
            "tended",
            "week was the bill",
            "yard remembered",
        ];
        assert!(line.contains(phrase), "{line}");
        assert_eq!(line.matches(phrase).count(), 1, "{line}");
        for other in PHRASES {
            if other == phrase {
                continue;
            }
            assert!(!line.contains(other), "{line} also cites {other}");
        }
    }

    #[test]
    fn hud_top_yield_pauses_then_resumes_for_five_seconds() {
        let (mut m, mut u) = (ThrivingMoments::default(), ThrivingMoments::default());
        (m.try_fire(ThrivingKind::FirstShare, 0.0), m.tick(0.0), u.try_fire(ThrivingKind::FirstShare, 0.0), u.tick(0.0));
        (0..24).for_each(|_| m.hold(true, 0.125));
        (m.hold(false, 1.0), m.tick(7.875), u.tick(4.875));
        assert!(m.current.is_some() && u.current.is_some(), "3 s yielded leaves 5 s shown");
        (m.tick(8.0), u.tick(5.0), u.hold(true, 0.5));
        assert!(m.current.is_none() && u.current.is_none() && u.display_until == 5.0, "expires on time; nothing current, nothing held");
        assert!(include_str!("thriving_moments.rs").contains(concat!("self.display_until = now + ", "5.0;")));
    }

    #[test]
    fn hud_top_yield_calm_burst_wins() {
        use bevy::ecs::system::RunSystemOnce;
        let (mut world, mut time, mut calm, mut m) = (World::new(), Time::<()>::default(), crate::first_harvest_epiphany::CalmQueue::default(), ThrivingMoments::default());
        time.advance_by(std::time::Duration::from_secs_f64(0.5)); calm.seed(-4.5, None, String::new()); m.try_fire(ThrivingKind::FirstMercyHarvest, 0.0);
        let left = calm.left(0.5);
        world.insert_resource(time); world.insert_resource(calm); world.insert_resource(m);
        world.insert_resource(crate::hud_presets::HudYielded(vec![crate::hud_anchor_registry::ID_THRIVING]));
        world.run_system_once(tick_moments).unwrap();
        assert_eq!(world.resource::<ThrivingMoments>().display_until, 0.5 + left, "CalmQueue time, no double extension");
    }

    /// TOAST-VIS-NEQ-1 TV5–TV8. Last tape of (Visibility, changed) on ThrivingToastRoot; settle 2 then clear; 100 ms manual steps.
    #[derive(Resource, Default)] struct TvTape(Vec<(Visibility, bool)>);
    fn tv_tape(q: Query<Ref<Visibility>, With<ThrivingToastRoot>>, mut t: ResMut<TvTape>) { t.0.extend(q.iter().map(|v| (*v, v.is_changed()))); }
    fn tv_run(app: &mut App, n: usize) -> Vec<(Visibility, bool)> { for _ in 0..n { app.update(); } std::mem::take(&mut app.world_mut().resource_mut::<TvTape>().0) }
    fn tv_settle(mut app: App) -> App { app.update(); app.update(); app.world_mut().resource_mut::<TvTape>().0.clear(); app }
    fn tv_hide(app: &mut App) { let e = app.world_mut().query_filtered::<Entity, With<ThrivingToastRoot>>().single(app.world()).unwrap(); *app.world_mut().get_mut::<Visibility>(e).unwrap() = Visibility::Hidden; }
    const TV: Visibility = Visibility::Visible;
    const TH: Visibility = Visibility::Hidden;
    /// `hud` adds tick_moments with the real HudLayoutPlugin.
    fn tv_app(hud: Option<crate::hud_presets::HudPresetId>) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).init_resource::<ThrivingMoments>().init_resource::<TvTape>().insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::from_millis(100))).add_systems(Startup, spawn_toast).add_systems(Update, update_toast_ui).add_systems(Last, tv_tape);
        if hud.is_some() { app.add_systems(Update, tick_moments); }
        if let Some(id) = hud { app.add_plugins(crate::hud_presets::HudLayoutPlugin).insert_resource(crate::hud_presets::ActiveHudPreset { id: Some(id) }).insert_resource(crate::title_screen::LaunchDoor::InYard).init_resource::<crate::hex_travel::PlacesPlate>(); }
        tv_settle(app)
    }
    fn tv_current(app: &mut App, kind: Option<ThrivingKind>) { app.world_mut().resource_mut::<ThrivingMoments>().current = kind; }
    fn tv_fire(app: &mut App) { let now = app.world().resource::<Time>().elapsed_secs_f64(); app.world_mut().resource_mut::<ThrivingMoments>().try_fire(ThrivingKind::FirstShare, now); }
    #[test] fn toast_vis_neq_1_tv5_thriving_steady_hidden_unmarked() { let mut app = tv_app(None); assert_eq!(tv_run(&mut app, 3), vec![(TH, false); 3], "TV5 thriving steady hidden"); }
    #[test] fn toast_vis_neq_1_tv6_thriving_one_change_each_way() {
        let mut app = tv_app(None); tv_current(&mut app, Some(ThrivingKind::FirstShare));
        assert_eq!(tv_run(&mut app, 2), vec![(TV, true), (TV, false)], "TV6 thriving current: Visible once, then quiet");
        tv_current(&mut app, None);
        assert_eq!(tv_run(&mut app, 2), vec![(TH, true), (TH, false)], "TV6 thriving cleared: Hidden once, then quiet");
    }
    #[test] fn toast_vis_neq_1_tv7_thriving_direct_hidden_restored_once() { let mut app = tv_app(None); tv_current(&mut app, Some(ThrivingKind::FirstShare)); tv_run(&mut app, 2); tv_hide(&mut app); assert_eq!(tv_run(&mut app, 2), vec![(TV, true), (TV, false)], "TV7 thriving direct Hidden write: restored once, then quiet"); }
    #[test] fn toast_vis_neq_1_tv8_thriving_real_hud_layout_with_tick() {
        let mut app = tv_app(Some(crate::hud_presets::HudPresetId::Classic)); tv_fire(&mut app);
        assert_eq!(tv_run(&mut app, 2), vec![(TH, true); 2], "TV8 Classic InYard current: clean-hidden, changed every frame on main and branch alike");
        let mut app = tv_app(Some(crate::hud_presets::HudPresetId::Management)); tv_fire(&mut app); tv_run(&mut app, 1);
        assert_eq!(tv_run(&mut app, 2), vec![(TV, false); 2], "TV8 Management current: steady Visible, quiet");
        app.world_mut().spawn((Node::default(), Visibility::Visible, HudSlab(crate::hud_anchor_registry::ID_PULSE)));
        tv_run(&mut app, 1); let before = app.world().resource::<ThrivingMoments>().display_until;
        let t = tv_run(&mut app, 2); let yielded = app.world().resource::<crate::hud_presets::HudYielded>().0.contains(&THRIVING.id);
        let grew = app.world().resource::<ThrivingMoments>().display_until - before;
        assert_eq!((t, yielded), (vec![(TH, true); 2], true), "TV8 Management with a Pulse sentinel: Thriving yields by R2 into HudYielded");
        assert!((grew - 0.2).abs() < 1e-9, "TV8 CL-Y3 hold: display_until grows by dt per yielded frame, got {grew}");
    }

    /// THRIVING-TOAST-TEXT-NEQ-1 TT1–TT5. Last tape of (Text, changed) on ThrivingToastText over the tv_app builder; settle 2 then clear.
    /// Rows are compared as (text == kind.line(), changed), so no copy is quoted.
    #[derive(Resource, Default)] struct TtTape(Vec<(String, bool)>);
    fn tt_tape(q: Query<Ref<Text>, With<ThrivingToastText>>, mut t: ResMut<TtTape>) { t.0.extend(q.iter().map(|x| (x.0.clone(), x.is_changed()))); }
    fn tt_app() -> App { let mut app = tv_app(None); app.init_resource::<TtTape>().add_systems(Last, tt_tape); (app.update(), app.update()); tt_run(&mut app, 0); app }
    fn tt_run(app: &mut App, n: usize) -> Vec<(String, bool)> { for _ in 0..n { app.update(); } std::mem::take(&mut app.world_mut().resource_mut::<TtTape>().0) }
    fn tt_is(rows: &[(String, bool)], kind: ThrivingKind) -> Vec<(bool, bool)> { rows.iter().map(|(s, c)| (s == kind.line(), *c)).collect() }
    fn tt_text(app: &mut App) -> Entity { app.world_mut().query_filtered::<Entity, With<ThrivingToastText>>().single(app.world()).unwrap() }
    #[test] fn thriving_toast_text_neq_1_tt1_current_line_written_once() {
        let mut app = tt_app(); tv_current(&mut app, Some(ThrivingKind::FirstShare));
        assert_eq!(tt_is(&tt_run(&mut app, 4), ThrivingKind::FirstShare), vec![(true, true), (true, false), (true, false), (true, false)], "TT1 FirstShare current: line written once, then quiet");
    }
    #[test] fn thriving_toast_text_neq_1_tt2_none_steady_quiet() {
        let mut app = tt_app(); let t = tt_run(&mut app, 3);
        assert!(t.len() == 3 && t.iter().all(|r| !r.1 && r.0.is_empty()), "TT2 nothing current: text keeps its spawn value, quiet: {:?}", t.iter().map(|r| r.1).collect::<Vec<_>>());
    }
    #[test] fn thriving_toast_text_neq_1_tt3_switch_changes_once() {
        let mut app = tt_app(); tv_current(&mut app, Some(ThrivingKind::FirstShare)); tt_run(&mut app, 2); tv_current(&mut app, Some(ThrivingKind::FirstArrival));
        assert_eq!(tt_is(&tt_run(&mut app, 2), ThrivingKind::FirstArrival), vec![(true, true), (true, false)], "TT3 switch to FirstArrival: new line once, then quiet");
    }
    #[test] fn thriving_toast_text_neq_1_tt4_poke_restored_once() {
        let mut app = tt_app(); tv_current(&mut app, Some(ThrivingKind::FirstShare)); tt_run(&mut app, 2);
        let e = tt_text(&mut app); app.world_mut().get_mut::<Text>(e).unwrap().0.clear();
        assert_eq!(tt_is(&tt_run(&mut app, 2), ThrivingKind::FirstShare), vec![(true, true), (true, false)], "TT4 text poked empty: restored once, then quiet");
    }
    #[test] fn thriving_toast_text_neq_1_tt5_cleared_keeps_last_line_quiet() {
        let mut app = tt_app(); tv_current(&mut app, Some(ThrivingKind::FirstShare)); tt_run(&mut app, 2); tv_current(&mut app, None);
        assert_eq!(tt_is(&tt_run(&mut app, 2), ThrivingKind::FirstShare), vec![(true, false); 2], "TT5 current cleared: text keeps the last line, quiet");
    }
}
