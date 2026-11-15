//! COMBAT-TEXT-1a — read-only combat hooks, queue, and drain.

use bevy::prelude::*;
use shared::hex_travel::PlaceId;
use shared::local_settings::LocalSettings;

use crate::combat_core::{fight_present, ground_shot_connects, HIT, LUNGE_SECS, PREDATOR_NAME};
use crate::droid::{DROID_NAME, DROID_SHOT_SECS};
use crate::local_settings::LocalSettingsState;

// PLACEHOLDER COPY: council pick pending, Sherif may veto
pub const PLACEHOLDER_MISSED_FLOAT: &str = "[[MISSED_FLOAT]]";
// PLACEHOLDER COPY: council pick pending, Sherif may veto
pub const PLACEHOLDER_MISSED_LOG: &str = "[[MISSED_LOG]]";
// PLACEHOLDER COPY: council pick pending, Sherif may veto
pub const PLACEHOLDER_TIP_FLOAT: &str = "[[COMBAT_TIP_FLOAT]]";
// PLACEHOLDER COPY: council pick pending, Sherif may veto
pub const PLACEHOLDER_TIP_LOG: &str = "[[COMBAT_TIP_LOG]]";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatTextKind { Dealt, Missed, Taken, EnemyMissed, Tip }

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CombatTextEvent {
    pub kind: CombatTextKind,
    pub amount: u32,
    pub at: Vec3,
    pub name: &'static str,
}

#[derive(Resource, Debug, Default)]
pub struct CombatTextQueue { pub events: Vec<CombatTextEvent> }
#[derive(Resource, Debug, Default)]
pub struct CombatTextLog { pub rows: Vec<String> }
#[derive(Resource, Debug, Default)]
pub struct CombatTextFloaters { pub active: Vec<(String, Vec3)> }

fn push(queue: &mut CombatTextQueue, kind: CombatTextKind, amount: u32, at: Vec3, name: &'static str) {
    queue.events.push(CombatTextEvent { kind, amount, at, name });
}

pub fn wrap_count(timer: f32, dt: f32, on: bool, period: f32) -> u32 {
    if !on { return 0; }
    let (mut t, mut n) = (timer + dt.max(0.0), 0u32);
    while t >= period { t -= period; n += 1; }
    n
}

pub fn hook_player_strike(queue: &mut CombatTextQueue, place: PlaceId, before: u32, after: u32, hit: bool, at: Vec3) {
    if !fight_present(place) { return; }
    if hit {
        let d = after.saturating_sub(before);
        if d > 0 { push(queue, CombatTextKind::Dealt, d, at, PREDATOR_NAME); }
    } else {
        push(queue, CombatTextKind::Missed, 0, at, PREDATOR_NAME);
    }
}

pub fn hook_lunge_after(queue: &mut CombatTextQueue, before: u32, after: u32, wraps: u32, grounded: bool, at: Vec3, name: &'static str) {
    if wraps == 0 { return; }
    let d = after.saturating_sub(before);
    if d > 0 { push(queue, CombatTextKind::Taken, d, at, name); }
    else if !ground_shot_connects(grounded) { push(queue, CombatTextKind::EnemyMissed, 0, at, name); }
}

pub fn hook_droid_strike(queue: &mut CombatTextQueue, before: u32, after: u32, missed: bool, at: Vec3) {
    let d = after.saturating_sub(before);
    if d > 0 { push(queue, CombatTextKind::Dealt, d, at, DROID_NAME); }
    else if missed { push(queue, CombatTextKind::Missed, 0, at, DROID_NAME); }
}

pub fn hook_droid_shot_after(queue: &mut CombatTextQueue, before: u32, after: u32, wraps: u32, grounded: bool, at: Vec3) {
    hook_lunge_after(queue, before, after, wraps, grounded, at, DROID_NAME);
}

pub fn hook_combat_tip(queue: &mut CombatTextQueue, settings: &mut LocalSettings, at: Vec3) -> bool {
    if settings.combat_tip_shown { return false; }
    settings.combat_tip_shown = true;
    push(queue, CombatTextKind::Tip, 0, at, PREDATOR_NAME);
    true
}

pub fn hook_after_fire(q: Option<&mut CombatTextQueue>, place: PlaceId, pred_b: u32, pred_a: u32, hit: bool, at: Vec3) {
    if let Some(q) = q { hook_player_strike(q, place, pred_b, pred_a, hit, at); }
}

pub fn hook_after_lunge(q: Option<&mut CombatTextQueue>, t0: f32, p0: u32, dt: f32, reach: bool, grounded: bool, p1: u32, at: Vec3) {
    if let Some(q) = q { hook_lunge_after(q, p0, p1, wrap_count(t0, dt, reach, LUNGE_SECS), grounded, at, PREDATOR_NAME); }
}

pub fn hook_tip_edge(q: Option<&mut CombatTextQueue>, s: Option<&mut LocalSettingsState>, edge: bool, at: Vec3) {
    if let (Some(q), Some(s)) = (q, s) {
        if edge && hook_combat_tip(q, &mut s.inner, at) { s.dirty = true; }
    }
}

pub fn hook_droid_frame(
    q: Option<&mut CombatTextQueue>, s: Option<&mut LocalSettingsState>, strike_b: u32, strike_a: u32, miss: bool,
    shot_t0: f32, p0: u32, p1: u32, dt: f32, armed: bool, grounded: bool, tip_edge: bool, at: Vec3,
) {
    if let Some(q) = q {
        hook_droid_strike(q, strike_b, strike_a, miss, at);
        hook_droid_shot_after(q, p0, p1, wrap_count(shot_t0, dt, armed, DROID_SHOT_SECS), grounded, at);
        if let Some(s) = s {
            if tip_edge && hook_combat_tip(q, &mut s.inner, at) { s.dirty = true; }
        }
    }
}

fn line_for(event: &CombatTextEvent, log: bool) -> String {
    match event.kind {
        CombatTextKind::Dealt => format!("{} -{}", event.name, event.amount),
        CombatTextKind::Taken => format!("You -{}", event.amount),
        CombatTextKind::Missed | CombatTextKind::EnemyMissed => format!(
            "{} {}",
            if log { PLACEHOLDER_MISSED_LOG } else { PLACEHOLDER_MISSED_FLOAT },
            event.name
        ),
        CombatTextKind::Tip => if log { PLACEHOLDER_TIP_LOG.into() } else { PLACEHOLDER_TIP_FLOAT.into() },
    }
}

pub fn apply_event(event: CombatTextEvent, s: &LocalSettings, log: &mut CombatTextLog, float: &mut CombatTextFloaters) {
    let show = match event.kind {
        CombatTextKind::Dealt => s.combat_show_dealt,
        CombatTextKind::Taken => s.combat_show_taken,
        CombatTextKind::Missed | CombatTextKind::EnemyMissed => s.combat_show_missed,
        CombatTextKind::Tip => true,
    };
    if !show { return; }
    if s.combat_log_on { log.rows.push(line_for(&event, true)); }
    if s.combat_floating_on { float.active.push((line_for(&event, false), event.at)); }
}

pub(crate) fn drain_combat_text(
    mut queue: ResMut<CombatTextQueue>, mut log: ResMut<CombatTextLog>, mut float: ResMut<CombatTextFloaters>,
    settings: Option<Res<LocalSettingsState>>,
) {
    let Some(settings) = settings else { queue.events.clear(); return; };
    let s = settings.inner.clone();
    for ev in queue.events.drain(..) { apply_event(ev, &s, &mut log, &mut float); }
}

pub struct CombatTextPlugin;
impl Plugin for CombatTextPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CombatTextQueue>().init_resource::<CombatTextLog>().init_resource::<CombatTextFloaters>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn combat_text_acceptance_t1_defaults() {
        let s = LocalSettings::default();
        assert!(s.combat_floating_on && !s.combat_log_on && s.combat_show_dealt && !s.combat_tip_shown);
        assert_eq!((s.combat_text_size.as_str(), s.combat_layout.as_str()), ("normal", "default"));
    }

    #[test]
    fn combat_text_acceptance_t2_visibility_toggles() {
        let ev = CombatTextEvent { kind: CombatTextKind::Dealt, amount: HIT, at: Vec3::ZERO, name: PREDATOR_NAME };
        let (mut log, mut float, mut s) = (CombatTextLog::default(), CombatTextFloaters::default(), LocalSettings::default());
        s.combat_show_dealt = false;
        apply_event(ev, &s, &mut log, &mut float);
        assert!(log.rows.is_empty() && float.active.is_empty());
        s = LocalSettings::default();
        s.combat_log_on = true;
        s.combat_floating_on = false;
        apply_event(ev, &s, &mut log, &mut float);
        assert_eq!(log.rows.len(), 1);
        assert!(float.active.is_empty());
        s.combat_log_on = false;
        apply_event(ev, &s, &mut log, &mut float);
        assert_eq!(log.rows.len(), 1, "floating off does not clear log");
    }

    #[test]
    fn combat_text_acceptance_t6_tip_once() {
        let (mut settings, mut q) = (LocalSettings::default(), CombatTextQueue::default());
        assert!(hook_combat_tip(&mut q, &mut settings, Vec3::ZERO) && !hook_combat_tip(&mut q, &mut settings, Vec3::ZERO));
    }

    #[test]
    fn combat_text_h1_hook_queue_drains() {
        let (mut q, mut log, mut float) = (CombatTextQueue::default(), CombatTextLog::default(), CombatTextFloaters::default());
        hook_player_strike(&mut q, PlaceId::Heartwood, 0, HIT, true, Vec3::Y);
        for ev in q.events.drain(..) { apply_event(ev, &LocalSettings::default(), &mut log, &mut float); }
        assert!(log.rows.is_empty() && float.active.len() == 1);
    }
}