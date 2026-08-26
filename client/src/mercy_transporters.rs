/*!
 * Mercy Transporters — soft autonomous care helpers (v21.96.0)
 *
 * Toggle: **T** (Transport — ergonomic). Force cloud flush: **Shift+T**.
 *
 * PATSAGi + TOLC 8 · Contact: info@Rathor.ai · Yoi ⚡
 */

use bevy::prelude::*;

use crate::abundance_journey_echo::{AbundanceJourneyEcho, JourneyKind};
use crate::lattice_flow_share::LatticeFlowShare;
use crate::rbe_allocate_choice::RbeAllocateChoice;
use crate::soft_play_bindings;
use crate::steam_abundance_mirror::SteamAbundanceMirror;

const CARE_INTERVAL_SECS: f64 = 45.0;

#[derive(Resource, Debug)]
pub struct MercyTransporters {
    pub enabled: bool,
    pub carries: u32,
    pub last_carry_note: Option<String>,
    pub last_care_at: f64,
    pub last_seen_exports: u32,
    pub last_seen_choices: u32,
}

impl Default for MercyTransporters {
    fn default() -> Self {
        Self {
            enabled: true,
            carries: 0,
            last_carry_note: None,
            last_care_at: 0.0,
            last_seen_exports: 0,
            last_seen_choices: 0,
        }
    }
}

impl MercyTransporters {
    pub fn status_line(&self) -> String {
        if !self.enabled {
            "Mercy Transporters  resting (T to wake)".to_string()
        } else if self.carries == 0 {
            "Mercy Transporters  ready · caring for durable foundations".to_string()
        } else {
            format!(
                "Mercy Transporters  {} carries · {}",
                self.carries,
                self.last_carry_note
                    .as_deref()
                    .unwrap_or("abundance held with care")
            )
        }
    }
}

pub struct MercyTransportersPlugin;

impl Plugin for MercyTransportersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MercyTransporters>().add_systems(
            Update,
            (
                toggle_transporters,
                soft_nudge_on_progress,
                soft_idle_care,
                observe_successful_carries,
            ),
        );
    }
}

fn toggle_transporters(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut transporters: ResMut<MercyTransporters>,
    mut echo: ResMut<AbundanceJourneyEcho>,
) {
    // Plain T toggles transporters; Shift+T is reserved for force cloud flush.
    let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    if !keyboard.just_pressed(soft_play_bindings::MERCY_TRANSPORTERS) || shift {
        return;
    }
    transporters.enabled = !transporters.enabled;
    let note = if transporters.enabled {
        "Mercy Transporters woke — caring for durable foundations"
    } else {
        "Mercy Transporters resting — you hold the logistics yourself"
    };
    echo.push(JourneyKind::Note, note);
    info!(target: "powrush::mercy_transport", enabled = transporters.enabled, "{note}");
}

fn soft_nudge_on_progress(
    allocate: Res<RbeAllocateChoice>,
    lattice: Res<LatticeFlowShare>,
    mut transporters: ResMut<MercyTransporters>,
    mut mirror: ResMut<SteamAbundanceMirror>,
) {
    if !transporters.enabled {
        return;
    }
    let progressed = allocate.choices_made > transporters.last_seen_choices
        || lattice.last_exported_choices > transporters.last_seen_choices;
    if !progressed {
        return;
    }
    transporters.last_seen_choices = allocate
        .choices_made
        .max(lattice.last_exported_choices);
    mirror.force_pending = true;
    info!(
        target: "powrush::mercy_transport",
        choices = transporters.last_seen_choices,
        "Transporter nudged durable abundance stage"
    );
}

fn soft_idle_care(
    time: Res<Time>,
    mut transporters: ResMut<MercyTransporters>,
    mut mirror: ResMut<SteamAbundanceMirror>,
) {
    if !transporters.enabled {
        return;
    }
    let now = time.elapsed_seconds_f64();
    if now - transporters.last_care_at < CARE_INTERVAL_SECS {
        return;
    }
    transporters.last_care_at = now;
    if mirror.exports == 0 || !mirror.last_stage_ok {
        mirror.force_pending = true;
        info!(target: "powrush::mercy_transport", "Idle care — soft durability nudge");
    }
}

fn observe_successful_carries(
    mirror: Res<SteamAbundanceMirror>,
    mut transporters: ResMut<MercyTransporters>,
    mut echo: ResMut<AbundanceJourneyEcho>,
) {
    if !transporters.enabled {
        return;
    }
    if mirror.exports <= transporters.last_seen_exports {
        return;
    }
    if !mirror.last_stage_ok {
        transporters.last_seen_exports = mirror.exports;
        return;
    }
    transporters.last_seen_exports = mirror.exports;
    transporters.carries = transporters.carries.saturating_add(1);
    let note = mirror
        .last_note
        .clone()
        .unwrap_or_else(|| "abundance staged".into());
    let line = format!(
        "Mercy Transporter carried abundance to durable foundations · {}",
        note
    );
    transporters.last_carry_note = Some(line.clone());
    echo.push(JourneyKind::Note, line.clone());
    info!(target: "powrush::mercy_transport", carries = transporters.carries, "{line}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_enabled() {
        let t = MercyTransporters::default();
        assert!(t.enabled);
    }
}
