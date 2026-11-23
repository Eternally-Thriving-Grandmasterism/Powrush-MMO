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
use crate::hud_anchor_registry::{HudSlab, ALLOCATE_DOCK, ID_ALLOCATE};
use crate::title_screen::{TITLE_BORDER, TITLE_BTN_BG, TITLE_BTN_FG, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY, TITLE_TEXT_SECONDARY};
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
            (
                Node {
                    position_type: PositionType::Absolute,
                    bottom: ALLOCATE_DOCK.bottom(),
                    right: ALLOCATE_DOCK.right(),
                    width: Val::Px(520.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    border: UiRect::all(Val::Px(1.5)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG),
                crate::ui_frame::plate_frame(),
                BorderColor::all(TITLE_BORDER),
                Visibility::Hidden,
            ),
            AllocatePanelRoot,
            HudSlab(ID_ALLOCATE),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new("Allocate credit · Flow or Reserve"),
TextFont { font_size: FontSize::Px(15.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                AllocateBodyText,
            ));

            p.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    ..default()
                })
            .with_children(|row| {
                row.spawn((
                    (
                        bevy::ui_widgets::Button, Interaction::default(),
                        Node {
                            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BorderColor::all(TITLE_BORDER),
                        BackgroundColor(TITLE_BTN_BG),
                    ),
                    AllocateFlowButton,
                ))
                .with_children(|b| {
                    b.spawn((
Text::new("Flow · field restore"),
TextFont { font_size: FontSize::Px(14.0 / 1.2), ..default() },
TextColor(TITLE_BTN_FG),
));
                });

                row.spawn((
                    (
                        bevy::ui_widgets::Button, Interaction::default(),
                        Node {
                            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BorderColor::all(TITLE_BORDER),
                        BackgroundColor(TITLE_BTN_BG),
                    ),
                    AllocateReserveButton,
                ))
                .with_children(|b| {
                    b.spawn((
Text::new("Reserve · repair-rights"),
TextFont { font_size: FontSize::Px(14.0 / 1.2), ..default() },
TextColor(TITLE_BTN_FG),
));
                });
            });

            p.spawn((
Text::new("1 Flow · 2 Reserve · R close · credit logistics"),
TextFont { font_size: FontSize::Px(12.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_SECONDARY),
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
    (time, calm): (Option<Res<Time>>, Option<Res<crate::first_harvest_epiphany::CalmQueue>>),
) {
    let Some(rbe_ui) = rbe_ui else {
        return;
    };
    let Some(ref fb) = rbe_ui.last_harvest_feedback else {
        return;
    };
    if calm.as_deref().is_some_and(|c| !c.done(time.as_deref().map_or(0.0, |t| t.elapsed_secs_f64()))) { return; } // UI-CALM-1: defer, not drop
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

pub(crate) fn toggle_allocate_panel(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut allocate: ResMut<RbeAllocateChoice>,
    pads: Query<&Gamepad>,
    (door, label, licences, places, edit): (Option<Res<crate::title_screen::LaunchDoor>>, Option<Res<crate::title_screen::HouseLabel>>, Option<Res<crate::licences_screen::LicencesScreen>>, Option<Res<crate::hex_travel::PlacesPlate>>, Option<Res<crate::hud_edit_mode::HudEditMode>>),
) {
    let pad_live = door.is_none_or(|d| *d == crate::title_screen::LaunchDoor::InYard) && !label.is_some_and(|l| l.settings_open) && !licences.is_some_and(|l| l.open) && !places.is_some_and(|p| p.open) && !edit.is_some_and(|e| e.active);
    let pad = |b| pad_live && pads.iter().any(|g| g.just_pressed(b));
    if keyboard.just_pressed(soft_play_bindings::ALLOCATE) || pad(GamepadButton::DPadUp) {
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
        vis.set_if_neq(if show { Visibility::Visible } else { Visibility::Hidden });
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
        **text = body.clone();
    }
}

fn commit_allocate(
    allocate: &mut RbeAllocateChoice,
    moments: &mut ThrivingMoments,
    guidance: &mut FirstSessionGuidance,
    rumble: &mut MessageWriter<GamepadRumbleRequest>,
    gamepads: impl IntoIterator<Item = Entity>,
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
    mut rumble: MessageWriter<GamepadRumbleRequest>,
    gamepads: Query<Entity, With<Gamepad>>,
    mut bind: ResMut<LivedHourBind>,
    bridge: Option<Res<LivedSimBridge>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    flow_q: Query<&Interaction, (Changed<Interaction>, With<AllocateFlowButton>)>,
    reserve_q: Query<&Interaction, (Changed<Interaction>, With<AllocateReserveButton>)>,
    pads: Query<&Gamepad>,
    (door, label, licences, places, edit): (Option<Res<crate::title_screen::LaunchDoor>>, Option<Res<crate::title_screen::HouseLabel>>, Option<Res<crate::licences_screen::LicencesScreen>>, Option<Res<crate::hex_travel::PlacesPlate>>, Option<Res<crate::hud_edit_mode::HudEditMode>>),
) {
    if !allocate.panel_open {
        return;
    }
    let pad_live = door.is_none_or(|d| *d == crate::title_screen::LaunchDoor::InYard) && !label.is_some_and(|l| l.settings_open) && !licences.is_some_and(|l| l.open) && !places.is_some_and(|p| p.open) && !edit.is_some_and(|e| e.active);
    let pad = |b| pad_live && pads.iter().any(|g| g.just_pressed(b));
    let now = time.elapsed_secs_f64();
    let path = if keyboard.just_pressed(KeyCode::Digit1) || pad(GamepadButton::DPadLeft) {
        Some(AllocatePath::FlowOutward)
    } else if keyboard.just_pressed(KeyCode::Digit2) || pad(GamepadButton::DPadRight) {
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
            gamepads.iter(),
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
                gamepads.iter(),
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
                gamepads.iter(),
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

    /// CARD HUD-ANCHOR-REGISTRY-1 — Allocate sits on ALLOCATE_DOCK.
    #[test]
    fn allocate_panel_lands_on_allocate_dock() {
        use crate::hud_anchor_registry::{ALLOCATE_DOCK, ID_ALLOCATE};

        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_allocate_panel);
        app.update();
        let mut query = app
            .world_mut()
            .query_filtered::<&Node, With<AllocatePanelRoot>>();
        let style = query.single(app.world()).unwrap().clone();
        assert_eq!(style.bottom, ALLOCATE_DOCK.bottom());
        assert_eq!(style.right, ALLOCATE_DOCK.right());
        assert_eq!(style.left, Val::Auto);
        assert_eq!(style.width, Val::Px(ALLOCATE_DOCK.occupant(ID_ALLOCATE).width));
        assert_eq!(style.margin, UiRect::default());
        assert_eq!(style.padding, UiRect::all(Val::Px(16.0)));
        assert_eq!(style.border, UiRect::all(Val::Px(1.5)));
        let mut text = app
            .world_mut()
            .query_filtered::<&TextFont, With<AllocateBodyText>>();
        assert_eq!(text.single(app.world()).unwrap().font_size, FontSize::Px(15.0 / 1.2));
    }
    fn hp2_ui(w: u32, h: u32, s: f32) -> App {
        let mut app = App::new(); app.add_plugins((MinimalPlugins, bevy::asset::AssetPlugin::default(), bevy::input::InputPlugin, bevy::text::TextPlugin, bevy::ui::UiPlugin, crate::ui_font::UiFontPlugin)).init_asset::<Image>().init_asset::<TextureAtlasLayout>().add_message::<bevy::picking::pointer::PointerInput>().add_message::<bevy::picking::backend::PointerHits>().init_resource::<bevy::picking::hover::HoverMap>().init_resource::<bevy::picking::events::PointerState>().add_systems(Startup, spawn_allocate_panel);
        app.world_mut().spawn((Camera2d, bevy::ui::IsDefaultUiCamera, Camera { computed: bevy::camera::ComputedCameraValues { target_info: Some(bevy::camera::RenderTargetInfo { physical_size: UVec2::new(w, h), scale_factor: s }), ..default() }, ..default() })); app
    }
    fn hp2_root(app: &mut App) -> Entity { let w = app.world_mut(); w.query_filtered::<Entity, With<AllocatePanelRoot>>().single(w).unwrap() }
    #[test] fn hud_plate_hp2_a_allocate_carries_sliced_frame() {
        let mut app = App::new(); app.add_plugins(MinimalPlugins).add_systems(Startup, spawn_allocate_panel).update(); let root = hp2_root(&mut app);
        assert_eq!(app.world().get::<ImageNode>(root).cloned().map(|i| (i.image.id(), i.visual_box, i.image_mode)), Some((crate::ui_frame::FRAME_IMAGE.id(), VisualBox::PaddingBox, NodeImageMode::Sliced(crate::ui_frame::frame_slicer()))), "HP2-a allocate frame missing");
    }
    #[test] fn hud_plate_hp2_b_allocate_text_has_no_image() {
        let mut app = App::new(); app.add_plugins(MinimalPlugins).add_systems(Startup, spawn_allocate_panel).update(); let w = app.world_mut();
        assert_eq!(w.query_filtered::<Option<&ImageNode>, With<AllocateBodyText>>().iter(w).map(|i| i.is_some()).collect::<Vec<_>>(), vec![false], "HP2-b allocate");
    }
    #[test] fn hud_plate_hp2_c_allocate_layout_matches_without_image() {
        for (w, h, s) in [(1280u32, 720u32, 1f32), (2560, 1440, 2.)] {
            let mut app = hp2_ui(w, h, s); (app.update(), app.update()); let root = hp2_root(&mut app);
            let at = |app: &App| (app.world().get::<ComputedNode>(root).unwrap().size(), app.world().get::<bevy::ui::UiGlobalTransform>(root).unwrap().translation); let framed = at(&app);
            app.world_mut().entity_mut(root).remove::<ImageNode>(); app.update(); assert_eq!(at(&app), framed, "HP2-c allocate {w}x{h}");
        }
    }
    #[test] fn hud_plate_hp2_d_allocate_corner_is_one_at_spawned_font() { let mut app = hp2_ui(1280, 720, 1.); (app.update(), app.update()); let root = hp2_root(&mut app); let n = app.world().get::<ComputedNode>(root).unwrap(); let pad = n.padding_box().size() * n.inverse_scale_factor;
        assert!(pad.x >= 24. && pad.y >= 24. && (pad / crate::ui_frame::FRAME_PX as f32).min_element().min(crate::ui_frame::frame_slicer().max_corner_scale) == 1., "HP2-d allocate {pad}");
    }

    /// PANEL-VIS-NEQ-1 PV12–PV15. Last tape of (Visibility, changed) on AllocatePanelRoot; settle 2 then clear.
    #[derive(Resource, Default)] struct PvTape(Vec<(Visibility, bool)>);
    fn pv_tape(q: Query<Ref<Visibility>, With<AllocatePanelRoot>>, mut t: ResMut<PvTape>) { t.0.extend(q.iter().map(|v| (*v, v.is_changed()))); }
    fn pv_run(app: &mut App, n: usize) -> Vec<(Visibility, bool)> { for _ in 0..n { app.update(); } std::mem::take(&mut app.world_mut().resource_mut::<PvTape>().0) }
    fn pv_settle(mut app: App) -> App { app.update(); app.update(); app.world_mut().resource_mut::<PvTape>().0.clear(); app }
    fn pv_set(app: &mut App, v: Visibility) { let e = app.world_mut().query_filtered::<Entity, With<AllocatePanelRoot>>().single(app.world()).unwrap(); *app.world_mut().get_mut::<Visibility>(e).unwrap() = v; }
    const PV: Visibility = Visibility::Visible;
    const PH: Visibility = Visibility::Hidden;
    fn pv_app(open: bool, hud: Option<(crate::hud_presets::HudPresetId, crate::title_screen::LaunchDoor)>) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).insert_resource(RbeAllocateChoice { panel_open: open, ..Default::default() }).init_resource::<PvTape>().add_systems(Startup, spawn_allocate_panel).add_systems(Update, update_allocate_visibility).add_systems(Last, pv_tape);
        if let Some((id, door)) = hud { app.add_plugins(crate::hud_presets::HudLayoutPlugin).insert_resource(crate::hud_presets::ActiveHudPreset { id: Some(id) }).insert_resource(door).init_resource::<crate::hex_travel::PlacesPlate>(); }
        pv_settle(app)
    }
    fn pv_open(app: &mut App, open: bool) { app.world_mut().resource_mut::<RbeAllocateChoice>().panel_open = open; }
    #[test] fn panel_vis_neq_1_pv12_allocate_closed_steady_unmarked() { let mut app = pv_app(false, None); assert_eq!(pv_run(&mut app, 3), vec![(PH, false); 3], "PV12 allocate closed steady hidden"); }
    #[test] fn panel_vis_neq_1_pv13_allocate_open_close_once_each() {
        let mut app = pv_app(false, None); pv_open(&mut app, true);
        assert_eq!(pv_run(&mut app, 2), vec![(PV, true), (PV, false)], "PV13 allocate opens: Visible once, then quiet");
        pv_open(&mut app, false);
        assert_eq!(pv_run(&mut app, 2), vec![(PH, true), (PH, false)], "PV13 allocate closes: Hidden once, then quiet");
    }
    #[test] fn panel_vis_neq_1_pv14_allocate_direct_hidden_restored_once() { let mut app = pv_app(true, None); pv_set(&mut app, PH); assert_eq!(pv_run(&mut app, 2), vec![(PV, true), (PV, false)], "PV14 allocate direct Hidden write while open: restored once, then quiet"); }
    #[test] fn panel_vis_neq_1_pv15_allocate_real_hud_layout() {
        let mut app = pv_app(true, Some((crate::hud_presets::HudPresetId::Management, crate::title_screen::LaunchDoor::InYard)));
        assert_eq!(pv_run(&mut app, 2), vec![(PV, false); 2], "PV15 Management InYard open: steady Visible, quiet");
        app.insert_resource(crate::title_screen::HouseLabel { house: shared::house_name::HouseName::default(), persist_present: false, hour_two_held: false, book_held: false, settings_open: true, draft: String::new(), naming_offered: false, seals_offered: false });
        assert_eq!(pv_run(&mut app, 2), vec![(PH, true); 2], "PV15 settings open (R5 pause): Hidden, changed every frame on main and branch alike");
    }
    fn alloc_label(open: bool) -> crate::title_screen::HouseLabel { crate::title_screen::HouseLabel { house: shared::house_name::HouseName::default(), persist_present: false, hour_two_held: false, book_held: false, settings_open: open, draft: String::new(), naming_offered: false, seals_offered: false } }
    fn alloc_app(open: bool, with_pad: bool) -> (App, Entity) {
        use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent};
        let mut bind = test_bind(); assert!(matches!(bind.tend(1), TendResult::Taken { .. }) && matches!(bind.tend(2), TendResult::Taken { .. }), "satchel of 2");
        let mut app = App::new(); app.add_plugins((MinimalPlugins, bevy::input::InputPlugin)).insert_resource(bind).init_resource::<ThrivingMoments>().init_resource::<FirstSessionGuidance>().add_message::<GamepadRumbleRequest>().add_plugins(RbeAllocateChoicePlugin);
        let pad = if with_pad { let pad = app.world_mut().spawn_empty().id(); app.world_mut().write_message(GamepadConnectionEvent::new(pad, GamepadConnection::Connected { name: "pad".into(), vendor_id: None, product_id: None })); pad } else { Entity::PLACEHOLDER };
        crate::test_env::single_thread(&mut app);
        app.update(); { let mut choice = app.world_mut().resource_mut::<RbeAllocateChoice>(); choice.panel_open = open; choice.eligible = true; choice.surplus_signal = 4.0; choice.choices_made = 0; choice.last_choice = None; } (app, pad)
    }
    fn pad_set(app: &mut App, pad: Entity, button: GamepadButton, value: f32) { use bevy::input::gamepad::{RawGamepadButtonChangedEvent, RawGamepadEvent}; app.world_mut().write_message(RawGamepadEvent::Button(RawGamepadButtonChangedEvent::new(pad, button, value))); app.update(); }
    fn pad_hold(app: &mut App, pad: Entity, button: GamepadButton) { for value in [1.0, 1.0, 0.0] { pad_set(app, pad, button, value); } }
    fn key_tap(app: &mut App, key_code: KeyCode, logical_key: bevy::input::keyboard::Key) { use bevy::input::{keyboard::KeyboardInput, ButtonState}; app.world_mut().write_message(KeyboardInput { key_code, logical_key, state: ButtonState::Pressed, repeat: false, text: None, window: Entity::PLACEHOLDER }); app.update(); }
    fn arm_gate(app: &mut App, kind: u8, on: bool) {
        use crate::title_screen::LaunchDoor;
        match kind {
            0 => if app.world().get_resource::<crate::title_screen::HouseLabel>().is_none() { app.insert_resource(alloc_label(on)); } else { app.world_mut().resource_mut::<crate::title_screen::HouseLabel>().settings_open = on; }
            1 => if app.world().get_resource::<crate::licences_screen::LicencesScreen>().is_none() { let mut screen = crate::licences_screen::LicencesScreen::default(); screen.open = on; app.insert_resource(screen); } else { app.world_mut().resource_mut::<crate::licences_screen::LicencesScreen>().open = on; }
            2 => if app.world().get_resource::<crate::hex_travel::PlacesPlate>().is_none() { let mut plate = crate::hex_travel::PlacesPlate::default(); plate.open = on; app.insert_resource(plate); } else { app.world_mut().resource_mut::<crate::hex_travel::PlacesPlate>().open = on; }
            3 => if app.world().get_resource::<crate::hud_edit_mode::HudEditMode>().is_none() { let mut edit = crate::hud_edit_mode::HudEditMode::default(); edit.active = on; app.insert_resource(edit); } else { app.world_mut().resource_mut::<crate::hud_edit_mode::HudEditMode>().active = on; }
            _ => { app.insert_resource(if on { [LaunchDoor::Title, LaunchDoor::NameHouse, LaunchDoor::HouseDress][(kind - 4) as usize] } else { LaunchDoor::InYard }); }
        }
    }
    #[test] fn pad_tab_alloc_a1_dpad_up_toggles() {
        let (mut app, pad) = alloc_app(false, true); pad_hold(&mut app, pad, GamepadButton::DPadUp);
        assert!(app.world().resource::<RbeAllocateChoice>().panel_open, "A1 DPadUp opens"); pad_hold(&mut app, pad, GamepadButton::DPadUp);
        assert!(!app.world().resource::<RbeAllocateChoice>().panel_open, "A1 DPadUp again closes");
    }
    #[test] fn pad_tab_alloc_a2_dpad_left_flow_right_reserve() {
        let (mut app, pad) = alloc_app(true, true); pad_hold(&mut app, pad, GamepadButton::DPadLeft);
        assert_eq!(app.world().resource::<LivedHourBind>().hour.allocation.flow, 1, "A2 DPadLeft flow +1"); assert_eq!(app.world().resource::<RbeAllocateChoice>().last_choice, Some(AllocatePath::FlowOutward), "A2 DPadLeft is FlowOutward");
        let (mut app, pad) = alloc_app(true, true); pad_hold(&mut app, pad, GamepadButton::DPadRight);
        assert_eq!(app.world().resource::<LivedHourBind>().hour.allocation.reserve, 1, "A2 DPadRight reserve +1"); assert_eq!(app.world().resource::<RbeAllocateChoice>().last_choice, Some(AllocatePath::StewardReserve), "A2 DPadRight is StewardReserve");
    }
    #[test] fn pad_tab_alloc_a3_held_dpad_left_commits_once() {
        let (mut app, pad) = alloc_app(true, true); pad_hold(&mut app, pad, GamepadButton::DPadLeft);
        assert_eq!(app.world().resource::<RbeAllocateChoice>().choices_made, 1, "A3 DPadLeft hold commits once");
    }
    #[test] fn pad_tab_alloc_a4_gates_block_then_clear() {
        for kind in 0..7 {
            let (mut app, pad) = alloc_app(false, true); arm_gate(&mut app, kind, true); pad_set(&mut app, pad, GamepadButton::DPadUp, 1.0);
            assert!(!app.world().resource::<RbeAllocateChoice>().panel_open, "A4 gate {kind} DPadUp stays closed");
            pad_set(&mut app, pad, GamepadButton::DPadUp, 0.0); arm_gate(&mut app, kind, false); pad_set(&mut app, pad, GamepadButton::DPadUp, 1.0);
            assert!(app.world().resource::<RbeAllocateChoice>().panel_open, "A4 gate {kind} clear DPadUp opens");
            let (mut app, pad) = alloc_app(true, true); arm_gate(&mut app, kind, true); pad_set(&mut app, pad, GamepadButton::DPadLeft, 1.0);
            assert_eq!(app.world().resource::<LivedHourBind>().hour.allocation.flow, 0, "A4 gate {kind} DPadLeft flow unchanged"); assert_eq!(app.world().resource::<RbeAllocateChoice>().choices_made, 0, "A4 gate {kind} DPadLeft choices unchanged");
            pad_set(&mut app, pad, GamepadButton::DPadLeft, 0.0); arm_gate(&mut app, kind, false); pad_set(&mut app, pad, GamepadButton::DPadLeft, 1.0);
            assert_eq!(app.world().resource::<LivedHourBind>().hour.allocation.flow, 1, "A4 gate {kind} clear DPadLeft commits");
        }
    }
    #[test] fn pad_tab_alloc_a5_same_frame_once() {
        use bevy::input::gamepad::{RawGamepadButtonChangedEvent, RawGamepadEvent}; use bevy::input::{keyboard::{Key, KeyboardInput}, ButtonState};
        let (mut app, pad) = alloc_app(false, true);
        app.world_mut().write_message(KeyboardInput { key_code: KeyCode::KeyR, logical_key: Key::Character("r".into()), state: ButtonState::Pressed, repeat: false, text: None, window: Entity::PLACEHOLDER }); app.world_mut().write_message(RawGamepadEvent::Button(RawGamepadButtonChangedEvent::new(pad, GamepadButton::DPadUp, 1.0))); app.update();
        assert!(app.world().resource::<RbeAllocateChoice>().panel_open, "A5 R and DPadUp in one frame stay open once");
        let (mut app, pad) = alloc_app(true, true);
        app.world_mut().write_message(KeyboardInput { key_code: KeyCode::Digit1, logical_key: Key::Character("1".into()), state: ButtonState::Pressed, repeat: false, text: None, window: Entity::PLACEHOLDER }); app.world_mut().write_message(RawGamepadEvent::Button(RawGamepadButtonChangedEvent::new(pad, GamepadButton::DPadLeft, 1.0))); app.update();
        assert_eq!(app.world().resource::<RbeAllocateChoice>().choices_made, 1, "A5 Digit1 and DPadLeft in one frame commit once");
    }
    #[test] fn pad_tab_alloc_a6_keyboard_under_settings_open() {
        use bevy::input::keyboard::Key;
        let (mut app, _) = alloc_app(true, false); app.insert_resource(alloc_label(true)); key_tap(&mut app, KeyCode::Escape, Key::Escape);
        assert!(!app.world().resource::<RbeAllocateChoice>().panel_open, "A6 Esc closes under settings_open"); key_tap(&mut app, KeyCode::KeyR, Key::Character("r".into()));
        assert!(app.world().resource::<RbeAllocateChoice>().panel_open, "A6 R opens under settings_open"); key_tap(&mut app, KeyCode::Digit1, Key::Character("1".into()));
        assert_eq!(app.world().resource::<LivedHourBind>().hour.allocation.flow, 1, "A6 Digit1 commits under settings_open");
    }
    #[test] fn pad_tab_alloc_a7_inert_and_keyboard_without_pad() {
        use bevy::input::keyboard::Key;
        let (mut app, pad) = alloc_app(false, true); pad_hold(&mut app, pad, GamepadButton::DPadLeft); pad_hold(&mut app, pad, GamepadButton::DPadRight);
        assert!(!app.world().resource::<RbeAllocateChoice>().panel_open, "A7 closed panel stays closed"); assert_eq!(app.world().resource::<LivedHourBind>().hour.allocation.flow, 0, "A7 no flow"); assert_eq!(app.world().resource::<LivedHourBind>().hour.allocation.reserve, 0, "A7 no reserve"); assert_eq!(app.world().resource::<RbeAllocateChoice>().choices_made, 0, "A7 no commit");
        for button in [GamepadButton::DPadDown, GamepadButton::Select, GamepadButton::West, GamepadButton::North] { pad_hold(&mut app, pad, button); }
        assert!(!app.world().resource::<RbeAllocateChoice>().panel_open, "A7 DPadDown Select West North do not toggle");
        let (mut app, _) = alloc_app(false, false); key_tap(&mut app, KeyCode::KeyR, Key::Character("r".into()));
        assert!(app.world().resource::<RbeAllocateChoice>().panel_open, "A7 R opens with no pad"); key_tap(&mut app, KeyCode::Digit1, Key::Character("1".into()));
        assert_eq!(app.world().resource::<LivedHourBind>().hour.allocation.flow, 1, "A7 Digit1 commits with no pad");
    }
}
