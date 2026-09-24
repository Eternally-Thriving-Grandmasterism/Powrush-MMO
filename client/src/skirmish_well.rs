//! Lived-hour skirmish well — Slice 15 (v23.2.22) + P2 soft answer
//!
//! E contests the first well. Dawn after loss. Soft slab pulse on win (not a HUD).
//! Lives in Peace. Contact: info@Rathor.ai
//!
//! CARD FLESH-SKIRMISH-DAWN — the existing well slab line takes the same five
//! well words (Idle / Glowing / Tended / Resting / Stressed) and the Peace
//! dress already named here: dirt under the well. Lives in Peace. One slab.
//! Soft pulse stays `well_glow` / `well_glow_pulse`. No second HUD. No new verb.

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

/// Peace is where this well already lives. Not a PlaceId. Not a second slab.
const SKIRMISH_PLACE: &str = "Peace";

/// Place dress for the first well — dirt under the well. Lives in Peace.
const SKIRMISH_PLACE_DRESS: &str = "dirt under the well · Lives in Peace";

/// Five well words from hold, loss count, and glow already on the yard.
/// A contest-win pulse (`well_glow` > 0) is Glowing and outranks the hold.
/// Human after the breath settles is Tended. Aftercare is the dawn seam:
/// Resting. Traveler after a loss is Stressed. Traveler before any loss is Idle.
fn skirmish_well_word(hold: WellHold, glow: f32, losses: u32) -> &'static str {
    if glow.is_finite() && glow > 0.0 {
        return "Glowing";
    }
    match hold {
        WellHold::Human => "Tended",
        WellHold::Aftercare => "Resting",
        WellHold::Traveler if losses > 0 => "Stressed",
        WellHold::Traveler => "Idle",
    }
}

/// CARD FLESH-SKIRMISH-DAWN — place-color the existing slab line.
/// `{Peace} · {word} · dirt under the well · Lives in Peace · {body}`.
/// One string. The contest / dawn copy stays. No second HUD.
fn dress_skirmish_slab_line(body: &str, hold: WellHold, glow: f32, losses: u32) -> String {
    let word = skirmish_well_word(hold, glow, losses);
    format!("{SKIRMISH_PLACE} · {word} · {SKIRMISH_PLACE_DRESS} · {body}")
}

fn dressed_well_slab_line(yard: &WellYard) -> String {
    dress_skirmish_slab_line(
        &yard.well.slab_line(),
        yard.well.hold,
        yard.well_glow,
        yard.well.losses,
    )
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
    let line = dressed_well_slab_line(&yard);
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

    fn whole_token(line: &str, token: &str) -> bool {
        line.split(|c: char| c.is_whitespace() || c == '·')
            .any(|part| part == token)
    }

    /// CARD FLESH-SKIRMISH-DAWN — five well words from hold / glow / loss.
    /// Peace and dirt under the well dress the existing slab. One line.
    #[test]
    fn flesh_skirmish_dawn_place_colors_existing_slab() {
        let words = ["Idle", "Glowing", "Tended", "Resting", "Stressed"];
        let mut yard = WellYard::default();
        assert_eq!(skirmish_well_word(yard.well.hold, yard.well_glow, yard.well.losses), "Idle");
        let idle = dressed_well_slab_line(&yard);
        assert_eq!(
            idle,
            "Peace · Idle · dirt under the well · Lives in Peace · Well · Mira holds · E Contest"
        );
        assert!(idle.contains("E Contest"));
        assert!(whole_token(&idle, "Idle"));
        assert!(!idle.contains('\n'));

        assert_eq!(yard.well.contest(), "won");
        yard.well_glow = 1.0;
        assert_eq!(yard.well.hold, WellHold::Human);
        assert_eq!(skirmish_well_word(yard.well.hold, yard.well_glow, yard.well.losses), "Glowing");
        let glowing = dressed_well_slab_line(&yard);
        assert_eq!(
            glowing,
            "Peace · Glowing · dirt under the well · Lives in Peace · The well is yours — Mira stepped back"
        );
        assert!(whole_token(&glowing, "Glowing"));

        yard.well_glow = 0.0;
        assert_eq!(skirmish_well_word(yard.well.hold, yard.well_glow, yard.well.losses), "Tended");
        let tended = dressed_well_slab_line(&yard);
        assert!(tended.contains("Tended"));
        assert!(tended.contains("The well is yours"));
        assert!(whole_token(&tended, "Tended"));

        assert_eq!(yard.well.traveler_answers(), "lost");
        assert_eq!(yard.well.hold, WellHold::Aftercare);
        assert_eq!(yard.well_glow, 0.0);
        assert_eq!(skirmish_well_word(yard.well.hold, yard.well_glow, yard.well.losses), "Resting");
        let resting = dressed_well_slab_line(&yard);
        assert_eq!(
            resting,
            "Peace · Resting · dirt under the well · Lives in Peace · Dawn after loss — you still walk · E Rise"
        );
        assert!(resting.contains("Dawn after loss"));
        assert!(whole_token(&resting, "Resting"));

        assert_eq!(yard.well.dawn(), "dawn");
        assert_eq!(yard.well.hold, WellHold::Traveler);
        assert!(yard.well.losses > 0);
        assert_eq!(skirmish_well_word(yard.well.hold, yard.well_glow, yard.well.losses), "Stressed");
        let stressed = dressed_well_slab_line(&yard);
        assert_eq!(
            stressed,
            "Peace · Stressed · dirt under the well · Lives in Peace · Dawn — Mira holds the well again · E Contest"
        );
        assert!(whole_token(&stressed, "Stressed"));

        for line in [&idle, &glowing, &tended, &resting, &stressed] {
            assert!(line.starts_with("Peace · "));
            assert!(line.contains("dirt under the well"));
            assert!(line.contains("Lives in Peace"));
            assert_eq!(line.matches('\n').count(), 0);
            assert!(words.iter().any(|word| whole_token(line, word)));
            let lower = line.to_ascii_lowercase();
            assert!(!lower.contains("online"));
            assert!(!lower.contains("market"));
            assert!(!lower.contains("xp"));
            assert!(!lower.contains("kill"));
            assert!(!line.contains("week was the bill"));
            assert!(!line.contains("0.0.0.0"));
        }

        // Pulse still outranks hold, including a dawn seam if glow is up.
        assert_eq!(
            skirmish_well_word(WellHold::Aftercare, 0.2, 1),
            "Glowing"
        );
        assert_eq!(skirmish_well_word(WellHold::Human, f32::NAN, 0), "Tended");
        assert_eq!(skirmish_well_word(WellHold::Traveler, 0.0, 0), "Idle");
    }
}
