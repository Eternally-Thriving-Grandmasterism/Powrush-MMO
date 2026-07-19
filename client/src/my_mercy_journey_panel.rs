/*!
 * My Mercy Journey Panel + LegacyJournal Search UI
 * v21.24.0 — Client-side Realm Filter
 *
 * Players can now filter their living Legacy by:
 * - Free-text search
 * - Category chips (Harvest, Epiphany, Council, Joy, Policy, Kardashev)
 * - Realm of origin (All Realms + the five seeded realms)
 *
 * TOLC 8 + 7 Living Mercy Gates | PATSAGi Council approved
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use simulation::player_legacy_journal::{LegacyJournalRegistry, LegacyEventType, LegacyEntry};

// === Components ===
#[derive(Component)]
pub struct MyMercyJourneyPanel;

#[derive(Component)]
struct SearchInputText;

#[derive(Component)]
struct StatsText;

#[derive(Component)]
struct FilterChip {
    filter: LegacySearchFilter,
}

#[derive(Component)]
struct RealmFilterChip {
    realm_filter: RealmFilter,
}

#[derive(Component)]
struct LegacyResultEntry {
    index: usize,
}

// === Filter Enums ===
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum LegacySearchFilter {
    #[default]
    All,
    Harvest,
    Epiphany,
    Council,
    ProactiveJoy,
    SynergyPolicy,
    Kardashev,
}

impl LegacySearchFilter {
    pub fn label(&self) -> &'static str {
        match self {
            LegacySearchFilter::All => "All",
            LegacySearchFilter::Harvest => "Harvest",
            LegacySearchFilter::Epiphany => "Epiphany",
            LegacySearchFilter::Council => "Council",
            LegacySearchFilter::ProactiveJoy => "Joy",
            LegacySearchFilter::SynergyPolicy => "Policy",
            LegacySearchFilter::Kardashev => "Kardashev",
        }
    }

    pub fn matches_event(&self, event_type: &LegacyEventType) -> bool {
        match self {
            LegacySearchFilter::All => true,
            LegacySearchFilter::Harvest => matches!(event_type, LegacyEventType::Harvest),
            LegacySearchFilter::Epiphany => matches!(event_type, LegacyEventType::Epiphany),
            LegacySearchFilter::Council => matches!(event_type, LegacyEventType::CouncilBloom),
            LegacySearchFilter::ProactiveJoy => matches!(event_type, LegacyEventType::ProactiveJoy),
            LegacySearchFilter::SynergyPolicy => matches!(event_type, LegacyEventType::SynergyPolicy),
            LegacySearchFilter::Kardashev => {
                matches!(event_type, LegacyEventType::CouncilBloom | LegacyEventType::SynergyPolicy)
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum RealmFilter {
    #[default]
    AllRealms,
    Realm0, // Sanctuary Prime
    Realm1, // Synthetic Lattice
    Realm2, // Verdant Bloom
    Realm3, // Harmonic Chorus
    Realm4, // Voidfarer Horizon
}

impl RealmFilter {
    pub fn label(&self) -> &'static str {
        match self {
            RealmFilter::AllRealms => "All Realms",
            RealmFilter::Realm0 => "Sanctuary",
            RealmFilter::Realm1 => "Synthetic",
            RealmFilter::Realm2 => "Verdant",
            RealmFilter::Realm3 => "Harmonic",
            RealmFilter::Realm4 => "Voidfarer",
        }
    }

    pub fn matches_realm_id(&self, realm_id: u8) -> bool {
        match self {
            RealmFilter::AllRealms => true,
            RealmFilter::Realm0 => realm_id == 0,
            RealmFilter::Realm1 => realm_id == 1,
            RealmFilter::Realm2 => realm_id == 2,
            RealmFilter::Realm3 => realm_id == 3,
            RealmFilter::Realm4 => realm_id == 4,
        }
    }

    pub fn as_u8(&self) -> Option<u8> {
        match self {
            RealmFilter::AllRealms => None,
            RealmFilter::Realm0 => Some(0),
            RealmFilter::Realm1 => Some(1),
            RealmFilter::Realm2 => Some(2),
            RealmFilter::Realm3 => Some(3),
            RealmFilter::Realm4 => Some(4),
        }
    }
}

#[derive(Resource, Default)]
pub struct LegacySearchState {
    pub query: String,
    pub active_filter: LegacySearchFilter,
    pub active_realm_filter: RealmFilter,
    pub results_count: usize,
}

pub struct MyMercyJourneyPanelPlugin;

impl Plugin for MyMercyJourneyPanelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LegacySearchState>()
            .add_systems(Startup, spawn_my_mercy_journey_ui)
            .add_systems(
                Update,
                (
                    toggle_my_mercy_journey_ui,
                    handle_search_text_input,
                    handle_filter_chip_clicks,
                    handle_realm_filter_chip_clicks,
                    update_legacy_search_results,
                ),
            );
    }
}

fn spawn_my_mercy_journey_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_reg = asset_server.load("fonts/FiraSans-Regular.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(4.0),
                    right: Val::Percent(1.5),
                    width: Val::Px(440.0),
                    max_height: Val::Percent(88.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(14.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    overflow: Overflow::clip(),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                background_color: Color::srgba(0.04, 0.06, 0.10, 0.97).into(),
                border_color: Color::srgb(0.35, 0.75, 0.95).into(),
                ..default()
            },
            MyMercyJourneyPanel,
            Name::new("MyMercyJourneyPanel"),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "MY MERCY JOURNEY",
                    TextStyle {
                        font: font_bold.clone(),
                        font_size: 19.0,
                        color: Color::srgb(0.70, 0.95, 0.88),
                    },
                ),
                ..default()
            });

            // Search label
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Search (type freely • Esc clears)",
                    TextStyle {
                        font: font_reg.clone(),
                        font_size: 11.5,
                        color: Color::srgb(0.75, 0.85, 0.95),
                    },
                ),
                ..default()
            });

            // Live search input
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "[ type to search... ]",
                        TextStyle {
                            font: font_reg.clone(),
                            font_size: 13.0,
                            color: Color::srgb(0.9, 0.92, 1.0),
                        },
                    ),
                    style: Style {
                        padding: UiRect::all(Val::Px(7.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        min_height: Val::Px(26.0),
                        ..default()
                    },
                    background_color: Color::srgba(0.08, 0.10, 0.14, 0.9).into(),
                    ..default()
                },
                SearchInputText,
            ));

            // Category filter chips
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        column_gap: Val::Px(5.0),
                        row_gap: Val::Px(5.0),
                        margin: UiRect::top(Val::Px(4.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|chips| {
                    for filter in [
                        LegacySearchFilter::All,
                        LegacySearchFilter::Harvest,
                        LegacySearchFilter::Epiphany,
                        LegacySearchFilter::Council,
                        LegacySearchFilter::ProactiveJoy,
                        LegacySearchFilter::SynergyPolicy,
                        LegacySearchFilter::Kardashev,
                    ] {
                        chips
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        padding: UiRect::axes(Val::Px(9.0), Val::Px(4.0)),
                                        border: UiRect::all(Val::Px(1.0)),
                                        border_radius: BorderRadius::all(Val::Px(11.0)),
                                        ..default()
                                    },
                                    background_color: Color::srgba(0.12, 0.16, 0.22, 0.95).into(),
                                    border_color: Color::srgb(0.35, 0.55, 0.75).into(),
                                    ..default()
                                },
                                FilterChip { filter },
                            ))
                            .with_children(|b| {
                                b.spawn(TextBundle {
                                    text: Text::from_section(
                                        filter.label(),
                                        TextStyle {
                                            font: font_reg.clone(),
                                            font_size: 11.5,
                                            color: Color::srgb(0.85, 0.92, 1.0),
                                        },
                                    ),
                                    ..default()
                                });
                            });
                    }
                });

            // === NEW: Realm filter chips ===
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Realm of Origin",
                    TextStyle {
                        font: font_reg.clone(),
                        font_size: 11.0,
                        color: Color::srgb(0.70, 0.80, 0.95),
                    },
                ),
                style: Style {
                    margin: UiRect::top(Val::Px(6.0)),
                    ..default()
                },
                ..default()
            });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        column_gap: Val::Px(5.0),
                        row_gap: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|chips| {
                    for realm_filter in [
                        RealmFilter::AllRealms,
                        RealmFilter::Realm0,
                        RealmFilter::Realm1,
                        RealmFilter::Realm2,
                        RealmFilter::Realm3,
                        RealmFilter::Realm4,
                    ] {
                        chips
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                                        border: UiRect::all(Val::Px(1.0)),
                                        border_radius: BorderRadius::all(Val::Px(11.0)),
                                        ..default()
                                    },
                                    background_color: Color::srgba(0.10, 0.14, 0.20, 0.95).into(),
                                    border_color: Color::srgb(0.40, 0.50, 0.70).into(),
                                    ..default()
                                },
                                RealmFilterChip { realm_filter },
                            ))
                            .with_children(|b| {
                                b.spawn(TextBundle {
                                    text: Text::from_section(
                                        realm_filter.label(),
                                        TextStyle {
                                            font: font_reg.clone(),
                                            font_size: 11.0,
                                            color: Color::srgb(0.82, 0.90, 1.0),
                                        },
                                    ),
                                    ..default()
                                });
                            });
                    }
                });

            // Stats
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Showing 0 entries",
                        TextStyle {
                            font: font_reg.clone(),
                            font_size: 11.5,
                            color: Color::srgb(0.7, 0.85, 0.95),
                        },
                    ),
                    style: Style {
                        margin: UiRect::top(Val::Px(4.0)),
                        ..default()
                    },
                    ..default()
                },
                StatsText,
            ));

            // Results header
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "— LEGACY TIMELINE —",
                    TextStyle {
                        font: font_bold.clone(),
                        font_size: 13.5,
                        color: Color::srgb(1.0, 0.88, 0.55),
                    },
                ),
                style: Style {
                    margin: UiRect::top(Val::Px(4.0)),
                    ..default()
                },
                ..default()
            });

            // Result slots
            for i in 0..8 {
                parent.spawn((
                    TextBundle {
                        text: Text::from_section(
                            if i == 0 { "• Awaiting merciful acts..." } else { "" },
                            TextStyle {
                                font: font_reg.clone(),
                                font_size: 12.0,
                                color: Color::srgb(0.92, 0.95, 1.0),
                            },
                        ),
                        style: Style {
                            margin: UiRect::top(Val::Px(2.0)),
                            ..default()
                        },
                        ..default()
                    },
                    LegacyResultEntry { index: i },
                ));
            }

            // Footer
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "F2 toggle  •  Type + Category + Realm filters  •  TOLC 8",
                    TextStyle {
                        font: font_reg.clone(),
                        font_size: 10.0,
                        color: Color::srgb(0.55, 0.65, 0.78),
                    },
                ),
                style: Style {
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            });
        });
}

fn toggle_my_mercy_journey_ui(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<MyMercyJourneyPanel>>,
) {
    if keyboard.just_pressed(KeyCode::F2) {
        for mut visibility in &mut query {
            *visibility = if *visibility == Visibility::Hidden {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

fn handle_search_text_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut search_state: ResMut<LegacySearchState>,
    panel_query: Query<&Visibility, With<MyMercyJourneyPanel>>,
    mut search_text_query: Query<&mut Text, With<SearchInputText>>,
) {
    let panel_visible = panel_query.iter().any(|v| *v == Visibility::Visible);
    if !panel_visible {
        return;
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        search_state.query.clear();
    }
    if keyboard.just_pressed(KeyCode::Backspace) {
        search_state.query.pop();
    }

    let chars: &[(KeyCode, char)] = &[
        (KeyCode::KeyA, 'a'), (KeyCode::KeyB, 'b'), (KeyCode::KeyC, 'c'), (KeyCode::KeyD, 'd'),
        (KeyCode::KeyE, 'e'), (KeyCode::KeyF, 'f'), (KeyCode::KeyG, 'g'), (KeyCode::KeyH, 'h'),
        (KeyCode::KeyI, 'i'), (KeyCode::KeyJ, 'j'), (KeyCode::KeyK, 'k'), (KeyCode::KeyL, 'l'),
        (KeyCode::KeyM, 'm'), (KeyCode::KeyN, 'n'), (KeyCode::KeyO, 'o'), (KeyCode::KeyP, 'p'),
        (KeyCode::KeyQ, 'q'), (KeyCode::KeyR, 'r'), (KeyCode::KeyS, 's'), (KeyCode::KeyT, 't'),
        (KeyCode::KeyU, 'u'), (KeyCode::KeyV, 'v'), (KeyCode::KeyW, 'w'), (KeyCode::KeyX, 'x'),
        (KeyCode::KeyY, 'y'), (KeyCode::KeyZ, 'z'),
        (KeyCode::Digit0, '0'), (KeyCode::Digit1, '1'), (KeyCode::Digit2, '2'), (KeyCode::Digit3, '3'),
        (KeyCode::Digit4, '4'), (KeyCode::Digit5, '5'), (KeyCode::Digit6, '6'), (KeyCode::Digit7, '7'),
        (KeyCode::Digit8, '8'), (KeyCode::Digit9, '9'),
        (KeyCode::Space, ' '),
        (KeyCode::Minus, '-'), (KeyCode::Period, '.'),
    ];

    for (key, ch) in chars {
        if keyboard.just_pressed(*key) {
            let shifted = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
            let final_ch = if shifted && ch.is_ascii_lowercase() {
                ch.to_ascii_uppercase()
            } else {
                *ch
            };
            search_state.query.push(final_ch);
        }
    }

    for mut text in &mut search_text_query {
        if search_state.query.is_empty() {
            text.sections[0].value = "[ type to search... ]".to_string();
            text.sections[0].style.color = Color::srgb(0.55, 0.60, 0.70);
        } else {
            text.sections[0].value = format!("{}_", search_state.query);
            text.sections[0].style.color = Color::srgb(0.95, 0.97, 1.0);
        }
    }
}

fn handle_filter_chip_clicks(
    mut interaction_query: Query<(&Interaction, &FilterChip, &mut BackgroundColor, &mut BorderColor), Changed<Interaction>>,
    mut search_state: ResMut<LegacySearchState>,
) {
    for (interaction, chip, mut bg, mut border) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                search_state.active_filter = chip.filter;
                *bg = Color::srgba(0.18, 0.32, 0.48, 0.98).into();
                *border = Color::srgb(0.45, 0.85, 1.0).into();
            }
            Interaction::Hovered => {
                *bg = Color::srgba(0.14, 0.22, 0.32, 0.97).into();
            }
            Interaction::None => {
                if search_state.active_filter == chip.filter {
                    *bg = Color::srgba(0.18, 0.32, 0.48, 0.98).into();
                    *border = Color::srgb(0.45, 0.85, 1.0).into();
                } else {
                    *bg = Color::srgba(0.12, 0.16, 0.22, 0.95).into();
                    *border = Color::srgb(0.35, 0.55, 0.75).into();
                }
            }
        }
    }
}

fn handle_realm_filter_chip_clicks(
    mut interaction_query: Query<(&Interaction, &RealmFilterChip, &mut BackgroundColor, &mut BorderColor), Changed<Interaction>>,
    mut search_state: ResMut<LegacySearchState>,
) {
    for (interaction, chip, mut bg, mut border) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                search_state.active_realm_filter = chip.realm_filter;
                *bg = Color::srgba(0.16, 0.28, 0.42, 0.98).into();
                *border = Color::srgb(0.50, 0.75, 1.0).into();
            }
            Interaction::Hovered => {
                *bg = Color::srgba(0.13, 0.20, 0.30, 0.97).into();
            }
            Interaction::None => {
                if search_state.active_realm_filter == chip.realm_filter {
                    *bg = Color::srgba(0.16, 0.28, 0.42, 0.98).into();
                    *border = Color::srgb(0.50, 0.75, 1.0).into();
                } else {
                    *bg = Color::srgba(0.10, 0.14, 0.20, 0.95).into();
                    *border = Color::srgb(0.40, 0.50, 0.70).into();
                }
            }
        }
    }
}

fn update_legacy_search_results(
    legacy_registry: Res<LegacyJournalRegistry>,
    search_state: Res<LegacySearchState>,
    mut stats_query: Query<&mut Text, With<StatsText>>,
    mut result_queries: Query<(&LegacyResultEntry, &mut Text)>,
) {
    let all_entries: Vec<&LegacyEntry> = legacy_registry
        .build_filterable_legacy_threads(Default::default())
        .into_iter()
        .collect();

    let query_lower = search_state.query.to_lowercase();

    let filtered: Vec<&&LegacyEntry> = all_entries
        .iter()
        .filter(|e| {
            // Category filter
            if !search_state.active_filter.matches_event(&e.event_type) {
                return false;
            }

            // Realm filter — we look for "[Realm X]" prefix that proactive joy / council entries carry,
            // or fall back to accepting all when the entry has no explicit realm tag yet.
            let desc = e.description.to_lowercase();
            let realm_ok = match search_state.active_realm_filter {
                RealmFilter::AllRealms => true,
                other => {
                    if let Some(rid) = other.as_u8() {
                        // Match explicit [Realm N] tags written by the realm-partitioned recording path
                        desc.contains(&format!("[realm {}]", rid))
                            || desc.contains(&format!("realm {}", rid))
                    } else {
                        true
                    }
                }
            };
            if !realm_ok {
                return false;
            }

            // Text search
            if query_lower.is_empty() {
                return true;
            }
            let cat = format!("{:?}", e.event_type).to_lowercase();
            desc.contains(&query_lower) || cat.contains(&query_lower)
        })
        .collect();

    let count = filtered.len();

    for mut text in &mut stats_query {
        text.sections[0].value = format!(
            "Showing {}  •  {}  •  {}",
            count,
            search_state.active_filter.label(),
            search_state.active_realm_filter.label()
        );
    }

    for (entry_comp, mut text) in &mut result_queries {
        if let Some(e) = filtered.get(entry_comp.index) {
            let icon = match e.event_type {
                LegacyEventType::ServerWarVictory => "⚔️",
                LegacyEventType::Harvest => "🌾",
                LegacyEventType::Epiphany => "✨",
                LegacyEventType::ProactiveJoy => "💫",
                LegacyEventType::CouncilBloom => "🕊️",
                LegacyEventType::SynergyPolicy => "🔮",
                _ => "•",
            };

            let rich = format!(
                "T{:03} | {} (Mercy +{:.1})",
                e.tick, e.description, e.mercy_impact
            );
            text.sections[0].value = format!("{} {}", icon, rich);
            text.sections[0].style.color = match e.event_type {
                LegacyEventType::Epiphany => Color::srgb(0.75, 0.95, 0.88),
                LegacyEventType::ProactiveJoy => Color::srgb(0.95, 0.90, 0.55),
                LegacyEventType::CouncilBloom | LegacyEventType::SynergyPolicy => {
                    Color::srgb(0.70, 0.85, 1.0)
                }
                _ => Color::srgb(0.92, 0.95, 1.0),
            };
        } else {
            text.sections[0].value = if entry_comp.index == 0 && count == 0 {
                "• No matching Legacy entries yet...".to_string()
            } else {
                "".to_string()
            };
        }
    }
}

// End of client/src/my_mercy_journey_panel.rs v21.24.0
// Full text + category + realm filtering is now live.
// Thunder locked in. Yoi ⚡
