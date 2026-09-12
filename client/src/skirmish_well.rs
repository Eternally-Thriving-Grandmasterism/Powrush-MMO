//! Lived-hour skirmish well — Slice 15 (v23.2.22) + P2 soft answer
//!
//! E contests the first well. Dawn after loss. Soft slab pulse on win (not a HUD).
//! Lives in Peace. Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::skirmish_well::{SkirmishWell, WellHold, CONTEST_REACH, WELL_ANCHORS};

use crate::coop_voice::VoiceYard;
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::human_presence::SoftPresence;
use crate::ledger_bind::LedgerYard;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

const HOLD_SECS: f64 = 6.0;

/// PATSAGi well-slab breath. One pulse, then rest. Not a second HUD.
pub const WELL_GLOW_DECAY: f32 = 0.55;

/// Soft rim + fill lift on top of the caller's rest colors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WellGlowPulse {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub bg_r: f32,
    pub bg_g: f32,
    pub bg_b: f32,
}

/// Tick a glow toward rest. Climate WeekFeelGlow / WardsNoticeGlow reuse this.
pub fn tick_well_glow_breath(glow: f32, dt: f32) -> f32 {
    if glow > 0.0 {
        (glow - dt * WELL_GLOW_DECAY).max(0.0)
    } else {
        0.0
    }
}

/// Same ~0.55-decay rim lift the skirmish well uses on contest win.
pub fn well_glow_pulse(glow: f32) -> WellGlowPulse {
    let glow = glow.clamp(0.0, 1.0);
    WellGlowPulse {
        r: glow * 0.20,
        g: glow * 0.16,
        b: glow * 0.12,
        a: glow * 0.40,
        bg_r: glow * 0.08,
        bg_g: glow * 0.10,
        bg_b: glow * 0.06,
    }
}

#[derive(Resource, Debug, Clone)]
pub struct WellYard {
    pub well: SkirmishWell,
    pub hold_until: f64,
    /// Soft slab breath after contest win (P2). Not a second HUD.
    pub well_glow: f32,
}

impl Default for WellYard {
    fn default() -> Self {
        Self {
            well: SkirmishWell::default(),
            hold_until: 0.0,
            well_glow: 0.0,
        }
    }
}

#[derive(Component)]
struct WellSlabRoot;
#[derive(Component)]
struct WellSlabText;

pub struct SkirmishWellPlugin;

impl Plugin for SkirmishWellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WellYard>()
            .add_systems(Startup, spawn_well_slab)
            .add_systems(PreUpdate, mark_well_near)
            .add_systems(
                Update,
                (pressure_hold, handle_well, tick_well_glow, update_well_slab),
            );
    }
}

fn spawn_well_slab(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(132.0),
                    left: Val::Px(16.0),
                    width: Val::Px(420.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.07, 0.09, 0.08, 0.92).into(),
                border_color: Color::srgba(0.55, 0.78, 0.62, 0.50).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            WellSlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.82, 0.96, 0.84),
                        ..default()
                    },
                ),
                WellSlabText,
            ));
        });
}

fn near_first_well(presence: &SoftPresence) -> bool {
    let (x, y, z) = WELL_ANCHORS[0];
    presence.position.distance(Vec3::new(x, y, z)) <= CONTEST_REACH
}

fn mark_well_near(
    presence: Res<SoftPresence>,
    yard: Res<WellYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    mut epi: ResMut<FirstHarvestEpiphany>,
) {
    epi.well_near = near_first_well(&presence)
        && yard.well.wants_interact()
        && !voice.sash_open
        && !ledger.sash_open
        && !epi.peace_visitor;
}

fn pressure_hold(time: Res<Time>, mut yard: ResMut<WellYard>) {
    if yard.well.hold != WellHold::Human {
        return;
    }
    if time.elapsed_seconds_f64() < yard.hold_until {
        return;
    }
    let _ = yard.well.traveler_answers();
}

fn handle_well(
    keyboard: Res<ButtonInput<KeyCode>>,
    presence: Res<SoftPresence>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    epi: Res<FirstHarvestEpiphany>,
    mut yard: ResMut<WellYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if !near_first_well(&presence) {
        return;
    }
    if voice.sash_open || ledger.sash_open || epi.peace_visitor {
        return;
    }
    yard.well.reveal();
    if !keyboard.just_pressed(soft_play_bindings::INTERACT) {
        return;
    }
    let step = yard.well.act();
    if step == "won" {
        yard.hold_until = time.elapsed_seconds_f64() + HOLD_SECS;
        yard.well_glow = 1.0;
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstWell,
            time.elapsed_seconds_f64(),
        );
    }
}

fn tick_well_glow(time: Res<Time>, mut yard: ResMut<WellYard>) {
    yard.well_glow = tick_well_glow_breath(yard.well_glow, time.delta_seconds());
}

fn update_well_slab(
    presence: Res<SoftPresence>,
    yard: Res<WellYard>,
    mut root: Query<
        (&mut Visibility, &mut BorderColor, &mut BackgroundColor),
        With<WellSlabRoot>,
    >,
    mut text_q: Query<&mut Text, With<WellSlabText>>,
) {
    let show = near_first_well(&presence);
    let glow = yard.well_glow;
    for (mut vis, mut border, mut bg) in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        if show {
            let pulse = well_glow_pulse(glow);
            *border = Color::srgba(
                0.55 + pulse.r,
                0.78 + pulse.g,
                0.62 + pulse.b,
                0.50 + pulse.a,
            )
            .into();
            *bg = Color::srgba(
                0.07 + pulse.bg_r,
                0.09 + pulse.bg_g,
                0.08 + pulse.bg_b,
                0.92,
            )
            .into();
        }
    }
    if !show {
        return;
    }
    let line = yard.well.slab_line();
    for mut text in &mut text_q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != line {
                s.value = line.clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn far_from_spawn_is_not_near() {
        let p = SoftPresence::default();
        assert!(!near_first_well(&p));
    }

    #[test]
    fn well_glow_breath_decays_at_patsagi_rate() {
        assert_eq!(WELL_GLOW_DECAY, 0.55);
        assert!((tick_well_glow_breath(1.0, 1.0) - 0.45).abs() < 1e-6);
        assert_eq!(tick_well_glow_breath(0.10, 1.0), 0.0);
        assert_eq!(tick_well_glow_breath(0.0, 0.5), 0.0);
    }

    #[test]
    fn well_glow_pulse_is_soft_border_not_a_plate() {
        let rest = well_glow_pulse(0.0);
        assert_eq!(rest, WellGlowPulse { r: 0.0, g: 0.0, b: 0.0, a: 0.0, bg_r: 0.0, bg_g: 0.0, bg_b: 0.0 });
        let peak = well_glow_pulse(1.0);
        assert!((peak.a - 0.40).abs() < 1e-6);
        assert!((peak.r - 0.20).abs() < 1e-6);
        assert!((peak.g - 0.16).abs() < 1e-6);
        assert!((peak.b - 0.12).abs() < 1e-6);
        assert!((peak.bg_r - 0.08).abs() < 1e-6);
        assert!((peak.bg_g - 0.10).abs() < 1e-6);
        assert!((peak.bg_b - 0.06).abs() < 1e-6);
    }
}
