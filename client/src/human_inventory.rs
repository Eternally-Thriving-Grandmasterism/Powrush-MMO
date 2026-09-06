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

use crate::companion_bond::CompanionBond;
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::harvest_feel::SoftRbePool;
use crate::human_soft_panels::HumanSoftPanels;
use crate::lived_hour_bind::LivedHourBind;
use crate::title_screen::HouseLabel;
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

#[derive(Component)]
struct WatchStripRoot;
#[derive(Component)]
struct WatchStripText;
#[derive(Component)]
struct SatchelRoot;
#[derive(Component)]
struct SatchelBody;
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
                ),
            );
    }
}

fn spawn_inventory_surfaces(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(16.0),
                    left: Val::Px(16.0),
                    width: Val::Px(340.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.04, 0.06, 0.07, 0.78).into(),
                border_color: Color::srgba(0.55, 0.80, 0.62, 0.35).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            WatchStripRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 13.0,
                        color: Color::srgb(0.86, 0.96, 0.88),
                        ..default()
                    },
                ),
                WatchStripText,
            ));
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
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
                background_color: Color::srgba(0.05, 0.08, 0.07, 0.94).into(),
                border_color: Color::srgba(0.50, 0.88, 0.62, 0.50).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            SatchelRoot,
        ))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                "SATCHEL",
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(0.75, 0.96, 0.82),
                    ..default()
                },
            ));
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 13.5,
                        color: Color::srgb(0.90, 0.96, 0.92),
                        ..default()
                    },
                ),
                SatchelBody,
            ));
            p.spawn(TextBundle::from_section(
                "I close · 1–3 highlight · R allocate surplus",
                TextStyle {
                    font_size: 11.0,
                    color: Color::srgb(0.55, 0.70, 0.62),
                    ..default()
                },
            ));
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(38.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(360.0),
                    margin: UiRect::left(Val::Px(-180.0)),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.08, 0.12, 0.06, 0.88).into(),
                border_color: Color::srgba(0.85, 0.95, 0.45, 0.55).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            PickupFlashRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.98, 1.0, 0.72),
                        ..default()
                    },
                ),
                PickupFlashText,
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

fn notice_pickup(
    pool: Res<SoftRbePool>,
    time: Res<Time>,
    mut inv: ResMut<HumanInventory>,
) {
    if pool.harvests == inv.last_seen_harvests {
        return;
    }
    inv.last_seen_harvests = pool.harvests;
    inv.pickup_until = time.elapsed_seconds_f64() + 1.8;
    inv.pickup_line = format!(
        "+{:.1} vitality  ·  satchel grew",
        pool.last_credit
    );
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
            "V {:.1}{}  H {:.1}  J {:.1}  · I",
            pool.vitality, vmark, pool.harmony, pool.joy
        )
    } else {
        format!(
            "V {:.1}{}  H {:.1}  J {:.1}  · {}",
            pool.vitality, vmark, pool.harmony, pool.joy, companion
        )
    };
    for mut text in &mut text_q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != line {
                s.value = line.clone();
            }
        }
    }
}

fn update_satchel(
    inv: Res<HumanInventory>,
    pool: Res<SoftRbePool>,
    bind: Res<LivedHourBind>,
    house_label: Res<HouseLabel>,
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
    let body_line = format!(
        "{face}

abundance is held, not hoarded

{} [1] Vitality   {:.1}
{} [2] Harmony    {:.1}
{} [3] Joy        {:.1}

Harvests {}",
        mark(SatchelSlot::Vitality),
        pool.vitality,
        mark(SatchelSlot::Harmony),
        pool.harmony,
        mark(SatchelSlot::Joy),
        pool.joy,
        pool.harvests
    );
    for mut text in &mut body {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != body_line {
                s.value = body_line.clone();
            }
        }
    }
}

fn update_pickup_flash(
    inv: Res<HumanInventory>,
    time: Res<Time>,
    mut root: Query<&mut Visibility, With<PickupFlashRoot>>,
    mut text_q: Query<&mut Text, With<PickupFlashText>>,
) {
    let now = time.elapsed_seconds_f64();
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
            if let Some(s) = text.sections.get_mut(0) {
                if s.value != inv.pickup_line {
                    s.value = inv.pickup_line.clone();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::house_name::HouseName;
    use shared::pause_ledger_face::{face_from, face_is_steward_honest, LETHAL_DECLARED_LINE};
    use shared::week_audit::WeekAudit;

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
}
