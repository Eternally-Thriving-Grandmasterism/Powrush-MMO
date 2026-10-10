/*!
 * First Harvest Epiphany — lived first-hour loop (v22.2.0)
 *
 * Tap E / pad West = take. Hold E ~0.42s = tend (node recovers).
 *
 * H-2026-09-16-HOLD-E-TEND: after tap-E take, hold-E lands breathe/harmony
 * (prompt still tap vs hold) before any Care-cycle Temper/Ward card.
 *
 * PATSAGi + TOLC 8 | Contact: info@Rathor.ai | Yoi ⚡
 *
 * CARD FLESH-EPIPHANY-PLACE — one existing tend-success pulse may name the
 * Place the player stands in (Sanctuary / Heartwood / Threshold-near / Depths).
 * Threshold-near is Heartwood plus existing shelf reach (`threshold_near` or
 * shelf). Wards stay Heartwood. No new PlaceId. Same tend words. No second HUD.
 * First tend still advances guidance. No Title chrome. Online grey.
 */

use bevy::input::gamepad::GamepadRumbleRequest;
use bevy::prelude::*;

use crate::abundance_journey_echo::{AbundanceJourneyEcho, JourneyKind};
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};
use crate::first_session_guidance::{credit_epiphany, credit_harvest, FirstSessionGuidance, GuidanceObjective};
use crate::hud_anchor_registry::{
    action_bar_prompts_showing, HudSlab, ACTION_BAR, ID_CARE_PROMPT, PULSE, WELCOME,
};
use crate::lived_hour_bind::LivedHourBind;
use crate::hex_travel::HexTravelState;
use crate::human_presence::SoftPresence;
use shared::hex_travel::PlaceId;
use crate::harvest_feel::{credit_soft_and_global, rumble_harvest, rumble_mercy_harvest, SoftRbePool};
use crate::hour_sacred::HourSacred;
use crate::input::PlayerInput;
use crate::mercy_harvest_nodes::{
    apply_node_harvest, apply_node_tend, CareCycleOffer, MercyHarvestNode, NearbyMercyNode,
};
use crate::lived_hour_support::RbeGlobalState;
use crate::lived_hour_support::RbeUiSync;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};
use crate::ui_above_world::LivedUiPlate;
use crate::world_answer::{fire_world_answer, AnswerKind, WorldAnswer};

const PROMPT_LINGER: f64 = 2.4;
const PULSE_SECS: f64 = 4.2;
const WELCOME_SECS: f64 = 6.0;
const REPEAT_COOLDOWN: f64 = 1.2;
/// Hold duration that distinguishes tap-E take from hold-E tend.
pub const TEND_HOLD: f64 = 0.42;

#[derive(Resource, Debug)]
pub struct FirstHarvestEpiphany {
    pub first_harvest_lived: bool,
    pub first_epiphany_lived: bool,
    pub welcome_shown: bool,
    /// Soft border breath on Hour-two welcome-back only. Rest at 0 on first Play
    /// (no XP sparkle) and after well/bench decay.
    pub welcome_glow: f32,
    pub prompt_until: f64,
    pub pulse_until: f64,
    pub pulse_line: String,
    pub last_interact_at: f64,
    /// CARD WELCOME-FIELD-1 — welcome-back plate deadline. `Some` only when
    /// `maybe_welcome_back` actually wrote a line; `None` until then.
    pub welcome_until: Option<f64>,
    pub harvests_this_session: u32,
    pub tends_this_session: u32,
    /// Frontier+ hex, no charter_id. E must not harvest.
    pub peace_visitor: bool,
    /// Voice sash open with a live card. E votes; do not harvest.
    pub beacon_voice: bool,
    /// Ledger sash open. E Bind/Escort; do not harvest.
    pub ledger_bind: bool,
    /// Embassy lamp live, not yet seated. E Request seat; do not harvest.
    pub embassy_lamp: bool,
    /// Crownstone live, not yet witnessed. E Witness; do not harvest.
    pub crownstone_near: bool,
    /// Sylvaris grove live, tend not yet offered. E Offer a tend; do not harvest.
    pub redemption_near: bool,
    /// Hybrid live, not yet attuned. E Attune; do not harvest.
    pub hybrid_near: bool,
    /// At the first well, traveler holds or dawn. E Contest / Rise; do not harvest.
    pub well_near: bool,
    /// CARD WELL-BEFORE-WARDS-1 — body is inside the skirmish well reach
    /// (`near_first_well`). Wards may claim Use only while this is false.
    pub well_in_reach: bool,
    /// At the Heartwood Threshold pipe. E Tends the node; do not harvest.
    pub threshold_near: bool,
    /// At the Heartwood Wards posts. E tends session dress; do not harvest.
    pub wards_near: bool,
    /// At the Depths Peace node. E restores that hex file; do not harvest.
    pub depths_near: bool,
    /// CARD FLESH-EPIPHANY-PLACE — stood Place for the tend pulse. None until travel exists.
    pub stood_place: Option<&'static str>,
}

impl Default for FirstHarvestEpiphany {
    fn default() -> Self {
        Self {
            first_harvest_lived: false,
            first_epiphany_lived: false,
            welcome_shown: false,
            welcome_glow: 0.0,
            prompt_until: 9999.0,
            pulse_until: 0.0,
            pulse_line: String::new(),
            last_interact_at: -999.0,
            welcome_until: None,
            harvests_this_session: 0,
            tends_this_session: 0,
            peace_visitor: false,
            beacon_voice: false,
            ledger_bind: false,
            embassy_lamp: false,
            crownstone_near: false,
            redemption_near: false,
            hybrid_near: false,
            well_near: false,
            well_in_reach: false,
            threshold_near: false,
            wards_near: false,
            depths_near: false,
            stood_place: None,
        }
    }
}

/// CARD WELL-BEFORE-WARDS-1 — Wards claim Use only when no well is in reach.
/// `well_in_reach` is the skirmish well reach (`near_first_well`), not a new radius.
pub(crate) fn wards_may_claim_use(wards_near: bool, well_in_reach: bool) -> bool {
    wards_near && !well_in_reach
}

impl FirstHarvestEpiphany {
    /// A door already owns this Use edge, so nothing may speak or credit a
    /// harvest from it — not the harvest tap, not the practice strip, not the
    /// climate ledger. The Threshold pipe and the Depths Peace node are those
    /// doors. Wards posts are too, but only when no well is in the reach
    /// `near_first_well` already uses. Take stays the choice everywhere else.
    pub fn harvest_use_is_claimed(&self) -> bool {
        self.threshold_near
            || self.depths_near
            || wards_may_claim_use(self.wards_near, self.well_in_reach)
    }

    pub fn prompt_visible(&self, now: f64, guidance: &FirstSessionGuidance) -> bool {
        if self.first_harvest_lived && now > self.prompt_until {
            return false;
        }
        matches!(
            guidance.objective,
            GuidanceObjective::ApproachGlowingNode
                | GuidanceObjective::HarvestWithInteract
                | GuidanceObjective::MoveAround
        ) || !self.first_harvest_lived
            || now < self.prompt_until
    }
}

/// Hold-E (not a second tap) after the ~0.42s threshold.
pub fn is_hold_e_tend(hold_secs: f64) -> bool {
    hold_secs >= TEND_HOLD
}

/// World-care line while in range. Hour 1 must keep tap vs hold readable.
pub fn tap_vs_hold_prompt() -> &'static str {
    "tap E take  ·  hold E tend"
}

pub fn tend_breathe_answer() -> &'static str {
    "tended — the node breathes"
}

pub fn tend_harmony_pulse_line(node_name: &str, credited: f32) -> String {
    format!("Tended {node_name} · +{credited:.1} harmony · vitality returns")
}

/// CARD FLESH-EPIPHANY-PLACE — spoken room while standing.
/// Threshold-near is Heartwood plus existing shelf reach, not a PlaceId.
/// Wards posts stay Heartwood.
fn epiphany_stood_place_name(place: PlaceId, threshold_near: bool) -> &'static str {
    match place {
        PlaceId::Sanctuary => "Sanctuary",
        PlaceId::Depths => "Depths",
        PlaceId::Heartwood if threshold_near => "Threshold-near",
        PlaceId::Heartwood => "Heartwood",
    }
}

/// Place label when travel is already in the world. None until that state exists.
/// `threshold_near` or shelf reach names Threshold-near. `_wards_near` does not.
fn epiphany_stood_place_label(
    travel: Option<&HexTravelState>,
    presence: Option<&SoftPresence>,
    threshold_near: bool,
    _wards_near: bool,
) -> Option<&'static str> {
    let travel = travel?;
    let shelf = presence.is_some_and(|body| {
        shared::threshold_shelf::threshold_use_in_reach(
            travel.current,
            body.position.x,
            body.position.z,
        )
    });
    let near = threshold_near || shelf;
    Some(epiphany_stood_place_name(travel.current, near))
}

/// CARD FLESH-EPIPHANY-PLACE — the existing tend-success pulse may prefix Place.
/// Harmony and vitality words stay. No place → the same sentence as before.
pub fn tend_pulse_at_place(node_name: &str, credited: f32, place: Option<&str>) -> String {
    let line = tend_harmony_pulse_line(node_name, credited);
    match place {
        Some(place) => format!("{place} · {line}"),
        None => line,
    }
}

pub fn world_care_prompt_line(
    in_range: bool,
    nodes_exist: bool,
    first_harvest_lived: bool,
) -> &'static str {
    if nodes_exist && in_range {
        tap_vs_hold_prompt()
    } else if nodes_exist {
        "Walk toward the glowing node"
    } else if first_harvest_lived {
        "The node remembers your care"
    } else {
        tap_vs_hold_prompt()
    }
}

/// After tap-E take, in-range still names tap vs hold (FIRST_HOUR line 4).
pub fn world_care_prompt_visible(
    in_range: bool,
    nodes_exist: bool,
    first_harvest_lived: bool,
    dismissed: bool,
    prompt_visible: bool,
) -> bool {
    if dismissed {
        return false;
    }
    if nodes_exist && in_range {
        return true;
    }
    prompt_visible || (nodes_exist && !first_harvest_lived)
}

/// After tap-E take, hold-E tend is a different verb — take cooldown must not eat it.
pub fn hold_e_tend_blocked(
    harvests_this_session: u32,
    tends_this_session: u32,
    last_interact_at: f64,
    now: f64,
) -> bool {
    if harvests_this_session > tends_this_session {
        return false;
    }
    last_interact_at > 0.0 && now - last_interact_at.abs() < REPEAT_COOLDOWN
}

#[derive(Default)]
struct InteractHold {
    holding: bool,
    started: f64,
    tended: bool,
    /// Previous frame's Use held level. Release is the falling edge.
    was_down: bool,
}

#[derive(Component)]
pub struct WorldCarePromptRoot;
#[derive(Component)]
pub struct WorldCarePromptText;
#[derive(Component)]
pub struct HarvestPulseRoot;
#[derive(Component)]
pub struct HarvestPulseText;
#[derive(Component)]
pub struct WelcomeBackRoot;
#[derive(Component)]
pub struct WelcomeBackText;

pub struct FirstHarvestEpiphanyPlugin;

impl Plugin for FirstHarvestEpiphanyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FirstHarvestEpiphany>()
            .add_systems(Startup, spawn_lived_surfaces)
            .add_systems(
                Update,
                (
                    mark_peace_visitor,
                    maybe_welcome_back,
                    mark_epiphany_place,
                    handle_interact_harvest
                        .after(crate::first_session_guidance::track_simple_progress_signals)
                        .after(crate::input::InputMapSet),
                    update_world_care_prompt,
                    update_harvest_pulse,
                    update_welcome_back,
                ).chain(),
            );
    }
}

fn spawn_lived_surfaces(mut commands: Commands) {
    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    bottom: ACTION_BAR.bottom(),
                    right: ACTION_BAR.right(),
                    width: Val::Px(460.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG),
                BorderColor::all(TITLE_BORDER),
                Visibility::Visible,
            ),
            WorldCarePromptRoot,
            HudSlab(ID_CARE_PROMPT),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new("Walk toward a glowing node"),
TextFont { font_size: FontSize::Px(16.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                WorldCarePromptText,
            ));
        });

    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: PULSE.top(),
                    left: PULSE.left(),
                    width: Val::Px(560.0),
                    margin: PULSE.margin(),
                    padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.2)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG),
                BorderColor::all(TITLE_BORDER),
                Visibility::Hidden,
            ),
            HarvestPulseRoot,
            HudSlab(PULSE.id),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(""),
TextFont { font_size: FontSize::Px(16.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                HarvestPulseText,
            ));
        });

    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: WELCOME.top(),
                    left: WELCOME.left(),
                    width: Val::Px(380.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG),
                BorderColor::all(TITLE_BORDER.with_alpha(0.40)),
                Visibility::Hidden,
            ),
            WelcomeBackRoot,
            HudSlab(WELCOME.id),
            LivedUiPlate,
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(""),
TextFont { font_size: FontSize::Px(13.5 / 1.2), ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                WelcomeBackText,
            ));
        });
}

fn maybe_welcome_back(
    echo: Res<AbundanceJourneyEcho>,
    hour: Option<Res<HourSacred>>,
    mut state: ResMut<FirstHarvestEpiphany>,
    time: Res<Time>,
    mut text_q: Query<&mut Text, With<WelcomeBackText>>,
) {
    if state.welcome_shown {
        return;
    }
    let held = hour.as_ref().map(|h| h.complete).unwrap_or(false);
    let hour_three = hour.as_ref().map(|h| h.hour_three_complete).unwrap_or(false);
    if !held && !echo.loaded {
        return;
    }
    let last = echo.lines.last().map(|l| l.text.as_str());
    let Some(line) = apply_resume_welcome(
        &mut state,
        hour_three,
        held,
        echo.last_practice_sealed,
        last,
    ) else {
        if echo.loaded {
            state.welcome_shown = true;
        }
        return;
    };
    let now = time.elapsed_secs_f64();
    for mut text in &mut text_q {
        **text = line.clone();
    }
    state.welcome_until = Some(now + WELCOME_SECS);
}

/// Quit/rerun welcome. First Play stays quiet (glow 0). Held pack names the yard.
pub fn apply_resume_welcome(
    state: &mut FirstHarvestEpiphany,
    hour_three_held: bool,
    hour_two_held: bool,
    sealed: bool,
    last_echo: Option<&str>,
) -> Option<String> {
    let line = crate::hour_two_resume::welcome_line(
        hour_three_held,
        hour_two_held,
        sealed,
        last_echo,
    )?;
    state.welcome_shown = true;
    state.welcome_glow =
        crate::hour_two_resume::welcome_glow_from_line(Some(line.as_str()));
    Some(line)
}

fn welcome_visible(state: &FirstHarvestEpiphany, now: f64) -> bool {
    // CARD WELCOME-FIELD-1 — the deadline lives in `welcome_until`, set only
    // when maybe_welcome_back wrote a line. An E interact after the write
    // (`last_interact_at` later than the write time) hides it.
    let Some(until) = state.welcome_until else {
        return false;
    };
    // Not just `> 0.0`: with no negative write any more, a positive cooldown
    // stamp from an E press before the write survives and must not hide it.
    let interacted_since = state.last_interact_at > until - WELCOME_SECS;
    state.welcome_shown && now < until && !interacted_since
}

fn mark_peace_visitor(hour: Res<HourSacred>, mut state: ResMut<FirstHarvestEpiphany>) {
    state.peace_visitor = hour.session.peace_visitor_on_frontier();
}

/// CARD FLESH-EPIPHANY-PLACE — refresh the stood Place before the tend pulse.
fn mark_epiphany_place(
    travel: Option<Res<HexTravelState>>,
    presence: Option<Res<SoftPresence>>,
    mut state: ResMut<FirstHarvestEpiphany>,
) {
    state.stood_place = epiphany_stood_place_label(
        travel.as_deref(),
        presence.as_deref(),
        state.threshold_near,
        state.wards_near,
    );
}

fn handle_interact_harvest(
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    player_input: Res<PlayerInput>,
    mut hold: Local<InteractHold>,
    mut state: ResMut<FirstHarvestEpiphany>,
    mut guidance: ResMut<FirstSessionGuidance>,
    mut moments: ResMut<ThrivingMoments>,
    mut echo: ResMut<AbundanceJourneyEcho>,
    mut nearby: ResMut<NearbyMercyNode>,
    mut nodes: Query<&mut MercyHarvestNode>,
    mut pool: ResMut<SoftRbePool>,
    mut global: Option<ResMut<RbeGlobalState>>,
    mut rumble: MessageWriter<GamepadRumbleRequest>,
    gamepads: Query<Entity, With<Gamepad>>,
    mut rbe_ui: Option<ResMut<RbeUiSync>>,
    mut answer: ResMut<WorldAnswer>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs_f64();
    // Hold and release read PlayerInput. `e_down` is the held level.
    // `e_up` is the falling edge (held last frame, not this frame).
    // Record it before any return so a press-frame take still arms the edge.
    let e_down = player_input.interact_held;
    let e_up = hold.was_down && !e_down;
    hold.was_down = e_down;
    let pad_tap = player_input.interact;

    if state.peace_visitor {
        let key_edge = keyboard
            .as_ref()
            .is_some_and(|keys| keys.just_pressed(soft_play_bindings::INTERACT));
        if pad_tap || key_edge || e_up {
            state.pulse_until = now + 2.4;
            state.pulse_line = "Not your charter / Peace visitor".into();
        }
        return;
    }

    if state.beacon_voice {
        return;
    }

    if state.ledger_bind {
        return;
    }

    if state.embassy_lamp {
        return;
    }

    if state.crownstone_near {
        return;
    }

    if state.redemption_near {
        return;
    }

    if state.hybrid_near {
        return;
    }

    if state.well_near {
        return;
    }

    // U10: at the Threshold pipe the Use is a Tend. The harvest tap must not
    // swallow it, so no Take, no thriving-moment harvest line, no stock drop.
    if state.harvest_use_is_claimed() {
        return;
    }

    if pad_tap {
        resolve_take(
            now, &mut state, &mut guidance, &mut moments, &mut echo,
            &mut nearby, &mut nodes, &mut pool, global.as_deref_mut(),
            &mut rumble, gamepads.iter(), rbe_ui.as_deref_mut(), &mut answer,
        );
        return;
    }

    if e_down && !hold.holding {
        hold.holding = true;
        hold.started = now;
        hold.tended = false;
        if nearby.nodes_exist && !nearby.in_range {
            state.pulse_until = now + 2.0;
            let name = nearby.name.unwrap_or("the glowing node");
            state.pulse_line = format!("Step closer to {name} · tap E take · hold E tend");
        }
    }

    if hold.holding && e_down && !hold.tended && nearby.in_range && now - hold.started >= TEND_HOLD {
        let place = state.stood_place;
        if resolve_tend(
            now, &mut state, &mut guidance, &mut nearby, &mut nodes, &mut pool,
            &mut rumble, gamepads.iter(), &mut answer, place,
        ) {
            hold.tended = true;
        }
    }

    if e_up {
        if hold.holding && !hold.tended {
            resolve_take(
                now, &mut state, &mut guidance, &mut moments, &mut echo,
                &mut nearby, &mut nodes, &mut pool, global.as_deref_mut(),
                &mut rumble, gamepads.iter(), rbe_ui.as_deref_mut(), &mut answer,
            );
        }
        hold.holding = false;
        hold.tended = false;
    }
}

fn resolve_take(
    now: f64,
    state: &mut FirstHarvestEpiphany,
    guidance: &mut FirstSessionGuidance,
    moments: &mut ThrivingMoments,
    echo: &mut AbundanceJourneyEcho,
    nearby: &mut NearbyMercyNode,
    nodes: &mut Query<&mut MercyHarvestNode>,
    pool: &mut SoftRbePool,
    global: Option<&mut RbeGlobalState>,
    rumble: &mut MessageWriter<GamepadRumbleRequest>,
    gamepads: impl IntoIterator<Item = Entity>,
    rbe_ui: Option<&mut RbeUiSync>,
    answer: &mut WorldAnswer,
) {
    if now - state.last_interact_at.abs() < REPEAT_COOLDOWN && state.last_interact_at > 0.0 {
        return;
    }
    if nearby.nodes_exist && !nearby.in_range {
        state.pulse_until = now + 2.2;
        let name = nearby.name.unwrap_or("the glowing node");
        state.pulse_line = format!("Step closer to {name} · then E");
        return;
    }

    state.last_interact_at = now;
    state.harvests_this_session = state.harvests_this_session.saturating_add(1);

    let mut node_vitality = 1.0;
    if let Some(entity) = nearby.entity {
        if let Ok(mut node) = nodes.get_mut(entity) {
            node_vitality = node.vitality;
            apply_node_harvest(&mut node);
            nearby.last_harvested = Some(entity);
        }
    }

    let credited = credit_soft_and_global(pool, global, node_vitality);
    let first = !state.first_harvest_lived;
    rumble_harvest(rumble, gamepads, first);
    pool.punch(first);
    credit_harvest(guidance);
    if first {
        credit_epiphany(guidance);
        state.first_epiphany_lived = true;
    }
    fire_thriving(moments, ThrivingKind::FirstMercyHarvest, now);
    fire_world_answer(answer, AnswerKind::Take, now, "taken with mercy");

    state.first_harvest_lived = true;
    state.prompt_until = now + PROMPT_LINGER;
    state.pulse_until = now + PULSE_SECS;
    let node_name = nearby.name.unwrap_or("the node");
    state.pulse_line = if first {
        format!("{node_name} still glows · +{credited:.1} vitality · hold E to tend")
    } else {
        format!("Take at {node_name} · +{credited:.1} · {}", pool.line())
    };

    if first {
        echo.push(
            JourneyKind::Note,
            format!("First mercy harvest at {node_name} — left glowing (+{credited:.1})"),
        );
    }
    if let Some(ui) = rbe_ui {
        ui.last_harvest_feedback = Some(format!("Sustainable +{credited:.1} at {node_name}"));
    }
}

fn resolve_tend(
    now: f64,
    state: &mut FirstHarvestEpiphany,
    guidance: &mut FirstSessionGuidance,
    nearby: &mut NearbyMercyNode,
    nodes: &mut Query<&mut MercyHarvestNode>,
    pool: &mut SoftRbePool,
    rumble: &mut MessageWriter<GamepadRumbleRequest>,
    gamepads: impl IntoIterator<Item = Entity>,
    answer: &mut WorldAnswer,
    place: Option<&str>,
) -> bool {
    if hold_e_tend_blocked(
        state.harvests_this_session,
        state.tends_this_session,
        state.last_interact_at,
        now,
    ) {
        return false;
    }
    if !nearby.in_range {
        return false;
    }
    state.last_interact_at = now;
    state.tends_this_session = state.tends_this_session.saturating_add(1);

    let mut node_vitality = 1.0;
    if let Some(entity) = nearby.entity {
        if let Ok(mut node) = nodes.get_mut(entity) {
            node_vitality = node.vitality;
            apply_node_tend(&mut node);
        }
    }
    let credited = pool.credit_tend(node_vitality);
    credit_harvest(guidance);
    rumble_mercy_harvest(rumble, gamepads);
    fire_world_answer(answer, AnswerKind::Tend, now, tend_breathe_answer());

    state.pulse_until = now + PULSE_SECS;
    let node_name = nearby.name.unwrap_or("the node");
    state.pulse_line = tend_pulse_at_place(node_name, credited, place);
    info!(target: "powrush::epiphany", node = node_name, "hold-E tend");
    true
}

pub(crate) fn update_world_care_prompt(
    state: Res<FirstHarvestEpiphany>,
    guidance: Res<FirstSessionGuidance>,
    nearby: Res<NearbyMercyNode>,
    time: Res<Time>,
    care: Option<Res<CareCycleOffer>>,
    bind: Option<Res<LivedHourBind>>,
    mut root: Query<&mut Visibility, With<WorldCarePromptRoot>>,
    mut text_q: Query<&mut Text, With<WorldCarePromptText>>,
) {
    let now = time.elapsed_secs_f64();
    let guidance_hidden = bind.as_ref().is_some_and(|bind| bind.guidance_hidden);
    let (_care_strip, show) = action_bar_prompts_showing(
        care.as_deref(),
        Some(state.as_ref()),
        Some(nearby.as_ref()),
        guidance.as_ref(),
        now,
        guidance_hidden,
    );
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    let line = world_care_prompt_line(
        nearby.in_range,
        nearby.nodes_exist,
        state.first_harvest_lived,
    );
    for mut text in &mut text_q {
        if text.as_str() != line {
            **text = line.to_string();
        }
    }
}

fn update_harvest_pulse(
    state: Res<FirstHarvestEpiphany>,
    time: Res<Time>,
    mut root: Query<&mut Visibility, With<HarvestPulseRoot>>,
    mut text_q: Query<&mut Text, With<HarvestPulseText>>,
) {
    let now = time.elapsed_secs_f64();
    let show = now < state.pulse_until && !state.pulse_line.is_empty();
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if show {
        for mut text in &mut text_q {
            if text.as_str() != state.pulse_line {
            **text = state.pulse_line.clone();
        }
        }
    }
}

fn update_welcome_back(
    mut state: ResMut<FirstHarvestEpiphany>,
    time: Res<Time>,
    mut root: Query<
        (&mut Visibility, &mut BorderColor, &mut BackgroundColor),
        With<WelcomeBackRoot>,
    >,
) {
    // Same decay as well_glow / bench_glow — one breath, then rest.
    if state.welcome_glow > 0.0 {
        state.welcome_glow = (state.welcome_glow - time.delta_secs() * 0.55).max(0.0);
    }
    let show = welcome_visible(&state, time.elapsed_secs_f64());
    let glow = state.welcome_glow;
    for (mut vis, mut border, mut bg) in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        if show {
            // Rest colors stay the existing welcome card; lift matches well_glow.
            let a = 0.40 + glow * 0.40;
            // state: welcome-back pulse — palette rest base + the existing glow lift.
            let rim = TITLE_BORDER.to_srgba();
            let plate = TITLE_PLATE_BG.to_srgba();
            *border = Color::srgba(
                (rim.red + glow * 0.20).min(1.0),
                (rim.green + glow * 0.16).min(1.0),
                (rim.blue + glow * 0.04).min(1.0),
                a,
            )
            .into();
            *bg = Color::srgba(
                plate.red + glow * 0.08,
                plate.green + glow * 0.10,
                plate.blue + glow * 0.06,
                1.0,
            )
            .into();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hour_sacred::HourSacred;
    use shared::space_law::HexFlag;

    #[test]
    fn prompt_shows_before_first_harvest() {
        let state = FirstHarvestEpiphany::default();
        let g = FirstSessionGuidance::default();
        assert!(state.prompt_visible(0.5, &g));
    }

    #[test]
    fn prompt_hides_after_linger() {
        let mut state = FirstHarvestEpiphany::default();
        state.first_harvest_lived = true;
        state.prompt_until = 2.0;
        let mut g = FirstSessionGuidance::default();
        g.objective = GuidanceObjective::FreeExploration;
        assert!(!state.prompt_visible(5.0, &g));
    }

    #[test]
    fn peace_hour_is_not_a_visitor() {
        let h = HourSacred::default();
        assert!(!h.session.peace_visitor_on_frontier());
        let s = FirstHarvestEpiphany::default();
        assert!(!s.peace_visitor);
    }

    #[test]
    fn frontier_without_charter_is_a_visitor() {
        let mut h = HourSacred::default();
        h.session.hex = HexFlag::Frontier;
        assert!(h.session.peace_visitor_on_frontier());
        h.session.charter_id = Some("iec-1".into());
        assert!(!h.session.peace_visitor_on_frontier());
    }

    #[test]
    fn welcome_held_yard() {
        let line = crate::hour_two_resume::welcome_line(false, true, false, None).unwrap();
        assert!(line.contains("yard remembers"));
        assert!(line.contains("climate"));
        assert!(!line.to_lowercase().contains("lethal"));
        assert!(crate::hour_two_resume::hour_two_welcome_reward(Some(
            line.as_str()
        )));
        assert_eq!(
            crate::hour_two_resume::welcome_glow_from_line(Some(line.as_str())),
            1.0
        );
    }

    #[test]
    fn first_boot_welcome_glow_stays_quiet() {
        // First Play boot: default state + no held pack → glow stays 0 (no XP sparkle).
        let s = FirstHarvestEpiphany::default();
        assert!(!s.welcome_shown);
        assert_eq!(s.welcome_glow, 0.0);
        assert!(!crate::hour_two_resume::hour_two_welcome_reward(None));
        assert_eq!(crate::hour_two_resume::welcome_glow_from_line(None), 0.0);
        assert_eq!(
            crate::hour_two_resume::welcome_line(false, false, false, None),
            None
        );
    }

    #[test]
    fn welcome_glow_only_lights_for_hour_two_held() {
        // Mirrors maybe_welcome_back's glow assignment without Bevy.
        let mut lit = FirstHarvestEpiphany::default();
        let hour_two = crate::hour_two_resume::welcome_line(false, true, false, None).unwrap();
        lit.welcome_glow =
            crate::hour_two_resume::welcome_glow_from_line(Some(hour_two.as_str()));
        assert_eq!(lit.welcome_glow, 1.0);

        let mut quiet = FirstHarvestEpiphany::default();
        for (h3, h2, sealed, echo) in [
            (true, true, false, None),
            (false, false, true, None),
            (false, false, false, Some("tend")),
        ] {
            let line = crate::hour_two_resume::welcome_line(h3, h2, sealed, echo).unwrap();
            quiet.welcome_glow =
                crate::hour_two_resume::welcome_glow_from_line(Some(line.as_str()));
            assert_eq!(
                quiet.welcome_glow, 0.0,
                "non-hour-two welcome must not light glow: {line}"
            );
        }
    }

    /// CARD WELCOME-FIELD-1 — Default keeps the -999.0 last_interact_at and
    /// starts with no welcome deadline.
    #[test]
    fn hud_top_welcome_default_has_no_deadline() {
        let s = FirstHarvestEpiphany::default();
        assert_eq!(s.last_interact_at, -999.0);
        assert_eq!(s.welcome_until, None);
        assert!(!welcome_visible(&s, 0.0));
    }

    /// CARD VP-HUD-TOP-1 / WELCOME-FIELD-1 — welcome_shown with no line written
    /// (first Play, echo loaded) keeps the plate hidden: no empty top-left bar.
    #[test]
    fn hud_top_welcome_shown_without_line_is_not_visible() {
        let mut s = FirstHarvestEpiphany::default();
        s.welcome_shown = true;
        assert_eq!(s.welcome_until, None);
        for now in [0.0, 1.0, 5.9, 30.0, 500.0, 993.0, 995.0, 998.9, 1200.0] {
            assert!(!welcome_visible(&s, now), "no line, now={now}");
        }
    }

    /// Mirrors maybe_welcome_back's write at `t`.
    fn hud_top_write_welcome_at(t: f64) -> FirstHarvestEpiphany {
        let mut s = FirstHarvestEpiphany::default();
        let line = apply_resume_welcome(&mut s, false, true, false, None);
        assert!(line.is_some());
        s.welcome_until = Some(t + WELCOME_SECS);
        s
    }

    /// CARD VP-HUD-TOP-1 / WELCOME-FIELD-1 — once a line is written at `t`,
    /// the plate shows until t + WELCOME_SECS, then hides; an E interact after
    /// the write hides it. last_interact_at keeps its -999.0 default.
    #[test]
    fn hud_top_welcome_with_line_visible_until_welcome_secs() {
        for t in [0.0, 2.5, 40.0, 993.25, 2000.0] {
            let mut s = hud_top_write_welcome_at(t);
            assert_eq!(s.last_interact_at, -999.0, "write leaves last_interact_at");
            assert!(welcome_visible(&s, t), "at write t={t}");
            assert!(welcome_visible(&s, t + WELCOME_SECS - 0.01), "just before end t={t}");
            assert!(!welcome_visible(&s, t + WELCOME_SECS), "at end t={t}");
            assert!(!welcome_visible(&s, t + WELCOME_SECS + 10.0), "after end t={t}");
            s.last_interact_at = t + 1.0;
            assert!(!welcome_visible(&s, t + 1.5), "after E t={t}");
        }
    }

    /// CARD WELCOME-FIELD-1 — the t = 993.0 write that collided with the old
    /// -999.0 sentinel on #644 now shows for the full WELCOME_SECS.
    #[test]
    fn welcome_field_line_written_at_993_stays_visible() {
        let t = 993.0;
        let s = hud_top_write_welcome_at(t);
        assert_eq!(s.welcome_until, Some(999.0));
        for dt in [0.0, 0.5, 3.0, 5.99] {
            assert!(welcome_visible(&s, t + dt), "t=993 + {dt}");
        }
        assert!(!welcome_visible(&s, t + WELCOME_SECS), "ends at 999");
    }

    /// CARD WELCOME-FIELD-1 (Core ruling) — E pressed before the welcome line
    /// is written leaves a positive last_interact_at that now survives the
    /// write; the welcome must still show. An E after the write still hides it.
    #[test]
    fn welcome_field_e_before_write_still_shows_welcome() {
        let mut s = FirstHarvestEpiphany::default();
        s.last_interact_at = 38.5; // E at t = 38.5, before the write
        assert!(apply_resume_welcome(&mut s, false, true, false, None).is_some());
        s.welcome_until = Some(40.0 + WELCOME_SECS); // written at t = 40
        assert_eq!(s.last_interact_at, 38.5, "write keeps the earlier stamp");
        assert!(welcome_visible(&s, 40.0));
        assert!(welcome_visible(&s, 45.9));
        assert!(!welcome_visible(&s, 46.0));
        s.last_interact_at = 41.0; // E after the write
        assert!(!welcome_visible(&s, 41.5));
    }

    /// CARD WELCOME-FIELD-1 — the cooldown readers (hold_e_tend_blocked, the
    /// L604 gate) still only act on a positive last_interact_at.
    #[test]
    fn welcome_field_cooldown_reads_positive_only() {
        assert!(!hold_e_tend_blocked(0, 0, -999.0, 1.0));
        assert!(hold_e_tend_blocked(0, 0, 10.0, 10.5));
        assert!(!hold_e_tend_blocked(0, 0, 10.0, 11.5));
    }

    /// Playtest H2-RESUME: quit/rerun names the yard, skips WASD, First Play glow 0.
    #[test]
    fn h2_resume_welcome_skips_wasd_and_pack_survives() {
        let mut first_play = FirstHarvestEpiphany::default();
        assert!(apply_resume_welcome(&mut first_play, false, false, false, None).is_none());
        assert!(!first_play.welcome_shown);
        assert_eq!(first_play.welcome_glow, 0.0);

        let mut held = FirstHarvestEpiphany::default();
        let line = apply_resume_welcome(&mut held, false, true, false, None).unwrap();
        assert!(line.contains("Welcome back"));
        assert!(line.contains("Hour two held"));
        assert!(line.contains("the yard remembers"));
        assert!(held.welcome_shown);
        assert_eq!(held.welcome_glow, 1.0);

        let mut g = FirstSessionGuidance::default();
        assert_eq!(g.objective, GuidanceObjective::MoveAround);
        g.hour_two_held = true;
        g.resume_from_pack();
        assert_ne!(g.objective, GuidanceObjective::MoveAround);
        assert_eq!(g.objective, GuidanceObjective::PlantFabricator);

        let pack = shared::stranger_loop_proof::hour_two_held_fixture();
        let json = serde_json::to_string(&pack).expect("pack");
        let loaded = shared::hour_two::HourTwoPack::from_json(&json);
        assert!(loaded.complete);
        assert!(loaded.ledger_settled());
        assert!(loaded.session.charter_skin_live());
        assert_eq!(crate::hour_sacred::HOUR_TWO_PATH, "data/powrush_hour_two.json");
    }

    #[test]
    fn wards_claim_use_as_dress_not_take() {
        let mut s = FirstHarvestEpiphany::default();
        s.wards_near = true;
        assert!(!s.well_in_reach);
        assert!(s.harvest_use_is_claimed());
        assert!(!s.threshold_near);
    }

    /// CARD WELL-BEFORE-WARDS-1 — Ward post and skirmish well both in reach.
    /// E tends the well. Wards do not claim Use. The climate slab line is untouched.
    #[test]
    fn both_near_e_tends_the_well() {
        use bevy::input::gamepad::GamepadRumbleRequest;
        use bevy::prelude::*;
        use shared::ledger_bind::LedgerBoard;
        use shared::skirmish_well::{WellHold, WELL_ANCHORS};

        use crate::coop_voice::VoiceYard;
        use crate::harvest_feel::SoftRbePool;
        use crate::heartwood_wards::{claim_ward_use, WardSession};
        use crate::human_presence::SoftPresence;
        use crate::ledger_bind::LedgerYard;
        use crate::skirmish_well::{
            handle_well, mark_well_near, near_first_well, skirmish_well_word, WellYard,
        };
        use crate::soft_play_bindings;
        use crate::thriving_moments::ThrivingMoments;

        let (x, y, z) = WELL_ANCHORS[0];
        let presence = SoftPresence {
            position: Vec3::new(x, y, z),
            ..Default::default()
        };
        assert!(near_first_well(&presence));

        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .insert_resource(presence)
            .init_resource::<WellYard>()
            .init_resource::<VoiceYard>()
            .insert_resource(LedgerYard {
                board: LedgerBoard::default(),
                sash_open: false,
            })
            .init_resource::<FirstHarvestEpiphany>()
            .init_resource::<ThrivingMoments>()
            .init_resource::<SoftRbePool>()
            .init_resource::<ButtonInput<KeyCode>>()
            .add_message::<GamepadRumbleRequest>()
            .add_systems(PreUpdate, mark_well_near)
            .add_systems(Update, handle_well);

        app.world_mut()
            .resource_mut::<FirstHarvestEpiphany>()
            .wards_near = true;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(soft_play_bindings::INTERACT);
        app.update();

        let well_in_reach = {
            let epi = app.world().resource::<FirstHarvestEpiphany>();
            assert!(epi.wards_near, "ward post still in reach");
            assert!(epi.well_in_reach, "skirmish well reach");
            assert!(
                !epi.harvest_use_is_claimed(),
                "wards must not claim Use while a well is in reach"
            );
            assert!(!wards_may_claim_use(epi.wards_near, epi.well_in_reach));
            epi.well_in_reach
        };

        let yard = app.world().resource::<WellYard>();
        assert_eq!(yard.well.wins, 1);
        assert_eq!(yard.well.hold, WellHold::Human);
        assert_eq!(
            skirmish_well_word(yard.well.hold, 0.0, yard.well.losses),
            "Tended",
            "rested well word after E"
        );
        assert!(yard.well.last_line.contains("The well is yours"));

        let mut wards = WardSession::default();
        assert!(!claim_ward_use(true, well_in_reach, true, &mut wards));
        assert_eq!(wards.dress.tends, 0);
    }

    #[test]
    fn depths_claims_use_as_restore_not_take() {
        let mut s = FirstHarvestEpiphany::default();
        s.depths_near = true;
        assert!(s.harvest_use_is_claimed());
        assert!(!s.threshold_near);
        assert!(!s.wards_near);
    }

    #[test]
    fn tap_e_and_hold_e_stay_distinct() {
        assert!(!is_hold_e_tend(0.20));
        assert!(!is_hold_e_tend(0.39));
        assert!(is_hold_e_tend(TEND_HOLD));
        assert!(is_hold_e_tend(0.50));
        assert_eq!(tap_vs_hold_prompt(), "tap E take  ·  hold E tend");
        assert_ne!(tap_vs_hold_prompt(), "E tend the glow");
    }

    /// PLAYTEST-1 line 4 / CARD H-2026-09-16-HOLD-E-TEND:
    /// after tap-E take, hold-E tend yields breathe/harmony before Care cycle.
    #[test]
    fn after_take_hold_e_tend_lands_breathe_harmony_before_care_cycle() {
        let harvests = 1u32;
        let tends = 0u32;
        let take_at = 10.0;
        let hold_at = take_at + TEND_HOLD;
        assert!(
            !hold_e_tend_blocked(harvests, tends, take_at, hold_at),
            "take cooldown must not eat hold-E tend"
        );
        assert!(is_hold_e_tend(TEND_HOLD));

        let pulse = tend_harmony_pulse_line("Sanctuary ember", 0.4);
        assert!(pulse.contains("harmony"));
        assert!(pulse.contains("Tended"));
        assert!(!pulse.contains("Care cycle"));
        assert!(!pulse.contains("Temper"));
        assert!(!pulse.contains("Distill Ward"));
        assert_eq!(tend_breathe_answer(), "tended — the node breathes");
        assert_eq!(
            world_care_prompt_line(true, true, true),
            tap_vs_hold_prompt()
        );
        assert!(world_care_prompt_visible(true, true, true, false, false));

        let card = crate::mercy_harvest_nodes::care_cycle_card_line(false);
        assert!(card.contains("Care cycle"));
        assert!(card.contains("Temper"));
        let breathe_until = hold_at + PULSE_SECS;
        assert!(!crate::mercy_harvest_nodes::care_cycle_may_raise(
            1,
            hold_at + 0.5,
            breathe_until,
            &pulse,
            true
        ));
        assert!(crate::mercy_harvest_nodes::care_cycle_may_raise(
            1,
            breathe_until + 0.1,
            breathe_until,
            &pulse,
            true
        ));
    }

    /// CARD CARE-PROMPT-YIELD-1 — an active care strip hides the world-care
    /// prompt that would otherwise share the centred bottom-128 anchor.
    /// `active: false` keeps the pre-gate Visible result.
    #[test]
    fn care_prompt_yield_1_hides_when_care_cycle_active() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .init_resource::<FirstHarvestEpiphany>()
            .init_resource::<FirstSessionGuidance>()
            .init_resource::<NearbyMercyNode>()
            .add_systems(Update, update_world_care_prompt);
        app.world_mut().spawn((WorldCarePromptRoot, Visibility::Hidden));
        {
            let mut nearby = app.world_mut().resource_mut::<NearbyMercyNode>();
            nearby.in_range = true;
            nearby.nodes_exist = true;
        }

        let mut active = CareCycleOffer::default();
        active.active = true;
        app.insert_resource(active);
        app.update();
        assert_eq!(
            world_care_prompt_vis(&mut app),
            Visibility::Hidden,
            "active care strip yields the world-care prompt"
        );

        let mut quiet = CareCycleOffer::default();
        quiet.active = false;
        app.insert_resource(quiet);
        app.update();
        assert_eq!(
            world_care_prompt_vis(&mut app),
            Visibility::Visible,
            "inactive care strip leaves the in-range prompt visible"
        );
    }

    fn world_care_prompt_vis(app: &mut App) -> Visibility {
        let mut q = app
            .world_mut()
            .query_filtered::<&Visibility, With<WorldCarePromptRoot>>();
        *q.iter(app.world())
            .next()
            .expect("world-care prompt root")
    }

    /// CARD HUD-ANCHOR-REGISTRY-1B — on MoveAround, with nodes and none in
    /// range, Guidance shows and CarePrompt hides so the WASD line is on
    /// screen. In range, or after MoveAround advances, R2 applies. Exactly
    /// one of the two is visible. Visibility only.
    #[test]
    fn hour1_move_around_guidance_outranks_care_prompt() {
        use crate::first_session_guidance::{
            update_guidance_visibility, FirstSessionGuidanceStrip,
        };
        use crate::title_screen::LaunchDoor;

        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .init_resource::<FirstHarvestEpiphany>()
            .init_resource::<FirstSessionGuidance>()
            .init_resource::<NearbyMercyNode>()
            .insert_resource(LaunchDoor::InYard)
            .add_systems(
                Update,
                (update_world_care_prompt, update_guidance_visibility),
            );
        app.world_mut()
            .spawn((WorldCarePromptRoot, Visibility::Hidden));
        app.world_mut()
            .spawn((FirstSessionGuidanceStrip, Visibility::Hidden));
        {
            let mut nearby = app.world_mut().resource_mut::<NearbyMercyNode>();
            nearby.nodes_exist = true;
            nearby.in_range = false;
        }

        let stamp = hour1_order_stamp(&app);
        app.update();
        assert_eq!(
            guidance_strip_vis(&mut app),
            Visibility::Visible,
            "MoveAround with no node in range shows Guidance"
        );
        assert_eq!(
            world_care_prompt_vis(&mut app),
            Visibility::Hidden,
            "CarePrompt hides so the WASD line can show"
        );
        assert!(
            exactly_one_action_bar_line(&mut app),
            "exactly one of Guidance or CarePrompt"
        );
        assert_eq!(hour1_order_stamp(&app), stamp);

        app.world_mut().resource_mut::<NearbyMercyNode>().in_range = true;
        app.update();
        assert_eq!(
            world_care_prompt_vis(&mut app),
            Visibility::Visible,
            "a node in range returns the CarePrompt under R2"
        );
        assert_eq!(guidance_strip_vis(&mut app), Visibility::Hidden);
        assert!(exactly_one_action_bar_line(&mut app));
        assert_eq!(
            app.world().resource::<FirstSessionGuidance>().objective,
            GuidanceObjective::MoveAround
        );
        assert!(!app.world().resource::<FirstHarvestEpiphany>().first_harvest_lived);

        app.world_mut().resource_mut::<NearbyMercyNode>().in_range = false;
        app.world_mut().resource_mut::<FirstSessionGuidance>().objective =
            GuidanceObjective::ApproachGlowingNode;
        let advanced = hour1_order_stamp(&app);
        app.update();
        assert_eq!(
            world_care_prompt_vis(&mut app),
            Visibility::Visible,
            "after MoveAround advances, R2 lets the CarePrompt show"
        );
        assert_eq!(guidance_strip_vis(&mut app), Visibility::Hidden);
        assert!(exactly_one_action_bar_line(&mut app));
        assert_eq!(hour1_order_stamp(&app), advanced);
    }

    fn hour1_order_stamp(app: &App) -> (GuidanceObjective, bool, bool, f64, bool, f64) {
        let guidance = app.world().resource::<FirstSessionGuidance>();
        let epi = app.world().resource::<FirstHarvestEpiphany>();
        (
            guidance.objective.clone(),
            guidance.active,
            guidance.dismissed,
            guidance.shown_at_seconds,
            epi.first_harvest_lived,
            epi.prompt_until,
        )
    }

    fn guidance_strip_vis(app: &mut App) -> Visibility {
        use crate::first_session_guidance::FirstSessionGuidanceStrip;
        let mut q = app
            .world_mut()
            .query_filtered::<&Visibility, With<FirstSessionGuidanceStrip>>();
        *q.iter(app.world())
            .next()
            .expect("guidance strip")
    }

    fn exactly_one_action_bar_line(app: &mut App) -> bool {
        let guidance_on = guidance_strip_vis(app) == Visibility::Visible;
        let prompt_on = world_care_prompt_vis(app) == Visibility::Visible;
        guidance_on ^ prompt_on
    }

    #[test]
    fn unmatched_take_does_not_block_hold_e_tend() {
        assert!(!hold_e_tend_blocked(1, 0, 10.0, 10.1));
        assert!(hold_e_tend_blocked(1, 1, 10.0, 10.5));
        assert!(!hold_e_tend_blocked(1, 1, 10.0, 11.3));
    }

    /// CARD FLESH-EPIPHANY-PLACE — tend pulse may name the Place stood in.
    /// Tend words, breathe answer, and guidance advance stay. No new PlaceId.
    #[test]
    fn flesh_epiphany_place_names_stood_place_tend_line_stays() {
        use crate::hex_travel::HexTravelState;
        use crate::human_presence::SoftPresence;
        use crate::title_screen::TITLE_CHROME_CONTINUE;
        use shared::hex_travel::PlaceId;
        use shared::persona::STEWARD_ONLINE_YES;

        let bare = tend_harmony_pulse_line("Sanctuary ember", 0.4);
        assert_eq!(
            bare,
            "Tended Sanctuary ember · +0.4 harmony · vitality returns"
        );
        assert_eq!(tend_pulse_at_place("Sanctuary ember", 0.4, None), bare);
        assert!(bare.contains("Tended"));
        assert!(bare.contains("harmony"));
        assert!(bare.contains("vitality returns"));
        assert!(!bare.contains("Care cycle"));

        assert_eq!(epiphany_stood_place_name(PlaceId::Sanctuary, false), "Sanctuary");
        assert_eq!(epiphany_stood_place_name(PlaceId::Heartwood, false), "Heartwood");
        assert_eq!(
            epiphany_stood_place_name(PlaceId::Heartwood, true),
            "Threshold-near"
        );
        assert_eq!(epiphany_stood_place_name(PlaceId::Depths, false), "Depths");
        assert_eq!(epiphany_stood_place_name(PlaceId::Depths, true), "Depths");
        assert_eq!(
            epiphany_stood_place_name(PlaceId::Sanctuary, true),
            "Sanctuary"
        );

        let node_bare = tend_harmony_pulse_line("the node", 0.4);
        for place in ["Sanctuary", "Heartwood", "Threshold-near", "Depths"] {
            let line = tend_pulse_at_place("the node", 0.4, Some(place));
            assert_eq!(line, format!("{place} · {node_bare}"));
            assert!(line.contains("harmony"));
            assert!(line.contains("vitality returns"));
            assert!(line.contains("Tended"));
        }

        let near = tend_pulse_at_place(
            "the node",
            0.4,
            Some(epiphany_stood_place_name(PlaceId::Heartwood, true)),
        );
        assert!(near.contains("Threshold-near"));
        assert!(near.contains("harmony"));

        assert_eq!(tend_breathe_answer(), "tended — the node breathes");
        assert_eq!(tap_vs_hold_prompt(), "tap E take  ·  hold E tend");
        assert_eq!(TITLE_CHROME_CONTINUE, "Continue");
        assert!(!STEWARD_ONLINE_YES);

        let mut g = FirstSessionGuidance::default();
        g.objective = GuidanceObjective::HarvestWithInteract;
        crate::first_session_guidance::credit_harvest(&mut g);
        assert_eq!(g.objective, GuidanceObjective::OpenInventory);
        assert!(GuidanceObjective::HarvestWithInteract
            .prompt()
            .contains("tend"));

        let heart = HexTravelState {
            current: PlaceId::Heartwood,
        };
        let mut body = SoftPresence::default();
        assert_eq!(
            epiphany_stood_place_label(Some(&heart), Some(&body), false, false),
            Some("Heartwood")
        );
        assert_eq!(
            epiphany_stood_place_label(Some(&heart), Some(&body), false, true),
            Some("Heartwood"),
            "wards stay Heartwood"
        );
        assert_eq!(
            epiphany_stood_place_label(Some(&heart), Some(&body), true, false),
            Some("Threshold-near")
        );
        let shelf = shared::threshold_shelf::THRESHOLD_SHELF_CENTER;
        body.position.x = shelf[0];
        body.position.z = shelf[2];
        assert_eq!(
            epiphany_stood_place_label(Some(&heart), Some(&body), false, false),
            Some("Threshold-near")
        );
        assert_eq!(
            epiphany_stood_place_label(Some(&heart), Some(&body), false, true),
            Some("Threshold-near")
        );

        let sanctuary = HexTravelState {
            current: PlaceId::Sanctuary,
        };
        assert_eq!(
            epiphany_stood_place_label(Some(&sanctuary), Some(&body), false, false),
            Some("Sanctuary")
        );
        let depths = HexTravelState {
            current: PlaceId::Depths,
        };
        assert_eq!(
            epiphany_stood_place_label(Some(&depths), Some(&body), true, false),
            Some("Depths")
        );
        assert!(epiphany_stood_place_label(None, Some(&body), true, true).is_none());

        let week = GuidanceObjective::HourTwoHeld.prompt();
        assert!(week.contains("week"));
        assert!(week.contains("tons"));
    }

    /// CARD HUD-ANCHOR-REGISTRY-1 — CarePrompt sits on ACTION_BAR.
    /// CARD HUD-ANCHOR-REGISTRY-2B — Pulse and Welcome read the registry and
    /// stay on the coded anchors.
    #[test]
    fn care_prompt_lands_on_action_bar_pulse_and_welcome_stay() {
        use crate::hud_anchor_registry::{ACTION_BAR, ID_CARE_PROMPT};

        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_lived_surfaces);
        app.update();

        let mut prompt = app
            .world_mut()
            .query_filtered::<&Node, With<WorldCarePromptRoot>>();
        let prompt = prompt.single(app.world()).unwrap().clone();
        assert_eq!(prompt.bottom, ACTION_BAR.bottom());
        assert_eq!(prompt.right, ACTION_BAR.right());
        assert_eq!(prompt.left, Val::Auto);
        assert_eq!(prompt.width, Val::Px(ACTION_BAR.occupant(ID_CARE_PROMPT).width));
        assert_eq!(prompt.margin, UiRect::default());
        assert_eq!(prompt.padding, UiRect::axes(Val::Px(14.0), Val::Px(8.0)));
        assert_eq!(prompt.border, UiRect::all(Val::Px(1.0)));

        let mut pulse = app
            .world_mut()
            .query_filtered::<&Node, With<HarvestPulseRoot>>();
        let pulse = pulse.single(app.world()).unwrap().clone();
        assert_eq!(pulse.top, Val::Px(118.0));
        assert_eq!(pulse.left, Val::Percent(50.0));
        assert_eq!(pulse.width, Val::Px(560.0));
        assert_eq!(pulse.margin.left, Val::Px(-280.0));
        assert_eq!(pulse.top, PULSE.top());
        assert_eq!(pulse.left, PULSE.left());
        assert_eq!(pulse.margin, PULSE.margin());
        assert_eq!(pulse.width, Val::Px(PULSE.width));
        let pulse_coded = Node {
            position_type: PositionType::Absolute,
            top: Val::Px(118.0),
            left: Val::Percent(50.0),
            width: Val::Px(560.0),
            margin: UiRect::left(Val::Px(-280.0)),
            padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.2)),
            ..default()
        };
        assert_eq!(pulse, pulse_coded);

        let mut welcome = app
            .world_mut()
            .query_filtered::<&Node, With<WelcomeBackRoot>>();
        let welcome = welcome.single(app.world()).unwrap().clone();
        assert_eq!(welcome.top, Val::Px(16.0));
        assert_eq!(welcome.left, Val::Px(16.0));
        assert_eq!(welcome.width, Val::Px(380.0));
        assert_eq!(welcome.top, WELCOME.top());
        assert_eq!(welcome.left, WELCOME.left());
        assert_eq!(welcome.margin, UiRect::default());
        assert_eq!(welcome.width, Val::Px(WELCOME.width));
        let welcome_coded = Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Px(16.0),
            width: Val::Px(380.0),
            padding: UiRect::all(Val::Px(12.0)),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };
        assert_eq!(welcome, welcome_coded);

        let mut prompt_text = app
            .world_mut()
            .query_filtered::<&TextFont, With<WorldCarePromptText>>();
        assert_eq!(prompt_text.single(app.world()).unwrap().font_size, FontSize::Px(16.0 / 1.2));
        let mut pulse_text = app
            .world_mut()
            .query_filtered::<&TextFont, With<HarvestPulseText>>();
        assert_eq!(pulse_text.single(app.world()).unwrap().font_size, FontSize::Px(16.0 / 1.2));
        let mut welcome_text = app
            .world_mut()
            .query_filtered::<&TextFont, With<WelcomeBackText>>();
        assert_eq!(welcome_text.single(app.world()).unwrap().font_size, FontSize::Px(13.5 / 1.2));
    }

    /// InputMapSet fills PlayerInput before hold-to-tend reads it.
    #[test]
    fn hold_tend_is_ordered_after_input_map() {
        use bevy::ecs::system::{IntoSystem, System};
        use std::any::TypeId;

        fn update_order(with_feel: bool) -> Vec<TypeId> {
            let mut app = App::new();
            app.add_plugins(bevy::MinimalPlugins);
            app.add_plugins(crate::input::InputPlugin);
            app.add_plugins(crate::first_session_guidance::FirstSessionGuidancePlugin);
            if with_feel {
                app.add_plugins(crate::feel_move::FeelMovePlugin);
            }
            app.add_plugins(FirstHarvestEpiphanyPlugin);
            let mut schedules = app
                .world_mut()
                .remove_resource::<bevy::ecs::schedule::Schedules>()
                .unwrap();
            let schedule = schedules.get_mut(Update).unwrap();
            schedule.initialize(app.world_mut()).unwrap();
            schedule
                .systems()
                .unwrap()
                .map(|(_, system)| System::system_type(&**system))
                .collect()
        }

        let with_feel = update_order(true);
        let without_feel = update_order(false);
        let feel_ids: Vec<TypeId> = with_feel
            .iter()
            .copied()
            .filter(|id| !without_feel.contains(id))
            .collect();
        let want_input = IntoSystem::system_type_id(&crate::input::handle_player_input);
        let want_tend = IntoSystem::system_type_id(&handle_interact_harvest);
        let input_at = with_feel.iter().position(|id| *id == want_input).unwrap();
        let tend_at = with_feel.iter().position(|id| *id == want_tend).unwrap();
        assert!(
            !feel_ids.is_empty(),
            "FeelMovePlugin stays registered; this card does not reorder it"
        );
        assert!(
            input_at < tend_at,
            "input fill {input_at} runs before tend {tend_at}"
        );
    }

    /// CARD SCRIPT-HOLD-TEND-1 — held level alone, for TEND_HOLD or longer, tends.
    /// No keyboard and no gamepad on this path.
    #[test]
    fn player_input_held_for_tend_hold_tends() {
        assert!((TEND_HOLD - 0.42).abs() < 1e-9);
        let mut app = player_input_tend_app();
        set_use_level(&mut app, false, true);
        advance_secs(&mut app, 0.0);
        let started = app.world().resource::<Time>().elapsed_secs_f64();
        assert_eq!(session_counts(&app), (0, 0), "hold starts before 0.42");

        set_use_level(&mut app, false, true);
        advance_secs(&mut app, 0.5);
        let now = app.world().resource::<Time>().elapsed_secs_f64();
        assert!(
            now - started >= TEND_HOLD,
            "held span {started} -> {now} must reach TEND_HOLD"
        );
        assert_eq!(
            session_counts(&app),
            (0, 1),
            "held level tends without a take"
        );
        assert_eq!(app.world().resource::<WorldAnswer>().kind, AnswerKind::Tend);
    }

    /// CARD SCRIPT-HOLD-TEND-1 — falling edge before 0.42 takes.
    /// No keyboard and no gamepad on this path.
    #[test]
    fn player_input_release_before_tend_hold_takes() {
        let mut app = player_input_tend_app();
        set_use_level(&mut app, false, true);
        advance_secs(&mut app, 0.0);
        let started = app.world().resource::<Time>().elapsed_secs_f64();

        set_use_level(&mut app, false, true);
        advance_secs(&mut app, 0.2);
        let now = app.world().resource::<Time>().elapsed_secs_f64();
        assert!(now - started < TEND_HOLD, "0.2s is still a take");
        assert_eq!(session_counts(&app), (0, 0));

        set_use_level(&mut app, false, false);
        advance_secs(&mut app, 0.0);
        assert_eq!(session_counts(&app), (1, 0), "release before 0.42 takes");
        assert_eq!(app.world().resource::<WorldAnswer>().kind, AnswerKind::Take);
    }

    /// Same press timing through the keyboard fill and through PlayerInput directly.
    /// The press frame is a take (`pad_tap`); a later hold of 0.5s tends; a release
    /// at 0.2s does not tend. Counts include that press-frame take.
    #[test]
    fn player_input_hold_counts_match_keyboard_path() {
        let direct_hold = run_use_script(UseDrive::PlayerInput, UseScript::Hold);
        let keyboard_hold = run_use_script(UseDrive::Keyboard, UseScript::Hold);
        assert_eq!(
            direct_hold, keyboard_hold,
            "hold counts must match the keyboard path"
        );
        assert_eq!(
            direct_hold,
            (1, 1),
            "press-frame take plus a hold of 0.42s or more tends"
        );

        let direct_release = run_use_script(UseDrive::PlayerInput, UseScript::Release);
        let keyboard_release = run_use_script(UseDrive::Keyboard, UseScript::Release);
        assert_eq!(
            direct_release, keyboard_release,
            "release counts must match the keyboard path"
        );
        assert_eq!(
            direct_release,
            (1, 0),
            "press-frame take, release at about 0.2s does not tend"
        );
    }

    fn session_counts(app: &App) -> (u32, u32) {
        let state = app.world().resource::<FirstHarvestEpiphany>();
        (state.harvests_this_session, state.tends_this_session)
    }

    fn advance_secs(app: &mut App, secs: f64) {
        *app.world_mut()
            .resource_mut::<bevy::time::TimeUpdateStrategy>() =
            bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::from_secs_f64(
                secs,
            ));
        app.update();
    }

    fn set_use_level(app: &mut App, edge: bool, held: bool) {
        let mut input = app.world_mut().resource_mut::<PlayerInput>();
        input.interact = edge;
        input.interact_held = held;
    }

    fn player_input_tend_app() -> App {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
                std::time::Duration::ZERO,
            ))
            .init_resource::<PlayerInput>()
            .init_resource::<FirstHarvestEpiphany>()
            .init_resource::<FirstSessionGuidance>()
            .init_resource::<ThrivingMoments>()
            .init_resource::<AbundanceJourneyEcho>()
            .init_resource::<NearbyMercyNode>()
            .init_resource::<SoftRbePool>()
            .init_resource::<WorldAnswer>()
            .add_message::<bevy::input::gamepad::GamepadRumbleRequest>()
            .add_systems(Update, handle_interact_harvest);
        widen_virtual_clock(&mut app);
        {
            let mut nearby = app.world_mut().resource_mut::<NearbyMercyNode>();
            nearby.in_range = true;
            nearby.nodes_exist = true;
            nearby.name = Some("Sanctuary ember");
        }
        app
    }

    fn widen_virtual_clock(app: &mut App) {
        app.world_mut()
            .resource_mut::<Time<bevy::time::Virtual>>()
            .set_max_delta(std::time::Duration::from_secs(2));
    }

    #[derive(Clone, Copy)]
    enum UseDrive {
        PlayerInput,
        Keyboard,
    }

    #[derive(Clone, Copy)]
    enum UseScript {
        Hold,
        Release,
    }

    #[derive(Clone, Copy)]
    enum UsePhase {
        Press,
        Hold,
        Release,
    }

    fn run_use_script(drive: UseDrive, script: UseScript) -> (u32, u32) {
        let mut app = match drive {
            UseDrive::PlayerInput => player_input_tend_app(),
            UseDrive::Keyboard => keyboard_tend_app(),
        };
        // Bevy's first clock update records the instant and does not advance.
        // Land at t = 1.0 before the press so the take arms the existing cooldown.
        advance_secs(&mut app, 0.0);
        advance_secs(&mut app, 1.0);
        apply_phase(&mut app, drive, UsePhase::Press);
        advance_secs(&mut app, 0.0);
        apply_phase(&mut app, drive, UsePhase::Hold);
        advance_secs(&mut app, 0.0);
        match script {
            UseScript::Hold => {
                apply_phase(&mut app, drive, UsePhase::Hold);
                advance_secs(&mut app, 0.5);
            }
            UseScript::Release => {
                apply_phase(&mut app, drive, UsePhase::Release);
                advance_secs(&mut app, 0.2);
            }
        }
        session_counts(&app)
    }

    fn apply_phase(app: &mut App, drive: UseDrive, phase: UsePhase) {
        match drive {
            UseDrive::PlayerInput => match phase {
                UsePhase::Press => set_use_level(app, true, true),
                UsePhase::Hold => set_use_level(app, false, true),
                UsePhase::Release => set_use_level(app, false, false),
            },
            UseDrive::Keyboard => {
                let mut keys = app
                    .world_mut()
                    .resource_mut::<ButtonInput<bevy::input::keyboard::KeyCode>>();
                match phase {
                    UsePhase::Press => keys.press(bevy::input::keyboard::KeyCode::KeyE),
                    UsePhase::Hold => {
                        keys.press(bevy::input::keyboard::KeyCode::KeyE);
                        keys.clear();
                    }
                    UsePhase::Release => keys.release(bevy::input::keyboard::KeyCode::KeyE),
                }
            }
        }
    }

    fn keyboard_tend_app() -> App {
        let mut app = player_input_tend_app();
        app.init_resource::<ButtonInput<bevy::input::keyboard::KeyCode>>()
            .insert_resource(crate::local_settings::LocalSettingsState {
                inner: shared::local_settings::LocalSettings::peace_defaults(),
                dirty: false,
            })
            .add_systems(
                Update,
                crate::input::handle_player_input
                    .in_set(crate::input::InputMapSet)
                    .before(handle_interact_harvest),
            );
        app
    }
}
