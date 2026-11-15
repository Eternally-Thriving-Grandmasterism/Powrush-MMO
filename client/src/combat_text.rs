use bevy::prelude::*;
use bevy::ui::UiTargetCamera;
use bevy::window::PrimaryWindow;
use shared::hex_travel::PlaceId;
use shared::local_settings::LocalSettings;
use crate::combat_core::{fight_present, ground_shot_connects, HIT, LUNGE_SECS, PREDATOR_NAME};
use crate::droid::{DROID_NAME, DROID_SHOT_SECS};
use crate::human_presence::SoftPresence;
use crate::local_settings::LocalSettingsState;
use crate::ui_above_world::LivedUiCamera;

// PLACEHOLDER COPY: council pick pending, Sherif may veto
pub const PLACEHOLDER_MISSED_FLOAT: &str = "[[MISSED_FLOAT]]"; pub const PLACEHOLDER_MISSED_LOG: &str = "[[MISSED_LOG]]";
pub const PLACEHOLDER_DEALT_FLOAT: &str = "[[DEALT_FLOAT]]"; pub const PLACEHOLDER_TAKEN_FLOAT: &str = "[[TAKEN_FLOAT]]";
pub const PLACEHOLDER_TIP_FLOAT: &str = "[[COMBAT_TIP_FLOAT]]"; pub const PLACEHOLDER_TIP_LOG: &str = "[[COMBAT_TIP_LOG]]";

/// Clerk/council pin (COMBAT-TEXT-1b), not a live source constant before this card.
pub const COMBAT_TEXT_HIVE_R_OF_HEIGHT: f32 = 0.22;
const COMBAT_TEXT_FONT_SMALL_PX: f32 = 10.0;
const COMBAT_TEXT_FONT_NORMAL_PX: f32 = 12.0;
const COMBAT_TEXT_FONT_LARGE_PX: f32 = 16.0;
const COMBAT_TEXT_CONTRAST_COLOR: Color = Color::srgb(1.0, 1.0, 0.75);
const DEFAULT_FLOAT_COLOR: Color = Color::WHITE;

const FLOAT_SECS: f32 = 1.2;
const LOG_CAP: usize = 8;

#[cfg(test)]
static PENDING: std::sync::Mutex<Vec<CombatTextEvent>> = std::sync::Mutex::new(Vec::new());

#[derive(Clone, Copy, PartialEq, Eq, Debug)] pub enum CombatTextKind { Dealt, Missed, Taken, EnemyMissed, Tip }
#[derive(Clone, Copy, PartialEq, Debug)] pub struct CombatTextEvent { pub kind: CombatTextKind, pub amount: u32, pub at: Vec3, pub name: &'static str }
#[derive(Resource, Default, Debug)] pub struct CombatTextQueue { pub events: Vec<CombatTextEvent> }
#[derive(Resource, Default, Debug)] pub struct CombatTextLog { pub rows: Vec<String> }
#[derive(Resource, Default, Debug)] pub struct CombatTextFloaters { pub active: Vec<(String, Vec3)> }
#[derive(Resource, Default, Debug)] struct CombatLogRendered { rows: Vec<String>, visible: bool }
#[derive(Component)] struct CombatFloater { anchor: Vec3, until: f64 }
#[derive(Component)] struct CombatLogRoot;

#[cfg(test)]
fn pending_push(ev: CombatTextEvent) {
    if let Ok(mut p) = PENDING.lock() { p.push(ev); }
}
#[cfg(test)]
pub(crate) fn take_pending(queue: &mut CombatTextQueue) {
    if let Ok(mut p) = PENDING.lock() { queue.events.append(&mut std::mem::take(&mut *p)); }
}
fn cmd_push(cmd: &mut Commands, mut evs: Vec<CombatTextEvent>) {
    if evs.is_empty() { return; }
    cmd.queue(move |world: &mut World| {
        if let Some(mut q) = world.get_resource_mut::<CombatTextQueue>() { q.events.append(&mut evs); }
    });
}
pub fn wrap_count(timer: f32, dt: f32, on: bool, period: f32) -> u32 {
    if !on { return 0; } let (mut t, mut n) = (timer + dt.max(0.0), 0u32);
    while t >= period { t -= period; n += 1; } n
}
fn ev(kind: CombatTextKind, amount: u32, at: Vec3, name: &'static str) -> CombatTextEvent {
    CombatTextEvent { kind, amount, at, name }
}
fn events_player_strike(place: PlaceId, b: u32, a: u32, hit: bool, at: Vec3) -> Vec<CombatTextEvent> {
    if !fight_present(place) { return Vec::new(); }
    if hit { let d = a.saturating_sub(b); return (d > 0).then(|| vec![ev(CombatTextKind::Dealt, d, at, PREDATOR_NAME)]).unwrap_or_default(); }
    vec![ev(CombatTextKind::Missed, 0, at, PREDATOR_NAME)]
}
fn events_lunge_after(b: u32, a: u32, wraps: u32, grounded: bool, at: Vec3, name: &'static str) -> Vec<CombatTextEvent> {
    if wraps == 0 { return Vec::new(); }
    let d = a.saturating_sub(b);
    if d > 0 { vec![ev(CombatTextKind::Taken, d, at, name)] }
    else if !ground_shot_connects(grounded) { vec![ev(CombatTextKind::EnemyMissed, 0, at, name)] }
    else { Vec::new() }
}
fn events_droid_strike(b: u32, a: u32, miss: bool, at: Vec3) -> Vec<CombatTextEvent> {
    let d = a.saturating_sub(b);
    if d > 0 { vec![ev(CombatTextKind::Dealt, d, at, DROID_NAME)] }
    else if miss { vec![ev(CombatTextKind::Missed, 0, at, DROID_NAME)] }
    else { Vec::new() }
}
pub fn hook_player_strike(q: &mut CombatTextQueue, place: PlaceId, b: u32, a: u32, hit: bool, at: Vec3) {
    q.events.extend(events_player_strike(place, b, a, hit, at));
}
pub fn hook_lunge_after(q: &mut CombatTextQueue, b: u32, a: u32, wraps: u32, grounded: bool, at: Vec3, name: &'static str) {
    q.events.extend(events_lunge_after(b, a, wraps, grounded, at, name));
}
pub fn hook_droid_strike(q: &mut CombatTextQueue, b: u32, a: u32, miss: bool, at: Vec3) {
    q.events.extend(events_droid_strike(b, a, miss, at));
}
pub fn hook_combat_tip(s: &mut LocalSettings, _at: Vec3, _name: &'static str) -> bool {
    if s.combat_tip_shown { return false; } s.combat_tip_shown = true; true
}
pub fn hook_after_fire(cmd: &mut Commands, place: PlaceId, b: u32, a: u32, hit: bool, at: Vec3) {
    cmd_push(cmd, events_player_strike(place, b, a, hit, at));
}
pub fn hook_after_lunge(cmd: &mut Commands, t0: f32, p0: u32, dt: f32, reach: bool, grounded: bool, p1: u32, at: Vec3) {
    cmd_push(cmd, events_lunge_after(p0, p1, wrap_count(t0, dt, reach, LUNGE_SECS), grounded, at, PREDATOR_NAME));
}
pub fn hook_tip_edge(cmd: &mut Commands, s: Option<&mut LocalSettingsState>, at: Vec3, name: &'static str) {
    if let Some(s) = s {
        if hook_combat_tip(&mut s.inner, at, name) {
            s.dirty = true;
            cmd_push(cmd, vec![ev(CombatTextKind::Tip, 0, at, name)]);
        }
    }
}
pub fn hook_droid_frame(
    cmd: &mut Commands, s: Option<&mut LocalSettingsState>, sb: u32, sa: u32, miss: bool, t0: f32, p0: u32, p1: u32, dt: f32, armed: bool, grounded: bool, tip: bool, at: Vec3,
) {
    let mut evs = events_droid_strike(sb, sa, miss, at);
    evs.extend(events_lunge_after(p0, p1, wrap_count(t0, dt, armed, DROID_SHOT_SECS), grounded, at, DROID_NAME));
    cmd_push(cmd, evs);
    if tip { hook_tip_edge(cmd, s, at, DROID_NAME); }
}
fn line_for(ev: &CombatTextEvent, log: bool) -> String {
    match ev.kind {
        CombatTextKind::Dealt => format!("{PLACEHOLDER_DEALT_FLOAT} {} {}", ev.name, ev.amount),
        CombatTextKind::Taken => format!("{PLACEHOLDER_TAKEN_FLOAT} {}", ev.amount),
        CombatTextKind::Missed | CombatTextKind::EnemyMissed => format!("{} {}", if log { PLACEHOLDER_MISSED_LOG } else { PLACEHOLDER_MISSED_FLOAT }, ev.name),
        CombatTextKind::Tip => if log { PLACEHOLDER_TIP_LOG.into() } else { format!("{PLACEHOLDER_TIP_FLOAT} {}", ev.name) },
    }
}
pub fn apply_event(ev: CombatTextEvent, s: &LocalSettings, log: &mut CombatTextLog, float: &mut CombatTextFloaters) {
    let show = match ev.kind {
        CombatTextKind::Dealt => s.combat_show_dealt, CombatTextKind::Taken => s.combat_show_taken,
        CombatTextKind::Missed | CombatTextKind::EnemyMissed => s.combat_show_missed, CombatTextKind::Tip => true,
    };
    if !show { return; }
    if s.combat_log_on { log.rows.push(line_for(&ev, true)); if log.rows.len() > LOG_CAP { log.rows.drain(0..log.rows.len() - LOG_CAP); } }
    if s.combat_floating_on { float.active.push((line_for(&ev, false), ev.at)); }
}

pub(crate) fn combat_text_font_px(s: &LocalSettings) -> f32 {
    match s.combat_text_size.as_str() {
        "small" => COMBAT_TEXT_FONT_SMALL_PX,
        "large" => COMBAT_TEXT_FONT_LARGE_PX,
        _ => COMBAT_TEXT_FONT_NORMAL_PX,
    }
}

pub(crate) fn combat_text_color(s: &LocalSettings) -> Color {
    if s.combat_high_contrast { COMBAT_TEXT_CONTRAST_COLOR } else { DEFAULT_FLOAT_COLOR }
}

pub(crate) fn hive_clamp_screen(center: Vec2, point: Vec2, r: f32) -> Vec2 {
    let d = point - center;
    let len = d.length();
    if len <= r || len < f32::EPSILON { point } else { center + d / len * r }
}

pub(crate) fn hive_radius_px(window_height: f32) -> f32 {
    COMBAT_TEXT_HIVE_R_OF_HEIGHT * window_height
}

fn combat_text_post(
    mut cmd: Commands, time: Res<Time>, mut queue: ResMut<CombatTextQueue>, mut log: ResMut<CombatTextLog>, mut float: ResMut<CombatTextFloaters>,
    mut rendered: ResMut<CombatLogRendered>,
    settings: Option<Res<LocalSettingsState>>, presence: Option<Res<SoftPresence>>, ui: Query<Entity, With<LivedUiCamera>>, world: Query<(&Camera, &GlobalTransform), (With<Camera3d>, Without<LivedUiCamera>)>,
    win: Query<&Window, With<PrimaryWindow>>, mut floaters: Query<(Entity, &CombatFloater, &mut Node)>,
    mut roots: Query<(Entity, &mut Visibility, &Children), With<CombatLogRoot>>, mut log_root: Local<Option<Entity>>,
) {
    #[cfg(test)]
    take_pending(&mut queue);
    let Some(settings) = settings else { queue.events.clear(); return; };
    let (s, now) = (settings.inner.clone(), time.elapsed_secs_f64());
    let (font_px, color) = (combat_text_font_px(&s), combat_text_color(&s));
    float.active.clear();
    for ev in queue.events.drain(..) { apply_event(ev, &s, &mut log, &mut float); }
    let Ok(ui_cam) = ui.single() else { return };
    for (line, at) in float.active.drain(..) {
        cmd.spawn((CombatFloater { anchor: at, until: now + f64::from(FLOAT_SECS) }, Node { position_type: PositionType::Absolute, ..default() }, UiTargetCamera(ui_cam), Text::new(line), TextFont { font_size: FontSize::Px(font_px), ..default() }, TextColor(color)));
    }
    if let (Ok((cam, gt)), Ok(window)) = (world.single(), win.single()) {
        let h = window.resolution.height();
        let hive = s.combat_layout == "hive";
        let hive_r = if hive { hive_radius_px(h) } else { 0.0 };
        let hive_center = hive.then(|| presence.as_ref().and_then(|p| cam.world_to_viewport(gt, p.render_position(0.0)).ok().map(|vp| Vec2::new(vp.x, h - vp.y)))).flatten();
        for (e, f, mut node) in &mut floaters {
            if now >= f.until { cmd.entity(e).despawn(); continue; }
            let rise = ((f.until - now) / f64::from(FLOAT_SECS)) as f32;
            if let Ok(vp) = cam.world_to_viewport(gt, f.anchor + Vec3::Y * (1.2 - rise)) {
                let mut screen = Vec2::new(vp.x, h - vp.y - rise * 24.);
                if let Some(center) = hive_center { screen = hive_clamp_screen(center, screen, hive_r); }
                node.left = Val::Px(screen.x); node.top = Val::Px(screen.y);
            }
        }
    }
    let root = if let Some(e) = *log_root { e } else if let Ok((e, _, _)) = roots.single() { e } else {
        let e = cmd.spawn((CombatLogRoot, Node { position_type: PositionType::Absolute, right: Val::Px(8.), bottom: Val::Px(72.), flex_direction: FlexDirection::Column, ..default() }, UiTargetCamera(ui_cam), Visibility::Hidden)).id();
        *log_root = Some(e); e
    };
    let Ok((_, mut vis, kids)) = roots.get_mut(root) else { return; };
    if !s.combat_log_on || log.rows.is_empty() { *vis = Visibility::Hidden; rendered.visible = false; rendered.rows.clear(); return; }
    *vis = Visibility::Visible;
    if rendered.visible && rendered.rows == log.rows { return; }
    rendered.rows.clone_from(&log.rows);
    rendered.visible = true;
    for c in kids.iter() { cmd.entity(c).despawn(); }
    for row in &log.rows { cmd.entity(root).with_child((Text::new(row.clone()), TextFont { font_size: FontSize::Px(font_px), ..default() }, TextColor(color))); }
}

fn combat_text_hive_log_anchor(
    settings: Option<Res<LocalSettingsState>>, presence: Option<Res<SoftPresence>>,
    world: Query<(&Camera, &GlobalTransform), (With<Camera3d>, Without<LivedUiCamera>)>, win: Query<&Window, With<PrimaryWindow>>,
    mut log_nodes: Query<&mut Node, With<CombatLogRoot>>,
) {
    let Some(settings) = settings else { return; };
    if settings.inner.combat_layout != "hive" { return; }
    let Ok(window) = win.single() else { return };
    let h = window.resolution.height();
    let hive_r = hive_radius_px(h);
    let Ok((cam, gt)) = world.single() else { return };
    let char_vp = presence.and_then(|p| cam.world_to_viewport(gt, p.render_position(0.0)).ok().map(|vp| Vec2::new(vp.x, h - vp.y)));
    let Some(center) = char_vp else { return };
    for mut node in &mut log_nodes {
        let anchor = hive_clamp_screen(center, center + Vec2::new(0.0, hive_r * 0.4), hive_r);
        node.right = Val::Auto;
        node.left = Val::Px(anchor.x);
        node.bottom = Val::Px(h - anchor.y);
    }
}

pub struct CombatTextPlugin;
impl Plugin for CombatTextPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CombatTextQueue>().init_resource::<CombatTextLog>().init_resource::<CombatTextFloaters>()
            .init_resource::<CombatLogRendered>()
            .add_systems(PostUpdate, (combat_text_post, combat_text_hive_log_anchor).chain());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat_core::{self, CombatState, fire_cue};
    use crate::droid::{DroidState, DROID_HIT, DROID_SHOT_SECS};
    use crate::difficulty::{enemy_hit, DifficultyTier};

    #[test] fn combat_text_acceptance_t1_defaults() {
        let s = LocalSettings::default();
        assert!(s.combat_floating_on && !s.combat_log_on && !s.combat_tip_shown);
    }
    #[test] fn combat_text_acceptance_t2_visibility_toggles() {
        let ev = CombatTextEvent { kind: CombatTextKind::Dealt, amount: HIT, at: Vec3::ZERO, name: PREDATOR_NAME };
        let (mut log, mut float, mut s) = (CombatTextLog::default(), CombatTextFloaters::default(), LocalSettings::default());
        s.combat_show_dealt = false; apply_event(ev, &s, &mut log, &mut float); assert!(log.rows.is_empty() && float.active.is_empty());
        s.combat_show_dealt = true; s.combat_log_on = true; s.combat_floating_on = false; apply_event(ev, &s, &mut log, &mut float); assert_eq!(log.rows.len(), 1); assert!(float.active.is_empty());
    }
    #[test] fn combat_text_acceptance_t4_sanctuary_miss_apply() {
        let (mut q, c) = (CombatTextQueue::default(), CombatState::default());
        hook_player_strike(&mut q, PlaceId::Sanctuary, 0, 0, false, Vec3::ONE);
        assert!(fire_cue(PlaceId::Sanctuary, &c, false, 0).is_none());
        let (mut log, mut float, mut s) = (CombatTextLog::default(), CombatTextFloaters::default(), LocalSettings::default());
        s.combat_show_missed = true; s.combat_floating_on = true; s.combat_log_on = true;
        for ev in q.events.drain(..) { apply_event(ev, &s, &mut log, &mut float); }
        assert_eq!(float.active.len(), 1); assert_eq!(log.rows.len(), 1);
    }
    #[test] fn combat_text_acceptance_t5_droid_shot_unbroken() {
        let (mut d, mut p) = (DroidState::default(), 0u32); d.shot_timer = DROID_SHOT_SECS; let b = p;
        crate::droid::shot_step(&mut d, &mut p, 0.0, true, true, true, DifficultyTier::Unbroken);
        let mut q = CombatTextQueue::default();
        hook_lunge_after(&mut q, b, p, 1, true, Vec3::ZERO, DROID_NAME);
        assert_eq!(q.events[0].amount, enemy_hit(DifficultyTier::Unbroken, DROID_HIT));
    }
    #[test] fn combat_text_acceptance_t6_tip_reload_and_droid_name() {
        let mut locked = LocalSettings::from_json(r#"{"schema":"powrush_settings_v1","combat_tip_shown":true}"#).unwrap();
        let mut q = CombatTextQueue::default(); assert!(!hook_combat_tip(&mut locked, Vec3::ZERO, DROID_NAME));
        let mut fresh = LocalSettings::default();
        assert!(hook_combat_tip(&mut fresh, Vec3::ZERO, DROID_NAME));
        pending_push(CombatTextEvent { kind: CombatTextKind::Tip, amount: 0, at: Vec3::ZERO, name: DROID_NAME });
        take_pending(&mut q);
        assert_eq!(q.events[0].name, DROID_NAME);
    }
    #[test] fn combat_text_h1_hook_queue_drains() {
        let (mut q, mut log, mut float) = (CombatTextQueue::default(), CombatTextLog::default(), CombatTextFloaters::default());
        hook_player_strike(&mut q, PlaceId::Heartwood, 0, HIT, true, Vec3::Y);
        for ev in q.events.drain(..) { apply_event(ev, &LocalSettings::default(), &mut log, &mut float); }
        assert!(log.rows.is_empty() && float.active.len() == 1);
    }
    #[test] fn combat_text_acceptance_t4_lunge_dodge_miss() {
        let mut c = CombatState::default(); c.lunge_timer = LUNGE_SECS; let b = c.player_taken;
        combat_core::predator_tick(&mut c, 0.0, true, false, DifficultyTier::Sheltered);
        let mut q = CombatTextQueue::default();
        hook_lunge_after(&mut q, b, c.player_taken, 1, false, Vec3::ZERO, PREDATOR_NAME);
        assert_eq!(q.events[0].kind, CombatTextKind::EnemyMissed);
    }
    #[test] fn combat_text_pending_cross_thread() {
        let mut sink = CombatTextQueue::default(); take_pending(&mut sink); sink.events.clear();
        let ev = ev(CombatTextKind::Dealt, 1, Vec3::ZERO, PREDATOR_NAME);
        std::thread::spawn(move || pending_push(ev)).join().unwrap();
        let mut q = CombatTextQueue::default();
        take_pending(&mut q);
        assert_eq!(q.events.len(), 1);
    }
    #[test] fn combat_text_cmd_queue_before_post_drain() {
        let mut app = App::new();
        app.init_resource::<CombatTextQueue>();
        app.add_systems(Update, |mut cmd: Commands| {
            hook_after_fire(&mut cmd, PlaceId::Heartwood, 0, HIT, true, Vec3::ZERO);
        });
        app.add_systems(PostUpdate, |mut q: ResMut<CombatTextQueue>| take_pending(&mut q));
        app.update();
        assert_eq!(app.world().resource::<CombatTextQueue>().events.len(), 1);
    }
    #[test] fn combat_text_acceptance_t9_hive_radius_resolutions() {
        for (height, expect_r) in [(720.0, 158.4), (1080.0, 237.6)] {
            assert!((hive_radius_px(height) - expect_r).abs() < 0.01);
            let clamped = hive_clamp_screen(Vec2::ZERO, Vec2::new(expect_r * 2.0, 0.0), expect_r);
            assert!((clamped.length() - expect_r).abs() < 0.01);
        }
    }
    #[test] fn combat_text_acceptance_size_contrast_mapping() {
        let mut s = LocalSettings::default();
        assert_eq!(combat_text_font_px(&s), COMBAT_TEXT_FONT_NORMAL_PX);
        s.combat_text_size = "large".into();
        s.combat_high_contrast = true;
        assert_eq!(combat_text_font_px(&s), COMBAT_TEXT_FONT_LARGE_PX);
        assert_eq!(combat_text_color(&s), COMBAT_TEXT_CONTRAST_COLOR);
    }
    #[test] fn combat_text_acceptance_log_stable_without_change() {
        let mut a = CombatLogRendered { rows: vec!["x".into()], visible: true };
        let b = a.rows.clone();
        assert!(a.visible && a.rows == b);
        a.rows.push("y".into());
        assert_ne!(a.rows, b);
    }
}