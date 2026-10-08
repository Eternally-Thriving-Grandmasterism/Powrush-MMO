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

use bevy::input::gamepad::{GamepadRumbleRequest, Gamepads};

use crate::coop_voice::VoiceYard;
use crate::hud_anchor_registry::{HudSlab, WELL};
use crate::harvest_feel::{rumble_harvest, SoftRbePool};
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::human_presence::SoftPresence;
use crate::ledger_bind::LedgerYard;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};

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
                    bottom: WELL.bottom(),
                    left: WELL.left(),
                    width: Val::Px(420.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: TITLE_PLATE_BG.with_alpha(1.0).into(),
                border_color: TITLE_BORDER.with_alpha(1.0).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            WellSlabRoot,
            HudSlab(WELL.id),
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: TITLE_TEXT_PRIMARY,
                        ..default()
                    },
                ),
                WellSlabText,
            ));
        });
}

/// Reach the first well already uses. Wards consult this same check.
pub(crate) fn near_first_well(presence: &SoftPresence) -> bool {
    let (x, y, z) = WELL_ANCHORS[0];
    presence.position.distance(Vec3::new(x, y, z)) <= CONTEST_REACH
}

pub(crate) fn mark_well_near(
    presence: Res<SoftPresence>,
    yard: Res<WellYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    mut epi: ResMut<FirstHarvestEpiphany>,
) {
    let in_reach = near_first_well(&presence);
    epi.well_in_reach = in_reach;
    epi.well_near = in_reach
        && yard.well.wants_interact()
        && !voice.sash_open
        && !ledger.sash_open
        && !epi.peace_visitor;
}

fn pressure_hold(
    time: Res<Time>,
    mut yard: ResMut<WellYard>,
    mut pool: ResMut<SoftRbePool>,
    gamepads: Res<Gamepads>,
    mut rumble: EventWriter<GamepadRumbleRequest>,
) {
    if yard.well.hold != WellHold::Human {
        return;
    }
    if time.elapsed_seconds_f64() < yard.hold_until {
        return;
    }
    let step = yard.well.traveler_answers();
    if let Some(kick) = well_contest_kick(step) {
        pool.kick = kick;
        rumble_harvest(&mut rumble, &gamepads, false);
    }
}

pub(crate) fn handle_well(
    keyboard: Res<ButtonInput<KeyCode>>,
    presence: Res<SoftPresence>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    epi: Res<FirstHarvestEpiphany>,
    mut yard: ResMut<WellYard>,
    mut moments: ResMut<ThrivingMoments>,
    mut pool: ResMut<SoftRbePool>,
    gamepads: Res<Gamepads>,
    mut rumble: EventWriter<GamepadRumbleRequest>,
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
        if let Some(kick) = well_contest_kick(step) {
            pool.kick = kick;
            rumble_harvest(&mut rumble, &gamepads, true);
        }
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstWell,
            time.elapsed_seconds_f64(),
        );
    }
}

/// CARD WELL-CONTEST-FEEL-1 — win kicks harder than a loss. No new verb.
fn well_contest_kick(step: &str) -> Option<f32> {
    match step {
        "won" => Some(1.0),
        "lost" => Some(0.42),
        _ => None,
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
pub(crate) fn skirmish_well_word(hold: WellHold, glow: f32, losses: u32) -> &'static str {
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
    let body = if yard.well.hold == WellHold::Human {
        "The well is yours for now — Mira will answer".to_string()
    } else {
        yard.well.slab_line()
    };
    dress_skirmish_slab_line(&body, yard.well.hold, yard.well_glow, yard.well.losses)
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
                // VP-RIM-POLISH-1: the pulse lifts rim rgb (capped per channel);
                // rim alpha stays 1.0, so pulse.a is not used on the rim.
                (TITLE_BORDER.to_srgba().red + pulse.r).min(1.0),
                (TITLE_BORDER.to_srgba().green + pulse.g).min(1.0),
                (TITLE_BORDER.to_srgba().blue + pulse.b).min(1.0),
                1.0,
            )
            .into();
            *bg = Color::srgba(
                TITLE_PLATE_BG.to_srgba().red + pulse.bg_r,
                TITLE_PLATE_BG.to_srgba().green + pulse.bg_g,
                TITLE_PLATE_BG.to_srgba().blue + pulse.bg_b,
                1.0,
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

    /// CARD HUD-ANCHOR-REGISTRY-2B — Well position is the registry, Style is the coded literal.
    #[test]
    fn well_style_byte_identical_to_coded_place() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_well_slab);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<&Style, With<WellSlabRoot>>();
        let style = q.single(app.world()).clone();
        let coded = Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(132.0),
            left: Val::Px(16.0),
            width: Val::Px(420.0),
            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
            justify_content: JustifyContent::FlexStart,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };
        assert_eq!(style, coded);
        assert_eq!(style.bottom, WELL.bottom());
        assert_eq!(style.left, WELL.left());
        assert_eq!(style.margin, UiRect::default());
        assert_eq!(style.width, Val::Px(WELL.width));
    }

    #[test]
    fn well_contest_kick_is_win_then_loss() {
        assert_eq!(well_contest_kick("won"), Some(1.0));
        assert_eq!(well_contest_kick("lost"), Some(0.42));
        assert_eq!(well_contest_kick("idle"), None);
        assert!(well_contest_kick("won").unwrap() > well_contest_kick("lost").unwrap());
    }

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
            "Peace · Glowing · dirt under the well · Lives in Peace · The well is yours for now — Mira will answer"
        );
        assert!(whole_token(&glowing, "Glowing"));
        assert!(!glowing.contains("Mira stepped back"));

        yard.well_glow = 0.0;
        assert_eq!(skirmish_well_word(yard.well.hold, yard.well_glow, yard.well.losses), "Tended");
        let tended = dressed_well_slab_line(&yard);
        assert!(tended.contains("Tended"));
        assert!(tended.contains("The well is yours for now — Mira will answer"));
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

    /// CARD VP-HUD-GOLD-1 — rgb channels of one colour (srgb space).
    fn hud_gold_rgb(c: Color) -> (f32, f32, f32, f32) {
        let s = c.to_srgba();
        (s.red, s.green, s.blue, s.alpha)
    }

    fn hud_gold_assert_rgb(c: Color, want: Color, alpha: f32, what: &str) {
        let (r, g, b, a) = hud_gold_rgb(c);
        let (wr, wg, wb, _) = hud_gold_rgb(want);
        assert!((r - wr).abs() < 1e-6, "{what} red {r} vs {wr}");
        assert!((g - wg).abs() < 1e-6, "{what} green {g} vs {wg}");
        assert!((b - wb).abs() < 1e-6, "{what} blue {b} vs {wb}");
        assert!((a - alpha).abs() < 1e-6, "{what} alpha {a} vs {alpha}");
    }

    fn hud_gold_not_green(c: Color, what: &str) {
        let (r, g, b, _) = hud_gold_rgb(c);
        assert!(!(g > r && g > b), "{what} reads green: {r} {g} {b}");
    }

    /// Spawn the well slab, stand on the first well, run the sync once.
    fn hud_gold_well_app(glow: f32) -> App {
        let (x, y, z) = WELL_ANCHORS[0];
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .insert_resource(SoftPresence {
                position: Vec3::new(x, y, z),
                ..default()
            })
            .insert_resource(WellYard {
                well_glow: glow,
                ..default()
            })
            .add_systems(Startup, spawn_well_slab)
            .add_systems(Update, update_well_slab);
        app.update();
        app
    }

    fn hud_gold_well_colors(app: &mut App) -> (Color, Color, Color, Visibility) {
        let mut q = app
            .world_mut()
            .query_filtered::<(&BorderColor, &BackgroundColor, &Visibility), With<WellSlabRoot>>();
        let (border, bg, vis) = q.single(app.world());
        let (border, bg, vis) = (border.0, bg.0, *vis);
        let mut t = app
            .world_mut()
            .query_filtered::<&Text, With<WellSlabText>>();
        let text = t.single(app.world()).sections[0].style.color;
        (border, bg, text, vis)
    }

    /// CARD VP-HUD-GOLD-1 — at glow 0 the well slab rests on the title
    /// palette: gold rim (alpha 1.0, VP-RIM-POLISH-1), opaque plate fill (alpha 1.0), cream text.
    #[test]
    fn hud_gold_well_slab_rest_is_title_palette() {
        let mut app = hud_gold_well_app(0.0);
        let (border, bg, text, vis) = hud_gold_well_colors(&mut app);
        assert_eq!(vis, Visibility::Visible);
        hud_gold_assert_rgb(border, TITLE_BORDER, 1.0, "well border");
        hud_gold_assert_rgb(bg, TITLE_PLATE_BG, 1.0, "well fill");
        hud_gold_assert_rgb(text, TITLE_TEXT_PRIMARY, 1.0, "well text");
    }

    /// CARD VP-HUD-GOLD-1 — the spawn colours (before any sync) carry the
    /// same palette; none of the base colours reads green.
    #[test]
    fn hud_gold_well_slab_base_colours_not_green() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_well_slab);
        app.update();
        let (border, bg, text, _) = hud_gold_well_colors(&mut app);
        hud_gold_assert_rgb(border, TITLE_BORDER, 1.0, "well spawn border");
        hud_gold_assert_rgb(bg, TITLE_PLATE_BG, 1.0, "well spawn fill");
        hud_gold_assert_rgb(text, TITLE_TEXT_PRIMARY, 1.0, "well spawn text");
        for (c, what) in [(border, "border"), (bg, "fill"), (text, "text")] {
            hud_gold_not_green(c, what);
        }
    }

    /// CARD VP-HUD-GOLD-1 — the contest-win breath still adds on top of the
    /// gold rest base (pulse untouched); the lifted rim stays gold, not green.
    #[test]
    fn hud_gold_well_slab_pulse_adds_on_gold_base() {
        let mut app = hud_gold_well_app(1.0);
        let (border, bg, _, _) = hud_gold_well_colors(&mut app);
        let p = well_glow_pulse(1.0);
        let rest = TITLE_BORDER.to_srgba();
        let fill = TITLE_PLATE_BG.to_srgba();
        let (r, g, b, a) = hud_gold_rgb(border);
        assert!((r - (rest.red + p.r).min(1.0)).abs() < 1e-6);
        assert!((g - (rest.green + p.g).min(1.0)).abs() < 1e-6);
        assert!((b - (rest.blue + p.b).min(1.0)).abs() < 1e-6);
        assert_eq!(a, 1.0, "rim alpha stays 1.0 under the pulse");
        let (br, bgg, bb, ba) = hud_gold_rgb(bg);
        assert!((br - (fill.red + p.bg_r)).abs() < 1e-6);
        assert!((bgg - (fill.green + p.bg_g)).abs() < 1e-6);
        assert!((bb - (fill.blue + p.bg_b)).abs() < 1e-6);
        assert_eq!(ba, 1.0);
        hud_gold_not_green(border, "lifted border");
    }

    /// CARD VP-RIM-POLISH-1 — the rim pulse lives in rgb: at a full breath the
    /// well rim rgb differs from the gold rest, and rim alpha stays 1.0.
    #[test]
    fn rim_polish_well_slab_peak_rim_rgb_differs_from_rest() {
        let mut app = hud_gold_well_app(0.0);
        let (rest, _, _, _) = hud_gold_well_colors(&mut app);
        let mut app = hud_gold_well_app(1.0);
        let (peak, _, _, _) = hud_gold_well_colors(&mut app);
        let (rr, rg, rb, ra) = hud_gold_rgb(rest);
        let (pr, pg, pb, pa) = hud_gold_rgb(peak);
        assert!(
            (pr - rr).abs() + (pg - rg).abs() + (pb - rb).abs() > 1e-3,
            "peak rim rgb must differ from rest: rest=({rr},{rg},{rb}) peak=({pr},{pg},{pb})"
        );
        assert!(pr <= 1.0 && pg <= 1.0 && pb <= 1.0, "rim rgb capped at 1.0");
        assert_eq!(ra, 1.0, "rest rim alpha");
        assert_eq!(pa, 1.0, "peak rim alpha");
    }

    /// CARD VP-HUD-GOLD-1 (amended) — the well slab fill is opaque: alpha is
    /// exactly 1.0 at spawn and after a pulse sync at rest and at full breath.
    #[test]
    fn hud_gold_well_slab_fill_alpha_is_opaque() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_well_slab);
        app.update();
        let (_, bg, _, _) = hud_gold_well_colors(&mut app);
        assert_eq!(bg.to_srgba().alpha, 1.0, "spawn fill alpha");
        for glow in [0.0, 0.5, 1.0] {
            let mut app = hud_gold_well_app(glow);
            let (_, bg, _, vis) = hud_gold_well_colors(&mut app);
            assert_eq!(vis, Visibility::Visible);
            assert_eq!(bg.to_srgba().alpha, 1.0, "synced fill alpha at glow {glow}");
        }
    }
}
