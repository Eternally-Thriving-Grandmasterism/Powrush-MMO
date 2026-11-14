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
use shared::temper::{lumen_slots, TemperedItem, ToolTier, WardKind, NAMED_FINDS};

use crate::companion_bond::CompanionBond;
use crate::hud_anchor_registry::{HudSlab, PICKUP, WATCH};
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::harvest_feel::SoftRbePool;
use crate::hex_travel::HexTravelState;
use crate::human_soft_panels::HumanSoftPanels;
use crate::lived_hour_bind::LivedHourBind;
use crate::local_settings::LocalSettingsState;
use crate::loot_drops::{find_line, FoundFinds};
use crate::title_screen::{HouseLabel, TITLE_TEXT_PRIMARY, TITLE_TEXT_SECONDARY};
use crate::ui_kit;
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
const WATCH_STRIP_FONT_BASE: f32 = ui_kit::TYPE_SMALL;
/// CARD PICKUP-STRIP-SCALE-1 — pickup flash spawn size.
const PICKUP_FLASH_FONT_BASE: f32 = ui_kit::TYPE_CUE;

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
            (
                Node {
                    position_type: PositionType::Absolute,
                    bottom: WATCH.bottom(),
                    left: WATCH.left(),
                    width: Val::Px(340.0),
                    padding: UiRect::all(Val::Px(ui_kit::SPACE_L)),
                    border: UiRect::all(Val::Px(ui_kit::BORDER_HAIRLINE)),
                    ..default()
                },
                ui_kit::plate(),
                Visibility::Hidden,
            ),
            WatchStripRoot,
            HudSlab(WATCH.id),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(""),
ui_kit::font(WATCH_STRIP_FONT_BASE),
TextColor(TITLE_TEXT_PRIMARY),
),
                WatchStripText,
                StripFlashFontBase(WATCH_STRIP_FONT_BASE),
            ));
        });

    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Percent(22.0),
                    left: Val::Px(16.0),
                    width: Val::Px(300.0),
                    padding: UiRect::all(Val::Px(ui_kit::SPACE_2XL)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(ui_kit::SPACE_S),
                    border: UiRect::all(Val::Px(ui_kit::BORDER_PLATE)),
                    ..default()
                },
                // Opaque I/Ledger face — Title contrast law on soft GPU.
                ui_kit::plate(),
                Visibility::Hidden,
            ),
GlobalZIndex(LIVED_UI_Z_LEDGER),
            SatchelRoot,
            LivedUiPlate,
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new("SATCHEL"),
ui_kit::font(ui_kit::TYPE_LABEL),
TextColor(TITLE_TEXT_SECONDARY),
),
                SatchelFontBase(ui_kit::TYPE_LABEL),
            ));
            p.spawn((
                (
Text::new(""),
ui_kit::font(ui_kit::TYPE_READ),
TextColor(TITLE_TEXT_PRIMARY),
),
                SatchelBody,
                SatchelFontBase(ui_kit::TYPE_READ),
            ));
            p.spawn((
                (
Text::new("I close · 1–3 highlight · R allocate surplus"),
ui_kit::font(ui_kit::TYPE_HINT),
TextColor(TITLE_TEXT_SECONDARY),
),
                SatchelFontBase(ui_kit::TYPE_HINT),
            ));
        });

    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: PICKUP.top(),
                    left: PICKUP.left(),
                    width: Val::Px(360.0),
                    margin: PICKUP.margin(),
                    padding: UiRect::axes(Val::Px(ui_kit::SPACE_2XL), Val::Px(ui_kit::SPACE_M)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(ui_kit::BORDER_HAIRLINE)),
                    ..default()
                },
                ui_kit::plate(),
                Visibility::Hidden,
            ),
            PickupFlashRoot,
            HudSlab(PICKUP.id),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(""),
ui_kit::font(PICKUP_FLASH_FONT_BASE),
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
    if pool.harvests == 1 { return; } // UI-CALM-1: the first harvest's credit rides the Pulse
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
    (time, calm): (Option<Res<Time>>, Option<Res<crate::first_harvest_epiphany::CalmQueue>>),
) {
    let now = time.map_or(0.0, |t| t.elapsed_secs_f64()); // UI-CALM-1: Watch waits for the burst
    let show = (harvest.first_harvest_lived || pool.harvests > 0) && calm.as_deref().map_or(true, |c| c.done(now));
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


/// CARD PLAYTEST-HUD-COPY-1 — one tooltip line for the highlighted satchel item:
/// what it is and what to do with it. Reuses the plate's own words.
fn satchel_slot_tooltip(slot: SatchelSlot) -> &'static str {
    match slot {
        SatchelSlot::Vitality => "Vitality · grows with each tend (E) · R allocate surplus",
        SatchelSlot::Harmony => "Harmony · held, not hoarded · R allocate surplus",
        SatchelSlot::Joy => "Joy · held, not hoarded · R allocate surplus",
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
    ui_kit::scaled_px(base, text_scale)
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
        match font.font_size {
            FontSize::Px(current) if (current - px / 1.2).abs() <= 0.01 => {}
            _ => font.font_size = FontSize::Px(px / 1.2),
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
        match font.font_size {
            FontSize::Px(current) if (current - px / 1.2).abs() <= 0.01 => {}
            _ => font.font_size = FontSize::Px(px / 1.2),
        }
    }
}

fn update_satchel(
    inv: Res<HumanInventory>,
    pool: Res<SoftRbePool>,
    bind: Res<LivedHourBind>,
    house_label: Res<HouseLabel>,
    yard: Option<Res<crate::fabricator::FabricatorYard>>,
    finds: Option<Res<FoundFinds>>,
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
    // LOOT-TO-SATCHEL-1: one shipped find line per owned name, in table order, never repeated.
    let find_rows: String = NAMED_FINDS
        .iter()
        .filter(|n| finds.as_ref().is_some_and(|f| f.names.contains(&n.name)))
        .map(|n| format!("\n{}", find_line(n)))
        .collect();
    let find_block = if find_rows.is_empty() {
        find_rows
    } else {
        format!("\n{find_rows}")
    };
    // WOOD-USE-1: the shipped LaneCrate name with its wood-refined stock, only once stocked.
    let crate_block = yard
        .as_ref()
        .map(|y| y.fab.crate_stock)
        .filter(|n| *n > 0)
        .map(|n| format!("\n\nLaneCrate {n}"))
        .unwrap_or_default();
    let body_line = format!(
        "{face}

abundance is held, not hoarded

{} [1] Vitality   {:.1}
{} [2] Harmony    {:.1}
{} [3] Joy        {:.1}

Harvests {}{find_block}{crate_block}{temper_block}

{}",
        mark(SatchelSlot::Vitality),
        pool.vitality,
        mark(SatchelSlot::Harmony),
        pool.harmony,
        mark(SatchelSlot::Joy),
        pool.joy,
        pool.harvests,
        satchel_slot_tooltip(inv.selected)
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

    /// CARD PLAYTEST-HUD-COPY-1 — every satchel item has a named tooltip line.
    #[test]
    fn satchel_tooltip_names_each_item_and_its_use() {
        for (slot, word) in [
            (SatchelSlot::Vitality, "Vitality"),
            (SatchelSlot::Harmony, "Harmony"),
            (SatchelSlot::Joy, "Joy"),
        ] {
            let line = satchel_slot_tooltip(slot);
            assert!(line.starts_with(word), "{line}");
            assert!(line.contains("R allocate surplus"), "{line}");
            assert!(!line.contains('\n'), "one line: {line}");
        }
    }

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
        let watch = watch_q.single(app.world()).unwrap().clone();
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
        let satchel = satchel_q.single(app.world()).unwrap().clone();
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
        let pickup = pickup_q.single(app.world()).unwrap().clone();
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
        let spawned = satchel_font_rows(&mut app);
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

        let after = satchel_font_rows(&mut app);
        assert_eq!(after.len(), 3);
        for (before, (base, px, value)) in spawned.iter().zip(after.iter()) {
            assert_eq!(before.0, *base);
            assert_eq!(*px, satchel_font_px(*base, new_scale) / 1.2);
            assert_eq!(value, &before.2);
        }
    }

    // compiler-forced: removed World::iter_entities (deprecated since 0.17.0; the 0.17→0.18 guide does not name it).
    fn satchel_font_rows(app: &mut App) -> Vec<(f32, f32, String)> {
        let mut rows = Vec::new();
        let mut fonts = app
            .world_mut()
            .query::<(&SatchelFontBase, &Text, &TextFont)>();
        for (base, text, font) in fonts.iter(app.world()) {
            let FontSize::Px(size) = font.font_size else {
                panic!("satchel font size is {:?}", font.font_size);
            };
            rows.push((base.0, size, text.as_str().to_string()));
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
        let strip = strip_flash_font_rows(&mut app);
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
        let plate = satchel_font_rows(&mut app);
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

        let after = strip_flash_font_rows(&mut app);
        assert_eq!(after.len(), 2);
        for (before, (base, px, value, kind)) in strip.iter().zip(after.iter()) {
            assert_eq!(before.0, *base);
            assert_eq!(before.3, *kind);
            assert_eq!(*px, satchel_font_px(*base, new_scale) / 1.2);
            assert_eq!(value, &before.2);
            assert!(*px > *base / 1.2);
        }
        let plate_after = satchel_font_rows(&mut app);
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
            for (base, px, _, _) in strip_flash_font_rows(&mut app) {
                assert_eq!(px, satchel_font_px(base, extreme) / 1.2);
            }
            for (base, px, _) in satchel_font_rows(&mut app) {
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
        let rows = strip_flash_font_rows(&mut app);
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

    // compiler-forced: removed World::iter_entities (deprecated since 0.17.0; the 0.17→0.18 guide does not name it).
    fn strip_flash_font_rows(app: &mut App) -> Vec<(f32, f32, String, &'static str)> {
        let mut rows = Vec::new();
        let mut fonts = app.world_mut().query::<(
            &StripFlashFontBase,
            Option<&WatchStripText>,
            Option<&PickupFlashText>,
            &Text,
            &TextFont,
        )>();
        for (base, watch, pickup, text, font) in fonts.iter(app.world()) {
            let kind = if watch.is_some() {
                "strip"
            } else if pickup.is_some() {
                "flash"
            } else {
                panic!("StripFlashFontBase without strip or flash text");
            };
            let FontSize::Px(size) = font.font_size else {
                panic!("strip font size is {:?}", font.font_size);
            };
            rows.push((base.0, size, text.as_str().to_string(), kind));
        }
        rows.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("strip flash font base"));
        rows
    }

    /// CARD UI-KIT-1 — watch, satchel and pickup spawn from the ui_kit tokens only.
    #[test]
    fn satchel_surfaces_spawn_from_ui_kit_tokens() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_systems(Startup, spawn_inventory_surfaces);
        app.update();
        let world = app.world_mut();
        let fills: Vec<Color> = world.query_filtered::<&BackgroundColor, Without<Text>>().iter(world).map(|c| c.0).collect();
        assert_eq!(fills, vec![ui_kit::TITLE_PLATE_BG; 3]);
        assert!(world.query_filtered::<&BorderColor, Without<Text>>().iter(world).all(|b| *b == BorderColor::all(ui_kit::TITLE_BORDER)));
        let scale = [ui_kit::TYPE_CUE, ui_kit::TYPE_LABEL, ui_kit::TYPE_READ, ui_kit::TYPE_SMALL, ui_kit::TYPE_HINT];
        let fonts: Vec<FontSize> = world.query::<&TextFont>().iter(world).map(|f| f.font_size).collect();
        assert_eq!(fonts.len(), 5);
        assert!(fonts.iter().all(|f| scale.iter().any(|px| *f == FontSize::Px(px / 1.2))), "{fonts:?}");
        assert_eq!(satchel_font_px(ui_kit::TYPE_READ, 1.35), ui_kit::scaled_px(13.5, 1.35));
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

    use crate::difficulty::DifficultyTier::Sworn;
    use crate::loot_drops::tests::{in_user_dir, OLD_FINDS};
    use crate::loot_drops::{load_finds, loot_hash, roll_named_find, FINDS_SAVE, LOOT_BASE_CHANCE, LOOT_BPS};

    /// H1: MinimalPlugins I satchel, open, in a temp user dir. `old` seeds the finds save and runs `load_finds`.
    fn open_satchel(tag: &str, old: Option<&[u8]>, finds: Option<FoundFinds>) -> App {
        in_user_dir(tag, |dir| {
            let mut app = App::new();
            app.add_plugins(MinimalPlugins)
                .insert_resource(HumanInventory { open: true, ..default() })
                .insert_resource(SoftRbePool { vitality: 2.5, harmony: 1.5, joy: 0.5, harvests: 3, ..default() })
                .insert_resource(LivedHourBind::default()).insert_resource(HouseLabel::default())
                .add_systems(Startup, spawn_inventory_surfaces).add_systems(Update, update_satchel);
            if let Some(bytes) = old {
                std::fs::write(dir.join(FINDS_SAVE), bytes).unwrap();
                app.init_resource::<FoundFinds>().add_systems(Startup, load_finds);
            }
            if let Some(f) = finds { app.insert_resource(f); }
            app.update();
            app
        })
    }
    fn body(app: &mut App) -> String { app.world_mut().query_filtered::<&Text, With<SatchelBody>>().single(app.world()).unwrap().as_str().into() }
    fn rows(body: &str) -> Vec<&'static str> { NAMED_FINDS.iter().filter(|n| body.contains(n.sentence)).map(|n| n.name).collect() }
    fn landed(s: u32) -> Option<FoundFinds> { let mut f = FoundFinds::default(); f.drop_once(Sworn, s, 1).then_some(f) }
    fn all() -> FoundFinds { FoundFinds { names: NAMED_FINDS.iter().map(|n| n.name).collect(), pending: None } }

    /// LTS1 (H1) — a forced-roll find lands exactly one `find_line` row on I; goods, harvests, tooltip and three slots stay.
    #[test]
    fn lts1_forced_roll_find_lands_one_row_on_i() {
        let f = (0..2048).find_map(landed).expect("a forced roll lands a find");
        let body = body(&mut open_satchel("lts1", None, Some(f.clone())));
        assert_eq!((body.matches(f.pending.as_deref().unwrap()).count(), rows(&body)), (1, f.names.clone()), "{body}");
        assert!(["[1] Vitality   2.5", "[2] Harmony    1.5", "[3] Joy        0.5", "Harvests 3"].iter().all(|w| body.contains(w)), "{body}");
        assert!(body.ends_with(satchel_slot_tooltip(SatchelSlot::Vitality)) && SatchelSlot::Joy as u8 == 2);
    }

    /// LTS2 (H1 + H3) — the exact old newline save loads on relaunch and shows on I, each row once, with no pending pulse.
    #[test]
    fn lts2_old_newline_finds_load_and_show_on_i() {
        let mut app = open_satchel("lts2-i", Some(OLD_FINDS), None);
        let names = vec![NAMED_FINDS[1].name, NAMED_FINDS[4].name];
        assert_eq!(app.world().resource::<FoundFinds>(), &FoundFinds { names: names.clone(), pending: None });
        let body = body(&mut app);
        assert_eq!(rows(&body), names, "{body}");
        for n in [&NAMED_FINDS[1], &NAMED_FINDS[4]] { assert_eq!(body.matches(&find_line(n)).count(), 1, "{body}"); }
    }

    /// LTS6 — a roll that picks an owned name adds no name and no second row.
    #[test]
    fn lts6_owned_find_rolled_again_stays_one_row() {
        let owned = FoundFinds { pending: None, ..(0..2048).find_map(landed).unwrap() };
        let pick = |h: u32| roll_named_find(Sworn, LOOT_BASE_CHANCE, None, (h % LOOT_BPS) as u16, loot_hash(h, 1, 1)).map(|f| f.name);
        let s = (0..8192).find(|s| pick(loot_hash(*s, 1, 1)) == Some(owned.names[0])).expect("a roll that picks the owned name");
        let mut again = owned.clone();
        assert!(!again.drop_once(Sworn, s, 1) && again == owned);
        let line = find_line(NAMED_FINDS.iter().find(|n| n.name == owned.names[0]).unwrap());
        let mut app = open_satchel("lts6", None, Some(again));
        assert_eq!(body(&mut app).matches(&line).count(), 1);
        let mut app = open_satchel("lts6-dup", None, Some(FoundFinds { names: vec![owned.names[0]; 2], pending: None }));
        assert_eq!(body(&mut app).matches(&line).count(), 1, "a repeated name still shows once");
    }

    /// LTS7 — every find owned, and I still shows no %, chance, weight, standing, temper, Lumen count or Ward.
    #[test]
    fn lts7_no_odds_on_i() {
        let mut app = open_satchel("lts7", None, Some(all()));
        let body = body(&mut app);
        for banned in ["%", "chance", "odds", "weight", "+", "Lumen ", "Ward", "0.25", "Kept", "Proven", "Storied"] {
            assert!(!body.contains(banned), "{banned} in {body}");
        }
        let src = include_str!("human_inventory.rs").split("#[cfg(test)]").next().unwrap();
        assert!(["LOOT_BASE_CHANCE", "roll_named_find", "find_weight", "standing_weight", ".standing()"].iter().all(|w| !src.contains(w)));
    }

    /// LTS8 — showing finds leaves SoftRbePool goods and the goods lines as they were (R5: a find is not stock).
    #[test]
    fn lts8_finds_leave_pool_amounts() {
        let mut bare = open_satchel("lts8-bare", None, None);
        let mut with = open_satchel("lts8", None, Some(all()));
        let pool = with.world().resource::<SoftRbePool>();
        assert_eq!((pool.vitality, pool.harmony, pool.joy, pool.harvests), (2.5, 1.5, 0.5, 3));
        let goods = |b: String| b.lines().filter(|l| l.contains("] ") || l.starts_with("Harvests")).map(str::to_string).collect::<Vec<_>>();
        assert_eq!(goods(body(&mut bare)), goods(body(&mut with)));
    }

    /// A yard whose Fabricator JSON carries `crate_stock: n`.
    fn crate_yard(n: u32) -> crate::fabricator::FabricatorYard {
        let raw = format!(r#"{{"planted":true,"reserve":0.0,"pack":{{"repair":true,"logi":true}},"last_line":"","crate_stock":{n}}}"#);
        crate::fabricator::FabricatorYard { fab: serde_json::from_str(&raw).unwrap(), bench_glow: 0.0 }
    }

    /// WU6 (H1) — stocked LaneCrate is one `LaneCrate n` line after the LTS find rows and before temper.
    /// Stock 0 adds nothing; removing the line gives back the exact old body; slots stay three.
    #[test]
    fn wu6_lane_crate_stock_is_one_line_after_finds() {
        let mut app = open_satchel("wu6", None, Some(all()));
        let before = body(&mut app);
        assert_eq!((rows(&before), before.matches("LaneCrate").count()), (all().names, 0), "{before}");
        app.insert_resource(crate_yard(0));
        app.update();
        assert_eq!(body(&mut app), before, "stock 0 adds no line");
        app.insert_resource(crate_yard(2));
        app.update();
        let after = body(&mut app);
        assert_eq!((after.matches("LaneCrate").count(), after.matches("\n\nLaneCrate 2\n").count()), (1, 1), "{after}");
        assert_eq!(rows(&after), all().names, "{after}");
        let at = after.find("LaneCrate 2").unwrap();
        assert!(NAMED_FINDS.iter().all(|n| after.find(n.sentence).unwrap() < at) && after.find("Harvests 3").unwrap() < at);
        assert_eq!(after.replacen("\n\nLaneCrate 2", "", 1), before);
        assert!(shared::fabricator::manufacture_copy_is_honest("LaneCrate 2") && SatchelSlot::Joy as u8 == 2);
        assert!(after.ends_with(satchel_slot_tooltip(SatchelSlot::Vitality)));

        let item = TemperedItem { tier: ToolTier::TendHook, ..TemperedItem::hands(1, "hands") };
        app.world_mut().resource_mut::<crate::fabricator::FabricatorYard>().fab.last_tempered = Some(item.clone());
        app.update();
        let tempered = body(&mut app);
        assert!(tempered.find("LaneCrate 2").unwrap() < tempered.find(&satchel_temper_display(&item)).unwrap(), "{tempered}");
    }
}
