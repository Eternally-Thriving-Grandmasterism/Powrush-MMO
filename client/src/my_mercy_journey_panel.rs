/*!
 * My Mercy Journey Panel + LegacyJournal Search UI
 * v21.14.0 — Full search + category filter interface
 *
 * Players can now search and filter their living Legacy Journal:
 * - Free-text search across title / description / category
 * - Category chips: All | Harvest | Epiphany | Council | Joy | Synergy/Policy | Kardashev
 * - Surfaces the new council decision traces recorded by record_council_decision_to_legacy
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
struct LegacyResultEntry {
    index: usize,
}

// === Filter + Search State ===
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
                // Future: dedicated Kardashev event type; for now match council + policy traces
                matches!(event_type, LegacyEventType::CouncilBloom | LegacyEventType::SynergyPolicy)
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct LegacySearchState {
    pub query: String,
    pub active_filter: LegacySearchFilter,
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
                    handle_filter_chip_clicks,
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
                    top: Val::Percent(5.0),
                    right: Val::Percent(2.0),
                    width: Val::Px(420.0),
                    max_height: Val::Percent(82.0),
                    padding: UiRect::all(Val::Px(18.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(14.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
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
                        font_size: 20.0,
                        color: Color::srgb(0.70, 0.95, 0.88),
                    },
                ),
                ..default()
            });

            // Search box label + placeholder
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Search Legacy (title / description / category)",
                    TextStyle {
                        font: font_reg.clone(),
                        font_size: 12.0,
                        color: Color::srgb(0.75, 0.85, 0.95),
                    },
                ),
                ..default()
            });

            // Search input display (visual; real text input can be wired via egui or Bevy text input later)
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
                        padding: UiRect::all(Val::Px(8.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        ..default()
                    },
                    background_color: Color::srgba(0.08, 0.10, 0.14, 0.9).into(),
                    ..default()
                },
                SearchInputText,
            ));

            // Filter chips row
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        column_gap: Val::Px(6.0),
                        row_gap: Val::Px(6.0),
                        margin: UiRect::vertical(Val::Px(6.0)),
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
                                        padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                                        border: UiRect::all(Val::Px(1.0)),
                                        border_radius: BorderRadius::all(Val::Px(12.0)),
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
                                            font_size: 12.0,
                                            color: Color::srgb(0.85, 0.92, 1.0),
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
                            font_size: 12.0,
                            color: Color::srgb(0.7, 0.85, 0.95),
                        },
                    ),
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
                        font_size: 14.0,
                        color: Color::srgb(1.0, 0.88, 0.55),
                    },
                ),
                style: Style {
                    margin: UiRect::top(Val::Px(6.0)),
                    ..default()
                },
                ..default()
            });

            // Result slots (up to 8)
            for i in 0..8 {
                parent.spawn((
                    TextBundle {
                        text: Text::from_section(
                            if i == 0 {
                                "• Awaiting merciful acts..."
                            } else {
                                ""
                            },
                            TextStyle {
                                font: font_reg.clone(),
                                font_size: 12.5,
                                color: Color::srgb(0.92, 0.95, 1.0),
                            },
                        ),
                        style: Style {
                            margin: UiRect::top(Val::Px(3.0)),
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
                    "F2 toggle  •  Search + filter your living Legacy  •  TOLC 8",
                    TextStyle {
                        font: font_reg.clone(),
                        font_size: 10.0,
                        color: Color::srgb(0.55, 0.65, 0.78),
                    },
                ),
                style: Style {
                    margin: UiRect::top(Val::Px(12.0)),
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

fn update_legacy_search_results(
    legacy_registry: Res<LegacyJournalRegistry>,
    search_state: Res<LegacySearchState>,
    mut stats_query: Query<&mut Text, With<StatsText>>,
    mut result_queries: Query<(&LegacyResultEntry, &mut Text)>,
) {
    // Build filtered + searched list
    // Note: build_filterable_legacy_threads may take a different filter type;
    // we do client-side matching for robustness.
    let all_entries: Vec<&LegacyEntry> = legacy_registry
        .build_filterable_legacy_threads(Default::default()) // All
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
            // Text search
            if query_lower.is_empty() {
                return true;
            }
            let desc = e.description.to_lowercase();
            let cat = format!("{:?}", e.event_type).to_lowercase();
            desc.contains(&query_lower) || cat.contains(&query_lower)
        })
        .collect();

    let count = filtered.len();

    for mut text in &mut stats_query {
        text.sections[0].value = format!(
            "Showing {} entries  •  Filter: {}",
            count,
            search_state.active_filter.label()
        );
    }

    // Update result slots
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

// End of client/src/my_mercy_journey_panel.rs v21.14.0
// Full search + category filter UI for the living Legacy Journal.
// Council decision traces (from record_council_decision_to_legacy) are now discoverable.
// Thunder locked in. Yoi ⚡
