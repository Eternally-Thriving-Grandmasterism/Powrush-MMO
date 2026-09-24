/*!
 * Soft RBE Allocate Choice — credit face (CREDIT_RESERVE · H-2026-09-11-D4)
 *
 * After meaningful surplus, invite a voluntary allocation:
 *   • Flow — field restore / share credit into the lattice
 *   • Reserve — repair-rights steward hold for later mend
 *
 * Credit ≠ gold. Neither path is sell / price / ticker / Market.
 * Still-frame names logistics, not currency.
 *
 * Controls: **R** toggles panel when eligible · **1** Flow · **2** Reserve · Esc / R closes
 *
 * CARD FLESH-ALLOCATE-LINE — Flow / Reserve invite may name the Place already
 * on LivedHourBind climate hex (`PlaceId::parse` / `display_name`).
 * Flow stays field restore. Reserve stays repair-rights hold.
 * `local-hex` is Sanctuary dirt. Unknown hex keeps this copy.
 * Threshold-near is shelf reach, not a wake Place. No new PlaceId.
 * Peak memory locked (cite only): walked · tended · week was the bill · yard remembered.
 *
 * PATSAGi + TOLC 8 | AG-SML v1.0 | Contact: info@Rathor.ai
 * Thunder locked in. Yoi ⚡
 */

use bevy::input::gamepad::GamepadRumbleRequest;
use bevy::prelude::*;

use shared::climate_node::AllocKind;
use shared::hex_travel::PlaceId;

use crate::first_session_guidance::{credit_share, FirstSessionGuidance};
use crate::harvest_feel::rumble_mercy_harvest;
use crate::lived_hour_bind::LivedHourBind;
use crate::lived_sim_bridge::{emit_lived_event, LivedSimBridge};
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocatePath {
    FlowOutward,
    StewardReserve,
}

impl AllocatePath {
    pub fn title(self) -> &'static str {
        match self {
            AllocatePath::FlowOutward => "Flow · field restore",
            AllocatePath::StewardReserve => "Reserve · repair-rights",
        }
    }

    pub fn line(self) -> &'static str {
        match self {
            AllocatePath::FlowOutward => {
                "Share credit into the lattice — restore shared field"
            }
            AllocatePath::StewardReserve => {
                "Hold repair-rights credit — steward for later mend"
            }
        }
    }
}

/// Climate hex on [`LivedHourBind`] → Place display name.
/// `PlaceId::parse` maps `local-hex` to Sanctuary dirt. Unknown hex, including
/// Threshold-near shelf reach, is not a wake Place.
pub fn allocate_place_name(hex_id: &str) -> Option<&'static str> {
    PlaceId::parse(hex_id).map(PlaceId::display_name)
}

fn dress_allocate_copy(base: &str, place: Option<&str>) -> String {
    match place.map(str::trim).filter(|name| !name.is_empty()) {
        Some(place) => format!("{place} · {base}"),
        None => base.to_string(),
    }
}

/// Invite the panel opens with. Known Place prefixes the existing credit line.
pub fn allocate_invite_at_place(place: Option<&str>) -> String {
    dress_allocate_copy("Allocate credit · Flow or Reserve", place)
}

/// Flow stays field restore. Reserve stays a repair-rights hold.
/// Known Place dresses the pair. Unknown keeps the existing sentence.
pub fn allocate_flow_reserve_at_place(place: Option<&str>) -> String {
    dress_allocate_copy("Flow restores field · Reserve holds repair-rights", place)
}

/// One path line, dressed when the climate hex names a Place.
pub fn allocate_path_line_at_place(path: AllocatePath, place: Option<&str>) -> String {
    dress_allocate_copy(path.line(), place)
}

/// Live panel body. Unknown hex keeps the undressed credit counts.
pub fn allocate_body_at_place(
    ready: f32,
    flowed: f32,
    reserved: f32,
    place: Option<&str>,
) -> String {
    let counts = format!(
        "Allocate credit · ready {ready:.1}  ·  flowed {flowed:.1}  ·  reserved {reserved:.1}"
    );
    format!(
        "{}\n{}",
        dress_allocate_copy(&counts, place),
        allocate_flow_reserve_at_place(place)
    )
}

/// Allocate-face copy stays credit logistics — never gold / Market / sell / price.
pub fn allocate_copy_is_honest(s: &str) -> bool {
    let low = s.to_lowercase();
    !low.contains("gold")
        && !low.contains("market")
        && !low.contains("price")
        && !low.contains("sell")
        && !low.contains("ticker")
        && !low.contains("currency")
        && !low.contains("auction")
}

#[derive(Resource, Debug)]
pub struct RbeAllocateChoice {
    pub panel_open: bool,
    pub eligible: bool,
    pub surplus_signal: f32,
    pub last_choice: Option<AllocatePath>,
    pub choices_made: u32,
    pub flow_total: f32,
    pub reserve_total: f32,
    pub auto_offered: bool,
    /// Soft threshold to become eligible after harvest feedback / practice.
    pub eligibility_threshold: f32,
}

impl Default for RbeAllocateChoice {
    fn default() -> Self {
        Self {
            panel_open: false,
            eligible: false,
            surplus_signal: 0.0,
            last_choice: None,
            choices_made: 0,
            flow_total: 0.0,
            reserve_total: 0.0,
            auto_offered: false,
            eligibility_threshold: 1.0,
        }
    }
}

impl RbeAllocateChoice {
    pub fn note_surplus(&mut self, amount: f32) {
        if amount <= 0.0 {
            return;
        }
        self.surplus_signal = (self.surplus_signal + amount).min(32.0);
        if self.surplus_signal >= self.eligibility_threshold {
            self.eligible = true;
        }
    }

    /// E tend fills the lived-hour satchel. That take is surplus for R.
    /// Without this, only RBE harvest strings opened the panel and R+2
    /// could show reserved 0.0 with no banked confirm.
    pub fn note_satchel_delta(&mut self, prev: usize, now: usize) {
        if now > prev {
            self.note_surplus((now - prev) as f32);
        }
    }

    pub fn apply(&mut self, path: AllocatePath, portion: f32) {
        let take = portion.clamp(0.1, self.surplus_signal.max(0.1));
        match path {
            AllocatePath::FlowOutward => self.flow_total += take,
            AllocatePath::StewardReserve => self.reserve_total += take,
        }
        self.surplus_signal = (self.surplus_signal - take).max(0.0);
        self.last_choice = Some(path);
        self.choices_made = self.choices_made.saturating_add(1);
        if self.surplus_signal < self.eligibility_threshold * 0.25 {
            self.eligible = false;
            self.panel_open = false;
        }
    }
}

#[derive(Component)]
pub struct AllocatePanelRoot;

#[derive(Component)]
pub struct AllocateBodyText;

#[derive(Component)]
pub struct AllocateFlowButton;

#[derive(Component)]
pub struct AllocateReserveButton;

pub struct RbeAllocateChoicePlugin;

impl Plugin for RbeAllocateChoicePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RbeAllocateChoice>()
            .add_systems(Startup, spawn_allocate_panel)
            .add_systems(
                Update,
                (
                    surplus_from_lived_satchel,
                    soft_surplus_from_rbe_feedback,
                    toggle_allocate_panel,
                    update_allocate_visibility,
                    update_allocate_body,
                    handle_allocate_buttons,
                ),
            );
    }
}

fn spawn_allocate_panel(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(140.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(520.0),
                    margin: UiRect::left(Val::Px(-260.0)),
                    padding: UiRect::all(Val::Px(16.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    border: UiRect::all(Val::Px(1.5)),
                    ..default()
                },
                background_color: Color::srgba(0.04, 0.08, 0.10, 0.94).into(),
                border_color: Color::srgba(0.55, 0.88, 0.70, 0.55).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            AllocatePanelRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "Allocate credit · Flow or Reserve",
                    TextStyle {
                        font_size: 15.0,
                        color: Color::srgb(0.85, 0.98, 0.90),
                        ..default()
                    },
                ),
                AllocateBodyText,
            ));

            p.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|row| {
                row.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.12, 0.28, 0.22, 0.95).into(),
                        border_color: Color::srgb(0.45, 0.90, 0.70).into(),
                        ..default()
                    },
                    AllocateFlowButton,
                ))
                .with_children(|b| {
                    b.spawn(TextBundle::from_section(
                        "Flow · field restore",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::srgb(0.90, 1.0, 0.95),
                            ..default()
                        },
                    ));
                });

                row.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.14, 0.18, 0.28, 0.95).into(),
                        border_color: Color::srgb(0.55, 0.75, 0.95).into(),
                        ..default()
                    },
                    AllocateReserveButton,
                ))
                .with_children(|b| {
                    b.spawn(TextBundle::from_section(
                        "Reserve · repair-rights",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::srgb(0.90, 0.95, 1.0),
                            ..default()
                        },
                    ));
                });
            });

            p.spawn(TextBundle::from_section(
                "1 Flow · 2 Reserve · R close · credit logistics",
                TextStyle {
                    font_size: 12.0,
                    color: Color::srgb(0.65, 0.80, 0.75),
                    ..default()
                },
            ));
        });
}

fn surplus_from_lived_satchel(
    bind: Option<Res<LivedHourBind>>,
    mut allocate: ResMut<RbeAllocateChoice>,
    mut last: Local<usize>,
) {
    let Some(bind) = bind else {
        return;
    };
    let now = bind.satchel_count();
    allocate.note_satchel_delta(*last, now);
    *last = now;
}

/// Digit2 is Reserve while the allocate panel is open — Distill Ward waits.
pub fn allocate_owns_digit2(panel_open: bool) -> bool {
    panel_open
}

/// Bind first. UI credit only when the satchel actually spent.
/// Empty satchel must not paint reserved 0.0 as a successful bank.
pub fn try_commit_allocate(
    allocate: &mut RbeAllocateChoice,
    bind: &mut LivedHourBind,
    path: AllocatePath,
) -> bool {
    let kind = match path {
        AllocatePath::FlowOutward => AllocKind::Flow,
        AllocatePath::StewardReserve => AllocKind::Reserve,
    };
    let ok = bind.allocate(kind);
    if ok {
        allocate.apply(path, 1.0);
    }
    ok
}

fn soft_surplus_from_rbe_feedback(
    rbe_ui: Option<Res<crate::lived_hour_support::RbeUiSync>>,
    mut allocate: ResMut<RbeAllocateChoice>,
    mut last: Local<Option<String>>,
) {
    let Some(rbe_ui) = rbe_ui else {
        return;
    };
    let Some(ref fb) = rbe_ui.last_harvest_feedback else {
        return;
    };
    if last.as_ref() == Some(fb) {
        return;
    }
    *last = Some(fb.clone());

    // Soft parse: any positive harvest line adds a unit of surplus signal
    let positive = fb.contains('+')
        || fb.contains("Sustainable")
        || fb.contains("Epiphany")
        || fb.contains("abundance")
        || fb.contains("Council");
    if positive && !fb.contains("failed") && !fb.contains("Failed") {
        allocate.note_surplus(1.0);
        if allocate.eligible && !allocate.auto_offered && !allocate.panel_open {
            allocate.panel_open = true;
            allocate.auto_offered = true;
        }
    }
}

fn toggle_allocate_panel(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut allocate: ResMut<RbeAllocateChoice>,
) {
    if keyboard.just_pressed(soft_play_bindings::ALLOCATE) {
        if allocate.panel_open {
            allocate.panel_open = false;
        } else if allocate.eligible || allocate.surplus_signal > 0.0 {
            allocate.eligible = true;
            allocate.panel_open = true;
        }
    }
    if keyboard.just_pressed(KeyCode::Escape) && allocate.panel_open {
        allocate.panel_open = false;
    }
}

fn update_allocate_visibility(
    allocate: Res<RbeAllocateChoice>,
    mut q: Query<&mut Visibility, With<AllocatePanelRoot>>,
) {
    let show = allocate.panel_open;
    for mut vis in &mut q {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn update_allocate_body(
    allocate: Res<RbeAllocateChoice>,
    bind: Option<Res<LivedHourBind>>,
    mut q: Query<&mut Text, With<AllocateBodyText>>,
) {
    let bind_changed = bind.as_ref().is_some_and(|hour| hour.is_changed());
    if !allocate.is_changed() && !bind_changed {
        return;
    }
    let place = bind
        .as_ref()
        .and_then(|hour| allocate_place_name(&hour.climate.hex_id));
    let body = allocate_body_at_place(
        allocate.surplus_signal,
        allocate.flow_total,
        allocate.reserve_total,
        place,
    );
    for mut text in &mut q {
        if let Some(s) = text.sections.get_mut(0) {
            s.value = body.clone();
        }
    }
}

fn commit_allocate(
    allocate: &mut RbeAllocateChoice,
    moments: &mut ThrivingMoments,
    guidance: &mut FirstSessionGuidance,
    rumble: &mut EventWriter<GamepadRumbleRequest>,
    gamepads: &Gamepads,
    bind: &mut LivedHourBind,
    bridge: Option<&LivedSimBridge>,
    path: AllocatePath,
    now: f64,
) {
    let ok = try_commit_allocate(allocate, bind, path);
    if ok {
        rumble_mercy_harvest(rumble, gamepads);
        fire_thriving(moments, ThrivingKind::FirstShare, now);
        credit_share(guidance);
    }
    let action = match path {
        AllocatePath::FlowOutward => "allocate_flow",
        AllocatePath::StewardReserve => "allocate_reserve",
    };
    if let Some(bridge) = bridge {
        emit_lived_event(
            bridge.session_id,
            now,
            action,
            bind.last_line.clone(),
            None,
        );
    }
    if ok {
        info!(target: "powrush::rbe", ?path, line = %bind.last_line, "Allocate committed");
    } else {
        warn!(target: "powrush::rbe", ?path, line = %bind.last_line, "Allocate refused");
    }
}

fn handle_allocate_buttons(
    mut allocate: ResMut<RbeAllocateChoice>,
    mut moments: ResMut<ThrivingMoments>,
    mut guidance: ResMut<FirstSessionGuidance>,
    mut rumble: EventWriter<GamepadRumbleRequest>,
    gamepads: Res<Gamepads>,
    mut bind: ResMut<LivedHourBind>,
    bridge: Option<Res<LivedSimBridge>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    flow_q: Query<&Interaction, (Changed<Interaction>, With<AllocateFlowButton>)>,
    reserve_q: Query<&Interaction, (Changed<Interaction>, With<AllocateReserveButton>)>,
) {
    if !allocate.panel_open {
        return;
    }
    let now = time.elapsed_seconds_f64();
    let path = if keyboard.just_pressed(KeyCode::Digit1) {
        Some(AllocatePath::FlowOutward)
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        Some(AllocatePath::StewardReserve)
    } else {
        None
    };
    if let Some(path) = path {
        commit_allocate(
            &mut allocate,
            &mut moments,
            &mut guidance,
            &mut rumble,
            &gamepads,
            &mut bind,
            bridge.as_deref(),
            path,
            now,
        );
        return;
    }
    for inter in &flow_q {
        if *inter == Interaction::Pressed {
            commit_allocate(
                &mut allocate,
                &mut moments,
                &mut guidance,
                &mut rumble,
                &gamepads,
                &mut bind,
                bridge.as_deref(),
                AllocatePath::FlowOutward,
                now,
            );
            return;
        }
    }
    for inter in &reserve_q {
        if *inter == Interaction::Pressed {
            commit_allocate(
                &mut allocate,
                &mut moments,
                &mut guidance,
                &mut rumble,
                &gamepads,
                &mut bind,
                bridge.as_deref(),
                AllocatePath::StewardReserve,
                now,
            );
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::climate_node::{LivedHour, TendResult};

    #[test]
    fn surplus_unlocks_and_allocate_reduces() {
        let mut a = RbeAllocateChoice::default();
        a.note_surplus(1.5);
        assert!(a.eligible);
        a.apply(AllocatePath::FlowOutward, 1.0);
        assert!(a.flow_total >= 1.0);
        assert!(a.surplus_signal < 1.5);
    }

    #[test]
    fn both_paths_valid() {
        let mut a = RbeAllocateChoice::default();
        a.note_surplus(2.0);
        a.apply(AllocatePath::StewardReserve, 1.0);
        a.apply(AllocatePath::FlowOutward, 1.0);
        assert_eq!(a.choices_made, 2);
    }

    #[test]
    fn digit_labels_are_flow_then_reserve() {
        assert_eq!(AllocatePath::FlowOutward.title(), "Flow · field restore");
        assert_eq!(AllocatePath::StewardReserve.title(), "Reserve · repair-rights");
    }

    #[test]
    fn allocate_face_names_credit_not_gold() {
        for path in [AllocatePath::FlowOutward, AllocatePath::StewardReserve] {
            assert!(allocate_copy_is_honest(path.title()), "{}", path.title());
            assert!(allocate_copy_is_honest(path.line()), "{}", path.line());
        }
        assert!(AllocatePath::FlowOutward.line().contains("field"));
        assert!(AllocatePath::StewardReserve.line().contains("repair-rights"));
        assert!(allocate_copy_is_honest(
            "Allocate credit · Flow or Reserve"
        ));
        assert!(allocate_copy_is_honest(
            "1 Flow · 2 Reserve · R close · credit logistics"
        ));
        assert!(!allocate_copy_is_honest("sell gold on Market"));
        assert!(!allocate_copy_is_honest("price ticker"));
    }

    /// CARD FLESH-ALLOCATE-LINE — Place dress stays credit logistics.
    /// Known climate hex names Sanctuary / Heartwood / Depths.
    /// `local-hex` is Sanctuary dirt. Unknown and Threshold-near stay undressed.
    #[test]
    fn flesh_allocate_line_dresses_place_and_stays_honest() {
        let bare_invite = "Allocate credit · Flow or Reserve";
        let bare_pair = "Flow restores field · Reserve holds repair-rights";
        assert_eq!(allocate_invite_at_place(None), bare_invite);
        assert_eq!(allocate_flow_reserve_at_place(None), bare_pair);
        assert_eq!(
            allocate_path_line_at_place(AllocatePath::FlowOutward, None),
            AllocatePath::FlowOutward.line()
        );
        assert_eq!(
            allocate_path_line_at_place(AllocatePath::StewardReserve, None),
            AllocatePath::StewardReserve.line()
        );

        let bare_body = allocate_body_at_place(1.0, 0.0, 0.0, None);
        assert_eq!(
            bare_body,
            "Allocate credit · ready 1.0  ·  flowed 0.0  ·  reserved 0.0\nFlow restores field · Reserve holds repair-rights"
        );
        assert!(!bare_body.contains("Sanctuary"));
        assert!(!bare_body.contains("Threshold"));

        for unknown in ["", "nowhere", "threshold-near", "Threshold-near"] {
            assert!(allocate_place_name(unknown).is_none(), "{unknown}");
            assert_eq!(
                allocate_invite_at_place(allocate_place_name(unknown)),
                bare_invite
            );
            assert_eq!(
                allocate_body_at_place(1.0, 0.0, 0.0, allocate_place_name(unknown)),
                bare_body
            );
            assert_eq!(
                allocate_path_line_at_place(
                    AllocatePath::FlowOutward,
                    allocate_place_name(unknown)
                ),
                AllocatePath::FlowOutward.line()
            );
        }

        assert_eq!(allocate_place_name("local-hex"), Some("Sanctuary"));
        assert_eq!(
            allocate_place_name("sanctuary"),
            Some(PlaceId::Sanctuary.display_name())
        );
        assert_eq!(allocate_place_name("heartwood"), Some("Heartwood"));
        assert_eq!(allocate_place_name("depths"), Some("Depths"));
        assert_ne!(allocate_place_name("heartwood"), Some("Threshold-near"));

        for hex in ["local-hex", "sanctuary", "sanctuary-prime", "heartwood", "depths"] {
            let place = allocate_place_name(hex).expect(hex);
            let invite = allocate_invite_at_place(Some(place));
            let pair = allocate_flow_reserve_at_place(Some(place));
            let body = allocate_body_at_place(2.5, 1.0, 0.5, Some(place));
            let flow = allocate_path_line_at_place(AllocatePath::FlowOutward, Some(place));
            let reserve =
                allocate_path_line_at_place(AllocatePath::StewardReserve, Some(place));

            assert!(invite.starts_with(&format!("{place} · ")), "{invite}");
            assert!(invite.contains("Allocate credit"));
            assert!(invite.contains("Flow or Reserve"));
            assert!(pair.starts_with(&format!("{place} · ")), "{pair}");
            assert!(pair.contains("Flow restores field"));
            assert!(pair.contains("Reserve holds repair-rights"));
            assert!(body.contains(place), "{body}");
            assert!(body.contains("ready 2.5"));
            assert!(body.contains("field"));
            assert!(body.contains("repair-rights"));
            assert!(flow.contains(place) && flow.contains("field"), "{flow}");
            assert!(
                reserve.contains(place) && reserve.contains("repair-rights"),
                "{reserve}"
            );
            assert!(!flow.contains("Threshold"));
            assert!(!reserve.contains("Threshold"));

            for line in [&invite, &pair, &body, &flow, &reserve] {
                assert!(allocate_copy_is_honest(line), "{line}");
                assert!(!line.contains("XP"), "{line}");
                assert!(!line.to_lowercase().contains("gold"), "{line}");
                assert!(!line.contains("Market"), "{line}");
            }
        }

        let dirt = allocate_invite_at_place(allocate_place_name("local-hex"));
        assert!(dirt.contains("Sanctuary"));
        assert!(dirt.contains("Flow"));
        assert!(dirt.contains("Reserve"));
        let heart = allocate_body_at_place(1.0, 0.0, 0.0, allocate_place_name("heartwood"));
        assert!(heart.contains("Heartwood"));
        assert!(!heart.contains("Threshold"));
    }

    fn test_bind() -> LivedHourBind {
        LivedHourBind {
            hour: LivedHour::new_demo(),
            climate: shared::shard_climate::ShardClimate::default(),
            standing: shared::shard_standing::ShardStanding::default(),
            week: shared::week_audit::WeekAudit::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        }
    }

    #[test]
    fn satchel_take_unlocks_r_panel() {
        let mut a = RbeAllocateChoice::default();
        assert!(!a.eligible);
        a.note_satchel_delta(0, 1);
        assert!(a.eligible);
        assert!(a.surplus_signal >= 1.0);
    }

    #[test]
    fn digit2_is_reserve_while_allocate_open() {
        assert!(allocate_owns_digit2(true));
        assert!(!allocate_owns_digit2(false));
    }

    #[test]
    fn r_then_2_banks_reserve_from_satchel() {
        let mut a = RbeAllocateChoice::default();
        let mut bind = test_bind();
        a.note_surplus(1.0);
        assert!(matches!(bind.tend(1), TendResult::Taken { .. }));
        assert!(try_commit_allocate(
            &mut a,
            &mut bind,
            AllocatePath::StewardReserve
        ));
        assert_eq!(bind.hour.allocation.reserve, 1);
        assert_eq!(bind.climate.reserve_pool, 1);
        assert!(a.reserve_total >= 1.0);
        let line = bind.hour.allocation.reserve_bank_line().expect("banked");
        assert!(line.contains("Reserve 1"));
        assert!(!line.contains("0.0"));
        assert!(bind.last_line.contains("repair-rights"));
    }

    #[test]
    fn empty_satchel_does_not_paint_a_zero_reserve_bank() {
        let mut a = RbeAllocateChoice::default();
        let mut bind = test_bind();
        a.note_surplus(1.0);
        assert!(!try_commit_allocate(
            &mut a,
            &mut bind,
            AllocatePath::StewardReserve
        ));
        assert_eq!(a.reserve_total, 0.0);
        assert_eq!(bind.hour.allocation.reserve, 0);
        assert_eq!(bind.climate.reserve_pool, 0);
        assert!(bind.hour.allocation.reserve_bank_line().is_none());
        assert_eq!(bind.last_line, "satchel empty");
    }
}
