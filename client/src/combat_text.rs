use bevy::prelude::*;
use bevy::ui::UiTargetCamera;
use bevy::window::PrimaryWindow;
use shared::hex_travel::PlaceId;
use shared::local_settings::LocalSettings;
use crate::combat_core::{fight_present, ground_shot_connects, HIT, LUNGE_SECS, PREDATOR_NAME};
use crate::droid::{DROID_NAME, DROID_SHOT_SECS};
use crate::local_settings::LocalSettingsState;
use crate::ui_above_world::LivedUiCamera;

// PLACEHOLDER COPY: council pick pending, Sherif may veto
pub const PLACEHOLDER_MISSED_FLOAT: &str = "[[MISSED_FLOAT]]"; pub const PLACEHOLDER_MISSED_LOG: &str = "[[MISSED_LOG]]";
pub const PLACEHOLDER_DEALT_FLOAT: &str = "[[DEALT_FLOAT]]"; pub const PLACEHOLDER_TAKEN_FLOAT: &str = "[[TAKEN_FLOAT]]";
pub const PLACEHOLDER_TIP_FLOAT: &str = "[[COMBAT_TIP_FLOAT]]"; pub const PLACEHOLDER_TIP_LOG: &str = "[[COMBAT_TIP_LOG]]";

const FLOAT_SECS: f32 = 1.2;
const LOG_CAP: usize = 8;

static PENDING: std::sync::Mutex<Vec<CombatTextEvent>> = std::sync::Mutex::new(Vec::new());

#[derive(Clone, Copy, PartialEq, Eq, Debug)] pub enum CombatTextKind { Dealt, Missed, Taken, EnemyMissed, Tip }
#[derive(Clone, Copy, PartialEq, Debug)] pub struct CombatTextEvent { pub kind: CombatTextKind, pub amount: u32, pub at: Vec3, pub name: &'static str }
#[derive(Resource, Default, Debug)] pub struct CombatTextQueue { pub events: Vec<CombatTextEvent> }
#[derive(Resource, Default, Debug)] pub struct CombatTextLog { pub rows: Vec<String> }
#[derive(Resource, Default, Debug)] pub struct CombatTextFloaters { pub active: Vec<(String, Vec3)> }
#[derive(Component)] struct CombatFloater { anchor: Vec3, until: f64 }
#[derive(Component)] struct CombatLogRoot;

fn pending_push(ev: CombatTextEvent) {
    if let Ok(mut p) = PENDING.lock() { p.push(ev); }
}
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
fn combat_text_post(
    mut cmd: Commands, time: Res<Time>, mut queue: ResMut<CombatTextQueue>, mut log: ResMut<CombatTextLog>, mut float: ResMut<CombatTextFloaters>,
    settings: Option<Res<LocalSettingsState>>, ui: Query<Entity, With<LivedUiCamera>>, world: Query<(&Camera, &GlobalTransform), (With<Camera3d>, Without<LivedUiCamera>)>,
    win: Query<&Window, With<PrimaryWindow>>, mut floaters: Query<(Entity, &CombatFloater, &mut Node)>,
    mut roots: Query<(Entity, &mut Visibility, &Children), With<CombatLogRoot>>, mut log_root: Local<Option<Entity>>,
) {
    take_pending(&mut queue);
    let Some(settings) = settings else { queue.events.clear(); return; };
    let (s, now) = (settings.inner.clone(), time.elapsed_secs_f64());
    float.active.clear();
    for ev in queue.events.drain(..) { apply_event(ev, &s, &mut log, &mut float); }
    let Ok(ui_cam) = ui.single() else { return };
    for (line, at) in float.active.drain(..) {
        cmd.spawn((CombatFloater { anchor: at, until: now + f64::from(FLOAT_SECS) }, Node { position_type: PositionType::Absolute, ..default() }, UiTargetCamera(ui_cam), Text::new(line), TextFont { font_size: FontSize::Px(12.), ..default() }));
    }
    if let (Ok((cam, gt)), Ok(window)) = (world.single(), win.single()) {
        let h = window.resolution.height();
        for (e, f, mut node) in &mut floaters {
            if now >= f.until { cmd.entity(e).despawn(); continue; }
            let rise = ((f.until - now) / f64::from(FLOAT_SECS)) as f32;
            if let Ok(vp) = cam.world_to_viewport(gt, f.anchor + Vec3::Y * (1.2 - rise)) {
                node.left = Val::Px(vp.x); node.top = Val::Px(h - vp.y - rise * 24.);
            }
        }
    }
    let root = if let Some(e) = *log_root { e } else if let Ok((e, _, _)) = roots.single() { e } else {
        let e = cmd.spawn((CombatLogRoot, Node { position_type: PositionType::Absolute, right: Val::Px(8.), bottom: Val::Px(72.), flex_direction: FlexDirection::Column, ..default() }, UiTargetCamera(ui_cam), Visibility::Hidden)).id();
        *log_root = Some(e); e
    };
    let Ok((_, mut vis, kids)) = roots.get_mut(root) else { return; };
    if !s.combat_log_on || log.rows.is_empty() { *vis = Visibility::Hidden; return; }
    *vis = Visibility::Visible;
    for c in kids.iter() { cmd.entity(c).despawn(); }
    for row in &log.rows { cmd.entity(root).with_child((Text::new(row.clone()), TextFont { font_size: FontSize::Px(12.), ..default() })); }
}
pub struct CombatTextPlugin;
impl Plugin for CombatTextPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CombatTextQueue>().init_resource::<CombatTextLog>().init_resource::<CombatTextFloaters>()
            .add_systems(PostUpdate, combat_text_post);
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
}