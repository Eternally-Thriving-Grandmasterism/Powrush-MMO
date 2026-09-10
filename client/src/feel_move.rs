//! Feel-move — fixed 60 Hz Use buffer (SC2-feel contract).
//!
//! One buffered Use ≤120 ms when entering range. Move runs on FixedUpdate
//! in `human_presence`. Does not change Peace verbs, Title Online, A4, or rooms.
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use crate::depths_landing::DepthsPeaceTend;
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::input::PlayerInput;
use crate::mercy_harvest_nodes::NearbyMercyNode;

/// Fixed sim rate for move + Use (not render frame).
pub const SIM_HZ: f64 = 60.0;
/// Buffer one Use this long when pressed before in-range.
pub const USE_BUFFER_SECS: f64 = 0.120;

#[derive(Resource, Debug, Default, Clone, PartialEq)]
pub struct UseBuffer {
    /// Elapsed seconds when the buffered Use expires (exclusive).
    pub until: Option<f64>,
}

#[derive(Resource, Debug, Default, Clone, PartialEq)]
pub struct SimUsePulse {
    /// True for one fixed tick when a buffered Use should fire in range.
    pub fire: bool,
}

pub struct FeelMovePlugin;

impl Plugin for FeelMovePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(SIM_HZ))
            .init_resource::<UseBuffer>()
            .init_resource::<SimUsePulse>()
            .add_systems(
                Update,
                (raise_buffered_use, arm_use_buffer_from_edge)
                    .chain()
                    .after(crate::input::InputMapSet),
            )
            .add_systems(FixedUpdate, resolve_use_buffer);
    }
}

/// Fold Fixed-tick buffered Use into the same `PlayerInput.interact` Peace hooks already read.
fn raise_buffered_use(mut input: ResMut<PlayerInput>, mut pulse: ResMut<SimUsePulse>) {
    if pulse.fire {
        input.interact = true;
        pulse.fire = false;
    }
}

fn arm_use_buffer_from_edge(
    input: Res<PlayerInput>,
    time: Res<Time>,
    mut buf: ResMut<UseBuffer>,
    nearby: Option<Res<NearbyMercyNode>>,
    depths: Option<Res<DepthsPeaceTend>>,
    epiphany: Option<Res<FirstHarvestEpiphany>>,
) {
    if !input.interact {
        return;
    }
    let in_range = use_target_in_range(nearby.as_deref(), depths.as_deref(), epiphany.as_deref());
    if in_range {
        // Immediate edge this Update — do not leave a buffer that double-fires next Fixed.
        buf.until = None;
        return;
    }
    let now = time.elapsed_seconds_f64();
    buf.until = Some(now + USE_BUFFER_SECS);
}

fn resolve_use_buffer(
    time: Res<Time>,
    mut buf: ResMut<UseBuffer>,
    mut pulse: ResMut<SimUsePulse>,
    nearby: Option<Res<NearbyMercyNode>>,
    depths: Option<Res<DepthsPeaceTend>>,
    epiphany: Option<Res<FirstHarvestEpiphany>>,
) {
    // Do not clear `pulse.fire` here — sticky until Update `raise_buffered_use`
    // consumes it (multi Fixed ticks in one frame must not drop the pulse).
    let Some(until) = buf.until else {
        return;
    };
    let now = time.elapsed_seconds_f64();
    if now > until {
        buf.until = None;
        return;
    }
    let in_range = use_target_in_range(nearby.as_deref(), depths.as_deref(), epiphany.as_deref());
    if !in_range {
        return;
    }
    pulse.fire = true;
    buf.until = None;
}

/// Any Peace Use claim that can accept a buffered tap.
pub fn use_target_in_range(
    nearby: Option<&NearbyMercyNode>,
    depths: Option<&DepthsPeaceTend>,
    epiphany: Option<&FirstHarvestEpiphany>,
) -> bool {
    if nearby.map(|n| n.in_range).unwrap_or(false) {
        return true;
    }
    if depths.map(|d| d.near).unwrap_or(false) {
        return true;
    }
    let Some(e) = epiphany else {
        return false;
    };
    e.threshold_near || e.wards_near || e.depths_near
}

/// True when this tick should treat Use as pressed (edge or buffer pulse).
pub fn use_should_fire(input: &PlayerInput, pulse: &SimUsePulse) -> bool {
    input.interact || pulse.fire
}

/// Pure: arm buffer timestamp from a press at `now`.
pub fn arm_use_at(now: f64) -> f64 {
    now + USE_BUFFER_SECS
}

/// Pure: buffered Use still live at `now`?
pub fn buffer_alive(until: f64, now: f64) -> bool {
    now <= until
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sim_tick_is_sixty() {
        assert!((SIM_HZ - 60.0).abs() < f64::EPSILON);
        let step = 1.0 / SIM_HZ;
        assert!((step - 1.0 / 60.0).abs() < 1e-9);
    }

    #[test]
    fn use_buffer_window_is_120ms() {
        assert!((USE_BUFFER_SECS - 0.120).abs() < 1e-9);
        let armed = arm_use_at(1.0);
        assert!(buffer_alive(armed, 1.0));
        assert!(buffer_alive(armed, 1.12));
        assert!(!buffer_alive(armed, 1.121));
    }

    #[test]
    fn use_should_fire_from_edge_or_pulse() {
        let mut input = PlayerInput::default();
        let mut pulse = SimUsePulse::default();
        assert!(!use_should_fire(&input, &pulse));
        input.interact = true;
        assert!(use_should_fire(&input, &pulse));
        input.interact = false;
        pulse.fire = true;
        assert!(use_should_fire(&input, &pulse));
    }
}
