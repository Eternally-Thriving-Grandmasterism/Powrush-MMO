/*!
 * Player Progress UI + Full Dedicated 'My Mercy Journey' Panel
 * + Filterable Legacy Threads View
 *
 * Now supports category filtering using build_filterable_legacy_threads():
 * - Harvest
 * - Epiphany
 * - War & Victory
 * - Joy & Redemption
 * - Council
 *
 * Keyboard filtering (keys 1-5) with visual active filter indicator.
 * Fully dynamic and data-driven from the live LegacyJournalRegistry.
 */

use bevy::prelude::*;
use simulation::player_persistence::PlayerSaveData;
use simulation::player_legacy_journal::{LegacyJournalRegistry, LegacyEventType, LegacyEntry, LegacyFilter};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Component)]
struct PlayerProgressPanel;

#[derive(Component)]
struct EpiphanyCountText;

#[derive(Component)]
struct MuscleMemoryText;

#[derive(Component)]
struct ActiveMultiplierText;

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

/// Simple filter state for the My Mercy Journey panel
#[derive(Resource, Default, Clone, Copy, PartialEq, Eq)]
pub struct MyMercyJourneyFilter {
    pub active: LegacyFilter,
}

pub struct PlayerProgressUIPlugin;

impl Plugin for PlayerProgressUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MyMercyJourneyFilter>()
            .add_systems(Startup, spawn_player_progress_ui)
            .add_systems(Update, (
                toggle_player_progress_ui,
                update_player_progress_ui,
                handle_mercy_journey_filters,
                update_my_mercy_journey_ui,
            ));
    }
}

fn spawn_player_progress_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(6.0),
                    right: Val::Percent(2.5),
                    width: Val::Px(400.0),
                    max_height: Val::Percent(80.0),
                    padding: UiRect::all(Val::Px(20.0)),
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
            PlayerProgressPanel,
            Name::new("MyMercyJourneyPanel"),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("MY MERCY JOURNEY", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 20.0, color: Color::srgb(0.7, 0.95, 0.85) }),
                style: Style { margin: UiRect::bottom(Val::Px(8.0)), ..default() },
                ..default()
            });

            parent.spawn((TextBundle {
                text: Text::from_section("Humble Origin: The journey begins with a single seed of mercy.", TextStyle { font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 12.5, color: Color::srgb(0.85, 0.92, 1.0) }),
                style: Style { margin: UiRect::bottom(Val::Px(8.0)), ..default() },
                ..default()
            }, HumbleOriginEchoText));

            parent.spawn((TextBundle {
                text: Text::from_section("Legacy Threads: 0  |  Cross-Realm: 0  |  Merciful Victories: 0", TextStyle { font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 11.5, color: Color::WHITE }),
                style: Style { margin: UiRect::bottom(Val::Px(6.0)), ..default() },
                ..default()
            }, LegacyThreadsCountText));

            // Active Filter indicator
            parent.spawn((TextBundle {
                text: Text::from_section("Filter: All", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 12.0, color: Color::srgb(1.0, 0.9, 0.5) }),
                style: Style { margin: UiRect::bottom(Val::Px(6.0)), ..default() },
                ..default()
            }, ActiveFilterText));

            parent.spawn(TextBundle {
                text: Text::from_section("— LEGACY TIMELINE (1-5 to filter) —", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 13.0, color: Color::srgb(1.0, 0.88, 0.6) }),
                style: Style { margin: UiRect::vertical(Val::Px(6.0)), ..default() },
                ..default()
            });

            parent.spawn((TextBundle { text: Text::from_section("• Loading...", TextStyle { font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 11.5, color: Color::WHITE }), ..default() }, LegacyEntry1));
            parent.spawn((TextBundle { text: Text::from_section("", TextStyle { font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 11.5, color: Color::WHITE }), style: Style { margin: UiRect::top(Val::Px(3.0)), ..default() }, ..default() }, LegacyEntry2));
            parent.spawn((TextBundle { text: Text::from_section("", TextStyle { font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 11.5, color: Color::srgb(0.7, 0.95, 0.85) }), style: Style { margin: UiRect::top(Val::Px(3.0)), ..default() }, ..default() }, LegacyEntry3));
            parent.spawn((TextBundle { text: Text::from_section("", TextStyle { font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 11.5, color: Color::srgb(0.6, 1.0, 0.7) }), style: Style { margin: UiRect::top(Val::Px(3.0)), ..default() }, ..default() }, LegacyEntry4));

            parent.spawn(TextBundle {
                text: Text::from_section("F2 toggle  •  1=All  2=Harvest  3=Epiphany  4=War/Victory  5=Joy/Redemption", TextStyle { font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 9.5, color: Color::srgb(0.55, 0.65, 0.75) }),
                style: Style { margin: UiRect::top(Val::Px(12.0)), ..default() },
                ..default()
            });
        });
}

fn toggle_player_progress_ui(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Visibility, With<PlayerProgressPanel>>) {
    if keyboard.just_pressed(KeyCode::F2) {
        for mut visibility in query.iter_mut() {
            *visibility = if *visibility == Visibility::Hidden { Visibility::Visible } else { Visibility::Hidden };
        }
    }
}

/// Keyboard-driven filter handling for My Mercy Journey panel
fn handle_mercy_journey_filters(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut filter: ResMut<MyMercyJourneyFilter>,
) {
    if keyboard.just_pressed(KeyCode::Digit1) || keyboard.just_pressed(KeyCode::Numpad1) {
        filter.active = LegacyFilter::All;
    }
    if keyboard.just_pressed(KeyCode::Digit2) || keyboard.just_pressed(KeyCode::Numpad2) {
        filter.active = LegacyFilter::Harvest;
    }
    if keyboard.just_pressed(KeyCode::Digit3) || keyboard.just_pressed(KeyCode::Numpad3) {
        filter.active = LegacyFilter::Epiphany;
    }
    if keyboard.just_pressed(KeyCode::Digit4) || keyboard.just_pressed(KeyCode::Numpad4) {
        filter.active = LegacyFilter::WarVictory;
    }
    if keyboard.just_pressed(KeyCode::Digit5) || keyboard.just_pressed(KeyCode::Numpad5) {
        filter.active = LegacyFilter::JoyRedemption;
    }
}

fn update_player_progress_ui(save_data: Res<PlayerSaveData>, mut epiphany_text: Query<&mut Text, With<EpiphanyCountText>>, mut muscle_text: Query<&mut Text, With<MuscleMemoryText>>, mut multiplier_text: Query<&mut Text, With<ActiveMultiplierText>>) {
    for mut text in epiphany_text.iter_mut() { text.sections[0].value = format!("Epiphanies: {}", save_data.epiphanies.len()); }
    for mut text in muscle_text.iter_mut() { text.sections[0].value = format!("Muscle Memory: {:.2}x", save_data.muscle_memory_level); }
    for mut text in multiplier_text.iter_mut() {
        if save_data.has_active_multiplier() {
            text.sections[0].value = format!("Multiplier: {:.2}x (active)", save_data.temporary_harvest_multiplier);
            text.sections[0].style.color = Color::srgb(1.0, 0.85, 0.4);
        } else {
            text.sections[0].value = "Multiplier: 1.00x (inactive)".to_string();
            text.sections[0].style.color = Color::srgb(0.7, 0.7, 0.7);
        }
    }
}

// === Filterable Legacy Threads View (Real Data + Category Filters) ===
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
        // Update active filter display
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
            text.sections[0].value = format!("Filter: {} (press 1-5)", filter_name);
        }

        // Humble Origin
        for mut text in humble_text.iter_mut() {
            let origin = registry.mercy_journey_summary.signature_quote.clone()
                .unwrap_or_else(|| "The journey begins with a single seed of mercy.".to_string());
            text.sections[0].value = format!("Humble Origin: {}", origin);
        }

        // Stats
        for mut text in stats_text.iter_mut() {
            let thread_count = registry.legacy_thread_count.max(registry.legacy_threads.len() as u32);
            let cross_realm = registry.cross_realm_impact_score as u32;
            let victories = registry.forgiveness_waves_participated + registry.mercy_journey_summary.server_war_victories;
            text.sections[0].value = format!("Legacy Threads: {}  |  Cross-Realm: {}  |  Merciful Victories: {}", thread_count, cross_realm, victories);
        }

        // === Real filtered data using build_filterable_legacy_threads() ===
        let filtered_entries: Vec<&LegacyEntry> = registry
            .build_filterable_legacy_threads(filter.active)
            .into_iter()
            .take(4)
            .collect();

        let mut entries_iter = filtered_entries.into_iter();

        // Populate 4 entries dynamically
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

// End of client/src/player_progress_ui.rs — Filterable Legacy Threads View