//! CARD GFX-PACE-1 — a log-only frame-pace hitch counter.
//! Counts real frames over a 60 Hz budget and a 2x hitch line; logs once per 1 s roll.

use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::time::Real;

/// One frame at 60 Hz, in seconds.
pub const FRAME_BUDGET_SECS: f64 = 1.0 / 60.0;

/// A hitch is a frame longer than two 60 Hz frames.
pub const HITCH_SECS: f64 = 2.0 * FRAME_BUDGET_SECS;

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
        if self.elapsed < roll_secs {
            return None;
        }
        Some(std::mem::take(self))
    }
}

/// Adds the frame-pace state and its log-only system. Registered from `main.rs` only.
pub struct FramePacePlugin;

impl Plugin for FramePacePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FramePace>()
            .add_systems(Update, frame_pace_system);
    }
}

fn frame_pace_system(time: Res<Time<Real>>, mut pace: ResMut<FramePace>) {
    let roll_secs = LogDiagnosticsPlugin::default().wait_duration.as_secs_f64();
    if let Some(c) = pace.tick(time.delta_secs_f64(), roll_secs) {
        info!(
            "frame_pace: frames={} over_budget={} hitches={}",
            c.frames, c.over_budget, c.hitches
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROLL: f64 = 1.0;

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
}
