//! Lived-hour climate visible — Slice 16 (v23.2.23) + P2 week feel
//!
//! Node states paint the three wells. Tick restores a tired field.
//! Teaching claim (23.2.25) may replace the hand hint with one sentence.
//! Soft border breath when the week audit line answers (not a second HUD).
//! Does not replace harvest_feel. Contact: info@Rathor.ai
//!
//! CARD L1 SANCTUARY-WANT — first minutes on the Sanctuary yard slab speak
//! People + Want. Cite ART_BIBLE: Human | warm grey-gold | Sanctuary — cite only.
//! Want = the yard needs tending or the well goes quiet.
//! Cite PLACE_DRESS Sanctuary yard · DRIVE_LORE practices-after-House — cite, no pack.
//! H hushes this clause; Place · well mood stay. Comfort Low is slab text, 0 meshes.
//!
//! CARD L6 LANDING-WANT — after People-door land, this same first-minutes
//! clause follows PlaceId (Sanctuary / Heartwood / Depths). Garden boot
//! (no land) stays on title `garden_boot_want_line` / GARDEN_WANT.
//! Copy already on tip. 0 meshes · 0 new verbs · Online grey.
//!
//! CARD FLESH-SANCTUARY-DRESS — Sanctuary well emissive lifts so the warm-gold
//! glow stays readable on graphite-warm earth. Other Places keep the shared
//! scale. Comfort L/M/H. No Ultra. Cite PEAK_MEMORY_LAW: walked to a well ·
//! tended it · week was the bill · yard remembered. 0 meshes · Online grey.
//!
//! CARD FLESH-THRESHOLD-DRESS — pipe-air iron fog lives on the Threshold
//! climate look. The shelf is still Heartwood plus `threshold_near` (no
//! PlaceId). This slab keeps the name "Threshold" and the shared well glow
//! so the tend-seam accent stays readable. No second HUD. No new verbs.
//!
//! CARD FLESH-WELL-SPEECH — the same well words (Idle / Glowing / Tended /
//! Resting / Stressed) take the Place dress already named on these looks.
//! Sanctuary warm-gold yard · Heartwood lamp hush · Threshold tend-seam /
//! pipe-air (still Heartwood + near) · Depths teal Peace / wet-stone quiet.
//! One slab. No new state verb. Peak memory stays the walked line; this
//! caption does not grow a second slogan. 0 meshes · Online grey.
//!
//! CARD OPT-REDUCED-MOTION-PULSE — when reduced_motion is on (Comfort Low
//! sets that same flag), the slab breath stays at rest. Place dress words
//! still name Idle / Glowing / Tended / Resting / Stressed. One slab.
//! No new state verb. No second HUD.

use bevy::prelude::*;

use shared::climate_node::{ClimateNode, NodeState};
use shared::heartwood_wards::WARDS_NOTICE;

use crate::climate_script::TeachingClaim;
use crate::hud_anchor_registry::{HudSlab, CLIMATE_STATE};
use crate::skirmish_well::{tick_well_glow_breath, well_glow_pulse};
use crate::first_session_guidance::{
    first_minutes_people_want_line, first_minutes_people_want_line_for_place, want_for_place,
    FirstSessionGuidance, DEPTHS_WANT, HEARTWOOD_WANT, SANCTUARY_PEOPLE, SANCTUARY_WANT,
};
use crate::heartwood_lip::ThresholdShelfSession;
use crate::hex_travel::HexTravelState;
use shared::hex_travel::PlaceId;
use crate::heartwood_wards::WardSession;
use crate::lived_hour_bind::LivedHourBind;
use crate::local_settings::{LocalColorblindWells, LocalSettingsState};
use crate::mercy_harvest_nodes::{MercyHarvestNode, NearbyMercyNode};
use crate::depths_landing::DepthsPeaceTend;
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};

#[derive(Component)]
struct ClimateStateRoot;
#[derive(Component)]
struct ClimateStateText;

/// Soft pulse when Take · Tend · week answers on the existing slab (P2 / C1).
/// Reuses skirmish `well_glow` breath — not a second HUD.
#[derive(Resource, Default)]
struct WeekFeelGlow {
    glow: f32,
    last_updated: u64,
    last_climate: u64,
    last_line: String,
    primed: bool,
}

impl WeekFeelGlow {
    /// Prime on first sight (no boot breath). Fire on later Take · Tend · week.
    fn note_bind(&mut self, week_updated: u64, climate_updated: u64, last_line: &str) {
        if week_feel_should_breathe(
            week_updated,
            self.last_updated,
            climate_updated,
            self.last_climate,
            last_line,
            &self.last_line,
            self.primed,
        ) {
            self.glow = 1.0;
        }
        self.last_updated = week_updated;
        self.last_climate = climate_updated;
        if self.last_line != last_line {
            self.last_line = last_line.to_string();
        }
        self.primed = true;
    }
}

/// Take writes "tended node N". Tend / week bump climate and week clocks.
fn week_feel_should_breathe(
    week_updated: u64,
    last_week: u64,
    climate_updated: u64,
    last_climate: u64,
    last_line: &str,
    prev_line: &str,
    primed: bool,
) -> bool {
    if !primed {
        return false;
    }
    week_updated != last_week
        || climate_updated != last_climate
        || take_line_fired(last_line, prev_line)
}

fn take_line_fired(last_line: &str, prev_line: &str) -> bool {
    last_line != prev_line && last_line.starts_with("tended node")
}

/// CARD OPT-REDUCED-MOTION-PULSE — ceiling on the glow fed to
/// [`well_glow_pulse`] while reduced motion is on. Full Take/Tend breath
/// is glow 1.0 (border alpha lift 0.40). This cap keeps that lift at rest
/// so the words stay the read. Comfort Low sets `reduced_motion`.
const REDUCED_MOTION_WELL_PULSE_CAP: f32 = 0.0;

/// Slab rim + fill for one glow. Reduced motion holds the rest colors.
fn well_slab_pulse(glow: f32, reduced_motion: bool) -> crate::skirmish_well::WellGlowPulse {
    let glow = if glow.is_finite() {
        glow.clamp(0.0, 1.0)
    } else {
        0.0
    };
    let glow = if reduced_motion {
        glow.min(REDUCED_MOTION_WELL_PULSE_CAP)
    } else {
        glow
    };
    well_glow_pulse(glow)
}

#[derive(Resource, Default)]
struct WardsNoticeGlow {
    glow: f32,
    last_near: bool,
    last_tends: u32,
}

pub struct ClimateVisiblePlugin;

impl Plugin for ClimateVisiblePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WeekFeelGlow>()
            .init_resource::<WardsNoticeGlow>()
            .add_systems(Startup, spawn_climate_state_slab)
            .add_systems(PreUpdate, focus_lived_hour_on_nearby)
            .add_systems(
                Update,
                (
                    paint_nodes_from_hour
                        .after(crate::mercy_harvest_nodes::pulse_harvested_nodes),
                    tick_week_feel_glow,
                    tick_wards_notice_glow,
                    update_climate_state_slab,
                ),
            );
    }
}

fn focus_lived_hour_on_nearby(
    nearby: Res<NearbyMercyNode>,
    nodes: Query<&MercyHarvestNode>,
    mut bind: ResMut<LivedHourBind>,
) {
    let id = nearby
        .entity
        .and_then(|e| nodes.get(e).ok())
        .map(|n| n.climate_id);
    if bind.focus_id != id {
        bind.focus_id = id;
    }
}

fn paint_nodes_from_hour(
    bind: Res<LivedHourBind>,
    travel: Option<Res<HexTravelState>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    nodes: Query<(
        &MercyHarvestNode,
        &MeshMaterial3d<StandardMaterial>,
        Option<&Children>,
    )>,
    mut lights: Query<&mut PointLight>,
) {
    // Boot yard is Sanctuary when travel has not landed yet.
    let place = travel.map(|t| t.current).unwrap_or(PlaceId::Sanctuary);
    for (node, handle, children) in &nodes {
        let state = well_state_in_hour(&bind.hour.nodes, node.climate_id);
        let mul = state.glow_mul();
        let (emissive_mul, intensity, range) = well_glow(place, mul, node.pulse);
        if let Some(mat) = materials.get_mut(handle) {
            mat.emissive = LinearRgba::from(mat.base_color) * emissive_mul;
        }
        if let Some(children) = children {
            for child in children.iter() {
                if let Ok(mut light) = lights.get_mut(child) {
                    light.intensity = intensity;
                    light.range = range;
                }
            }
        }
    }
}

/// CARD WELL-GLOW-OWNER-1 — harvest-pulse flash on the well emissive multiplier.
pub const WELL_PULSE_EMISSIVE: f32 = 6.2;
/// CARD WELL-GLOW-OWNER-1 — harvest-pulse flash on the well light intensity.
pub const WELL_PULSE_INTENSITY: f32 = 2800.0;
/// CARD WELL-GLOW-OWNER-1 — harvest-pulse flash on the well light range.
pub const WELL_PULSE_RANGE: f32 = 3.5;

/// CARD WELL-GLOW-OWNER-1 — the one well glow: `(emissive_mul, intensity,
/// range)` for a Place, the hour mood (`NodeState::glow_mul`) and the node
/// harvest `pulse` (kept by `mercy_harvest_nodes::pulse_harvested_nodes`).
/// At pulse 0 this is the hour painter alone.
pub fn well_glow(place: PlaceId, mood: f32, pulse: f32) -> (f32, f32, f32) {
    let scale = crate::climate_plane::well_glow_scale_for_place(place);
    (
        scale * mood + pulse * WELL_PULSE_EMISSIVE,
        well_point_intensity(place, mood) + pulse * WELL_PULSE_INTENSITY,
        well_point_range(place, mood) + pulse * WELL_PULSE_RANGE,
    )
}

/// Point-light strength for a well. Sanctuary reads on graphite-warm earth;
/// Heartwood and Depths keep the shared greybox lamp.
fn well_point_intensity(place: PlaceId, mul: f32) -> f32 {
    let (base, span) = match place {
        PlaceId::Sanctuary => (120.0, 540.0),
        PlaceId::Heartwood | PlaceId::Depths => (80.0, 420.0),
    };
    base + span * mul
}

fn well_point_range(place: PlaceId, mul: f32) -> f32 {
    let (base, span) = match place {
        PlaceId::Sanctuary => (3.6, 4.4),
        PlaceId::Heartwood | PlaceId::Depths => (3.0, 4.0),
    };
    base + span * mul
}

fn tick_week_feel_glow(bind: Res<LivedHourBind>, time: Res<Time>, mut glow: ResMut<WeekFeelGlow>) {
    glow.note_bind(
        bind.week.updated_at,
        bind.climate.updated_at,
        &bind.last_line,
    );
    glow.glow = tick_well_glow_breath(glow.glow, time.delta_secs());
}

fn tick_wards_notice_glow(
    wards: Option<Res<WardSession>>,
    time: Res<Time>,
    mut glow: ResMut<WardsNoticeGlow>,
) {
    let (near, tends) = wards
        .as_deref()
        .map(|session| (session.near, session.dress.tends))
        .unwrap_or((false, 0));
    if (near && !glow.last_near) || tends != glow.last_tends {
        glow.glow = 1.0;
    }
    glow.last_near = near;
    glow.last_tends = tends;
    glow.glow = tick_well_glow_breath(glow.glow, time.delta_secs());
}

/// CARD UI-SCALE-SLABS-1 — climate slab type follows `text_scale` (11–22).
pub fn climate_slab_font_px(text_scale: f32) -> f32 {
    (14.0 * text_scale).clamp(11.0, 22.0)
}

fn spawn_climate_state_slab(mut commands: Commands) {
    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    bottom: CLIMATE_STATE.bottom(),
                    left: CLIMATE_STATE.left(),
                    width: Val::Px(420.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG.with_alpha(1.0)),
                BorderColor::all(TITLE_BORDER.with_alpha(1.0)),
                Visibility::Hidden,
            ),
            ClimateStateRoot,
            HudSlab(CLIMATE_STATE.id),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(""),
TextFont { font_size: 14.0 / 1.2, ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                ClimateStateText,
            ));
        });
}

fn update_climate_state_slab(
    nearby: Res<NearbyMercyNode>,
    bind: Res<LivedHourBind>,
    travel: Res<HexTravelState>,
    threshold: Option<Res<ThresholdShelfSession>>,
    wards: Option<Res<WardSession>>,
    depths: Option<Res<DepthsPeaceTend>>,
    claim: Option<Res<TeachingClaim>>,
    guidance: Option<Res<FirstSessionGuidance>>,
    week_glow: Res<WeekFeelGlow>,
    wards_glow: Res<WardsNoticeGlow>,
    colorblind: Res<LocalColorblindWells>,
    settings: Option<Res<LocalSettingsState>>,
    nodes: Query<&MercyHarvestNode>,
    mut root: Query<
        (&mut Visibility, &mut BorderColor, &mut BackgroundColor),
        With<ClimateStateRoot>,
    >,
    mut text_q: Query<(&mut Text, &mut TextFont), With<ClimateStateText>>,
) {
    let threshold_line = threshold_speech_if_near(threshold.as_deref());
    let wards_line = wards_notice_if_near(wards.as_deref());
    let depths_line = depths_restore_line(depths.as_deref());
    let guidance_hidden = climate_caption_guidance_hidden(
        bind.guidance_hidden,
        guidance.as_ref().map(|g| g.dismissed).unwrap_or(false),
    );
    let first_minutes = guidance
        .as_ref()
        .map(|g| g.objective.is_first_minutes())
        .unwrap_or(true);
    let place = place_clarity_label(travel.current, threshold_line.is_some());
    // H-2026-09-10-1: place + well mood stay on this one slab even when H hid the card.
    let show = climate_slab_should_show(
        nearby.in_range,
        threshold_line.is_some(),
        wards_line.is_some(),
        depths_line.is_some(),
        guidance_hidden,
    );
    let glow = week_glow.glow.max(if wards_line.is_some() {
        wards_glow.glow
    } else {
        0.0
    });
    for (mut vis, mut border, mut bg) in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        if show {
            // Same well_glow rim lift; Peace rest colors stay on this slab.
            // Reduced motion (Comfort Low sets the flag) holds that lift at rest.
            let reduced_motion = settings
                .as_ref()
                .map(|s| s.inner.reduced_motion)
                .unwrap_or(false);
            let pulse = well_slab_pulse(glow, reduced_motion);
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
    let nearest = nearby.entity.and_then(|e| nodes.get(e).ok());
    let shapes = colorblind.show_shapes;
    let well_line = nearby
        .in_range
        .then(|| {
            nearest.map(|n| {
                let state = well_state_in_hour(&bind.hour.nodes, n.climate_id);
                if guidance_hidden {
                    return well_state_sentence_in_place(n.name, state, shapes, place);
                }
                let hint = claim
                    .as_ref()
                    .and_then(|c| c.sentence_for(n.climate_id))
                    .unwrap_or(state.hand_hint());
                let mut line = format!(
                    "{} · {} · {}",
                    n.name,
                    place_color_well_caption(state, shapes, place),
                    hint
                );
                if let Some(slab) = bind.climate_slab.as_deref() {
                    line = format!("{line} · {slab}");
                }
                line
            })
        })
        .flatten();
    // H-2026-09-10-2: the nearest well's mood rides the slab even out of arm's reach,
    // so the room never reads as a bare teaching hint. Reach only buys the hand hint.
    let mood = nearest.map(|n| {
        well_state_sentence_in_place(
            n.name,
            well_state_in_hour(&bind.hour.nodes, n.climate_id),
            shapes,
            place,
        )
    });
    let fallback = climate_slab_fallback(
        mood,
        bind.climate_slab.as_deref(),
        &bind.last_line,
        guidance_hidden,
    );
    // P2: when a well and Wards posts overlap, keep both on one slab — well first.
    // P3: after a Depths Peace restore, show that line on this same slab.
    let body = compose_climate_slab_line(
        well_line,
        wards_line,
        depths_line,
        threshold_line,
        fallback,
    );
    let line = speak_first_minutes_people_want(
        place,
        travel.current,
        body,
        first_minutes,
        guidance_hidden,
    );
    let slab_px = settings
        .as_ref()
        .map(|state| climate_slab_font_px(state.inner.text_scale))
        .unwrap_or(14.0);
    for (mut text, mut font) in &mut text_q {
        if (font.font_size - slab_px / 1.2).abs() > 0.01 {
            font.font_size = slab_px / 1.2;
        }
        if text.as_str() != line {
            **text = line.clone();
        }
    }
}

fn climate_caption_guidance_hidden(bind_hidden: bool, session_dismissed: bool) -> bool {
    bind_hidden || session_dismissed
}

fn climate_slab_should_show(
    well_in_range: bool,
    threshold_near: bool,
    wards_near: bool,
    depths_restore: bool,
    guidance_hidden: bool,
) -> bool {
    let _ = (well_in_range, threshold_near, wards_near, depths_restore, guidance_hidden);
    // H-2026-09-10-1: one climate slab always carries the place name; well mood
    // and near-speech still compose on top. H hides the guidance card, not this line.
    true
}

/// Hour place name on the one climate slab (four rooms, not a second HUD).
fn place_clarity_label(current: PlaceId, threshold_near: bool) -> &'static str {
    match current {
        PlaceId::Sanctuary => "Sanctuary",
        PlaceId::Depths => "Depths",
        PlaceId::Heartwood if threshold_near => "Threshold",
        PlaceId::Heartwood => "Heartwood",
    }
}

fn place_clarity_line(place: &str, body: String) -> String {
    let body = body.trim();
    if body.is_empty() {
        place.to_string()
    } else if place_already_named(place, body) {
        body.to_string()
    } else {
        format!("{place} · {body}")
    }
}

/// CARD L1 SANCTUARY-WANT / CARD L6 LANDING-WANT — first minutes speak
/// People + Want for the Place the body is in. Garden boot (no land) is
/// title `garden_boot_want_line`, not this slab. H hushes the clause;
/// Place · mood stay. Comfort Low is mesh LOD, not a text gate.
fn speak_first_minutes_people_want(
    place: &str,
    place_id: PlaceId,
    body: String,
    first_minutes: bool,
    guidance_hidden: bool,
) -> String {
    let base = place_clarity_line(place, body);
    if !first_minutes || guidance_hidden {
        return base;
    }
    let want = first_minutes_people_want_line_for_place(place_id);
    let want_text = want_for_place(place_id);
    if base.contains(SANCTUARY_PEOPLE) && base.contains(want_text) {
        base
    } else if base.is_empty() {
        want
    } else {
        format!("{base} · {want}")
    }
}

/// Only a whole leading clause counts as the place. A well called
/// "Sanctuary ember" must not swallow the "Sanctuary" room name.
fn place_already_named(place: &str, body: &str) -> bool {
    body == place
        || body
            .strip_prefix(place)
            .is_some_and(|rest| rest.starts_with(" · "))
}

/// Slab news when nothing is within arm's reach: the nearest well's mood leads,
/// and the teaching/standing clause hushes with H like the rest of the guidance.
fn climate_slab_fallback(
    mood: Option<String>,
    slab: Option<&str>,
    last_line: &str,
    guidance_hidden: bool,
) -> String {
    let clause = if guidance_hidden { None } else { slab };
    match (mood, clause) {
        (Some(mood), Some(slab)) => format!("{mood} · {slab}"),
        (Some(mood), None) => mood,
        // No well seeded in this room — keep the older teaching + clause shape.
        (None, Some(slab)) if !last_line.is_empty() => format!("{last_line} · {slab}"),
        (None, Some(slab)) => slab.to_string(),
        (None, None) if guidance_hidden => String::new(),
        (None, None) => last_line.to_string(),
    }
}

fn well_state_in_hour(nodes: &[ClimateNode], climate_id: u32) -> NodeState {
    nodes
        .iter()
        .find(|c| c.id == climate_id)
        .map(|c| c.state)
        .unwrap_or(NodeState::Idle)
}

fn well_state_caption(state: NodeState, shapes: bool) -> String {
    if shapes {
        format!("{} · {}", state.label(), well_state_token(state))
    } else {
        state.label().to_string()
    }
}

/// B3 colorblind tokens — nameable without hue alone (AGENT pack shapes).
fn well_state_token(state: NodeState) -> &'static str {
    match state {
        NodeState::Idle => "ring",
        NodeState::Glowing => "pip",
        NodeState::Tended => "notch",
        NodeState::Resting => "rest-bar",
        NodeState::Stressed => "crack",
    }
}

fn well_state_sentence(name: &str, state: NodeState, shapes: bool) -> String {
    format!("{name} is {}", well_state_caption(state, shapes))
}

/// CARD FLESH-WELL-SPEECH — dress already named on the FLESH climate looks.
/// Phrases the existing well words. Not a new state. Not a second slab row.
/// Threshold stays the Heartwood shelf label (`place_clarity_label`), not a PlaceId.
fn well_place_dress(place_label: &str) -> &'static str {
    match place_label {
        "Heartwood" => "lamp hush",
        "Threshold" => "tend-seam · pipe-air",
        "Depths" => "teal Peace · wet-stone quiet",
        // Sanctuary, including the boot yard before a named land.
        _ => "warm-gold yard",
    }
}

fn place_color_well_caption(state: NodeState, shapes: bool, place_label: &str) -> String {
    format!(
        "{} · {}",
        well_state_caption(state, shapes),
        well_place_dress(place_label)
    )
}

fn well_state_sentence_in_place(
    name: &str,
    state: NodeState,
    shapes: bool,
    place_label: &str,
) -> String {
    format!(
        "{} · {}",
        well_state_sentence(name, state, shapes),
        well_place_dress(place_label)
    )
}

fn depths_restore_line(depths: Option<&DepthsPeaceTend>) -> Option<String> {
    depths
        .map(|session| session.last_line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
}

fn compose_climate_slab_line(
    well: Option<String>,
    wards: Option<String>,
    depths: Option<String>,
    threshold: Option<String>,
    fallback: String,
) -> String {
    match (well, wards) {
        (Some(well), Some(wards)) => format!("{well} · {wards}"),
        (Some(well), None) => well,
        (None, Some(wards)) => wards,
        (None, None) => depths.or(threshold).unwrap_or(fallback),
    }
}

fn threshold_speech_if_near(threshold: Option<&ThresholdShelfSession>) -> Option<String> {
    threshold
        .filter(|session| session.near)
        .map(|session| session.node.speech())
}

fn wards_notice_if_near(wards: Option<&WardSession>) -> Option<String> {
    wards
        .filter(|session| session.near)
        .map(|_| WARDS_NOTICE.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::climate_node::LivedHour;
    use shared::threshold_shelf::THRESHOLD_PEACE_VERBS;

    /// CARD HUD-ANCHOR-REGISTRY-2B — ClimateState position is the registry, Style is the coded literal.
    #[test]
    fn climate_state_style_byte_identical_to_coded_place() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_climate_state_slab);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<&Node, With<ClimateStateRoot>>();
        let style = q.single(app.world()).unwrap().clone();
        let coded = Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(176.0),
            left: Val::Px(16.0),
            width: Val::Px(420.0),
            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
            justify_content: JustifyContent::FlexStart,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };
        assert_eq!(style, coded);
        assert_eq!(style.bottom, CLIMATE_STATE.bottom());
        assert_eq!(style.left, CLIMATE_STATE.left());
        assert_eq!(style.margin, UiRect::default());
        assert_eq!(style.width, Val::Px(CLIMATE_STATE.width));
    }

    const ALL_PLACES: [PlaceId; 3] = [PlaceId::Sanctuary, PlaceId::Heartwood, PlaceId::Depths];
    const ALL_MOODS: [NodeState; 5] = [
        NodeState::Glowing,
        NodeState::Tended,
        NodeState::Idle,
        NodeState::Resting,
        NodeState::Stressed,
    ];

    /// CARD WELL-GLOW-OWNER-1 — at pulse 0 `well_glow` is exactly the tip
    /// hour painter (place scale × mood, well_point_intensity / _range) for
    /// all three Places and every mood.
    #[test]
    fn well_glow_owner_pulse_zero_matches_hour_painter() {
        for place in ALL_PLACES {
            let scale = crate::climate_plane::well_glow_scale_for_place(place);
            for state in ALL_MOODS {
                let mood = state.glow_mul();
                let (emissive_mul, intensity, range) = well_glow(place, mood, 0.0);
                assert_eq!(emissive_mul, scale * mood, "{place:?} {state:?} emissive");
                assert_eq!(intensity, well_point_intensity(place, mood), "{place:?} {state:?}");
                assert_eq!(range, well_point_range(place, mood), "{place:?} {state:?}");
            }
        }
    }

    /// CARD WELL-GLOW-OWNER-1 — a full pulse flashes brighter and wider,
    /// by the named consts.
    #[test]
    fn well_glow_owner_pulse_one_is_brighter_and_wider() {
        assert_eq!(
            (WELL_PULSE_EMISSIVE, WELL_PULSE_INTENSITY, WELL_PULSE_RANGE),
            (6.2, 2800.0, 3.5)
        );
        for place in ALL_PLACES {
            for state in ALL_MOODS {
                let mood = state.glow_mul();
                let calm = well_glow(place, mood, 0.0);
                let flash = well_glow(place, mood, 1.0);
                assert!(flash.0 > calm.0, "{place:?} {state:?} emissive");
                assert!(flash.1 > calm.1, "{place:?} {state:?} intensity");
                assert!(flash.2 > calm.2, "{place:?} {state:?} range");
                assert!((flash.1 - calm.1 - WELL_PULSE_INTENSITY).abs() < 1e-3);
                assert!((flash.2 - calm.2 - WELL_PULSE_RANGE).abs() < 1e-5);
            }
        }
    }

    #[test]
    fn climate_slab_font_follows_text_scale() {
        assert_eq!(climate_slab_font_px(1.0), 14.0);
        assert!((climate_slab_font_px(1.35) - 18.9).abs() < 1e-4);
        assert!((climate_slab_font_px(0.85) - 11.9).abs() < 1e-4);
        assert_eq!(climate_slab_font_px(0.1), 11.0);
        assert_eq!(climate_slab_font_px(5.0), 22.0);
    }

    #[test]
    fn demo_nodes_map_to_three_wells() {
        let hour = LivedHour::new_demo();
        assert_eq!(hour.nodes.len(), 3);
        assert_eq!(hour.nodes[0].id, 1);
        assert_eq!(hour.nodes[1].id, 2);
        assert_eq!(hour.nodes[2].id, 3);
        assert_eq!(hour.nodes[2].state, NodeState::Idle);
    }

    #[test]
    fn stressed_is_dimmer_than_glow() {
        assert!(NodeState::Glowing.glow_mul() > NodeState::Tended.glow_mul());
        assert!(NodeState::Tended.glow_mul() > NodeState::Resting.glow_mul());
        assert!(NodeState::Resting.glow_mul() > NodeState::Stressed.glow_mul());
        assert!(NodeState::Stressed.glow_mul() > 0.0);
    }

    /// CARD FLESH-SANCTUARY-DRESS — Sanctuary well glow reads above the shared scale.
    /// State order holds: Glowing > Tended > Idle > Stressed. No new Place.
    #[test]
    fn flesh_sanctuary_well_glow_reads_above_shared_places() {
        let glow = NodeState::Glowing.glow_mul();
        let tended = NodeState::Tended.glow_mul();
        let idle = NodeState::Idle.glow_mul();
        let stressed = NodeState::Stressed.glow_mul();
        let sanctuary = PlaceId::Sanctuary;
        let heartwood = PlaceId::Heartwood;
        let depths = PlaceId::Depths;
        let s_scale = crate::climate_plane::well_glow_scale_for_place(sanctuary);
        let shared = crate::climate_plane::well_glow_scale_for_place(heartwood);
        assert!(s_scale > shared);
        assert_eq!(shared, crate::climate_plane::well_glow_scale_for_place(depths));
        assert!(s_scale * glow > s_scale * tended);
        assert!(s_scale * tended > s_scale * idle);
        assert!(s_scale * idle > s_scale * stressed);
        assert!(well_point_intensity(sanctuary, glow) > well_point_intensity(heartwood, glow));
        assert!(well_point_intensity(sanctuary, glow) > well_point_intensity(sanctuary, idle));
        assert!(well_point_intensity(sanctuary, idle) > well_point_intensity(sanctuary, stressed));
        assert!(well_point_range(sanctuary, glow) > well_point_range(depths, glow));
        assert!((well_point_intensity(heartwood, glow) - (80.0 + 420.0)).abs() < f32::EPSILON);
        assert!((well_point_range(depths, glow) - (3.0 + 4.0)).abs() < f32::EPSILON);
    }

    /// CARD FLESH-THRESHOLD-DRESS — shelf is Heartwood + near, not a PlaceId.
    /// Pipe-air fog is the climate look. This slab still names Threshold.
    /// Shared well glow keeps the tend-seam disk readable. No second HUD.
    #[test]
    fn flesh_threshold_dress_shelf_names_threshold_tend_seam_stays_readable() {
        use crate::climate_plane::{PlaceMood, PLACE_WELL_GLOW_SCALE, weather_bed_for};
        use shared::local_settings::WeatherFidelity;

        assert_eq!(place_clarity_label(PlaceId::Heartwood, true), "Threshold");
        assert_eq!(place_clarity_label(PlaceId::Heartwood, false), "Heartwood");
        assert_eq!(place_clarity_label(PlaceId::Sanctuary, true), "Sanctuary");
        assert_eq!(place_clarity_label(PlaceId::Depths, true), "Depths");
        let line = place_clarity_line(
            "Threshold",
            well_state_sentence_in_place("North Well", NodeState::Glowing, true, "Threshold"),
        );
        assert_eq!(
            line,
            "Threshold · North Well is Glowing · pip · tend-seam · pipe-air"
        );
        assert!(line.contains("Glowing"));
        assert!(line.contains("tend-seam"));
        assert!(line.contains("pipe-air"));
        assert!(climate_slab_should_show(false, true, false, false, true));
        // Heartwood disk shares the greybox lamp. Fog carries the shelf, not a second glow.
        let heart = crate::climate_plane::well_glow_scale_for_place(PlaceId::Heartwood);
        assert_eq!(heart, PLACE_WELL_GLOW_SCALE);
        assert_eq!(
            heart,
            crate::climate_plane::well_glow_scale_for_place(PlaceId::Depths)
        );
        assert!(crate::climate_plane::well_glow_scale_for_place(PlaceId::Sanctuary) > heart);
        let glow = NodeState::Glowing.glow_mul();
        let idle = NodeState::Idle.glow_mul();
        assert!(glow > idle);
        assert!(heart * glow > heart * idle);
        assert!(
            (well_point_intensity(PlaceId::Heartwood, glow) - (80.0 + 420.0)).abs() < f32::EPSILON
        );
        assert!((well_point_range(PlaceId::Heartwood, glow) - (3.0 + 4.0)).abs() < f32::EPSILON);
        let pipe = weather_bed_for(Some(1), WeatherFidelity::Medium);
        let horizon = weather_bed_for(Some(4), WeatherFidelity::Medium);
        assert_eq!(pipe.mood, PlaceMood::ThresholdPipeAir);
        assert_eq!(horizon.mood, pipe.mood);
        assert_eq!(horizon.fog, pipe.fog);
        assert_eq!(horizon.sky, pipe.sky);
        let yard = weather_bed_for(Some(0), WeatherFidelity::Medium);
        let wood = weather_bed_for(Some(2), WeatherFidelity::Medium);
        let wet = weather_bed_for(Some(3), WeatherFidelity::Medium);
        assert_ne!(pipe.fog, yard.fog);
        assert_ne!(pipe.fog, wood.fog);
        assert_ne!(pipe.fog, wet.fog);
        // Tend disk stays in the clear band. Pipe-air closes before the open yard.
        assert!(pipe.fog_start >= 10.0);
        assert!(pipe.fog_end > pipe.fog_start);
        assert!(pipe.fog_end < yard.fog_end);
        assert!(pipe.fog_end > wet.fog_end);
        let mut session = ThresholdShelfSession::default();
        assert_eq!(threshold_speech_if_near(Some(&session)), None);
        session.near = true;
        let speech = threshold_speech_if_near(Some(&session)).expect("Threshold speech");
        assert!(THRESHOLD_PEACE_VERBS.iter().all(|verb| speech.contains(verb)));
        assert!(speech.contains("Tend"));
        let lower = speech.to_ascii_lowercase();
        assert!(!lower.contains("portal"));
        assert!(!lower.contains("market"));
    }

    #[test]
    fn threshold_uses_the_existing_speech_slab() {
        let mut session = ThresholdShelfSession::default();
        assert_eq!(threshold_speech_if_near(Some(&session)), None);
        session.near = true;
        let line = threshold_speech_if_near(Some(&session)).expect("Threshold speech");
        assert!(THRESHOLD_PEACE_VERBS.iter().all(|verb| line.contains(verb)));
    }

    #[test]
    fn each_well_mood_keeps_its_word_and_distinct_shape() {
        let moods = [
            (NodeState::Idle, "Idle", "ring"),
            (NodeState::Glowing, "Glowing", "pip"),
            (NodeState::Tended, "Tended", "notch"),
            (NodeState::Resting, "Resting", "rest-bar"),
            (NodeState::Stressed, "Stressed", "crack"),
        ];
        for (state, label, token) in moods {
            assert_eq!(well_state_caption(state, true), format!("{label} · {token}"));
            assert_eq!(well_state_caption(state, false), label);
        }

        let mut tokens: Vec<_> = moods
            .iter()
            .map(|(state, _, _)| well_state_token(*state))
            .collect();
        tokens.sort_unstable();
        tokens.dedup();
        assert_eq!(tokens.len(), moods.len());
    }

    #[test]
    fn hidden_guidance_keeps_each_well_state_sentence_visible() {
        for (state, label, token) in [
            (NodeState::Idle, "Idle", "ring"),
            (NodeState::Glowing, "Glowing", "pip"),
            (NodeState::Tended, "Tended", "notch"),
            (NodeState::Resting, "Resting", "rest-bar"),
            (NodeState::Stressed, "Stressed", "crack"),
        ] {
            assert!(climate_slab_should_show(true, false, false, false, true));
            assert_eq!(
                well_state_sentence("North Well", state, true),
                format!("North Well is {label} · {token}")
            );
            // B2 captions stay when colorblind_wells is off (shapes gated).
            assert_eq!(
                well_state_sentence("North Well", state, false),
                format!("North Well is {label}")
            );
        }
        assert_eq!(
            well_state_sentence("Sanctuary ember", NodeState::Glowing, true),
            "Sanctuary ember is Glowing · pip"
        );
    }

    #[test]
    fn hidden_guidance_keeps_threshold_place_readable() {
        // H-2026-09-10-1: Threshold room name/speech survives H on this slab.
        assert!(climate_slab_should_show(false, true, false, false, true));
        assert!(climate_slab_should_show(false, true, false, false, false));
    }

    #[test]
    fn place_clarity_prefixes_well_mood_when_h_hid() {
        let line = place_clarity_line(
            "Sanctuary",
            well_state_sentence_in_place("North Well", NodeState::Idle, true, "Sanctuary"),
        );
        assert_eq!(
            line,
            "Sanctuary · North Well is Idle · ring · warm-gold yard"
        );
        assert!(line.contains("Idle"));
        assert!(line.contains("warm-gold yard"));
        let heart = place_clarity_label(PlaceId::Heartwood, false);
        assert_eq!(heart, "Heartwood");
        assert_eq!(place_clarity_label(PlaceId::Heartwood, true), "Threshold");
        assert_eq!(place_clarity_label(PlaceId::Depths, false), "Depths");
    }

    #[test]
    fn a_well_named_after_the_room_keeps_the_room_name() {
        // H-2026-09-10-2: "Sanctuary ember" is not the "Sanctuary" clause.
        let line = place_clarity_line(
            "Sanctuary",
            well_state_sentence("Sanctuary ember", NodeState::Glowing, true),
        );
        assert_eq!(line, "Sanctuary · Sanctuary ember is Glowing · pip");
        // Place dress does not let the well name swallow the room.
        assert_eq!(
            place_clarity_line(
                "Sanctuary",
                well_state_sentence_in_place(
                    "Sanctuary ember",
                    NodeState::Glowing,
                    true,
                    "Sanctuary",
                ),
            ),
            "Sanctuary · Sanctuary ember is Glowing · pip · warm-gold yard"
        );
        // A real leading place clause still de-duplicates.
        assert_eq!(
            place_clarity_line("Sanctuary", "Sanctuary · the yard holds peace".into()),
            "Sanctuary · the yard holds peace"
        );
        assert_eq!(
            place_clarity_line("Sanctuary", "Sanctuary".into()),
            "Sanctuary"
        );
        assert_eq!(place_clarity_line("Depths", "  ".into()), "Depths");
    }

    #[test]
    fn out_of_reach_still_reads_place_and_mood() {
        // H-2026-09-10-2: was blank of mood until the body stood on the well.
        let hour = LivedHour::new_demo();
        let mood = well_state_sentence_in_place(
            "North Well",
            well_state_in_hour(&hour.nodes, 3),
            true,
            "Sanctuary",
        );
        let body = compose_climate_slab_line(
            None,
            None,
            None,
            None,
            climate_slab_fallback(
                Some(mood.clone()),
                Some("the yard holds peace"),
                "walk to a glow",
                false,
            ),
        );
        let line = place_clarity_line("Sanctuary", body);
        assert_eq!(
            line,
            "Sanctuary · North Well is Idle · ring · warm-gold yard · the yard holds peace"
        );
        assert!(
            !line.contains("walk to a glow"),
            "mood replaces the stand-in"
        );
    }

    #[test]
    fn hidden_guidance_leaves_place_and_mood_alone_on_the_slab() {
        let mood = well_state_sentence_in_place("North Well", NodeState::Idle, true, "Sanctuary");
        let body = compose_climate_slab_line(
            None,
            None,
            None,
            None,
            climate_slab_fallback(
                Some(mood),
                Some("the yard holds peace"),
                "walk to a glow",
                true,
            ),
        );
        assert_eq!(
            place_clarity_line("Sanctuary", body),
            "Sanctuary · North Well is Idle · ring · warm-gold yard"
        );
    }

    #[test]
    fn every_mood_reads_out_of_reach_in_every_place() {
        for place in [PlaceId::Sanctuary, PlaceId::Heartwood, PlaceId::Depths] {
            for threshold_near in [false, true] {
                let label = place_clarity_label(place, threshold_near);
                for state in [
                    NodeState::Idle,
                    NodeState::Glowing,
                    NodeState::Tended,
                    NodeState::Resting,
                    NodeState::Stressed,
                ] {
                    let fallback = climate_slab_fallback(
                        Some(well_state_sentence_in_place("North Well", state, true, label)),
                        None,
                        "",
                        true,
                    );
                    let line = place_clarity_line(label, fallback);
                    assert!(line.starts_with(label), "{line} must name the place");
                    assert!(line.contains(state.label()), "{line} must name the mood");
                    assert!(
                        line.contains(well_state_token(state)),
                        "{line} must carry the mood token"
                    );
                    assert!(
                        line.contains(well_place_dress(label)),
                        "{line} must speak the place dress"
                    );
                }
            }
        }
    }

    #[test]
    fn no_well_in_the_room_keeps_the_older_teaching_shape() {
        assert_eq!(
            climate_slab_fallback(None, Some("the yard holds peace"), "walk to a glow", false),
            "walk to a glow · the yard holds peace"
        );
        assert_eq!(
            climate_slab_fallback(None, Some("the yard holds peace"), "", false),
            "the yard holds peace"
        );
        assert_eq!(
            climate_slab_fallback(None, None, "walk to a glow", false),
            "walk to a glow"
        );
        // H hushes the teaching stand-in; the place alone still paints the slab.
        assert_eq!(
            climate_slab_fallback(None, None, "walk to a glow", true),
            ""
        );
        assert_eq!(
            place_clarity_line("Sanctuary", climate_slab_fallback(None, None, "", true)),
            "Sanctuary"
        );
    }

    #[test]
    fn near_speech_still_outranks_the_out_of_reach_mood() {
        let mood = well_state_sentence("North Well", NodeState::Idle, true);
        let fallback = climate_slab_fallback(Some(mood), None, "", false);
        assert_eq!(
            compose_climate_slab_line(
                None,
                Some(WARDS_NOTICE.to_string()),
                None,
                None,
                fallback.clone(),
            ),
            WARDS_NOTICE
        );
        assert_eq!(
            compose_climate_slab_line(
                None,
                None,
                Some("Depths Peace · restored".into()),
                None,
                fallback,
            ),
            "Depths Peace · restored"
        );
    }

    #[test]
    fn hour_lookup_falls_back_to_idle_for_an_unseeded_well() {
        let hour = LivedHour::new_demo();
        assert_eq!(well_state_in_hour(&hour.nodes, 1), NodeState::Glowing);
        assert_eq!(well_state_in_hour(&hour.nodes, 3), NodeState::Idle);
        assert_eq!(well_state_in_hour(&hour.nodes, 99), NodeState::Idle);
        assert_eq!(well_state_in_hour(&[], 1), NodeState::Idle);
    }

    #[test]
    fn dismissed_session_guidance_hides_climate_hints() {
        assert!(climate_caption_guidance_hidden(false, true));
        assert!(climate_caption_guidance_hidden(true, false));
        assert!(!climate_caption_guidance_hidden(false, false));
    }

    #[test]
    fn wards_use_the_existing_slab_even_when_guidance_is_hidden() {
        let mut wards = WardSession::default();
        wards.near = true;
        assert!(climate_slab_should_show(false, false, true, false, true));
        assert_eq!(
            wards_notice_if_near(Some(&wards)).as_deref(),
            Some(WARDS_NOTICE)
        );
    }
    #[test]
    fn well_stays_ahead_of_wards_when_both_are_near() {
        let well = well_state_sentence("Sanctuary ember", NodeState::Glowing, true);
        let line = compose_climate_slab_line(
            Some(well.clone()),
            Some(WARDS_NOTICE.to_string()),
            None,
            None,
            "fallback".into(),
        );
        assert!(line.starts_with(&well), "well sentence must remain first");
        assert!(line.contains(WARDS_NOTICE), "Wards notice stays on the same slab");
        assert_eq!(line, format!("{well} · {WARDS_NOTICE}"));
        assert_eq!(
            compose_climate_slab_line(Some(well.clone()), None, None, None, "fallback".into()),
            well
        );
        assert_eq!(
            compose_climate_slab_line(None, Some(WARDS_NOTICE.to_string()), None, None, "fallback".into()),
            WARDS_NOTICE
        );
    }

    #[test]
    fn depths_restore_line_shows_on_existing_slab() {
        let mut tend = DepthsPeaceTend::default();
        assert_eq!(depths_restore_line(Some(&tend)), None);
        // Place clarity: slab stays up with the place name even with no near speech.
        assert!(climate_slab_should_show(false, false, false, false, true));
        tend.last_line = "Depths Peace · restored".into();
        assert_eq!(
            depths_restore_line(Some(&tend)).as_deref(),
            Some("Depths Peace · restored")
        );
        assert!(climate_slab_should_show(false, false, false, true, true));
        assert_eq!(
            compose_climate_slab_line(
                None,
                None,
                Some("Depths Peace · restored".into()),
                None,
                "fallback".into(),
            ),
            "Depths Peace · restored"
        );
    }

    #[test]
    fn week_feel_reuses_well_glow_breath() {
        assert!((tick_well_glow_breath(1.0, 1.0) - 0.45).abs() < 1e-6);
        let peak = well_glow_pulse(1.0);
        assert!((peak.a - 0.40).abs() < 1e-6);
        assert!((peak.r - 0.20).abs() < 1e-6);
        // Peace rest alpha stays 0.42; lift matches the skirmish well.
        assert!(((0.42 + peak.a) - 0.82).abs() < 1e-6);
    }

    #[test]
    fn week_feel_breathes_on_take_tend_week() {
        let mut g = WeekFeelGlow::default();
        g.note_bind(1, 1, "walk to a glow");
        assert_eq!(g.glow, 0.0, "prime is not a breath");

        g.note_bind(1, 1, "tended node 1");
        assert_eq!(g.glow, 1.0, "Take");
        g.glow = tick_well_glow_breath(g.glow, 1.0);
        assert!((g.glow - 0.45).abs() < 1e-5);

        g.glow = 0.0;
        g.note_bind(1, 2, "tended node 1");
        assert_eq!(g.glow, 1.0, "Tend");

        g.glow = 0.0;
        g.note_bind(2, 2, "tended node 1");
        assert_eq!(g.glow, 1.0, "week");

        g.glow = 0.0;
        g.note_bind(2, 2, "tended node 1");
        assert_eq!(g.glow, 0.0, "idle");
    }

    #[test]
    fn take_tend_week_do_not_need_tons_to_breathe() {
        assert!(!week_feel_should_breathe(
            1, 0, 1, 0, "tended node 1", "", false
        ));
        assert!(week_feel_should_breathe(
            1, 1, 1, 1, "tended node 1", "walk to a glow", true
        ));
        assert!(week_feel_should_breathe(
            1, 1, 2, 1, "walk to a glow", "walk to a glow", true
        ));
        assert!(week_feel_should_breathe(
            2, 1, 1, 1, "this week · 0 tons · 0 restored", "walk to a glow", true
        ));
        assert!(!week_feel_should_breathe(
            1, 1, 1, 1, "walk to a glow", "walk to a glow", true
        ));
        assert!(take_line_fired("tended node 3", "walk to a glow"));
        assert!(!take_line_fired("walk to a glow", "walk to a glow"));
    }

    /// CARD L1 SANCTUARY-WANT — prove-line: People + Want on the Sanctuary slab.
    #[test]
    fn first_minutes_sanctuary_speaks_people_and_want() {
        let mood = well_state_sentence("North Well", NodeState::Idle, true);
        let line = speak_first_minutes_people_want(
            "Sanctuary",
            PlaceId::Sanctuary,
            mood,
            true,
            false,
        );
        assert!(line.contains("Sanctuary"));
        assert!(line.contains(SANCTUARY_PEOPLE));
        assert!(line.contains(SANCTUARY_WANT));
        assert!(line.contains("Human"));
        assert!(line.contains("the yard needs tending or the well goes quiet"));
        assert_eq!(
            first_minutes_people_want_line(),
            "Human · the yard needs tending or the well goes quiet"
        );
        // CARD L6 — Heartwood first minutes speak Heartwood Want, not Sanctuary well.
        let heart = speak_first_minutes_people_want(
            "Heartwood",
            PlaceId::Heartwood,
            well_state_sentence("North Well", NodeState::Idle, true),
            true,
            false,
        );
        assert!(!heart.contains(SANCTUARY_WANT));
        assert!(heart.contains(HEARTWOOD_WANT));
        assert!(!heart.contains("the well goes quiet"));
    }

    /// CARD L1 SANCTUARY-WANT — H hush drops People+Want; Place · mood stay.
    #[test]
    fn h_hush_drops_people_want_keeps_place_mood() {
        let mood = well_state_sentence_in_place("North Well", NodeState::Idle, true, "Sanctuary");
        let spoken = speak_first_minutes_people_want(
            "Sanctuary",
            PlaceId::Sanctuary,
            mood.clone(),
            true,
            false,
        );
        let hushed = speak_first_minutes_people_want(
            "Sanctuary",
            PlaceId::Sanctuary,
            mood,
            true,
            true,
        );
        assert!(spoken.contains(SANCTUARY_WANT));
        assert!(!hushed.contains(SANCTUARY_WANT));
        assert!(!hushed.contains("the yard needs tending or the well goes quiet"));
        assert_eq!(
            hushed,
            "Sanctuary · North Well is Idle · ring · warm-gold yard"
        );
        assert!(hushed.contains("Sanctuary"));
        assert!(hushed.contains("Idle"));
        assert!(hushed.contains("warm-gold yard"));
    }

    /// Comfort Low is mesh LOD — this Want is slab text, readable at every preset.
    #[test]
    fn comfort_low_keeps_people_want_readable() {
        use shared::local_settings::GraphicsPreset;
        let spoken = speak_first_minutes_people_want(
            "Sanctuary",
            PlaceId::Sanctuary,
            well_state_sentence("North Well", NodeState::Idle, true),
            true,
            false,
        );
        assert!(spoken.contains("Human"));
        assert!(spoken.contains("the yard needs tending or the well goes quiet"));
        assert!(spoken.contains("Sanctuary"));
        for _preset in GraphicsPreset::ALL {
            assert_eq!(
                speak_first_minutes_people_want(
                    "Sanctuary",
                    PlaceId::Sanctuary,
                    well_state_sentence("North Well", NodeState::Idle, true),
                    true,
                    false,
                ),
                spoken
            );
        }
        assert_eq!(GraphicsPreset::ALL.len(), 5);
        assert!(GraphicsPreset::ALL.contains(&GraphicsPreset::Low));
        assert!(GraphicsPreset::ALL.contains(&GraphicsPreset::Medium));
        assert!(GraphicsPreset::ALL.contains(&GraphicsPreset::High));
        assert_eq!(GraphicsPreset::Low.label(), "Low");
        assert_eq!(GraphicsPreset::Medium.label(), "Medium");
        assert_eq!(GraphicsPreset::High.label(), "High");
    }

    /// CARD L6 — Depths first minutes speak restore-not-Take, not Sanctuary well.
    #[test]
    fn first_minutes_depths_speaks_restore_not_take() {
        let line = speak_first_minutes_people_want(
            "Depths",
            PlaceId::Depths,
            well_state_sentence("North Well", NodeState::Idle, true),
            true,
            false,
        );
        assert!(line.contains(DEPTHS_WANT));
        assert!(line.contains("restored"));
        assert!(!line.contains("Take"));
        assert!(!line.contains(SANCTUARY_WANT));
    }

    /// Whole slab token. Hyphenated dress stays one word: `pip` is not `pipe` or `pipe-air`.
    fn slab_has_whole_token(line: &str, token: &str) -> bool {
        line.split(|c: char| c.is_whitespace() || c == '·')
            .any(|part| part == token)
    }

    /// CARD FLESH-WELL-SPEECH — existing well words stay; Place dress phrases them.
    /// Threshold is still Heartwood + near. No new state verb. No second HUD.
    /// Peak memory is not a new caption on this slab.
    #[test]
    fn flesh_well_speech_place_colors_existing_captions() {
        assert!(!slab_has_whole_token("pipe", "pip"));
        assert!(!slab_has_whole_token("pipe-air", "pip"));
        assert!(slab_has_whole_token("pip", "pip"));
        assert!(slab_has_whole_token(
            "Glowing · pip · tend-seam · pipe-air",
            "pip"
        ));
        assert!(!slab_has_whole_token(
            "Glowing · tend-seam · pipe-air",
            "pip"
        ));
        let states = [
            (NodeState::Idle, "Idle", "ring"),
            (NodeState::Glowing, "Glowing", "pip"),
            (NodeState::Tended, "Tended", "notch"),
            (NodeState::Resting, "Resting", "rest-bar"),
            (NodeState::Stressed, "Stressed", "crack"),
        ];
        let places = [
            (PlaceId::Sanctuary, false, "Sanctuary", "warm-gold yard"),
            (PlaceId::Heartwood, false, "Heartwood", "lamp hush"),
            (PlaceId::Heartwood, true, "Threshold", "tend-seam · pipe-air"),
            (PlaceId::Depths, false, "Depths", "teal Peace · wet-stone quiet"),
        ];
        for (place_id, near, label, dress) in places {
            assert_eq!(place_clarity_label(place_id, near), label);
            assert_eq!(well_place_dress(label), dress);
            for (state, word, token) in states {
                let bare = well_state_caption(state, true);
                assert_eq!(bare, format!("{word} · {token}"));
                let colored = place_color_well_caption(state, true, label);
                assert_eq!(colored, format!("{bare} · {dress}"));
                assert!(colored.starts_with(word));
                assert!(
                    slab_has_whole_token(&colored, token),
                    "{colored} must carry {token} as its own word"
                );
                let plain = place_color_well_caption(state, false, label);
                assert_eq!(plain, format!("{word} · {dress}"));
                assert!(
                    !slab_has_whole_token(&plain, token),
                    "{plain} must not count dress letters as {token}"
                );
                let sentence = well_state_sentence_in_place("North Well", state, true, label);
                assert_eq!(sentence, format!("North Well is {colored}"));
                let line = place_clarity_line(label, sentence);
                assert!(line.starts_with(label));
                assert!(line.contains(word));
                assert!(line.contains(dress));
                assert!(!line.contains("week was the bill"));
                let lower = line.to_ascii_lowercase();
                assert!(!lower.contains("online"));
                assert!(!lower.contains("portal"));
                assert!(!lower.contains("market"));
                // In-range slab still one line: name · colored words · existing hint.
                let in_range = format!(
                    "North Well · {} · {}",
                    place_color_well_caption(state, true, label),
                    state.hand_hint()
                );
                assert!(in_range.contains(word));
                assert!(in_range.contains(dress));
                assert!(!in_range.contains('\n'));
            }
        }
        assert_ne!(
            well_place_dress(place_clarity_label(PlaceId::Heartwood, true)),
            well_place_dress(place_clarity_label(PlaceId::Heartwood, false))
        );
        assert_eq!(
            place_clarity_line(
                "Sanctuary",
                well_state_sentence_in_place("North Well", NodeState::Idle, true, "Sanctuary"),
            ),
            "Sanctuary · North Well is Idle · ring · warm-gold yard"
        );
        assert_eq!(
            place_clarity_line(
                "Heartwood",
                well_state_sentence_in_place("North Well", NodeState::Tended, false, "Heartwood"),
            ),
            "Heartwood · North Well is Tended · lamp hush"
        );
        assert_eq!(
            place_clarity_line(
                "Depths",
                well_state_sentence_in_place("North Well", NodeState::Resting, true, "Depths"),
            ),
            "Depths · North Well is Resting · rest-bar · teal Peace · wet-stone quiet"
        );
    }

    /// CARD OPT-REDUCED-MOTION-PULSE — reduced motion (and Comfort Low, which
    /// sets that flag) holds the slab breath at rest. Place dress words still
    /// name Idle / Glowing / Tended / Resting / Stressed. One slab.
    #[test]
    fn reduced_motion_caps_well_pulse_place_words_still_speak() {
        use shared::local_settings::{GraphicsPreset, LocalSettings};

        let full = well_slab_pulse(1.0, false);
        let quiet = well_slab_pulse(1.0, true);
        let rest = well_glow_pulse(0.0);
        assert!((full.a - 0.40).abs() < 1e-6, "full breath still lifts the rim");
        assert!(full.r > quiet.r && full.a > quiet.a);
        assert_eq!(quiet, rest, "reduced motion holds the slab at rest");
        assert!(quiet.a <= REDUCED_MOTION_WELL_PULSE_CAP * 0.40 + 1e-6);
        assert_eq!(well_slab_pulse(1.0, true), well_slab_pulse(0.4, true));
        // A glow already under the cap is unchanged when motion is allowed.
        assert_eq!(well_slab_pulse(0.0, false), rest);

        let mut low = LocalSettings::peace_defaults();
        assert!(!low.reduced_motion);
        low.set_graphics_preset(GraphicsPreset::Low);
        assert!(low.reduced_motion, "Comfort Low is reduced motion");
        assert_eq!(well_slab_pulse(1.0, low.reduced_motion), quiet);

        let mut toggled = LocalSettings::peace_defaults();
        toggled.toggle_reduced_motion();
        assert!(toggled.reduced_motion);
        assert_eq!(well_slab_pulse(1.0, toggled.reduced_motion), quiet);

        let mut mid = LocalSettings::peace_defaults();
        mid.set_graphics_preset(GraphicsPreset::Medium);
        assert!(!mid.reduced_motion);
        assert_eq!(well_slab_pulse(1.0, mid.reduced_motion), full);

        let words = [
            (NodeState::Idle, "Idle", "ring"),
            (NodeState::Glowing, "Glowing", "pip"),
            (NodeState::Tended, "Tended", "notch"),
            (NodeState::Resting, "Resting", "rest-bar"),
            (NodeState::Stressed, "Stressed", "crack"),
        ];
        let dresses = [
            ("Sanctuary", "warm-gold yard"),
            ("Heartwood", "lamp hush"),
            ("Threshold", "tend-seam · pipe-air"),
            ("Depths", "teal Peace · wet-stone quiet"),
        ];
        for (state, word, token) in words {
            for (place, dress) in dresses {
                let colored = place_color_well_caption(state, true, place);
                assert!(colored.starts_with(word), "{colored}");
                assert!(colored.contains(dress), "{colored}");
                assert!(slab_has_whole_token(&colored, token), "{colored}");
                let line = place_clarity_line(
                    place,
                    well_state_sentence_in_place("North Well", state, true, place),
                );
                assert!(line.contains(word));
                assert!(line.contains(dress));
                assert!(!line.contains('\n'));
            }
        }
        assert_eq!(GraphicsPreset::ALL.len(), 5);
        assert_eq!(GraphicsPreset::Low.label(), "Low");
        assert_eq!(GraphicsPreset::Medium.label(), "Medium");
        assert_eq!(GraphicsPreset::High.label(), "High");
        assert!(!GraphicsPreset::Low.label().contains("Ultra"));
        assert!(!GraphicsPreset::Medium.label().contains("Ultra"));
        assert!(!GraphicsPreset::High.label().contains("Ultra"));
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

    /// Spawn the climate chip and run its sync once (Peace defaults, no disk
    /// settings). `week_glow` feeds the same breath the Take · Tend bind fires.
    fn hud_gold_climate_app(reduced_motion: bool, week_glow: f32) -> App {
        use shared::local_settings::LocalSettings;
        let mut inner = LocalSettings::peace_defaults();
        inner.reduced_motion = reduced_motion;
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .insert_resource(NearbyMercyNode::default())
            .insert_resource(LivedHourBind::default())
            .insert_resource(HexTravelState::default())
            .insert_resource(WeekFeelGlow {
                glow: week_glow,
                ..default()
            })
            .insert_resource(WardsNoticeGlow::default())
            .insert_resource(LocalColorblindWells::default())
            .insert_resource(LocalSettingsState { inner, dirty: false })
            .add_systems(Startup, spawn_climate_state_slab)
            .add_systems(Update, update_climate_state_slab);
        app.update();
        app
    }

    fn hud_gold_climate_colors(app: &mut App) -> (Color, Color, Color) {
        let mut q = app
            .world_mut()
            .query_filtered::<(&BorderColor, &BackgroundColor), With<ClimateStateRoot>>();
        let (border, bg) = q.single(app.world()).unwrap();
        let (border, bg) = (({
            assert_eq!(border.top, border.right, "border edges");
            assert_eq!(border.top, border.bottom, "border edges");
            assert_eq!(border.top, border.left, "border edges");
            border.top
        }), bg.0);
        let mut t = app
            .world_mut()
            .query_filtered::<&TextColor, With<ClimateStateText>>();
        let text = t.single(app.world()).unwrap().0;
        (border, bg, text)
    }

    /// CARD VP-HUD-GOLD-1 — at glow 0 the climate chip rests on the title
    /// palette: gold rim (alpha 1.0, VP-RIM-POLISH-1), opaque plate fill (alpha 1.0), cream text.
    #[test]
    fn hud_gold_climate_chip_rest_is_title_palette() {
        let mut app = hud_gold_climate_app(false, 0.0);
        let (border, bg, text) = hud_gold_climate_colors(&mut app);
        hud_gold_assert_rgb(border, TITLE_BORDER, 1.0, "chip border");
        hud_gold_assert_rgb(bg, TITLE_PLATE_BG, 1.0, "chip fill");
        hud_gold_assert_rgb(text, TITLE_TEXT_PRIMARY, 1.0, "chip text");
    }

    /// CARD VP-HUD-GOLD-1 — spawn colours carry the palette; no base colour
    /// on the chip reads green (green above both red and blue).
    #[test]
    fn hud_gold_climate_chip_base_colours_not_green() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_climate_state_slab);
        app.update();
        let (border, bg, text) = hud_gold_climate_colors(&mut app);
        hud_gold_assert_rgb(border, TITLE_BORDER, 1.0, "chip spawn border");
        hud_gold_assert_rgb(bg, TITLE_PLATE_BG, 1.0, "chip spawn fill");
        hud_gold_assert_rgb(text, TITLE_TEXT_PRIMARY, 1.0, "chip spawn text");
        for (c, what) in [(border, "border"), (bg, "fill"), (text, "text")] {
            hud_gold_not_green(c, what);
        }
    }

    /// CARD VP-HUD-GOLD-1 — reduced motion with a full breath pending holds
    /// the chip at the gold rest base; without it the pulse still adds on top.
    #[test]
    fn hud_gold_climate_chip_reduced_motion_holds_gold_rest() {
        let mut app = hud_gold_climate_app(true, 1.0);
        let (border, bg, _) = hud_gold_climate_colors(&mut app);
        hud_gold_assert_rgb(border, TITLE_BORDER, 1.0, "reduced border");
        hud_gold_assert_rgb(bg, TITLE_PLATE_BG, 1.0, "reduced fill");

        let mut app = hud_gold_climate_app(false, 1.0);
        let (border, _, _) = hud_gold_climate_colors(&mut app);
        let p = well_slab_pulse(1.0, false);
        let rest = TITLE_BORDER.to_srgba();
        let (r, g, b, a) = hud_gold_rgb(border);
        assert!((r - (rest.red + p.r).min(1.0)).abs() < 1e-6);
        assert!((g - (rest.green + p.g).min(1.0)).abs() < 1e-6);
        assert!((b - (rest.blue + p.b).min(1.0)).abs() < 1e-6);
        assert_eq!(a, 1.0, "rim alpha stays 1.0 under the pulse");
        hud_gold_not_green(border, "lifted chip border");
    }

    /// CARD VP-RIM-POLISH-1 — the rim pulse lives in rgb: at a full breath the
    /// chip rim rgb differs from the gold rest, and rim alpha stays 1.0.
    #[test]
    fn rim_polish_climate_chip_peak_rim_rgb_differs_from_rest() {
        let mut app = hud_gold_climate_app(false, 0.0);
        let (rest, _, _) = hud_gold_climate_colors(&mut app);
        let mut app = hud_gold_climate_app(false, 1.0);
        let (peak, _, _) = hud_gold_climate_colors(&mut app);
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

    /// CARD VP-HUD-GOLD-1 (amended) — the climate chip fill is opaque: alpha
    /// is exactly 1.0 at spawn and after a pulse sync (rest, breath, reduced motion).
    #[test]
    fn hud_gold_climate_chip_fill_alpha_is_opaque() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_climate_state_slab);
        app.update();
        let (_, bg, _) = hud_gold_climate_colors(&mut app);
        assert_eq!(bg.to_srgba().alpha, 1.0, "spawn fill alpha");
        for (reduced, glow) in [(false, 0.0), (false, 0.5), (false, 1.0), (true, 1.0)] {
            let mut app = hud_gold_climate_app(reduced, glow);
            let (_, bg, _) = hud_gold_climate_colors(&mut app);
            assert_eq!(
                bg.to_srgba().alpha,
                1.0,
                "synced fill alpha (reduced {reduced}, glow {glow})"
            );
        }
    }
}
