/*!
 * Human Inventory — v22.10.0 + S3 Pause face (v23.2.52)
 *
 * Watch reads cycles: ~ means vitality wants to go home.
 * Companion word when trust or a ride is live.
 * I satchel shows Pause/Ledger face: House · week · lethal if declared.
 * Quiet copy: abundance is held, not hoarded.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use shared::pause_ledger_face::face_from;
use shared::temper::{lumen_slots, TemperedItem, ToolTier, WardKind};

use crate::companion_bond::CompanionBond;
use crate::hud_anchor_registry::{HudSlab, PICKUP, WATCH};
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::harvest_feel::SoftRbePool;
use crate::hex_travel::HexTravelState;
use crate::human_soft_panels::HumanSoftPanels;
use crate::lived_hour_bind::LivedHourBind;
use crate::local_settings::LocalSettingsState;
use crate::title_screen::{HouseLabel, TITLE_PLATE_BG, TITLE_BORDER, TITLE_TEXT_PRIMARY, TITLE_TEXT_SECONDARY};
use crate::ui_above_world::{LivedUiPlate, LIVED_UI_Z_LEDGER};
use crate::living_freshness::LivingFreshness;
use crate::rbe_allocate_choice::RbeAllocateChoice;
use crate::soft_play_bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SatchelSlot {
    Vitality = 0,
    Harmony = 1,
    Joy = 2,
}

impl SatchelSlot {
    pub fn name(self) -> &'static str {
        match self {
            SatchelSlot::Vitality => "Vitality",
            SatchelSlot::Harmony => "Harmony",
            SatchelSlot::Joy => "Joy",
        }
    }
}

#[derive(Resource, Debug)]
pub struct HumanInventory {
    pub open: bool,
    pub selected: SatchelSlot,
    pub pickup_until: f64,
    pub pickup_line: String,
    pub last_seen_harvests: u32,
    pub first_open_lived: bool,
}

impl Default for HumanInventory {
    fn default() -> Self {
        Self {
            open: false,
            selected: SatchelSlot::Vitality,
            pickup_until: 0.0,
            pickup_line: String::new(),
            last_seen_harvests: 0,
            first_open_lived: false,
        }
    }
}

/// CARD PICKUP-STRIP-SCALE-1 — always-on satchel strip spawn size.
const WATCH_STRIP_FONT_BASE: f32 = 13.0;
/// CARD PICKUP-STRIP-SCALE-1 — pickup flash spawn size.
const PICKUP_FLASH_FONT_BASE: f32 = 16.0;

#[derive(Component)]
struct WatchStripRoot;
#[derive(Component)]
struct WatchStripText;
/// Watch-strip or pickup-flash base px. Plate type stays on [`SatchelFontBase`].
#[derive(Component, Clone, Copy, Debug, PartialEq)]
struct StripFlashFontBase(f32);
#[derive(Component)]
struct SatchelRoot;
#[derive(Component)]
struct SatchelBody;
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct SatchelFontBase(pub f32);
#[derive(Component)]
struct PickupFlashRoot;
#[derive(Component)]
struct PickupFlashText;

pub struct HumanInventoryPlugin;

impl Plugin for HumanInventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HumanInventory>()
            .add_systems(Startup, spawn_inventory_surfaces)
            .add_systems(
                Update,
                (
                    toggle_satchel,
                    slot_select_when_open,
                    notice_pickup,
                    feed_allocate_surplus,
                    update_watch_strip,
                    update_satchel,
                    update_pickup_flash,
                    scale_satchel_fonts,
                    scale_strip_and_flash_fonts,
                ),
            );
    }
}

fn spawn_inventory_surfaces(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                node: Node {
                    position_type: PositionType::Absolute,
                    bottom: WATCH.bottom(),
                    left: WATCH.left(),
                    width: Val::Px(340.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: TITLE_PLATE_BG.into(),
                border_color: TITLE_BORDER.into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            WatchStripRoot,
            HudSlab(WATCH.id),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(""),
TextFont { font_size: WATCH_STRIP_FONT_BASE / 1.2, ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                WatchStripText,
                StripFlashFontBase(WATCH_STRIP_FONT_BASE),
            ));
        });

    commands
        .spawn((
            NodeBundle {
                node: Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Percent(22.0),
                    left: Val::Px(16.0),
                    width: Val::Px(300.0),
                    padding: UiRect::all(Val::Px(14.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    border: UiRect::all(Val::Px(1.5)),
                    ..default()
                },
                // Opaque I/Ledger face — Title contrast law on soft GPU.
                background_color: TITLE_PLATE_BG.into(),
                border_color: TITLE_BORDER.into(),
                visibility: Visibility::Hidden,
                                ..default()
            },
GlobalZIndex(LIVED_UI_Z_LEDGER),
            SatchelRoot,
            LivedUiPlate,
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new("SATCHEL"),
TextFont { font_size: 14.0 / 1.2, ..default() },
TextColor(TITLE_TEXT_SECONDARY),
),
                SatchelFontBase(14.0),
            ));
            p.spawn((
                (
Text::new(""),
TextFont { font_size: 13.5 / 1.2, ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                SatchelBody,
                SatchelFontBase(13.5),
            ));
            p.spawn((
                (
Text::new("I close · 1–3 highlight · R allocate surplus"),
TextFont { font_size: 11.0 / 1.2, ..default() },
TextColor(TITLE_TEXT_SECONDARY),
),
                SatchelFontBase(11.0),
            ));
        });

    commands
        .spawn((
            NodeBundle {
                node: Node {
                    position_type: PositionType::Absolute,
                    top: PICKUP.top(),
                    left: PICKUP.left(),
                    width: Val::Px(360.0),
                    margin: PICKUP.margin(),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: TITLE_PLATE_BG.into(),
                border_color: TITLE_BORDER.into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            PickupFlashRoot,
            HudSlab(PICKUP.id),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(""),
TextFont { font_size: PICKUP_FLASH_FONT_BASE / 1.2, ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                PickupFlashText,
                StripFlashFontBase(PICKUP_FLASH_FONT_BASE),
            ));
        });
}

fn toggle_satchel(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut inv: ResMut<HumanInventory>,
    harvest: Res<FirstHarvestEpiphany>,
) {
    if !keyboard.just_pressed(soft_play_bindings::INVENTORY) {
        return;
    }
    inv.open = !inv.open;
    if inv.open {
        inv.first_open_lived = true;
        if harvest.harvests_this_session == 0 {
            inv.pickup_line = "Satchel is ready — harvest a glow to fill it".into();
            inv.pickup_until = 9999.0;
        }
    }
}

fn slot_select_when_open(
    keyboard: Res<ButtonInput<KeyCode>>,
    panels: Res<HumanSoftPanels>,
    allocate: Res<RbeAllocateChoice>,
    mut inv: ResMut<HumanInventory>,
) {
    if !inv.open || panels.realm_open || allocate.panel_open {
        return;
    }
    if keyboard.just_pressed(KeyCode::Digit1) || keyboard.just_pressed(KeyCode::Digit1) {
        inv.selected = SatchelSlot::Vitality;
    } else if keyboard.just_pressed(KeyCode::Digit2) || keyboard.just_pressed(KeyCode::Digit2) {
        inv.selected = SatchelSlot::Harmony;
    } else if keyboard.just_pressed(KeyCode::Digit3) || keyboard.just_pressed(KeyCode::Digit3) {
        inv.selected = SatchelSlot::Joy;
    }
}

/// CARD FLESH-SATCHEL-LINE — the one pickup flash may name the Place stood in.
/// Absent `HexTravelState` keeps the walked line word for word.
fn satchel_grew_line(last_credit: f32, place: Option<&str>) -> String {
    match place {
        Some(place) => format!("+{:.1} vitality  ·  satchel grew in {place}", last_credit),
        None => format!("+{:.1} vitality  ·  satchel grew", last_credit),
    }
}

fn notice_pickup(
    pool: Res<SoftRbePool>,
    time: Res<Time>,
    mut inv: ResMut<HumanInventory>,
    travel: Option<Res<HexTravelState>>,
) {
    if pool.harvests == inv.last_seen_harvests {
        return;
    }
    inv.last_seen_harvests = pool.harvests;
    inv.pickup_until = time.elapsed_secs_f64() + 1.8;
    let place = travel.as_ref().map(|state| state.chip_name());
    inv.pickup_line = satchel_grew_line(pool.last_credit, place);
}

fn feed_allocate_surplus(
    pool: Res<SoftRbePool>,
    mut allocate: ResMut<RbeAllocateChoice>,
    mut last: Local<u32>,
) {
    if pool.harvests == *last {
        return;
    }
    *last = pool.harvests;
    if pool.last_credit > 0.0 {
        allocate.note_surplus(pool.last_credit);
    }
}

fn update_watch_strip(
    pool: Res<SoftRbePool>,
    harvest: Res<FirstHarvestEpiphany>,
    fresh: Option<Res<LivingFreshness>>,
    bond: Option<Res<CompanionBond>>,
    mut root: Query<&mut Visibility, With<WatchStripRoot>>,
    mut text_q: Query<&mut Text, With<WatchStripText>>,
) {
    let show = harvest.first_harvest_lived || pool.harvests > 0;
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    let aging = fresh.map(|f| f.age > 24.0 && pool.vitality >= 0.45).unwrap_or(false);
    let vmark = if aging { "~" } else { "" };
    let companion = match bond {
        Some(b) if b.mounted => "ride",
        Some(b) if b.nearby && b.trust >= 0.55 => "E ride",
        Some(b) if b.trust >= 0.32 => "walk",
        _ => "",
    };
    let line = if companion.is_empty() {
        format!(
            "vitality {:.1}{}  harmony {:.1}  joy {:.1}\nI satchel",
            pool.vitality, vmark, pool.harmony, pool.joy
        )
    } else {
        format!(
            "vitality {:.1}{}  harmony {:.1}  joy {:.1}\n{}",
            pool.vitality, vmark, pool.harmony, pool.joy, companion
        )
    };
    for mut text in &mut text_q {
        if text.as_str() != line {
            **text = line.clone();
        }
    }
}


/// MERCY_TEMPER T3 — tool name on the existing I satchel face.
fn tool_tier_satchel_name(tier: ToolTier) -> &'static str {
    match tier {
        ToolTier::Hands => "Hands",
        ToolTier::TendHook => "Tend Hook",
        ToolTier::LaneCrateMk1 => "Lane Crate Mk1",
        ToolTier::ClimatePick => "Climate Pick",
        ToolTier::MendSpindle => "Mend Spindle",
        ToolTier::HarmonyLoom => "Harmony Loom",
    }
}

fn ward_kind_satchel_name(ward: WardKind) -> &'static str {
    match ward {
        WardKind::Shell => "Shell",
        WardKind::Current => "Current",
        WardKind::Reserve => "Reserve",
        WardKind::Dawn => "Dawn",
        WardKind::Book => "Book",
    }
}

/// Satchel temper line: `Tend Hook +N · Lumen a/b · Ward: Shell` (or resting).
/// Hidden by caller when no tempered item is present — keeps Hour 1–3 clean.
pub fn satchel_temper_display(item: &TemperedItem) -> String {
    let name = tool_tier_satchel_name(item.tier);
    if item.resting {
        return format!("{name} +{} · resting", item.temper);
    }
    let slots = lumen_slots(item.temper);
    let seated = item.lumens.iter().filter(|l| l.ward.is_some()).count();
    let mut line = format!("{name} +{} · Lumen {}/{}", item.temper, seated, slots);
    if let Some(ward) = item.lumens.iter().find_map(|l| l.ward) {
        line.push_str(&format!(" · Ward: {}", ward_kind_satchel_name(ward)));
    }
    line
}

/// CARD UI-SCALE-SLABS-2 — satchel plate type follows `text_scale` (11–22).
pub fn satchel_font_px(base: f32, text_scale: f32) -> f32 {
    (base * text_scale).clamp(11.0, 22.0)
}

/// CARD PICKUP-STRIP-SCALE-1 — watch strip and pickup flash follow the same step.
/// Plate bases stay on [`scale_satchel_fonts`]. Width and padding stay fixed.
fn scale_strip_and_flash_fonts(
    settings: Option<Res<LocalSettingsState>>,
    mut q: Query<(&StripFlashFontBase, &mut TextFont)>,
) {
    let scale = match &settings {
        Some(state) => state.inner.text_scale,
        None => 1.0,
    };
    for (base, mut font) in &mut q {
        let px = satchel_font_px(base.0, scale);
        if (font.font_size - px / 1.2).abs() > 0.01 {
            font.font_size = px / 1.2;
        }
    }
}

fn scale_satchel_fonts(
    settings: Option<Res<LocalSettingsState>>,
    mut q: Query<(&SatchelFontBase, &mut TextFont)>,
) {
    let scale = match &settings {
        Some(state) => state.inner.text_scale,
        None => 1.0,
    };
    for (base, mut font) in &mut q {
        let px = satchel_font_px(base.0, scale);
        if (font.font_size - px / 1.2).abs() > 0.01 {
            font.font_size = px / 1.2;
        }
    }
}

fn update_satchel(
    inv: Res<HumanInventory>,
    pool: Res<SoftRbePool>,
    bind: Res<LivedHourBind>,
    house_label: Res<HouseLabel>,
    yard: Option<Res<crate::fabricator::FabricatorYard>>,
    mut root: Query<&mut Visibility, With<SatchelRoot>>,
    mut body: Query<&mut Text, With<SatchelBody>>,
) {
    for mut vis in &mut root {
        *vis = if inv.open {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if !inv.open {
        return;
    }
    let mark = |slot: SatchelSlot| {
        if inv.selected == slot {
            ">"
        } else {
            " "
        }
    };
    // S3 Pause face on existing satchel - no second HUD.
    let face = face_from(
        &house_label.house,
        &bind.week,
        bind.standing.declared_lethal,
    );
    // T3: one temper line on the existing I face when a tempered tool exists.
    let temper_block = yard
        .as_ref()
        .and_then(|y| y.fab.last_tempered.as_ref())
        .map(|item| format!("\n\n{}", satchel_temper_display(item)))
        .unwrap_or_default();
    let body_line = format!(
        "{face}

abundance is held, not hoarded

{} [1] Vitality   {:.1}
{} [2] Harmony    {:.1}
{} [3] Joy        {:.1}

Harvests {}{temper_block}",
        mark(SatchelSlot::Vitality),
        pool.vitality,
        mark(SatchelSlot::Harmony),
        pool.harmony,
        mark(SatchelSlot::Joy),
        pool.joy,
        pool.harvests
    );
    for mut text in &mut body {
        if text.as_str() != body_line {
            **text = body_line.clone();
        }
    }
}

fn update_pickup_flash(
    inv: Res<HumanInventory>,
    time: Res<Time>,
    mut root: Query<&mut Visibility, With<PickupFlashRoot>>,
    mut text_q: Query<&mut Text, With<PickupFlashText>>,
) {
    let now = time.elapsed_secs_f64();
    let show = now < inv.pickup_until && !inv.pickup_line.is_empty() && inv.pickup_until < 9000.0;
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if show {
        for mut text in &mut text_q {
            if text.as_str() != inv.pickup_line {
            **text = inv.pickup_line.clone();
        }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::house_name::HouseName;
    use shared::pause_ledger_face::{face_from, face_is_steward_honest, LETHAL_DECLARED_LINE};
    use shared::temper::temper_copy_is_honest;
    use shared::week_audit::WeekAudit;

    /// CARD HUD-ANCHOR-REGISTRY-2B — Watch and Pickup read the registry.
    /// Satchel stays `bottom: Val::Percent(22.0)` and is not a joiner.
    #[test]
    fn watch_and_pickup_styles_match_coded_places_satchel_stays() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_inventory_surfaces);
        app.update();

        let mut watch_q = app
            .world_mut()
            .query_filtered::<&Node, With<WatchStripRoot>>();
        let watch = watch_q.single(app.world()).clone();
        let watch_coded = Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(16.0),
            left: Val::Px(16.0),
            width: Val::Px(340.0),
            padding: UiRect::all(Val::Px(10.0)),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };
        assert_eq!(watch, watch_coded);
        assert_eq!(watch.margin, UiRect::default());
        assert_eq!(watch.bottom, WATCH.bottom());
        assert_eq!(watch.left, WATCH.left());
        assert_eq!(watch.width, Val::Px(WATCH.width));

        let mut satchel_q = app
            .world_mut()
            .query_filtered::<&Node, With<SatchelRoot>>();
        let satchel = satchel_q.single(app.world()).clone();
        let satchel_coded = Node {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(22.0),
            left: Val::Px(16.0),
            width: Val::Px(300.0),
            padding: UiRect::all(Val::Px(14.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            border: UiRect::all(Val::Px(1.5)),
            ..default()
        };
        assert_eq!(satchel, satchel_coded);
        assert!(crate::hud_anchor_registry::CODED_JOINERS
            .iter()
            .all(|place| place.id != "Satchel"));

        let mut pickup_q = app
            .world_mut()
            .query_filtered::<&Node, With<PickupFlashRoot>>();
        let pickup = pickup_q.single(app.world()).clone();
        let pickup_coded = Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(38.0),
            left: Val::Percent(50.0),
            width: Val::Px(360.0),
            margin: UiRect::left(Val::Px(-180.0)),
            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
            justify_content: JustifyContent::Center,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };
        assert_eq!(pickup, pickup_coded);
        assert_eq!(pickup.top, PICKUP.top());
        assert_eq!(pickup.left, PICKUP.left());
        assert_eq!(pickup.margin, PICKUP.margin());
        assert_eq!(pickup.width, Val::Px(PICKUP.width));
    }

    #[test]
    fn slots_are_three() {
        assert_eq!(SatchelSlot::Vitality as u8, 0);
        assert_eq!(SatchelSlot::Joy as u8, 2);
    }

    #[test]
    fn pause_face_on_satchel_has_house_and_week() {
        let mut house = HouseName::default();
        house.confirm("Satchel Keep");
        let mut week = WeekAudit::default();
        week.sync_from_climate(5, 3);
        let face = face_from(&house, &week, false);
        assert!(face.contains("Satchel Keep"));
        assert!(face.contains("5 tons"));
        assert!(face.contains("3 restored"));
        assert!(!face.contains(LETHAL_DECLARED_LINE));
        assert!(face_is_steward_honest(&face));
        assert!(!face.to_lowercase().contains("peer"));
    }

    #[test]
    fn pause_face_lethal_gate() {
        let mut week = WeekAudit::default();
        week.sync_from_climate(1, 1);
        let quiet = face_from(&HouseName::default(), &week, false);
        assert!(!quiet.contains(LETHAL_DECLARED_LINE));
        let lethal = face_from(&HouseName::default(), &week, true);
        assert!(lethal.contains(LETHAL_DECLARED_LINE));
        assert!(lethal.contains("Unnamed House") || lethal.contains("this week"));
    }

    #[test]
    fn satchel_face_unnamed_week_zero_no_lethal() {
        let mut house = HouseName::default();
        house.skip();
        let week = WeekAudit::default();
        let face = face_from(&house, &week, false);
        assert!(face.contains("Unnamed House"));
        assert!(face.contains("0 tons"));
        assert!(face.contains("0 restored"));
        assert!(!face.contains(LETHAL_DECLARED_LINE));
        assert!(face_is_steward_honest(&face));
    }

    fn tend_hook(temper: u8, resting: bool, ward: Option<WardKind>) -> TemperedItem {
        let mut item = TemperedItem::hands(7, "stranger");
        item.tier = ToolTier::TendHook;
        item.temper = temper;
        item.resting = resting;
        let want = lumen_slots(temper) as usize;
        item.lumens = (0..want)
            .map(|i| shared::temper::Lumen {
                index: i as u8,
                ward: if i == 0 { ward } else { None },
            })
            .collect();
        item
    }

    #[test]
    fn satchel_temper_display_plus_n_lumen_ward() {
        let with_ward = tend_hook(3, false, Some(WardKind::Shell));
        let line = satchel_temper_display(&with_ward);
        assert_eq!(line, "Tend Hook +3 · Lumen 1/1 · Ward: Shell");
        assert!(temper_copy_is_honest(&line));

        let bare = tend_hook(1, false, None);
        let bare_line = satchel_temper_display(&bare);
        assert_eq!(bare_line, "Tend Hook +1 · Lumen 0/0");
        assert!(temper_copy_is_honest(&bare_line));

        let resting = tend_hook(4, true, None);
        let rest_line = satchel_temper_display(&resting);
        assert_eq!(rest_line, "Tend Hook +4 · resting");
        assert!(temper_copy_is_honest(&rest_line));
        assert!(!rest_line.to_lowercase().contains("gold"));
        assert!(!rest_line.to_lowercase().contains("mall"));
        assert!(!rest_line.to_lowercase().contains("p2w"));
    }

    /// CARD FLESH-SATCHEL-LINE — Place dresses the pickup line; no travel keeps it exact.
    #[test]
    fn flesh_satchel_line_names_chip_or_keeps_walked_line() {
        use shared::hex_travel::PlaceId;

        assert_eq!(
            satchel_grew_line(1.2, None),
            "+1.2 vitality  ·  satchel grew"
        );

        let cases = [
            (PlaceId::Sanctuary, "Sanctuary Prime"),
            (PlaceId::Heartwood, "Heartwood"),
            (PlaceId::Depths, "Depths"),
        ];
        for (id, name) in cases {
            let travel = HexTravelState { current: id };
            assert_eq!(travel.chip_name(), name);
            assert_eq!(
                satchel_grew_line(1.2, Some(travel.chip_name())),
                format!("+1.2 vitality  ·  satchel grew in {name}")
            );
        }

        let dressed = satchel_grew_line(0.4, Some("Heartwood"));
        let low = dressed.to_lowercase();
        assert!(!low.contains("gold"));
        assert!(!low.contains("market"));
        assert!(!low.contains("xp"));
        assert!(!dressed.contains("Threshold"));
    }

    /// CARD UI-SCALE-SLABS-2 — identity at 1.0, 1.35 step, clamp 11–22.
    #[test]
    fn satchel_font_px_follows_text_scale() {
        for base in [14.0_f32, 13.5, 11.0] {
            assert_eq!(satchel_font_px(base, 1.0), base);
            assert_eq!(satchel_font_px(base, 0.1), 11.0);
            assert_eq!(satchel_font_px(base, 5.0), 22.0);
        }
        assert!((satchel_font_px(14.0, 1.35) - 18.9).abs() < 1e-4);
        assert!((satchel_font_px(13.5, 1.35) - 18.225).abs() < 1e-4);
        assert!((satchel_font_px(11.0, 1.35) - 14.85).abs() < 1e-4);
    }

    /// CARD UI-SCALE-SLABS-2 — spawn stays at each plate base; one text-scale
    /// bump resizes the three satchel texts and leaves their strings alone.
    /// Peace defaults are explicit so the test does not read disk.
    #[test]
    fn satchel_plate_fonts_track_text_scale_bump() {
        use bevy::MinimalPlugins;
        use shared::local_settings::LocalSettings;

        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(LocalSettingsState {
                inner: LocalSettings::peace_defaults(),
                dirty: false,
            })
            .add_systems(Startup, spawn_inventory_surfaces)
            .add_systems(Update, scale_satchel_fonts);

        app.update();
        let spawned = satchel_font_rows(&app);
        assert_eq!(spawned.len(), 3);
        let bases: Vec<f32> = spawned.iter().map(|(base, _, _)| *base).collect();
        assert_eq!(bases, vec![11.0, 13.5, 14.0]);
        for (base, px, _) in &spawned {
            assert_eq!(*px, *base / 1.2);
        }

        app.world_mut()
            .resource_mut::<LocalSettingsState>()
            .inner
            .bump_text_scale();
        let new_scale = app
            .world()
            .resource::<LocalSettingsState>()
            .inner
            .text_scale;
        app.update();

        let after = satchel_font_rows(&app);
        assert_eq!(after.len(), 3);
        for (before, (base, px, value)) in spawned.iter().zip(after.iter()) {
            assert_eq!(before.0, *base);
            assert_eq!(*px, satchel_font_px(*base, new_scale) / 1.2);
            assert_eq!(value, &before.2);
        }
    }

    fn satchel_font_rows(app: &App) -> Vec<(f32, f32, String)> {
        let mut rows = Vec::new();
        for entity in app.world().iter_entities() {
            let Some(base) = entity.get::<SatchelFontBase>() else {
                continue;
            };
            let text = entity.get::<Text>().expect("satchel font text");
            let font = entity.get::<TextFont>().expect("satchel font");
            rows.push((base.0, font.font_size, text.as_str().to_string()));
        }
        rows.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("satchel font base"));
        rows
    }

    /// CARD PICKUP-STRIP-SCALE-1 — strip 13 and flash 16 follow the satchel
    /// text_scale step. Identity at 1.0, 1.35 step, clamp 11–22.
    /// Plate bases stay 14 / 13.5 / 11.
    #[test]
    fn strip_and_flash_font_px_follows_text_scale() {
        for base in [WATCH_STRIP_FONT_BASE, PICKUP_FLASH_FONT_BASE] {
            assert_eq!(satchel_font_px(base, 1.0), base);
            assert_eq!(satchel_font_px(base, 0.1), 11.0);
            assert_eq!(satchel_font_px(base, 5.0), 22.0);
        }
        assert!((satchel_font_px(WATCH_STRIP_FONT_BASE, 1.35) - 17.55).abs() < 1e-4);
        assert!((satchel_font_px(PICKUP_FLASH_FONT_BASE, 1.35) - 21.6).abs() < 1e-4);
        assert_eq!(satchel_font_px(14.0, 1.0), 14.0);
        assert_eq!(satchel_font_px(13.5, 1.0), 13.5);
        assert_eq!(satchel_font_px(11.0, 1.0), 11.0);
    }

    /// CARD PICKUP-STRIP-SCALE-1 — spawn stays 13 and 16; one text-scale bump
    /// resizes the strip and the flash and leaves their strings alone.
    /// The three satchel plate fonts still follow `satchel_font_px`.
    #[test]
    fn strip_and_flash_fonts_track_text_scale_bump() {
        use bevy::MinimalPlugins;
        use shared::local_settings::LocalSettings;

        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(LocalSettingsState {
                inner: LocalSettings::peace_defaults(),
                dirty: false,
            })
            .add_systems(Startup, spawn_inventory_surfaces)
            .add_systems(Update, (scale_satchel_fonts, scale_strip_and_flash_fonts));

        app.update();
        let strip = strip_flash_font_rows(&app);
        assert_eq!(strip.len(), 2);
        assert_eq!(
            strip
                .iter()
                .map(|(base, px, value, kind)| (*base, *px, value.as_str(), *kind))
                .collect::<Vec<_>>(),
            vec![
                (WATCH_STRIP_FONT_BASE, WATCH_STRIP_FONT_BASE / 1.2, "", "strip"),
                (PICKUP_FLASH_FONT_BASE, PICKUP_FLASH_FONT_BASE / 1.2, "", "flash"),
            ]
        );
        let plate = satchel_font_rows(&app);
        assert_eq!(plate.len(), 3);
        for (base, px, _) in &plate {
            assert_eq!(*px, *base / 1.2);
        }

        app.world_mut()
            .resource_mut::<LocalSettingsState>()
            .inner
            .bump_text_scale();
        let new_scale = app
            .world()
            .resource::<LocalSettingsState>()
            .inner
            .text_scale;
        app.update();

        let after = strip_flash_font_rows(&app);
        assert_eq!(after.len(), 2);
        for (before, (base, px, value, kind)) in strip.iter().zip(after.iter()) {
            assert_eq!(before.0, *base);
            assert_eq!(before.3, *kind);
            assert_eq!(*px, satchel_font_px(*base, new_scale) / 1.2);
            assert_eq!(value, &before.2);
            assert!(*px > *base / 1.2);
        }
        let plate_after = satchel_font_rows(&app);
        assert_eq!(plate_after.len(), 3);
        for (before, (base, px, value)) in plate.iter().zip(plate_after.iter()) {
            assert_eq!(before.0, *base);
            assert_eq!(*px, satchel_font_px(*base, new_scale) / 1.2);
            assert_eq!(value, &before.2);
        }

        for extreme in [0.1_f32, 5.0] {
            app.world_mut()
                .resource_mut::<LocalSettingsState>()
                .inner
                .text_scale = extreme;
            app.update();
            for (base, px, _, _) in strip_flash_font_rows(&app) {
                assert_eq!(px, satchel_font_px(base, extreme) / 1.2);
            }
            for (base, px, _) in satchel_font_rows(&app) {
                assert_eq!(px, satchel_font_px(base, extreme) / 1.2);
            }
        }
    }

    /// CARD PICKUP-STRIP-SCALE-1 — no settings resource keeps the spawn sizes.
    #[test]
    fn strip_and_flash_fonts_stay_spawn_size_without_settings() {
        use bevy::MinimalPlugins;

        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_inventory_surfaces)
            .add_systems(Update, scale_strip_and_flash_fonts);
        app.update();
        let rows = strip_flash_font_rows(&app);
        assert_eq!(
            rows.iter()
                .map(|(base, px, _, kind)| (*base, *px, *kind))
                .collect::<Vec<_>>(),
            vec![
                (WATCH_STRIP_FONT_BASE, WATCH_STRIP_FONT_BASE / 1.2, "strip"),
                (PICKUP_FLASH_FONT_BASE, PICKUP_FLASH_FONT_BASE / 1.2, "flash"),
            ]
        );
    }

    fn strip_flash_font_rows(app: &App) -> Vec<(f32, f32, String, &'static str)> {
        let mut rows = Vec::new();
        for entity in app.world().iter_entities() {
            let Some(base) = entity.get::<StripFlashFontBase>() else {
                continue;
            };
            let kind = if entity.get::<WatchStripText>().is_some() {
                "strip"
            } else if entity.get::<PickupFlashText>().is_some() {
                "flash"
            } else {
                panic!("StripFlashFontBase without strip or flash text");
            };
            let text = entity.get::<Text>().expect("strip flash font text");
            let font = entity.get::<TextFont>().expect("strip flash font");
            rows.push((base.0, font.font_size, text.as_str().to_string(), kind));
        }
        rows.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("strip flash font base"));
        rows
    }

    #[test]
    fn satchel_temper_display_absent_keeps_hour_clean() {
        // No last_tempered → helper is simply not called; empty block stays empty.
        let empty = Option::<&TemperedItem>::None;
        let block = empty
            .map(|item| format!("\n\n{}", satchel_temper_display(item)))
            .unwrap_or_default();
        assert!(block.is_empty());
    }
}
