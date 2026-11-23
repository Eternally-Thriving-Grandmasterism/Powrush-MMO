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
use crate::loot_drops::{find_line, save_finds, FoundFinds};
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
                    equip_when_open.before(update_satchel),
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
                crate::ui_frame::plate_frame(),
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
                crate::ui_frame::plate_frame(),
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
Text::new("I close · 1–3 highlight · R allocate surplus · 4: equip next find"),
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
    pads: Query<&Gamepad>,
    (door, label, licences, places, edit): (Option<Res<crate::title_screen::LaunchDoor>>, Option<Res<crate::title_screen::HouseLabel>>, Option<Res<crate::licences_screen::LicencesScreen>>, Option<Res<crate::hex_travel::PlacesPlate>>, Option<Res<crate::hud_edit_mode::HudEditMode>>),
) {
    let pad_live = door.is_none_or(|d| *d == crate::title_screen::LaunchDoor::InYard) && !label.is_some_and(|l| l.settings_open) && !licences.is_some_and(|l| l.open) && !places.is_some_and(|p| p.open) && !edit.is_some_and(|e| e.active);
    let pad = |b| pad_live && pads.iter().any(|g| g.just_pressed(b));
    if !(keyboard.just_pressed(soft_play_bindings::INVENTORY) || pad(GamepadButton::DPadDown)) {
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

fn equip_when_open(
    keyboard: Res<ButtonInput<KeyCode>>,
    panels: Res<HumanSoftPanels>,
    allocate: Res<RbeAllocateChoice>,
    label: Res<HouseLabel>,
    inv: Res<HumanInventory>,
    mut finds: ResMut<FoundFinds>,
) {
    if !keyboard.just_pressed(KeyCode::Digit4) || !inv.open || panels.realm_open || allocate.panel_open || label.settings_open {
        return;
    }
    if finds.cycle_equipped() { save_finds(&finds); }
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
        vis.set_if_neq(if show { Visibility::Visible } else { Visibility::Hidden });
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
        vis.set_if_neq(if inv.open { Visibility::Visible } else { Visibility::Hidden });
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
        .map(|n| {
            let mut row = find_line(n);
            if finds.as_ref().and_then(|f| f.equipped_find()).is_some_and(|e| e.name == n.name) {
                row.push_str(" · Equipped");
            }
            format!("\n{row}")
        })
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

{}

{} [1] Vitality   {:.1}
{} [2] Harmony    {:.1}
{} [3] Joy        {:.1}

Harvests {}{find_block}{crate_block}{temper_block}

{}",
        finds.as_ref().and_then(|f| f.equipped_find()).map(|f| format!("Gear: {}", f.name)).unwrap_or_else(|| "Gear: none equipped".into()),
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
        vis.set_if_neq(if show { Visibility::Visible } else { Visibility::Hidden });
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

    use crate::combat_core::{land_hit_paid, CombatState, HIT};
    use crate::combat_text::{hook_player_strike, CombatTextQueue};
    use crate::difficulty::DifficultyTier::Sworn;
    use crate::loot_drops::tests::{in_user_dir, OLD_FINDS};
    use crate::loot_drops::{load_finds, loot_hash, roll_named_find, FINDS_SAVE, LOOT_BASE_CHANCE, LOOT_BPS};
    use crate::skill_xp::SkillXp;
    use shared::hex_travel::PlaceId;

    fn two() -> FoundFinds { FoundFinds { names: vec![NAMED_FINDS[1].name, NAMED_FINDS[4].name], pending: None, equipped: None } }

    fn dir_satchel(dir: &std::path::Path, old: Option<&[u8]>, finds: Option<FoundFinds>) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(HumanInventory { open: true, ..default() })
            .insert_resource(SoftRbePool { vitality: 2.5, harmony: 1.5, joy: 0.5, harvests: 3, ..default() })
            .insert_resource(LivedHourBind::default()).insert_resource(HouseLabel::default())
            .init_resource::<ButtonInput<KeyCode>>().init_resource::<HumanSoftPanels>().init_resource::<RbeAllocateChoice>()
            .add_systems(Startup, spawn_inventory_surfaces).add_systems(Update, (equip_when_open.before(update_satchel), update_satchel));
        if let Some(bytes) = old { std::fs::write(dir.join(FINDS_SAVE), bytes).unwrap(); app.init_resource::<FoundFinds>().add_systems(Startup, load_finds); }
        if let Some(f) = finds { app.insert_resource(f); } else { app.init_resource::<FoundFinds>(); }
        app.update();
        app
    }

      fn with_satchel(tag: &str, finds: Option<FoundFinds>, run: impl FnOnce(&mut App)) { in_user_dir(tag, |dir| run(&mut dir_satchel(dir, None, finds))); }
    /// H1: MinimalPlugins I satchel, open, in a temp user dir. `old` seeds the finds save and runs `load_finds`.
    fn open_satchel(tag: &str, old: Option<&[u8]>, finds: Option<FoundFinds>) -> App {
        in_user_dir(tag, |dir| dir_satchel(dir, old, finds))
    }
    fn body(app: &mut App) -> String { app.world_mut().query_filtered::<&Text, With<SatchelBody>>().single(app.world()).unwrap().as_str().into() }
    fn tap4(app: &mut App) {
        let mut kb = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        kb.release(KeyCode::Digit4); kb.press(KeyCode::Digit4);
        drop(kb); app.update();
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().clear_just_pressed(KeyCode::Digit4);
    }
    fn rows(body: &str) -> Vec<&'static str> { NAMED_FINDS.iter().filter(|n| body.contains(n.sentence)).map(|n| n.name).collect() }
    fn landed(s: u32) -> Option<FoundFinds> { let mut f = FoundFinds::default(); f.drop_once(Sworn, s, 1).then_some(f) }
    fn all() -> FoundFinds { FoundFinds { names: NAMED_FINDS.iter().map(|n| n.name).collect(), pending: None, equipped: None } }

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
        assert_eq!(app.world().resource::<FoundFinds>(), &FoundFinds { names: names.clone(), pending: None, equipped: None });
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
        let mut app = open_satchel("lts6-dup", None, Some(FoundFinds { names: vec![owned.names[0]; 2], pending: None, equipped: None }));
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

    #[test]
    fn g1_digit4_equips_first_owned_and_saves() {
        in_user_dir("g1", |dir| {
            let mut app = dir_satchel(dir, None, Some(two()));
            tap4(&mut app);
            assert_eq!((app.world().resource::<FoundFinds>().equipped, std::fs::read_to_string(dir.join(FINDS_SAVE)).unwrap().contains("equipped")), (Some(NAMED_FINDS[1].name), true));
        });
    }
    #[test]
    fn g2_equipped_find_bonus_on_land_hit() {
        let finds = FoundFinds { equipped: Some(NAMED_FINDS[1].name), ..two() };
        let (mut c, mut s, b) = (CombatState::default(), SkillXp::default(), 0u32);
        let (hit, _) = land_hit_paid(&mut c, &mut s, true, Sworn, None, Some(&finds));
        let mut q = CombatTextQueue::default();
        hook_player_strike(&mut q, PlaceId::Sanctuary, b, c.predator_taken, hit, Vec3::ZERO);
        assert!((hit, c.predator_taken - b >= HIT + 1, c.predator_taken - b) == (true, true, q.events[0].amount));
    }
    #[test]
    fn g3_one_press_one_equipped_row() {
        with_satchel("g3", Some(two()), |app| {
            let before = app.world().resource::<FoundFinds>().names.clone();
            app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::Digit4);
            app.update();
            let eq = app.world().resource::<FoundFinds>().equipped;
            assert_eq!((body(app).matches(" · Equipped").count(), &app.world().resource::<FoundFinds>().names), (1, &before));
            app.world_mut().resource_mut::<ButtonInput<KeyCode>>().clear_just_pressed(KeyCode::Digit4);
            app.update();
            assert_eq!(app.world().resource::<FoundFinds>().equipped, eq);
        });
    }
    #[test]
    fn g4_cycle_two_finds_then_empty() {
        with_satchel("g4", Some(two()), |app| {
            tap4(app); tap4(app); tap4(app);
            let f = app.world().resource::<FoundFinds>();
            let (mut c, mut s) = (CombatState::default(), SkillXp::default());
            assert_eq!((f.equipped, f.encode().contains("equipped"), land_hit_paid(&mut c, &mut s, true, Sworn, None, Some(f)).0, c.predator_taken), (None, false, true, HIT));
        });
    }
    #[test]
    fn g5_load_finds_restores_equipped_fail_closed() {
        in_user_dir("g5", |dir| {
            tap4(&mut dir_satchel(dir, None, Some(two())));
            let mut r = App::new(); r.add_plugins(MinimalPlugins).init_resource::<FoundFinds>().add_systems(Startup, load_finds); r.update();
            assert_eq!(r.world().resource::<FoundFinds>().equipped, Some(NAMED_FINDS[1].name));
        });
        for json in [r#"{"schema":"powrush_finds_v1","names":["Steady Climate Pick"],"equipped":"Quiet Harmony Loom"}"#, r#"{"schema":"powrush_finds_v1","names":["Steady Climate Pick"],"equipped":"no such"}"#] {
            assert!(FoundFinds::decode(Some(json)).equipped.is_none());
        }
    }
    #[test]
    fn g6_equip_gated_and_empty_owned_no_save() {
        with_satchel("g6", Some(two()), |app| {
            tap4(app);
            let on = app.world().resource::<FoundFinds>().equipped;
            app.world_mut().resource_mut::<HumanInventory>().open = false; tap4(app);
            app.world_mut().resource_mut::<HumanInventory>().open = true;
            app.world_mut().resource_mut::<HumanSoftPanels>().realm_open = true; tap4(app);
            app.world_mut().resource_mut::<HumanSoftPanels>().realm_open = false;
            app.world_mut().resource_mut::<RbeAllocateChoice>().panel_open = true; tap4(app);
            app.world_mut().resource_mut::<RbeAllocateChoice>().panel_open = false;
            app.world_mut().resource_mut::<HouseLabel>().settings_open = true; tap4(app);
            assert_eq!(app.world().resource::<FoundFinds>().equipped, on);
        });
        in_user_dir("g6e", |dir| { tap4(&mut dir_satchel(dir, None, Some(FoundFinds::default()))); assert!(!dir.join(FINDS_SAVE).exists()); });
    }
    #[test]
    fn g7_no_double_bracket_on_i() {
        with_satchel("g7", Some(all()), |app| { tap4(app); assert!(!body(app).contains("[[")); });
    }
    #[test]
    fn g8_fresh_profile_gear_line_and_hint() {
        let mut app = open_satchel("g8", None, None);
        assert!(body(&mut app).contains("Gear: none equipped") && { let w = app.world_mut(); w.query::<&Text>().iter(w).any(|t| t.contains("4: equip next find")) });
    }
    #[test] fn hud_plate_hp1_1_satchel_carries_sliced_frame() {
        for assets in [false, true] {
            let mut app = App::new();
            app.add_plugins(MinimalPlugins);
            if assets { app.add_plugins(bevy::asset::AssetPlugin::default()).init_asset::<Image>(); }
            app.add_systems(Startup, spawn_inventory_surfaces).update();
            if assets { assert!(app.world().resource::<Assets<Image>>().get(crate::ui_frame::FRAME_IMAGE.id()).is_none(), "HP1-1"); }
            let w = app.world_mut();
            let root = w.query_filtered::<Entity, With<SatchelRoot>>().single(w).unwrap();
            assert_eq!(w.get::<ImageNode>(root).cloned().map(|i| (i.image.id(), i.visual_box, i.image_mode)), Some((crate::ui_frame::FRAME_IMAGE.id(), VisualBox::PaddingBox, NodeImageMode::Sliced(crate::ui_frame::frame_slicer()))), "HP1-1");
        }
    }
    #[test] fn hud_plate_hp1_3_watch_and_pickup_carry_no_image() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_systems(Startup, spawn_inventory_surfaces).update();
        let w = app.world_mut();
        assert!(w.query_filtered::<&ImageNode, With<PickupFlashRoot>>().iter(w).next().is_none(), "HP1-3");
    }
    #[test] fn hud_plate_hp1_2_open_padding_box_covers_frame() {
        for scale in [0.85_f32, shared::local_settings::TEXT_SCALE_MAX] {
            let mut app = App::new();
            app.insert_resource(LocalSettingsState { inner: shared::local_settings::LocalSettings { text_scale: scale, ..shared::local_settings::LocalSettings::peace_defaults() }, dirty: false }).insert_resource(HumanInventory { open: true, ..default() }).insert_resource(SoftRbePool::default()).insert_resource(LivedHourBind::default()).insert_resource(HouseLabel::default()).add_plugins((MinimalPlugins, bevy::asset::AssetPlugin::default(), bevy::input::InputPlugin, bevy::text::TextPlugin, bevy::ui::UiPlugin, crate::ui_font::UiFontPlugin)).init_asset::<Image>().init_asset::<TextureAtlasLayout>().add_message::<bevy::picking::pointer::PointerInput>().add_message::<bevy::picking::backend::PointerHits>().init_resource::<bevy::picking::hover::HoverMap>().init_resource::<bevy::picking::events::PointerState>().add_systems(Startup, spawn_inventory_surfaces).add_systems(Update, (scale_satchel_fonts, update_satchel));
            app.world_mut().spawn((Camera2d, bevy::ui::IsDefaultUiCamera, Camera { computed: bevy::camera::ComputedCameraValues { target_info: Some(bevy::camera::RenderTargetInfo { physical_size: UVec2::new(1280, 720), scale_factor: 1. }), ..default() }, ..default() }));
            (app.update(), app.update());
            let (logical, open) = { let w = app.world_mut(); let (node, vis) = w.query_filtered::<(&ComputedNode, &Visibility), With<SatchelRoot>>().single(w).unwrap(); (node.padding_box().size() * node.inverse_scale_factor, *vis) };
            assert!(open == Visibility::Visible && logical.x >= 24. && logical.y >= 24., "HP1-2 {scale} {logical}");
        }
    }
    fn hp2_ui(w: u32, h: u32, s: f32) -> App {
        let mut app = App::new(); app.add_plugins((MinimalPlugins, bevy::asset::AssetPlugin::default(), bevy::input::InputPlugin, bevy::text::TextPlugin, bevy::ui::UiPlugin, crate::ui_font::UiFontPlugin)).init_asset::<Image>().init_asset::<TextureAtlasLayout>().add_message::<bevy::picking::pointer::PointerInput>().add_message::<bevy::picking::backend::PointerHits>().init_resource::<bevy::picking::hover::HoverMap>().init_resource::<bevy::picking::events::PointerState>().add_systems(Startup, spawn_inventory_surfaces);
        app.world_mut().spawn((Camera2d, bevy::ui::IsDefaultUiCamera, Camera { computed: bevy::camera::ComputedCameraValues { target_info: Some(bevy::camera::RenderTargetInfo { physical_size: UVec2::new(w, h), scale_factor: s }), ..default() }, ..default() })); app
    }
    fn hp2_root(app: &mut App) -> Entity { let w = app.world_mut(); w.query_filtered::<Entity, With<WatchStripRoot>>().single(w).unwrap() }
    #[test] fn hud_plate_hp2_a_watch_carries_sliced_frame() {
        let mut app = App::new(); app.add_plugins(MinimalPlugins).add_systems(Startup, spawn_inventory_surfaces).update(); let root = hp2_root(&mut app);
        assert_eq!(app.world().get::<ImageNode>(root).cloned().map(|i| (i.image.id(), i.visual_box, i.image_mode)), Some((crate::ui_frame::FRAME_IMAGE.id(), VisualBox::PaddingBox, NodeImageMode::Sliced(crate::ui_frame::frame_slicer()))), "HP2-a watch frame missing");
    }
    #[test] fn hud_plate_hp2_b_watch_text_has_no_image() {
        let mut app = App::new(); app.add_plugins(MinimalPlugins).add_systems(Startup, spawn_inventory_surfaces).update(); let w = app.world_mut();
        assert_eq!(w.query_filtered::<Option<&ImageNode>, With<WatchStripText>>().iter(w).map(|i| i.is_some()).collect::<Vec<_>>(), vec![false], "HP2-b watch");
    }
    #[test] fn hud_plate_hp2_c_watch_layout_matches_without_image() {
        for (w, h, s) in [(1280u32, 720u32, 1f32), (2560, 1440, 2.)] {
            let mut app = hp2_ui(w, h, s); (app.update(), app.update()); let root = hp2_root(&mut app);
            let at = |app: &App| (app.world().get::<ComputedNode>(root).unwrap().size(), app.world().get::<bevy::ui::UiGlobalTransform>(root).unwrap().translation); let framed = at(&app);
            app.world_mut().entity_mut(root).remove::<ImageNode>(); app.update(); assert_eq!(at(&app), framed, "HP2-c watch {w}x{h}");
        }
    }
    #[test] fn hud_plate_hp2_d_watch_corner_is_one_at_text_scale_085() {
        let mut app = hp2_ui(1280, 720, 1.); app.insert_resource(crate::local_settings::LocalSettingsState { inner: shared::local_settings::LocalSettings { text_scale: 0.85, ..shared::local_settings::LocalSettings::peace_defaults() }, dirty: false }).add_systems(Update, (scale_strip_and_flash_fonts, |mut q: Query<&mut Text, With<WatchStripText>>| for mut t in &mut q { t.0 = "Harvests 1 · Vitality 1.2".into(); })); (app.update(), app.update()); let root = hp2_root(&mut app); let n = app.world().get::<ComputedNode>(root).unwrap(); let pad = n.padding_box().size() * n.inverse_scale_factor;
        assert!(pad.x >= 24. && pad.y >= 24. && (pad / crate::ui_frame::FRAME_PX as f32).min_element().min(crate::ui_frame::frame_slicer().max_corner_scale) == 1., "HP2-d watch {pad}");
    }

    /// HUMAN-INVENTORY-VIS-NEQ-1 HI1–HI10. Last tape of (Visibility, changed) on the satchel, watch and pickup roots.
    type HiVis = Vec<(Visibility, bool)>;
    #[derive(Resource, Default)] struct HiTape(HiVis, HiVis, HiVis);
    fn hi_row(v: Ref<Visibility>) -> (Visibility, bool) { (*v, v.is_changed()) }
    fn hi_tape(s: Query<Ref<Visibility>, With<SatchelRoot>>, w: Query<Ref<Visibility>, With<WatchStripRoot>>, p: Query<Ref<Visibility>, With<PickupFlashRoot>>, mut t: ResMut<HiTape>) {
        t.0.extend(s.iter().map(hi_row)); t.1.extend(w.iter().map(hi_row)); t.2.extend(p.iter().map(hi_row));
    }
    fn hi_arm(mut app: App) -> App { app.init_resource::<HiTape>().add_systems(Last, hi_tape); app.update(); app.world_mut().insert_resource(HiTape::default()); app }
    fn hi_run(app: &mut App, n: usize) -> HiTape { for _ in 0..n { app.update(); } std::mem::take(&mut *app.world_mut().resource_mut::<HiTape>()) }
    fn hi_hide<R: Component>(app: &mut App) { let e = app.world_mut().query_filtered::<Entity, With<R>>().single(app.world()).unwrap(); *app.world_mut().get_mut::<Visibility>(e).unwrap() = Visibility::Hidden; }
    /// (B) MinimalPlugins + spawn_inventory_surfaces with update_watch_strip and update_pickup_flash, 100 ms manual steps; `hud` adds (C) HudLayoutPlugin, Classic, InYard, PlacesPlate.
    fn hi_b(hud: bool) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).init_resource::<SoftRbePool>().init_resource::<FirstHarvestEpiphany>().init_resource::<HumanInventory>().insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::from_millis(100))).add_systems(Startup, spawn_inventory_surfaces).add_systems(Update, (update_watch_strip, update_pickup_flash));
        if hud { app.add_plugins(crate::hud_presets::HudLayoutPlugin).insert_resource(crate::hud_presets::ActiveHudPreset { id: Some(crate::hud_presets::HudPresetId::Classic) }).insert_resource(crate::title_screen::LaunchDoor::InYard).init_resource::<crate::hex_travel::PlacesPlate>(); }
        app.update(); hi_arm(app)
    }
    /// A live pickup flash: the 1.8 s window notice_pickup opens, from now.
    fn hi_pickup(app: &mut App) { let until = app.world().resource::<Time>().elapsed_secs_f64() + 1.8; let mut inv = app.world_mut().resource_mut::<HumanInventory>(); (inv.pickup_line, inv.pickup_until) = ("x".into(), until); }
    const IV: Visibility = Visibility::Visible;
    const IH: Visibility = Visibility::Hidden;
    #[test] fn human_inventory_vis_neq_1_hi1_satchel_steady_open_unmarked() { let mut app = hi_arm(open_satchel("hi1", None, None)); assert_eq!(hi_run(&mut app, 3).0, vec![(IV, false); 3], "HI1 satchel steady open"); }
    #[test] fn human_inventory_vis_neq_1_hi2_satchel_close_reopen_marks_once_each() {
        let mut app = hi_arm(open_satchel("hi2", None, None)); app.world_mut().resource_mut::<HumanInventory>().open = false;
        assert_eq!(hi_run(&mut app, 2).0, vec![(IH, true), (IH, false)], "HI2 satchel close: Hidden once, then quiet");
        app.world_mut().resource_mut::<HumanInventory>().open = true;
        assert_eq!(hi_run(&mut app, 2).0, vec![(IV, true), (IV, false)], "HI2 satchel reopen: Visible once, then quiet");
    }
    #[test] fn human_inventory_vis_neq_1_hi3_satchel_direct_hidden_restored_once() { let mut app = hi_arm(open_satchel("hi3", None, None)); hi_hide::<SatchelRoot>(&mut app); assert_eq!(hi_run(&mut app, 2).0, vec![(IV, true), (IV, false)], "HI3 satchel direct Hidden write: restored once, then quiet"); }
    #[test] fn human_inventory_vis_neq_1_hi4_watch_steady_hidden_unmarked() { let mut app = hi_b(false); assert_eq!(hi_run(&mut app, 3).1, vec![(IH, false); 3], "HI4 watch steady hidden"); }
    #[test] fn human_inventory_vis_neq_1_hi5_watch_first_harvest_marks_once() { let mut app = hi_b(false); app.world_mut().resource_mut::<SoftRbePool>().harvests = 1; assert_eq!(hi_run(&mut app, 2).1, vec![(IV, true), (IV, false)], "HI5 watch first harvest: Visible once, then quiet"); }
    #[test] fn human_inventory_vis_neq_1_hi6_watch_direct_hidden_restored_once() {
        let mut app = hi_b(false); app.world_mut().resource_mut::<SoftRbePool>().harvests = 1; hi_run(&mut app, 2); hi_hide::<WatchStripRoot>(&mut app);
        assert_eq!(hi_run(&mut app, 2).1, vec![(IV, true), (IV, false)], "HI6 watch direct Hidden write: restored once, then quiet");
    }
    #[test] fn human_inventory_vis_neq_1_hi7_pickup_steady_hidden_and_sentinel_unmarked() {
        let mut app = hi_b(false); assert_eq!(hi_run(&mut app, 3).2, vec![(IH, false); 3], "HI7 pickup steady hidden");
        { let mut inv = app.world_mut().resource_mut::<HumanInventory>(); (inv.pickup_line, inv.pickup_until) = ("x".into(), 9999.0); }
        assert_eq!(hi_run(&mut app, 2).2, vec![(IH, false); 2], "HI7 9999 sentinel stays hidden and quiet");
    }
    #[test] fn human_inventory_vis_neq_1_hi8_pickup_flash_on_then_off_once_each() {
        let mut app = hi_b(false); hi_pickup(&mut app);
        assert_eq!(hi_run(&mut app, 2).2, vec![(IV, true), (IV, false)], "HI8 pickup flash on: Visible once, then quiet");
        let t = hi_run(&mut app, 20).2; let k = t.iter().position(|r| r.0 == IH).expect("HI8 flash ends");
        assert_eq!(t, [vec![(IV, false); k], vec![(IH, true)], vec![(IH, false); 19 - k]].concat(), "HI8 pickup flash off: Hidden once, then quiet");
    }
    #[test] fn human_inventory_vis_neq_1_hi9_pickup_direct_hidden_restored_once() { let mut app = hi_b(false); hi_pickup(&mut app); hi_run(&mut app, 2); hi_hide::<PickupFlashRoot>(&mut app); assert_eq!(hi_run(&mut app, 2).2, vec![(IV, true), (IV, false)], "HI9 pickup direct Hidden write: restored once, then quiet"); }
    #[test] fn human_inventory_vis_neq_1_hi10_real_hud_layout_classic_in_yard() {
        let mut app = hi_b(true); app.world_mut().resource_mut::<SoftRbePool>().harvests = 1; hi_run(&mut app, 2);
        assert_eq!(hi_run(&mut app, 2).1, vec![(IV, false); 2], "HI10 watch steady under Classic InYard");
        hi_pickup(&mut app);
        let t = hi_run(&mut app, 2); assert_eq!((t.1, t.2), (vec![(IV, false); 2], vec![(IH, true); 2]), "HI10 live pickup clean-hidden, changed every frame on main and branch alike");
        app.world_mut().resource_mut::<crate::hex_travel::PlacesPlate>().open = true;
        let t = hi_run(&mut app, 2); assert!(t.1 == vec![(IH, true); 2] && t.2.iter().all(|r| r.0 == IH), "HI10 Places open hides watch and pickup");
        app.world_mut().resource_mut::<crate::hex_travel::PlacesPlate>().open = false;
        assert_eq!(hi_run(&mut app, 2).1, vec![(IV, true), (IV, false)], "HI10 Places close: watch restored once, then quiet");
        hi_run(&mut app, 15); assert_eq!(hi_run(&mut app, 2).2, vec![(IH, false); 2], "HI10 pickup after expiry: Hidden and quiet");
        app.world_mut().resource_mut::<crate::hud_presets::ActiveHudPreset>().id = Some(crate::hud_presets::HudPresetId::Management);
        app.world_mut().spawn((Node::default(), Visibility::Visible, HudSlab(crate::hud_anchor_registry::ID_PULSE))); hi_pickup(&mut app);
        let t = hi_run(&mut app, 2); let yielded = app.world().resource::<crate::hud_presets::HudYielded>().0.contains(&PICKUP.id);
        assert_eq!((t.2.last().unwrap().0, yielded), (IH, true), "HI10 Management with a Pulse sentinel: Pickup yields by R2 into HudYielded");
    }
    fn satchel_app() -> App {
        let mut app = App::new();
        app.add_plugins((bevy::MinimalPlugins, bevy::input::InputPlugin, HumanInventoryPlugin)).init_resource::<FirstHarvestEpiphany>().init_resource::<HumanSoftPanels>().init_resource::<RbeAllocateChoice>().init_resource::<SoftRbePool>().init_resource::<LivedHourBind>().init_resource::<HouseLabel>().init_resource::<FoundFinds>();
        app
    }
    fn satchel_open(app: &App) -> bool { app.world().resource::<HumanInventory>().open }
    fn hold(app: &mut App, button: GamepadButton, frames: usize) {
        use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent, RawGamepadButtonChangedEvent, RawGamepadEvent};
        let found = app.world_mut().query_filtered::<Entity, With<Gamepad>>().iter(app.world()).next();
        let id = found.unwrap_or_else(|| {
            let id = app.world_mut().spawn_empty().id();
            app.world_mut().write_message(GamepadConnectionEvent::new(id, GamepadConnection::Connected { name: "pad".into(), vendor_id: None, product_id: None }));
            app.update();
            id
        });
        for _ in 0..frames {
            app.world_mut().write_message(RawGamepadEvent::Button(RawGamepadButtonChangedEvent::new(id, button, 1.0)));
            app.update();
        }
    }
    fn release(app: &mut App, button: GamepadButton) {
        use bevy::input::gamepad::{RawGamepadButtonChangedEvent, RawGamepadEvent}; let id = app.world_mut().query_filtered::<Entity, With<Gamepad>>().iter(app.world()).next().unwrap(); app.world_mut().write_message(RawGamepadEvent::Button(RawGamepadButtonChangedEvent::new(id, button, 0.0))); app.update();
    }
    #[test] fn pad_satchel_s1_tap_opens_then_closes() {
        let mut app = satchel_app(); hold(&mut app, GamepadButton::DPadDown, 1); assert!(app.world().resource::<HumanInventory>().open && app.world().resource::<HumanInventory>().first_open_lived, "S1 tap opens");
        release(&mut app, GamepadButton::DPadDown); hold(&mut app, GamepadButton::DPadDown, 1); assert!(!satchel_open(&app), "S1 tap closes");
    }
    #[test] fn pad_satchel_s2_hold_opens_once() {
        let mut app = satchel_app(); hold(&mut app, GamepadButton::DPadDown, 4);
        assert!(app.world().resource::<HumanInventory>().open && app.world().resource::<HumanInventory>().first_open_lived, "S2 held 4 frames opens once, no flicker");
    }
    fn key_i(app: &mut App) {
        use bevy::input::{keyboard::{Key, KeyboardInput}, ButtonState};
        for state in [ButtonState::Released, ButtonState::Pressed] { app.world_mut().write_message(KeyboardInput { key_code: KeyCode::KeyI, logical_key: Key::Character("i".into()), state, repeat: false, text: None, window: Entity::PLACEHOLDER }); app.update(); }
    }
    fn arm_gate(app: &mut App, kind: u8, on: bool) {
        use crate::title_screen::LaunchDoor;
        match kind {
            0 => app.world_mut().resource_mut::<HouseLabel>().settings_open = on,
            1 => if app.world().get_resource::<crate::licences_screen::LicencesScreen>().is_none() { let mut s = crate::licences_screen::LicencesScreen::default(); s.open = on; app.insert_resource(s); } else { app.world_mut().resource_mut::<crate::licences_screen::LicencesScreen>().open = on; },
            2 => if app.world().get_resource::<crate::hex_travel::PlacesPlate>().is_none() { let mut p = crate::hex_travel::PlacesPlate::default(); p.open = on; app.insert_resource(p); } else { app.world_mut().resource_mut::<crate::hex_travel::PlacesPlate>().open = on; },
            3 => if app.world().get_resource::<crate::hud_edit_mode::HudEditMode>().is_none() { let mut e = crate::hud_edit_mode::HudEditMode::default(); e.active = on; app.insert_resource(e); } else { app.world_mut().resource_mut::<crate::hud_edit_mode::HudEditMode>().active = on; },
            _ => { app.insert_resource(if on { [LaunchDoor::Title, LaunchDoor::NameHouse, LaunchDoor::HouseDress][(kind - 4) as usize] } else { LaunchDoor::InYard }); }
        }
    }
    #[test] fn pad_satchel_s3_i_and_dpad_down_flip_once() {
        use bevy::input::gamepad::{RawGamepadButtonChangedEvent, RawGamepadEvent}; use bevy::input::{keyboard::{Key, KeyboardInput}, ButtonState};
        let mut app = satchel_app(); hold(&mut app, GamepadButton::DPadDown, 0);
        let id = app.world_mut().query_filtered::<Entity, With<Gamepad>>().iter(app.world()).next().unwrap();
        app.world_mut().write_message(KeyboardInput { key_code: KeyCode::KeyI, logical_key: Key::Character("i".into()), state: ButtonState::Pressed, repeat: false, text: None, window: Entity::PLACEHOLDER });
        app.world_mut().write_message(RawGamepadEvent::Button(RawGamepadButtonChangedEvent::new(id, GamepadButton::DPadDown, 1.0))); app.update();
        assert!(satchel_open(&app), "S3 I and D-pad Down in one frame flip once");
    }
    #[test] fn pad_satchel_s4_gates_block_then_clear_missing_fails_closed() {
        for kind in 0..7 {
            let mut closed = satchel_app(); arm_gate(&mut closed, kind, true); hold(&mut closed, GamepadButton::DPadDown, 1); assert!(!satchel_open(&closed), "S4 gate {kind} from closed stays closed");
            release(&mut closed, GamepadButton::DPadDown); arm_gate(&mut closed, kind, false); hold(&mut closed, GamepadButton::DPadDown, 1); assert!(satchel_open(&closed), "S4 gate {kind} clear toggles open");
            let mut opened = satchel_app(); opened.world_mut().resource_mut::<HumanInventory>().open = true; arm_gate(&mut opened, kind, true); hold(&mut opened, GamepadButton::DPadDown, 1); assert!(satchel_open(&opened), "S4 gate {kind} from open stays open");
            release(&mut opened, GamepadButton::DPadDown); arm_gate(&mut opened, kind, false); hold(&mut opened, GamepadButton::DPadDown, 1); assert!(!satchel_open(&opened), "S4 gate {kind} clear toggles closed");
        }
        let mut app = satchel_app(); let id = app.world_mut().spawn_empty().id(); app.world_mut().write_message(bevy::input::gamepad::RawGamepadEvent::Button(bevy::input::gamepad::RawGamepadButtonChangedEvent::new(id, GamepadButton::DPadDown, 1.0))); app.update(); assert!(!satchel_open(&app), "S4 missing gamepad resource fails closed");
    }
    #[test] fn pad_satchel_s5_other_buttons_idle_no_pad_no_panic() {
        let mut app = satchel_app(); for button in [GamepadButton::DPadUp, GamepadButton::DPadLeft, GamepadButton::DPadRight, GamepadButton::Select, GamepadButton::South, GamepadButton::East, GamepadButton::West, GamepadButton::North] { hold(&mut app, button, 1); release(&mut app, button); }
        assert!(!satchel_open(&app), "S5 other buttons do not toggle");
        let mut bare = satchel_app(); key_i(&mut bare); assert!(satchel_open(&bare), "S5 no pad, I toggles, no panic");
    }
    #[test] fn pad_satchel_s6_keyboard_parity_under_settings() {
        let mut app = satchel_app(); app.world_mut().resource_mut::<HouseLabel>().settings_open = true; key_i(&mut app); assert!(satchel_open(&app), "S6 I opens under settings_open"); key_i(&mut app); assert!(!satchel_open(&app), "S6 I closes under settings_open");
    }
}
