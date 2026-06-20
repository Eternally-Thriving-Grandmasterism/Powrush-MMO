/*!
 * My Mercy Journey Panel (Dedicated File)
 *
 * Full dedicated bevy_egui-style panel showing:
 * - Humble Origin prominently
 * - Clickable category filters (All / Harvest / Epiphany / War & Victory / Joy & Redemption / Council)
 * - Real-time filtered Legacy Timeline using build_filterable_legacy_threads()
 * - Dynamic stats from LegacyJournalRegistry
 *
 * Extracted for clean architecture from player_progress_ui.rs
 * TOLC 8 + 7 Living Mercy Gates aligned.
 */

use bevy::prelude::*;
use simulation::player_legacy_journal::{LegacyJournalRegistry, LegacyEventType, LegacyEntry, LegacyFilter};

#[derive(Component)]
pub struct MyMercyJourneyPanel;

#[derive(Component)]
struct HumbleOriginEchoText;

#[derive(Component)]
struct LegacyThreadsCountText;

#[derive(Component)]
struct ActiveFilterText;

#[derive(Component)]
struct LegacyEntry1;

#[derive(Component)]
struct LegacyEntry2;

#[derive(Component)]
struct LegacyEntry3;

#[derive(Component)]
struct LegacyEntry4;

#[derive(Component)]
struct FilterButton(LegacyFilter);

#[derive(Resource, Default, Clone, Copy, PartialEq, Eq)]
pub struct MyMercyJourneyFilter {
    pub active: LegacyFilter,
}

pub struct MyMercyJourneyPanelPlugin;

impl Plugin for MyMercyJourneyPanelPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MyMercyJourneyFilter>()
            .add_systems(Startup, spawn_my_mercy_journey_panel)
            .add_systems(Update, (
                toggle_my_mercy_journey_panel,
                handle_filter_button_clicks,
                update_my_mercy_journey_ui,
            ));
    }
}

fn spawn_my_mercy_journey_panel(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(6.0),
                    right: Val::Percent(2.5),
                    width: Val::Px(420.0),
                    max_height: Val::Percent(82.0),
                    padding: UiRect::all(Val::Px(18.0)),
                    border_radius: BorderRadius::all(Val::Px(16.0)),
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::clip(),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.07, 0.11, 0.97).into(),
                border_color: Color::srgb(0.35, 0.7, 0.95).into(),
                ..default()
            },
            MyMercyJourneyPanel,
            Name::new("MyMercyJourneyPanel"),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("MY MERCY JOURNEY", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 20.0, color: Color::srgb(0.7, 0.95, 0.85) }),
                style: Style { margin: UiRect::bottom(Val::Px(6.0)), ..default() },
                ..default()
            });

            parent.spawn((TextBundle {
                text: Text::from_section("Humble Origin: The journey begins with a single seed of mercy.", TextStyle { font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 12.5, color: Color::srgb(0.85, 0.92, 1.0) }),
                style: Style { margin: UiRect::bottom(Val::Px(6.0)), ..default() },
                ..default()
            }, HumbleOriginEchoText));

            parent.spawn((TextBundle {
                text: Text::from_section("Legacy Threads: 0  |  Cross-Realm: 0  |  Merciful Victories: 0", TextStyle { font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 11.5, color: Color::WHITE }),
                style: Style { margin: UiRect::bottom(Val::Px(6.0)), ..default() },
                ..default()
            }, LegacyThreadsCountText));

            // Clickable Filter Buttons Row
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    column_gap: Val::Px(6.0),
                    row_gap: Val::Px(4.0),
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                },
                ..default()
            }).with_children(|btns| {
                let filters = [
                    ("All", LegacyFilter::All),
                    ("Harvest", LegacyFilter::Harvest),
                    ("Epiphany", LegacyFilter::Epiphany),
                    ("War & Victory", LegacyFilter::WarVictory),
                    ("Joy & Redemption", LegacyFilter::JoyRedemption),
                    ("Council", LegacyFilter::Council),
                ];

                for (label, filter) in filters {
                    btns.spawn((
                        ButtonBundle {
                            style: Style {
                                padding: UiRect::horizontal(Val::Px(10.0)).with_vertical(Val::Px(4.0)),
                                border_radius: BorderRadius::all(Val::Px(8.0)),
                                ..default()
                            },
                            background_color: Color::srgb(0.15, 0.18, 0.22).into(),
                            border_color: Color::srgb(0.4, 0.5, 0.6).into(),
                            ..default()
                        },
                        FilterButton(filter),
                        Name::new(format!("FilterBtn_{}", label)),
                    )).with_children(|btn| {
                        btn.spawn(TextBundle {
                            text: Text::from_section(
                                label,
                                TextStyle { font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 10.5, color: Color::srgb(0.85, 0.9, 0.95) },
                            ),
                            ..default()
                        });
                    });
                }
            });

            parent.spawn((TextBundle {
                text: Text::from_section("Filter: All", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 11.5, color: Color::srgb(1.0, 0.9, 0.5) }),
                style: Style { margin: UiRect::bottom(Val::Px(6.0)), ..default() },
                ..default()
            }, ActiveFilterText));

            parent.spawn(TextBundle {
                text: Text::from_section("— LEGACY TIMELINE —", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 13.0, color: Color::srgb(1.0, 0.88, 0.6) }),
                style: Style { margin: UiRect::vertical(Val::Px(6.0)), ..default() },
                ..default()
            });

            parent.spawn((TextBundle { text: Text::from_section("• Loading...", TextStyle { font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 11.5, color: Color::WHITE }), ..default() }, LegacyEntry1));
            parent.spawn((TextBundle { text: Text::from_section("", TextStyle { font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 11.5, color: Color::WHITE }), style: Style { margin: UiRect::top(Val::Px(3.0)), ..default() }, ..default() }, LegacyEntry2));
            parent.spawn((TextBundle { text: Text::from_section("", TextStyle { font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 11.5, color: Color::srgb(0.7, 0.95, 0.85) }), style: Style { margin: UiRect::top(Val::Px(3.0)), ..default() }, ..default() }, LegacyEntry3));
            parent.spawn((TextBundle { text: Text::from_section("", TextStyle { font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 11.5, color: Color::srgb(0.6, 1.0, 0.7) }), style: Style { margin: UiRect::top(Val::Px(3.0)), ..default() }, ..default() }, LegacyEntry4));

            parent.spawn(TextBundle {
                text: Text::from_section("Click filter buttons above  •  Legacy grows with every merciful act", TextStyle { font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 9.5, color: Color::srgb(0.55, 0.65, 0.75) }),
                style: Style { margin: UiRect::top(Val::Px(10.0)), ..default() },
                ..default()
            });
        });
}

fn toggle_my_mercy_journey_panel(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Visibility, With<MyMercyJourneyPanel>>) {
    if keyboard.just_pressed(KeyCode::F2) {
        for mut visibility in query.iter_mut() {
            *visibility = if *visibility == Visibility::Hidden { Visibility::Visible } else { Visibility::Hidden };
        }
    }
}

/// Handle clicks on filter buttons
fn handle_filter_button_clicks(
    mut interaction_query: Query<(&Interaction, &FilterButton), Changed<Interaction>>,
    mut filter: ResMut<MyMercyJourneyFilter>,
) {
    for (interaction, filter_button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            filter.active = filter_button.0;
        }
    }
}

// === Filterable Legacy Threads with Clickable Buttons (Real Data) ===
fn update_my_mercy_journey_ui(
    legacy_registry: Option<Res<LegacyJournalRegistry>>,
    filter: Res<MyMercyJourneyFilter>,
    mut humble_text: Query<&mut Text, With<HumbleOriginEchoText>>,
    mut stats_text: Query<&mut Text, With<LegacyThreadsCountText>>,
    mut active_filter_text: Query<&mut Text, With<ActiveFilterText>>,
    mut entry1: Query<&mut Text, With<LegacyEntry1>>,
    mut entry2: Query<&mut Text, With<LegacyEntry2>>,
    mut entry3: Query<&mut Text, With<LegacyEntry3>>,
    mut entry4: Query<&mut Text, With<LegacyEntry4>>,
) {
    if let Some(registry) = legacy_registry {
        for mut text in active_filter_text.iter_mut() {
            let filter_name = match filter.active {
                LegacyFilter::All => "All",
                LegacyFilter::Harvest => "Harvest",
                LegacyFilter::Epiphany => "Epiphany",
                LegacyFilter::WarVictory => "War & Victory",
                LegacyFilter::JoyRedemption => "Joy & Redemption",
                LegacyFilter::Council => "Council",
                _ => "All",
            };
            text.sections[0].value = format!("Filter: {} (click buttons above)", filter_name);
        }

        for mut text in humble_text.iter_mut() {
            let origin = registry.mercy_journey_summary.signature_quote.clone()
                .unwrap_or_else(|| "The journey begins with a single seed of mercy.".to_string());
            text.sections[0].value = format!("Humble Origin: {}", origin);
        }

        for mut text in stats_text.iter_mut() {
            let thread_count = registry.legacy_thread_count.max(registry.legacy_threads.len() as u32);
            let cross_realm = registry.cross_realm_impact_score as u32;
            let victories = registry.forgiveness_waves_participated + registry.mercy_journey_summary.server_war_victories;
            text.sections[0].value = format!("Legacy Threads: {}  |  Cross-Realm: {}  |  Merciful Victories: {}", thread_count, cross_realm, victories);
        }

        let filtered_entries: Vec<&LegacyEntry> = registry
            .build_filterable_legacy_threads(filter.active)
            .into_iter()
            .take(4)
            .collect();

        let mut entries_iter = filtered_entries.into_iter();
        let entries = [&mut entry1, &mut entry2, &mut entry3, &mut entry4];
        let default_texts = [
            "• Humble seed planted — the journey begins",
            "• Sustainable harvest — abundance from mercy",
            "• Epiphany bloomed — True power serves the whole",
            "• Merciful Victory — Legacy Thread forged across realms",
        ];

        for (i, entry_query) in entries.iter_mut().enumerate() {
            if let Some(e) = entries_iter.next() {
                for mut text in entry_query.iter_mut() {
                    let icon = match e.event_type {
                        LegacyEventType::ServerWarVictory => "⚔️",
                        LegacyEventType::Harvest => "🌾",
                        LegacyEventType::Epiphany => "✨",
                        LegacyEventType::ProactiveJoy => "💫",
                        LegacyEventType::CouncilBloom => "🕊️",
                        _ => "•",
                    };
                    text.sections[0].value = format!("{} {} (Mercy +{:.0}, Valence +{:.2})", icon, e.description, e.mercy_impact, e.valence_delta);
                }
            } else {
                for mut text in entry_query.iter_mut() {
                    text.sections[0].value = default_texts[i].to_string();
                }
            }
        }
    }
}

// End of client/src/my_mercy_journey_panel.rs — Clean dedicated panel