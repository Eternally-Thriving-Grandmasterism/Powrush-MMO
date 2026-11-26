//! GFX-PACE-1 logs hitch counts over a 60 Hz budget and a 2x hitch line, once per 1 s roll.
//! LOOP-PACING-1 adds a Last waiter. That waiter is release spacing only: not a frame, present,
//! or render cap, and not a safety, thermal, or leak claim.

use std::io::Write;
use std::sync::atomic::{AtomicU32, AtomicU8, AtomicUsize, Ordering::Relaxed};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::render::renderer::RenderAdapterInfo;
use bevy::time::Real;
use bevy::window::PrimaryWindow;
use shared::local_settings::GraphicsPreset;

pub fn perf_log_on(v: Option<&str>) -> bool {
    matches!(v.map(|s| s.trim().to_ascii_lowercase()).as_deref(), Some("1" | "on" | "true" | "yes"))
}
#[derive(Resource, Clone, Copy)] struct PerfLog(pub bool);
#[derive(Resource, Default)] struct PerfLogHead(bool);
fn append_perf_line(s: &str) {
    let p = shared::user_persist::persist_path("data/perf.log");
    let _ = p.parent().map(std::fs::create_dir_all);
    let _ = std::fs::OpenOptions::new().create(true).append(true).open(&p).and_then(|mut f| f.write_all(s.as_bytes()).and_then(|_| f.write_all(b"\n")));
}

/// BENCH-WALK-1 (PERF-GATE). One seeded headless bench-walk record. CPU only: `p*_us` are wall times of whole
/// `App::update` calls on the machine that ran it. Headless has no renderer, so there is no GPU number;
/// `mesh_visible` counts ECS entities with a `Mesh3d` that are not `Visibility::Hidden`, not GPU draw calls.
/// `rss_kib` is process RSS (Linux `/proc/self/status`), shared with anything else in the same process.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BenchWalk { pub schema: u32, pub seed: u64, pub frames: u32, pub p50_us: u32, pub p95_us: u32, pub p99_us: u32, pub entities: u32, pub mesh_visible: u32, pub rss_kib: Option<u64> }
impl BenchWalk {
    pub fn from_frames(seed: u64, frame_us: &[u32], entities: u32, mesh_visible: u32, rss_kib: Option<u64>) -> Self {
        let mut v = frame_us.to_vec(); v.sort_unstable();
        Self { schema: 1, seed, frames: v.len() as u32, p50_us: nearest_rank(&v, 50), p95_us: nearest_rank(&v, 95), p99_us: nearest_rank(&v, 99), entities, mesh_visible, rss_kib }
    }
}
/// Nearest-rank percentile of an ascending slice; 0 when empty.
pub fn nearest_rank(sorted: &[u32], pct: usize) -> u32 {
    if sorted.is_empty() { return 0; }
    sorted[(pct.min(100) * sorted.len()).div_ceil(100).clamp(1, sorted.len()) - 1]
}
/// Warn band against a committed baseline. Counts are seeded and deterministic, so any change warns. A time warns only
/// past BOTH the ratio and the absolute floor, so CI runner noise cannot trip it; RSS warns only past its ratio.
#[derive(Debug, Clone, Copy)]
pub struct BenchBand { pub time_ratio: f64, pub time_floor_us: u32, pub rss_ratio: f64 }
pub const BENCH_BAND: BenchBand = BenchBand { time_ratio: 2.0, time_floor_us: 4_000, rss_ratio: 1.5 };
/// Warning lines only. Never an error: a CI step prints them and exits 0.
pub fn bench_warnings(base: &BenchWalk, now: &BenchWalk, band: &BenchBand) -> Vec<String> {
    let mut out = Vec::new();
    for (k, b, n) in [("frames", base.frames, now.frames), ("entities", base.entities, now.entities), ("mesh_visible", base.mesh_visible, now.mesh_visible)] {
        if b != n { out.push(format!("{k} {b} -> {n}")); }
    }
    for (k, b, n) in [("p50_us", base.p50_us, now.p50_us), ("p95_us", base.p95_us, now.p95_us), ("p99_us", base.p99_us, now.p99_us)] {
        if n as f64 > b as f64 * band.time_ratio && n.saturating_sub(b) > band.time_floor_us { out.push(format!("{k} {b} -> {n}")); }
    }
    if let (Some(b), Some(n)) = (base.rss_kib, now.rss_kib) { if n as f64 > b as f64 * band.rss_ratio { out.push(format!("rss_kib {b} -> {n}")); } }
    out
}
/// Process RSS in KiB from `/proc/self/status`; `None` off Linux or when unreadable.
pub fn process_rss_kib() -> Option<u64> {
    let s = std::fs::read_to_string("/proc/self/status").ok()?;
    s.lines().find_map(|l| l.strip_prefix("VmRSS:")).and_then(|v| v.trim().trim_end_matches("kB").trim().parse().ok())
}

/// One frame at 60 Hz, in seconds.
pub const FRAME_BUDGET_SECS: f64 = 1.0 / 60.0;

/// A hitch is a frame longer than two 60 Hz frames.
pub const HITCH_SECS: f64 = 2.0 * FRAME_BUDGET_SECS;

/// Focused release spacing. Not a rendered-frame budget.
const FOCUSED_PERIOD: Duration = Duration::from_nanos(16_666_667);

/// Inactive release spacing: the one primary window is unfocused, or the primary window is missing
/// or ambiguous.
const INACTIVE_PERIOD: Duration = Duration::from_nanos(250_000_000);

/// Shared edge for both lines: strictly over counts (`>`); a frame exactly on the line does not.
pub fn over_line(dt: f64, line: f64) -> bool {
    dt > line
}

/// Pure frame-pace state, fed one real delta per frame.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Default)]
pub struct FramePace {
    pub elapsed: f64,
    pub frames: u32,
    pub over_budget: u32,
    pub hitches: u32,
    /// Frame-time histogram: bucket i holds frames of [i, i+1) ms; bucket 31 holds 31 ms and over.
    pub frame_ms: [u32; 32],
}

impl FramePace {
    /// Counts one frame (zero delta is skipped). On reaching `roll_secs`, returns the
    /// finished counts and resets.
    pub fn tick(&mut self, dt: f64, roll_secs: f64) -> Option<FramePace> {
        if dt == 0.0 {
            return None;
        }
        self.elapsed += dt;
        self.frames += 1;
        if over_line(dt, FRAME_BUDGET_SECS) {
            self.over_budget += 1;
        }
        if over_line(dt, HITCH_SECS) {
            self.hitches += 1;
        }
        self.frame_ms[((dt * 1000.0).floor() as usize).min(31)] += 1;
        if self.elapsed < roll_secs {
            return None;
        }
        Some(std::mem::take(self))
    }
}

/// CRASH-REPORT-1. Build sha baked at compile time from `POWRUSH_BUILD_SHA`; `unknown` when unset (no build.rs, no git call).
pub const BUILD_SHA: &str = match option_env!("POWRUSH_BUILD_SHA") { Some(s) => s, None => "unknown" };
/// CRASH-REPORT-1. Snapshot the panic hook reads: the last 32 real frame times (µs), the GPU adapter name and the
/// quality tier. Atomics and `OnceLock::get` only, so the hook takes no lock and cannot deadlock; a slot read during a
/// concurrent write is whole, but the 32 slots may mix two frames.
pub struct CrashSnap { us: [AtomicU32; 32], n: AtomicUsize, adapter: OnceLock<String>, tier: AtomicU8 }
pub static CRASH_SNAP: CrashSnap = CrashSnap::new();
impl CrashSnap {
    pub const fn new() -> Self { Self { us: [const { AtomicU32::new(0) }; 32], n: AtomicUsize::new(0), adapter: OnceLock::new(), tier: AtomicU8::new(u8::MAX) } }
    /// Game side, once per frame. Zero deltas are skipped like `FramePace::tick`; the adapter is kept from its first sighting.
    pub fn note(&self, dt: f64, adapter: Option<&str>, tier: Option<GraphicsPreset>) {
        if dt > 0.0 { let i = self.n.fetch_add(1, Relaxed); self.us[i % 32].store((dt * 1e6).min(u32::MAX as f64) as u32, Relaxed); }
        if let (None, Some(a)) = (self.adapter.get(), adapter) { let _ = self.adapter.set(a.to_string()); }
        if let Some(t) = tier { self.tier.store(GraphicsPreset::ALL.iter().position(|p| *p == t).map_or(u8::MAX, |i| i as u8), Relaxed); }
    }
    /// Oldest first; fewer than 32 before the 32nd counted frame.
    pub fn frames_us(&self) -> Vec<u32> { let n = self.n.load(Relaxed); (n.saturating_sub(32)..n).map(|i| self.us[i % 32].load(Relaxed)).collect() }
    /// Hook side: one line, `{epoch} crash_report build=… version=… adapter="…" tier=… frames_ms=[…]\n`. No lock, no index panic.
    pub fn line(&self, epoch_secs: u64, build: &str) -> String {
        let clean = |s: &str| s.chars().filter(|c| !c.is_control()).map(|c| if c == '"' { '\'' } else { c }).take(96).collect::<String>();
        let tier = GraphicsPreset::ALL.get(self.tier.load(Relaxed) as usize).map_or("unknown", |p| p.label());
        let ms = self.frames_us().iter().map(|u| format!("{:.1}", *u as f64 / 1000.0)).collect::<Vec<_>>().join(",");
        format!("{epoch_secs} crash_report build={} version={} adapter=\"{}\" tier={tier} frames_ms=[{ms}]\n", clean(build), env!("CARGO_PKG_VERSION"), self.adapter.get().map_or("unknown".to_string(), |a| clean(a)))
    }
}

/// Clock injected into the Last waiter. The plugin inserts [`InstantClock`].
trait PaceClock: Send + Sync {
    fn now(&mut self) -> Instant;
    fn checked_add(&mut self, base: Instant, period: Duration) -> Option<Instant>;
    /// Production calls this at the release instant. The live clock ignores it.
    fn note_release(&mut self, at: Instant);
}

/// Sleeper injected into the Last waiter. The plugin inserts [`ThreadSleeper`].
trait PaceSleeper: Send + Sync {
    fn sleep(&mut self, remaining: Duration);
}

/// Live clock. Samples are `Instant::now`; addition is `Instant::checked_add`.
struct InstantClock;

impl PaceClock for InstantClock {
    fn now(&mut self) -> Instant {
        Instant::now()
    }

    fn checked_add(&mut self, base: Instant, period: Duration) -> Option<Instant> {
        base.checked_add(period)
    }

    fn note_release(&mut self, _at: Instant) {}
}

/// Live sleeper. `std::thread::sleep` for the scheduled remainder.
struct ThreadSleeper;

impl PaceSleeper for ThreadSleeper {
    fn sleep(&mut self, remaining: Duration) {
        std::thread::sleep(remaining);
    }
}

/// Injected clock resource.
#[derive(Resource)]
struct Clock(Box<dyn PaceClock>);

/// Injected sleeper resource.
#[derive(Resource)]
struct Sleeper(Box<dyn PaceSleeper>);

/// Last-waiter state.
#[derive(Resource, Default)]
struct ReleasePace {
    deadline: Option<Instant>,
    last_release: Option<Instant>,
    /// `None` until the first entry, then the inactive class of that entry.
    last_inactive: Option<bool>,
    /// Latched when `checked_add` cannot represent a deadline. Stays set.
    /// Degraded: each entry sleeps the active period once and releases once.
    /// There is no spacing proof in this mode.
    overflow_hold: bool,
}

/// Adds the hitch log on Update and the release-spacing waiter on Last.
/// Registered from `main.rs` only.
pub struct FramePacePlugin;

impl Plugin for FramePacePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FramePace>()
            .init_resource::<ReleasePace>()
            .init_resource::<PerfLogHead>()
            .insert_resource(PerfLog(perf_log_on(std::env::var("POWRUSH_PERF_LOG").ok().as_deref())))
            .insert_resource(Clock(Box::new(InstantClock)))
            .insert_resource(Sleeper(Box::new(ThreadSleeper)))
            .add_systems(Update, frame_pace_system)
            .add_systems(Last, release_waiter);
    }
}

fn frame_pace_system(time: Res<Time<Real>>, mut pace: ResMut<FramePace>, perf: Res<PerfLog>, mut head: ResMut<PerfLogHead>, adapter: Option<Res<RenderAdapterInfo>>, settings: Option<Res<crate::local_settings::LocalSettingsState>>) {
    CRASH_SNAP.note(time.delta_secs_f64(), adapter.as_deref().map(|a| a.name.as_str()), settings.as_deref().map(|s| s.inner.graphics_preset)); // CRASH-REPORT-1
    let roll_secs = LogDiagnosticsPlugin::default().wait_duration.as_secs_f64();
    if let Some(c) = pace.tick(time.delta_secs_f64(), roll_secs) {
        // p50 / p95 as the upper ms edge of the first bucket reaching that share of frames.
        let (mut seen, mut p50, mut p95) = (0u32, None, None);
        for (i, n) in c.frame_ms.iter().enumerate() {
            seen += n;
            if p50.is_none() && seen * 2 >= c.frames {
                p50 = Some(i);
            }
            if p95.is_none() && seen * 20 >= c.frames * 19 {
                p95 = Some(i);
            }
        }
        let edge = |i: usize| {
            if i == 31 {
                "31+".to_string()
            } else {
                (i + 1).to_string()
            }
        };
        if !perf.0 { return; }
        info!(
            "frame_pace: frames={} over_budget={} hitches={} p50_ms<={} p95_ms<={}",
            c.frames,
            c.over_budget,
            c.hitches,
            edge(p50.unwrap_or(31)),
            edge(p95.unwrap_or(31))
        );
        let mut lead = String::new();
        if !head.0 { head.0 = true; if let Some(a) = adapter.as_deref() { lead = format!("adapter={} ", a.name); } }
        append_perf_line(&format!("frame_pace: {lead}frames={} over_budget={} hitches={} p50_ms<={} p95_ms<={}", c.frames, c.over_budget, c.hitches, edge(p50.unwrap_or(31)), edge(p95.unwrap_or(31))));
    }
}

/// Waits on Last until the minimum release spacing has elapsed, then releases.
///
/// Release spacing only. Not a frame, present, or render cap. Not a safety, thermal, or leak
/// claim. The scheduled remainder is at most 250 ms; actual latency is excluded.
///
/// A representable deadline releases only when a fresh sample is at or after it. There is no
/// unchanged-sample escape: while the sample is behind, the waiter sleeps the positive remainder
/// and samples again. The next deadline is `checked_add(actual_release, period)`. An overdue
/// entry releases once and does not repay missed periods.
///
/// When `checked_add` returns `None`, no deadline is stored and `overflow_hold` latches. That
/// mode is degraded and has no spacing proof: every later entry sleeps the active period once
/// and then releases. It never sleeps zero and it does not tight-loop.
fn release_waiter(
    mut pace: ResMut<ReleasePace>,
    mut clock: ResMut<Clock>,
    mut sleeper: ResMut<Sleeper>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let inactive = primary_window_is_inactive(&windows);
    let period = if inactive {
        INACTIVE_PERIOD
    } else {
        FOCUSED_PERIOD
    };
    let focus_changed = pace
        .last_inactive
        .is_some_and(|previous| previous != inactive);
    pace.last_inactive = Some(inactive);

    if pace.overflow_hold {
        degraded_release(&mut pace, clock.0.as_mut(), sleeper.0.as_mut(), period);
        return;
    }
    if retarget_deadline(&mut pace, clock.0.as_mut(), focus_changed, period) {
        degraded_release(&mut pace, clock.0.as_mut(), sleeper.0.as_mut(), period);
        return;
    }
    if arm_first_deadline(&mut pace, clock.0.as_mut(), period) {
        degraded_release(&mut pace, clock.0.as_mut(), sleeper.0.as_mut(), period);
        return;
    }

    // No unchanged-sample escape. A fresh `now` releases only at or after the deadline.
    loop {
        let now = clock.0.now();
        let deadline = pace.deadline.expect("representable wait has a deadline");
        if now >= deadline {
            pace.last_release = Some(now);
            clock.0.note_release(now);
            match clock.0.checked_add(now, period) {
                Some(next) => pace.deadline = Some(next),
                None => {
                    pace.deadline = None;
                    pace.overflow_hold = true;
                }
            }
            return;
        }
        let remaining = deadline.saturating_duration_since(now);
        sleeper.0.sleep(remaining);
    }
}

/// Exactly one primary window uses its focused flag. Zero or several count as inactive.
fn primary_window_is_inactive(windows: &Query<&Window, With<PrimaryWindow>>) -> bool {
    let mut windows = windows.iter();
    match (windows.next(), windows.next()) {
        (Some(window), None) => !window.focused,
        _ => true,
    }
}

/// `deadline = max(existing, checked_add(last_release, new_period))`.
/// Returns `true` when the sum is not representable.
fn retarget_deadline(
    pace: &mut ReleasePace,
    clock: &mut dyn PaceClock,
    focus_changed: bool,
    period: Duration,
) -> bool {
    if !focus_changed {
        return false;
    }
    let Some(released) = pace.last_release else {
        return false;
    };
    match clock.checked_add(released, period) {
        Some(candidate) => {
            pace.deadline = Some(match pace.deadline {
                Some(existing) => existing.max(candidate),
                None => candidate,
            });
            false
        }
        None => true,
    }
}

/// First entry, or any entry with no deadline, waits one full period from a fresh sample.
/// Returns `true` when that sum is not representable.
fn arm_first_deadline(pace: &mut ReleasePace, clock: &mut dyn PaceClock, period: Duration) -> bool {
    if pace.deadline.is_some() {
        return false;
    }
    let now = clock.now();
    match clock.checked_add(now, period) {
        Some(deadline) => {
            pace.deadline = Some(deadline);
            false
        }
        None => true,
    }
}

/// Degraded hold. One positive sleep of the active period, then one release. No spacing proof.
fn degraded_release(
    pace: &mut ReleasePace,
    clock: &mut dyn PaceClock,
    sleeper: &mut dyn PaceSleeper,
    period: Duration,
) {
    pace.deadline = None;
    pace.overflow_hold = true;
    sleeper.sleep(period);
    let released = clock.now();
    pace.last_release = Some(released);
    clock.note_release(released);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    use std::sync::{Arc, Mutex};

    const ROLL: f64 = 1.0;
    /// Above any honest wait in these tests. A tight loop trips this instead of hanging.
    const CALL_CAP: usize = 500;

    #[derive(Clone, Copy)]
    enum Advance {
        Full,
        /// Sleep returns before the request and still moves the clock forward.
        Early,
        /// The next sleep moves the clock past the request by `extra`. Later sleeps are exact.
        OversleepOnce {
            extra: Duration,
        },
    }

    #[derive(Clone)]
    struct FakeInner {
        origin: Instant,
        now: Instant,
        sleeps: Vec<Duration>,
        samples: Vec<Instant>,
        releases: Vec<Instant>,
        now_calls: usize,
        advance: Advance,
        checked_add_none: bool,
        burns: VecDeque<Duration>,
    }

    impl FakeInner {
        fn new(advance: Advance, checked_add_none: bool) -> Self {
            let origin = Instant::now();
            Self {
                origin,
                now: origin,
                sleeps: Vec::new(),
                samples: Vec::new(),
                releases: Vec::new(),
                now_calls: 0,
                advance,
                checked_add_none,
                burns: VecDeque::new(),
            }
        }
    }

    struct FakeClock(Arc<Mutex<FakeInner>>);
    struct FakeSleeper(Arc<Mutex<FakeInner>>);

    impl PaceClock for FakeClock {
        fn now(&mut self) -> Instant {
            let mut inner = self.0.lock().expect("fake clock");
            inner.now_calls += 1;
            assert!(inner.now_calls <= CALL_CAP, "clock samples unbounded");
            let now = inner.now;
            inner.samples.push(now);
            now
        }

        fn checked_add(&mut self, base: Instant, period: Duration) -> Option<Instant> {
            let inner = self.0.lock().expect("fake clock");
            if inner.checked_add_none {
                None
            } else {
                base.checked_add(period)
            }
        }

        fn note_release(&mut self, at: Instant) {
            let mut inner = self.0.lock().expect("fake clock");
            inner.releases.push(at);
            assert!(inner.releases.len() <= CALL_CAP, "releases unbounded");
        }
    }

    impl PaceSleeper for FakeSleeper {
        fn sleep(&mut self, remaining: Duration) {
            assert!(!remaining.is_zero(), "sleep must be > 0");
            let mut inner = self.0.lock().expect("fake sleeper");
            inner.sleeps.push(remaining);
            assert!(inner.sleeps.len() <= CALL_CAP, "sleep calls unbounded");
            let mode = inner.advance;
            let step = match mode {
                Advance::Full => remaining,
                Advance::Early => {
                    let nanos = u64::try_from(remaining.as_nanos()).expect("sleep fits u64");
                    // A 1 ns remainder is the last step: advance it so the next sample can
                    // reach the deadline. Longer requests return early and still move forward.
                    let step_nanos = if nanos > 1 { nanos - 1 } else { nanos };
                    let step = Duration::from_nanos(step_nanos);
                    if nanos > 1 {
                        assert!(step < remaining, "early-wake fake must return early");
                    }
                    assert!(!step.is_zero(), "early-wake fake must advance");
                    step
                }
                Advance::OversleepOnce { extra } => {
                    assert!(!extra.is_zero(), "oversleep is longer than the request");
                    let step = remaining.checked_add(extra).expect("oversleep");
                    assert!(step > remaining, "oversleep advances past the request");
                    inner.advance = Advance::Full;
                    step
                }
            };
            inner.now = inner.now.checked_add(step).expect("fake clock advance");
        }
    }

    fn pace_app(advance: Advance, checked_add_none: bool) -> (App, Arc<Mutex<FakeInner>>) {
        let state = Arc::new(Mutex::new(FakeInner::new(advance, checked_add_none)));
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(FramePacePlugin)
            .insert_resource(Clock(Box::new(FakeClock(state.clone()))))
            .insert_resource(Sleeper(Box::new(FakeSleeper(state.clone()))));
        (app, state)
    }

    fn snapshot(state: &Arc<Mutex<FakeInner>>) -> FakeInner {
        state.lock().expect("fake").clone()
    }

    fn spawn_primaries(app: &mut App, count: usize, focused: bool) {
        for _ in 0..count {
            app.world_mut().spawn((
                Window {
                    focused,
                    ..default()
                },
                PrimaryWindow,
            ));
        }
    }

    fn set_focused(app: &mut App, focused: bool) {
        let mut windows = app.world_mut().query::<&mut Window>();
        for mut window in windows.iter_mut(app.world_mut()) {
            window.focused = focused;
        }
    }

    fn update_n(app: &mut App, n: usize) {
        for _ in 0..n {
            app.update();
        }
    }

    fn assert_representable(app: &App) {
        let pace = app.world().resource::<ReleasePace>();
        assert!(
            !pace.overflow_hold,
            "overflow is outside representable checks"
        );
        assert!(pace.deadline.is_some());
    }

    /// The late release already happened. The stored deadline is that instant plus one period,
    /// and the next update waits exactly that long: one sleep, one release, no catch-up.
    fn assert_next_update_waits_from_late_release(
        app: &mut App,
        state: &Arc<Mutex<FakeInner>>,
        late: Instant,
        period: Duration,
    ) {
        let expected = late.checked_add(period).expect("rearm");
        assert_eq!(
            app.world().resource::<ReleasePace>().deadline,
            Some(expected),
            "next deadline is actual release + period"
        );
        let before = snapshot(state);
        app.update();
        let after = snapshot(state);
        assert_eq!(
            after.releases.len(),
            before.releases.len() + 1,
            "one release, no burst and no repayment"
        );
        assert_eq!(
            after.sleeps.len(),
            before.sleeps.len() + 1,
            "one wait, no repayment of the missed time"
        );
        assert_eq!(*after.sleeps.last().expect("sleep"), period);
        assert_eq!(*after.releases.last().expect("release"), expected);
        let gap = after
            .releases
            .last()
            .expect("release")
            .checked_duration_since(late)
            .expect("gap");
        assert!(gap >= period);
        assert_eq!(gap, period);
        assert_representable(app);
    }

    fn assert_period_releases(origin: Instant, releases: &[Instant], period: Duration, n: usize) {
        assert_eq!(releases.len(), n);
        let mut expected = origin;
        for release in releases {
            expected = expected.checked_add(period).expect("period sum");
            assert_eq!(*release, expected);
        }
    }

    #[test]
    fn budget_edge_equal_not_counted_just_above_counted() {
        let mut p = FramePace::default();
        p.tick(FRAME_BUDGET_SECS, ROLL);
        assert_eq!(p.over_budget, 0);
        p.tick(FRAME_BUDGET_SECS + 1e-9, ROLL);
        assert_eq!(p.over_budget, 1);
        assert_eq!(p.hitches, 0);
    }

    #[test]
    fn hitch_edge_equal_not_counted_just_above_counted() {
        let mut p = FramePace::default();
        p.tick(HITCH_SECS, ROLL);
        assert_eq!((p.over_budget, p.hitches), (1, 0));
        p.tick(HITCH_SECS + 1e-9, ROLL);
        assert_eq!((p.over_budget, p.hitches), (2, 1));
    }

    #[test]
    fn counters_reset_on_roll() {
        let mut p = FramePace::default();
        let mut rolled = None;
        for _ in 0..200 {
            if let Some(c) = p.tick(0.02, ROLL) {
                rolled = Some(c);
                break;
            }
        }
        let c = rolled.expect("rolls within 1 s of frames");
        assert!((50..=51).contains(&c.frames));
        assert_eq!(c.over_budget, c.frames);
        assert_eq!(c.hitches, 0);
        assert_eq!(p, FramePace::default());
    }

    #[test]
    fn one_spike_counted_once() {
        let mut p = FramePace::default();
        assert_eq!(p.tick(0.1, ROLL), None);
        assert_eq!((p.frames, p.over_budget, p.hitches), (1, 1, 1));
    }

    #[test]
    fn zero_delta_not_counted() {
        let mut p = FramePace::default();
        assert_eq!(p.tick(0.0, ROLL), None);
        assert_eq!(p, FramePace::default());
    }

    #[test]
    fn frame_ms_buckets_known_deltas_and_overflow() {
        let mut p = FramePace::default();
        for _ in 0..19 {
            p.tick(0.0165, ROLL);
        }
        p.tick(0.0405, ROLL);
        p.tick(0.5, ROLL);
        assert_eq!(p.frame_ms[16], 19);
        assert_eq!(p.frame_ms[31], 2);
        assert_eq!(p.frame_ms.iter().sum::<u32>(), p.frames);
    }

    #[test]
    fn frame_ms_buckets_clear_on_roll() {
        let mut p = FramePace::default();
        let mut rolled = None;
        for _ in 0..200 {
            if let Some(c) = p.tick(0.0055, ROLL) {
                rolled = Some(c);
                break;
            }
        }
        let c = rolled.expect("rolls within 1 s of frames");
        assert_eq!(c.frame_ms[5], c.frames);
        assert_eq!(p.frame_ms, [0; 32]);
    }

    /// BENCH-WALK-1: the Sanctuary yard as `living_ecology` boots it headless (props, ClimatePlane), a fixed 1/60 s clock,
    /// warmed until every PropMesh has swapped, then a seeded camera circle of `frames` updates. CPU wall time only.
    fn bench_walk(seed: u64, frames: u32) -> BenchWalk {
        use bevy::tasks::tick_global_task_pools_on_main_thread;
        let _g = crate::test_env::lock();
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").canonicalize().unwrap();
        let mut app = App::new();
        app.set_error_handler(bevy::ecs::error::ignore).add_plugins((MinimalPlugins, AssetPlugin { file_path: root.join("assets").into_os_string().into_string().unwrap(), watch_for_changes_override: Some(false), use_asset_processor_override: Some(false), ..AssetPlugin::default() }, bevy::image::ImagePlugin::default(), bevy::mesh::MeshPlugin, bevy::gltf::GltfPlugin::default(), bevy::world_serialization::WorldSerializationPlugin)).init_resource::<crate::climate_plane::AmbientcgTexCache>().init_asset::<Image>().init_asset::<Mesh>().init_asset::<StandardMaterial>().init_asset::<crate::painterly_shader::PainterlyMaterial>().insert_resource(crate::local_settings::LocalMeshLodFeel { preset: shared::local_settings::GraphicsPreset::Medium }).insert_resource(crate::living_practice_loop::SoftPlayerRealm { current: Some(0) }).insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(FOCUSED_PERIOD)).add_plugins((crate::living_ecology::LivingEcologyPlugin, crate::climate_plane::ClimatePlanePlugin));
        app.finish(); app.cleanup(); crate::test_env::single_thread(&mut app); app.world_mut().run_schedule(Startup);
        let cam = app.world_mut().spawn((Camera3d::default(), Transform::default())).id();
        let step = |app: &mut App| crate::test_env::with_bevy_asset_root(&root, || { app.update(); tick_global_task_pools_on_main_thread(); });
        let ts = |w: &mut World| w.query::<(Entity, &crate::climate_plane::PropMesh, &Mesh3d)>().iter(w).filter(|(_, p, _)| matches!(p.fit, crate::climate_plane::PropMeshFit::Trunk | crate::climate_plane::PropMeshFit::Stone)).map(|(e, _, m)| (e, m.0.id())).collect::<Vec<_>>();
        let before = ts(app.world_mut()); assert!(!before.is_empty());
        let settled = |w: &mut World| before.iter().all(|(e, h0)| w.get::<Mesh3d>(*e).is_some_and(|m| m.0.id() != *h0)); // living_ecology's swap test (:1119)
        for _ in 0..2400 { step(&mut app); if settled(app.world_mut()) { break; } }
        assert!(settled(app.world_mut()), "bench yard never settled");
        let phase = (seed % 360) as f32;
        let mut us = Vec::with_capacity(frames as usize);
        for i in 0..frames {
            let a = (phase + i as f32 * 1.5).to_radians();
            *app.world_mut().get_mut::<Transform>(cam).unwrap() = Transform::from_xyz(14.0 * a.cos(), 6.0, 14.0 * a.sin()).looking_at(Vec3::ZERO, Vec3::Y);
            let t0 = Instant::now(); step(&mut app); us.push(t0.elapsed().as_micros().min(u32::MAX as u128) as u32);
        }
        let w = app.world_mut();
        let entities = w.query::<Entity>().iter(w).count() as u32;
        let mesh_visible = w.query::<(&Mesh3d, Option<&Visibility>)>().iter(w).filter(|(_, v)| !matches!(v, Some(Visibility::Hidden))).count() as u32;
        BenchWalk::from_frames(seed, &us, entities, mesh_visible, process_rss_kib())
    }
    const BENCH_SEED: u64 = 20_261_126;
    const BENCH_FRAMES: u32 = 240;

    #[test]
    fn bench_walk_1_percentiles_json_and_band() {
        let one_to_100: Vec<u32> = (1..=100).rev().collect();
        let b = BenchWalk::from_frames(7, &one_to_100, 50, 20, Some(1000));
        assert_eq!((b.schema, b.seed, b.frames, b.p50_us, b.p95_us, b.p99_us), (1, 7, 100, 50, 95, 99));
        assert_eq!((nearest_rank(&[], 95), nearest_rank(&[9], 1), nearest_rank(&[9], 100)), (0, 9, 9));
        let json = serde_json::to_string(&b).unwrap();
        for k in ["\"schema\":1", "\"seed\":7", "\"frames\":100", "\"p50_us\":50", "\"p95_us\":95", "\"p99_us\":99", "\"entities\":50", "\"mesh_visible\":20", "\"rss_kib\":1000"] { assert!(json.contains(k), "{k} in {json}"); }
        assert_eq!(serde_json::from_str::<BenchWalk>(&json).unwrap(), b);
        assert!(bench_warnings(&b, &b, &BENCH_BAND).is_empty());
        let mut n = b.clone(); n.p99_us = 99 * 3; assert!(bench_warnings(&b, &n, &BENCH_BAND).is_empty(), "3x but under the 4 ms floor is noise");
        let mut n = b.clone(); n.p95_us = 95 + 4_001; assert_eq!(bench_warnings(&b, &n, &BENCH_BAND), vec!["p95_us 95 -> 4096".to_string()]);
        let base = BenchWalk { p50_us: 10_000, ..b.clone() }; let mut n = base.clone(); n.p50_us = 19_999; assert!(bench_warnings(&base, &n, &BENCH_BAND).is_empty(), "under 2x is noise");
        let mut n = b.clone(); n.entities = 51; n.mesh_visible = 19; n.rss_kib = Some(1501); assert_eq!(bench_warnings(&b, &n, &BENCH_BAND), vec!["entities 50 -> 51".to_string(), "mesh_visible 20 -> 19".to_string(), "rss_kib 1000 -> 1501".to_string()]);
        let mut n = b.clone(); n.rss_kib = None; assert!(bench_warnings(&b, &n, &BENCH_BAND).is_empty());
        if cfg!(target_os = "linux") { assert!(process_rss_kib().is_some_and(|k| k > 0)); }
    }
    #[test]
    fn bench_walk_1_seeded_yard_walk_is_deterministic_and_cpu_only() {
        let a = bench_walk(BENCH_SEED, BENCH_FRAMES);
        let b = bench_walk(BENCH_SEED, BENCH_FRAMES);
        assert_eq!((a.frames, b.frames), (BENCH_FRAMES, BENCH_FRAMES));
        assert_eq!((a.entities, a.mesh_visible), (b.entities, b.mesh_visible), "seeded counts repeat");
        assert!(a.entities > 0 && a.mesh_visible > 0 && a.mesh_visible <= a.entities);
        assert!(a.p50_us <= a.p95_us && a.p95_us <= a.p99_us && a.p99_us > 0);
        assert!(bench_warnings(&a, &b, &BenchBand { time_ratio: f64::INFINITY, time_floor_us: 0, rss_ratio: f64::INFINITY }).is_empty(), "counts only: {:?}", bench_warnings(&a, &b, &BENCH_BAND));
        if let Some(out) = std::env::var_os("POWRUSH_BENCH_OUT") { std::fs::write(out, serde_json::to_string_pretty(&a).unwrap()).unwrap(); }
        // PERF-CI-WARN-1: CI sets POWRUSH_BENCH_BASE; a missing file prints nothing, and the band is BENCH_BAND itself (no copy in YAML).
        if let Some(p) = std::env::var_os("POWRUSH_BENCH_BASE").filter(|p| std::path::Path::new(p).is_file()) {
            match std::fs::read_to_string(&p).ok().and_then(|s| serde_json::from_str::<BenchWalk>(&s).ok()) {
                Some(base) => for w in bench_warnings(&base, &a, &BENCH_BAND) { println!("::warning title=PERF bench walk::{w}"); },
                None => println!("::warning title=PERF bench walk::baseline unreadable"),
            }
        }
    }

    #[test]
    fn focused_hundred_releases_wait_full_period() {
        let (mut app, state) = pace_app(Advance::Full, false);
        spawn_primaries(&mut app, 1, true);
        update_n(&mut app, 100);
        let inner = snapshot(&state);
        assert_eq!(inner.sleeps, vec![FOCUSED_PERIOD; 100]);
        assert_period_releases(inner.origin, &inner.releases, FOCUSED_PERIOD, 100);
        assert!(inner
            .releases
            .iter()
            .all(|release| inner.samples.contains(release)));
        assert_representable(&app);
    }

    #[test]
    fn inactive_hundred_releases_wait_full_period() {
        let (mut app, state) = pace_app(Advance::Full, false);
        spawn_primaries(&mut app, 1, false);
        update_n(&mut app, 100);
        let inner = snapshot(&state);
        assert_eq!(inner.sleeps, vec![INACTIVE_PERIOD; 100]);
        assert_period_releases(inner.origin, &inner.releases, INACTIVE_PERIOD, 100);
        assert!(inner
            .releases
            .iter()
            .all(|release| inner.samples.contains(release)));
        assert_representable(&app);
    }

    #[test]
    fn overdue_releases_once_without_repayment() {
        let (mut app, state) = pace_app(Advance::Full, false);
        spawn_primaries(&mut app, 1, true);
        app.update();
        let after_first = snapshot(&state).sleeps.len();
        {
            let mut inner = state.lock().expect("fake");
            let jump = FOCUSED_PERIOD.checked_mul(10).expect("jump");
            inner.now = inner.now.checked_add(jump).expect("jump");
        }
        app.update();
        let inner = snapshot(&state);
        assert_eq!(after_first, 1);
        assert_eq!(inner.sleeps.len(), 1, "overdue entry does not sleep");
        assert_eq!(inner.releases.len(), 2, "one release, no repayment");
        let gap = inner.releases[1]
            .checked_duration_since(inner.releases[0])
            .expect("gap");
        assert_eq!(gap, FOCUSED_PERIOD.checked_mul(10).expect("gap"));
        assert_representable(&app);
        assert_next_update_waits_from_late_release(
            &mut app,
            &state,
            inner.releases[1],
            FOCUSED_PERIOD,
        );
    }

    #[test]
    fn oversleep_next_update_waits_from_actual_release() {
        let extra = FOCUSED_PERIOD / 2;
        let (mut app, state) = pace_app(Advance::OversleepOnce { extra }, false);
        spawn_primaries(&mut app, 1, true);
        app.update();
        let late = snapshot(&state);
        assert_eq!(late.releases.len(), 1, "oversleep releases once");
        assert_eq!(late.sleeps.len(), 1, "oversleep is one sleep");
        assert_eq!(late.sleeps[0], FOCUSED_PERIOD);
        let advanced = late.releases[0]
            .checked_duration_since(late.origin)
            .expect("advance");
        assert!(
            advanced > late.sleeps[0],
            "clock advanced by more than the requested sleep"
        );
        assert_eq!(advanced.checked_sub(late.sleeps[0]).expect("extra"), extra);
        let armed = late.origin.checked_add(FOCUSED_PERIOD).expect("deadline");
        assert!(late.releases[0] > armed, "release is overdue");
        assert_next_update_waits_from_late_release(
            &mut app,
            &state,
            late.releases[0],
            FOCUSED_PERIOD,
        );
    }

    #[test]
    fn focus_to_inactive_keeps_the_later_deadline() {
        let (mut app, state) = pace_app(Advance::Full, false);
        spawn_primaries(&mut app, 1, true);
        app.update();
        set_focused(&mut app, false);
        app.update();
        let inner = snapshot(&state);
        assert_eq!(inner.sleeps, vec![FOCUSED_PERIOD, INACTIVE_PERIOD]);
        assert_eq!(
            inner.releases[1]
                .checked_duration_since(inner.releases[0])
                .expect("gap"),
            INACTIVE_PERIOD
        );
        assert_representable(&app);
    }

    #[test]
    fn focused_regain_does_not_pull_the_deadline_earlier() {
        let (mut app, state) = pace_app(Advance::Full, false);
        spawn_primaries(&mut app, 1, false);
        app.update();
        set_focused(&mut app, true);
        app.update();
        let inner = snapshot(&state);
        assert_eq!(inner.sleeps, vec![INACTIVE_PERIOD, INACTIVE_PERIOD]);
        assert_eq!(
            inner.releases[1]
                .checked_duration_since(inner.releases[0])
                .expect("gap"),
            INACTIVE_PERIOD
        );
        assert_ne!(inner.sleeps[1], FOCUSED_PERIOD);
        assert_representable(&app);
    }

    #[test]
    fn focus_flap_does_not_bypass() {
        let (mut app, state) = pace_app(Advance::Full, false);
        spawn_primaries(&mut app, 1, true);
        app.update();
        set_focused(&mut app, false);
        app.update();
        set_focused(&mut app, true);
        app.update();
        app.update();
        let inner = snapshot(&state);
        assert_eq!(
            inner.sleeps,
            vec![
                FOCUSED_PERIOD,
                INACTIVE_PERIOD,
                INACTIVE_PERIOD,
                FOCUSED_PERIOD
            ]
        );
        assert_eq!(inner.releases.len(), 4);
        assert_representable(&app);
    }

    #[test]
    fn missing_primary_window_is_inactive() {
        let (mut app, state) = pace_app(Advance::Full, false);
        app.update();
        let inner = snapshot(&state);
        assert_eq!(inner.sleeps, vec![INACTIVE_PERIOD]);
        assert_eq!(inner.releases.len(), 1);
        assert_representable(&app);
    }

    #[test]
    fn two_primary_windows_are_inactive() {
        let (mut app, state) = pace_app(Advance::Full, false);
        spawn_primaries(&mut app, 2, true);
        app.update();
        let inner = snapshot(&state);
        assert_eq!(inner.sleeps, vec![INACTIVE_PERIOD]);
        assert_eq!(inner.releases.len(), 1);
        assert_representable(&app);
    }

    /// PERF-PACE-PIN-1: today's pacing values, pinned as they are. Changing one is a council/Sherif call; this only makes it deliberate.
    #[test]
    fn perf_pace_pin_1_focused_and_inactive_spacing_values() {
        assert_eq!(FOCUSED_PERIOD, Duration::from_nanos(16_666_667), "focused release spacing (60 Hz)");
        assert_eq!(INACTIVE_PERIOD, Duration::from_millis(250), "unfocused / missing / ambiguous window release spacing (4 Hz)");
    }
    /// PERF-PACE-PIN-1: main.rs keeps Fifo and registers FramePacePlugin in `fn main`; neither perf file overrides Bevy's WinitSettings.
    #[test]
    fn perf_pace_pin_1_main_fifo_plugin_and_no_winit_override() {
        let code = |src: &'static str| src.split("\n#[cfg(test)]").next().unwrap().lines().map(|l| l.split("//").next().unwrap_or("")).collect::<Vec<_>>().join("\n");
        let main = code(include_str!("main.rs"));
        let body = main.split("fn main() {").nth(1).and_then(|s| s.split("\n}\n").next()).expect("fn main");
        assert_eq!(main.matches("PresentMode::").count(), 1, "one present mode in main.rs");
        assert!(body.contains("present_mode: bevy::window::PresentMode::Fifo,"), "Fifo in fn main");
        assert!(body.contains("powrush_client::frame_pace::FramePacePlugin,"), "FramePacePlugin registered in fn main");
        for (file, src) in [("main.rs", &main), ("frame_pace.rs", &code(include_str!("frame_pace.rs")))] { for t in ["WinitSettings", "UpdateMode"] { assert!(!src.contains(t), "{file} sets {t}"); } }
    }

    #[derive(Resource, Default)]
    struct LastProbe {
        captured: bool,
        sleeps_at_update_end: usize,
        sleeps_before: usize,
        sleeps_after: usize,
        update_frames_start: u32,
        logger_in_update: bool,
    }

    #[derive(Resource)]
    struct SharedFake(Arc<Mutex<FakeInner>>);

    fn probe_update_start(mut probe: ResMut<LastProbe>, pace: Res<FramePace>) {
        probe.update_frames_start = pace.frames;
    }

    fn probe_update_end(
        mut probe: ResMut<LastProbe>,
        pace: Res<FramePace>,
        shared: Res<SharedFake>,
    ) {
        let sleeps = shared.0.lock().expect("fake").sleeps.len();
        if !probe.captured {
            probe.sleeps_at_update_end = sleeps;
        }
        if pace.frames > probe.update_frames_start {
            probe.logger_in_update = true;
        }
    }

    fn probe_before(mut probe: ResMut<LastProbe>, shared: Res<SharedFake>) {
        if !probe.captured {
            probe.sleeps_before = shared.0.lock().expect("fake").sleeps.len();
        }
    }

    fn probe_after(mut probe: ResMut<LastProbe>, shared: Res<SharedFake>) {
        if !probe.captured {
            probe.sleeps_after = shared.0.lock().expect("fake").sleeps.len();
            probe.captured = true;
        }
    }

    #[test]
    fn plugin_registers_waiter_on_last() {
        let (mut app, state) = pace_app(Advance::Full, false);
        app.insert_resource(SharedFake(state))
            .init_resource::<LastProbe>()
            .add_systems(
                Update,
                (
                    probe_update_start.before(frame_pace_system),
                    probe_update_end.after(frame_pace_system),
                ),
            )
            .add_systems(
                Last,
                (
                    probe_before.before(release_waiter),
                    probe_after.after(release_waiter),
                ),
            );
        for _ in 0..8 {
            std::thread::sleep(Duration::from_millis(2));
            app.update();
        }
        let probe = app.world().resource::<LastProbe>();
        assert_eq!(probe.sleeps_at_update_end, 0);
        assert_eq!(probe.sleeps_before, 0);
        assert_eq!(probe.sleeps_after, 1);
        assert!(probe.logger_in_update);
    }

    #[derive(Resource)]
    struct DownstreamBurn(Arc<Mutex<FakeInner>>);

    fn burn_after_release(burn: Res<DownstreamBurn>) {
        let mut inner = burn.0.lock().expect("fake");
        let Some(step) = inner.burns.pop_front() else {
            return;
        };
        inner.now = inner.now.checked_add(step).expect("burn");
    }

    #[test]
    fn downstream_burn_does_not_shrink_release_gap() {
        let (mut app, state) = pace_app(Advance::Full, false);
        spawn_primaries(&mut app, 1, true);
        {
            let mut inner = state.lock().expect("fake");
            inner.burns.push_back(FOCUSED_PERIOD / 2);
            inner.burns.push_back(Duration::ZERO);
            inner
                .burns
                .push_back(FOCUSED_PERIOD.checked_mul(2).expect("burn"));
        }
        app.insert_resource(DownstreamBurn(state.clone()))
            .add_systems(Last, burn_after_release.after(release_waiter));
        update_n(&mut app, 4);
        let inner = snapshot(&state);
        let half = FOCUSED_PERIOD / 2;
        assert_eq!(
            inner.sleeps,
            vec![FOCUSED_PERIOD, FOCUSED_PERIOD - half, FOCUSED_PERIOD]
        );
        assert_eq!(inner.releases.len(), 4);
        let gaps = [
            inner.releases[1]
                .checked_duration_since(inner.releases[0])
                .expect("gap"),
            inner.releases[2]
                .checked_duration_since(inner.releases[1])
                .expect("gap"),
            inner.releases[3]
                .checked_duration_since(inner.releases[2])
                .expect("gap"),
        ];
        assert!(gaps.iter().all(|gap| *gap >= FOCUSED_PERIOD));
        assert_eq!(gaps[2], FOCUSED_PERIOD.checked_mul(2).expect("gap"));
        assert_representable(&app);
        assert_next_update_waits_from_late_release(
            &mut app,
            &state,
            inner.releases[3],
            FOCUSED_PERIOD,
        );
    }

    #[test]
    fn early_wake_resamples_until_deadline() {
        let (mut app, state) = pace_app(Advance::Early, false);
        spawn_primaries(&mut app, 1, true);
        app.update();
        let inner = snapshot(&state);
        assert!(inner.sleeps.len() >= 2, "early wake samples again");
        assert!(inner.sleeps.iter().all(|sleep| !sleep.is_zero()));
        assert!(
            inner.sleeps.iter().any(|sleep| *sleep < FOCUSED_PERIOD),
            "a later sample sleeps only the leftover remainder"
        );
        assert_eq!(inner.releases.len(), 1);
        assert_eq!(
            inner.releases[0],
            inner.origin.checked_add(FOCUSED_PERIOD).expect("deadline")
        );
        assert!(inner
            .samples
            .iter()
            .any(|sample| *sample < inner.releases[0]));
        assert_representable(&app);
    }

    #[test]
    fn never_releases_before_representable_deadline() {
        let (mut app, state) = pace_app(Advance::Early, false);
        spawn_primaries(&mut app, 1, true);
        update_n(&mut app, 8);
        let inner = snapshot(&state);
        assert!(inner.sleeps.len() > inner.releases.len());
        assert!(inner.sleeps.iter().all(|sleep| !sleep.is_zero()));
        assert_eq!(inner.releases.len(), 8);
        let mut due = inner.origin;
        for release in &inner.releases {
            due = due.checked_add(FOCUSED_PERIOD).expect("deadline");
            assert!(
                *release >= due,
                "release {release:?} before representable deadline {due:?}"
            );
            assert_eq!(*release, due);
            assert!(inner.samples.contains(release));
        }
        assert_representable(&app);
    }

    #[test]
    fn checked_add_none_sleeps_one_period_and_holds() {
        let (mut app, state) = pace_app(Advance::Full, true);
        spawn_primaries(&mut app, 1, true);
        for _ in 0..3 {
            let before = snapshot(&state);
            app.update();
            let after = snapshot(&state);
            assert_eq!(after.sleeps.len(), before.sleeps.len() + 1);
            assert_eq!(after.releases.len(), before.releases.len() + 1);
            assert_eq!(*after.sleeps.last().expect("sleep"), FOCUSED_PERIOD);
        }
        {
            let mut inner = state.lock().expect("fake");
            inner.checked_add_none = false;
            let jump = FOCUSED_PERIOD.checked_mul(50).expect("jump");
            inner.now = inner.now.checked_add(jump).expect("jump");
        }
        for _ in 0..2 {
            let before = snapshot(&state);
            app.update();
            let after = snapshot(&state);
            assert_eq!(
                after.sleeps.len(),
                before.sleeps.len() + 1,
                "hold sleeps once"
            );
            assert_eq!(after.releases.len(), before.releases.len() + 1, "no burst");
            assert_eq!(*after.sleeps.last().expect("sleep"), FOCUSED_PERIOD);
            assert!(after.sleeps.iter().all(|sleep| !sleep.is_zero()));
            let pace = app.world().resource::<ReleasePace>();
            assert!(
                pace.overflow_hold,
                "hold persists after checked_add recovers"
            );
            assert!(pace.deadline.is_none(), "hold stores no deadline");
        }
        let inner = snapshot(&state);
        assert_eq!(inner.sleeps.len(), 5);
        assert_eq!(inner.releases.len(), 5);
        assert!(inner.now_calls <= CALL_CAP);
        let pace = app.world().resource::<ReleasePace>();
        assert!(pace.overflow_hold);
        assert!(pace.deadline.is_none());
    }

    /// Restores the user-dir override and removes the scratch dir on drop, including a failed assert.
    struct PerfEnv(Option<String>, std::path::PathBuf);
    impl Drop for PerfEnv { fn drop(&mut self) { match self.0.take() { Some(v) => std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, v), None => std::env::remove_var(shared::user_persist::USER_DIR_OVERRIDE_ENV) } let _ = std::fs::remove_dir_all(&self.1); } }
    #[test]
    fn perf_log_env_and_roll_file() {
        for (v, on) in [(None, false), (Some("0"), false), (Some("junk"), false), (Some("1"), true), (Some("on"), true), (Some("true"), true), (Some("yes"), true)] { assert_eq!(perf_log_on(v), on); }
        let _g = crate::test_env::lock(); let k = shared::user_persist::USER_DIR_OVERRIDE_ENV; let dir = std::env::temp_dir().join(format!("p{}", std::process::id())); let _env = PerfEnv(std::env::var(k).ok(), dir.clone()); std::env::set_var(k, &dir);
        assert!(FramePace::default().tick(1.01, 1.0).is_some()); assert!(!shared::user_persist::persist_path("data/perf.log").exists());
        let c = { let mut p = FramePace::default(); p.tick(1.01, 1.0).unwrap() }; append_perf_line(&format!("frame_pace: frames={} over_budget={} hitches={} p50_ms<=2 p95_ms<=2", c.frames, c.over_budget, c.hitches));
        let s = std::fs::read_to_string(shared::user_persist::persist_path("data/perf.log")).unwrap(); assert!(s.contains("p50_ms") && !s.contains("adapter="));
    }

    #[test]
    fn crash_report_1_line_carries_build_version_adapter_tier_and_frames() {
        let snap = CrashSnap::new();
        assert_eq!(snap.line(7, "abc1234"), format!("7 crash_report build=abc1234 version={} adapter=\"unknown\" tier=unknown frames_ms=[]\n", env!("CARGO_PKG_VERSION")));
        snap.note(0.016_667, Some("llvm\npipe \"soft\""), Some(GraphicsPreset::Ultra)); snap.note(0.0, Some("second"), None); snap.note(0.033, None, None);
        let line = snap.line(9, "abc1234");
        assert_eq!(line, format!("9 crash_report build=abc1234 version={} adapter=\"llvmpipe 'soft'\" tier=Ultra frames_ms=[16.7,33.0]\n", env!("CARGO_PKG_VERSION")));
        assert_eq!(line.matches('\n').count(), 1); assert!(!line.contains('\r'));
        snap.note(0.016, None, Some(GraphicsPreset::Mobile)); assert!(snap.line(1, "x").contains(" tier=Mobile "));
        let long = CrashSnap::new(); long.note(0.01, Some(&"g".repeat(500)), None); assert!(long.line(1, "x").contains(&format!("adapter=\"{}\"", "g".repeat(96))));
        assert!(!BUILD_SHA.is_empty());
    }
    #[test]
    fn crash_report_1_ring_keeps_last_32_frames_oldest_first() {
        let snap = CrashSnap::new();
        for ms in 1..=5_u32 { snap.note(ms as f64 / 1000.0, None, None); }
        assert_eq!(snap.frames_us(), (1..=5).map(|ms| ms * 1000).collect::<Vec<_>>());
        for ms in 6..=40_u32 { snap.note(ms as f64 / 1000.0, None, None); }
        assert_eq!(snap.frames_us(), (9..=40).map(|ms| ms * 1000).collect::<Vec<_>>());
        snap.note(1e9, None, None); assert_eq!(snap.frames_us().last(), Some(&u32::MAX));
    }
    #[test]
    fn crash_report_1_system_feeds_snapshot_and_module_stays_offline_and_lock_free() {
        let prod = include_str!("frame_pace.rs").split("\n#[cfg(test)]\nmod tests {").next().unwrap();
        let sys = prod.split("fn frame_pace_system(").nth(1).and_then(|s| s.split("\n}\n").next()).expect("frame_pace_system");
        assert!(sys.contains("CRASH_SNAP.note(time.delta_secs_f64(), adapter.as_deref().map(|a| a.name.as_str()), settings.as_deref().map(|s| s.inner.graphics_preset))"));
        let snap = prod.split("pub struct CrashSnap").nth(1).and_then(|s| s.split("\n}\n").next()).expect("CrashSnap impl");
        for banned in ["Mutex", "RwLock", ".lock(", "get_or_init", "unwrap(", "expect(", "self.us[i]"] { assert!(!snap.contains(banned), "CrashSnap hook path uses {banned}"); }
        let main = include_str!("main.rs").split("\n#[cfg(test)]").next().unwrap();
        assert!(main.contains("powrush_client::frame_pace::CRASH_SNAP.line(epoch_secs, powrush_client::frame_pace::BUILD_SHA)"));
        for src in [prod, main] { for net in ["std::net", "TcpStream", "UdpSocket", "reqwest", "ureq", "hyper", "http://", "https://", "process::Command", "steamworks"] { assert!(!src.contains(net), "network or upload path: {net}"); } }
    }
}
