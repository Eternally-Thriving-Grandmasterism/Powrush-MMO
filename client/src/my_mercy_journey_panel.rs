/*!
 * My Mercy Journey Panel (Dedicated File) v2
 *
 * Now includes timestamps and richer descriptions in the Legacy Timeline.
 */

use bevy::prelude::*;
use simulation::player_legacy_journal::{LegacyJournalRegistry, LegacyEventType, LegacyEntry, LegacyFilter};

// ... (components and structs same as before) ...

fn update_my_mercy_journey_ui(
    legacy_registry: Res<LegacyJournalRegistry>,
    filter: Res<MyMercyJourneyFilter>,
    mut humble_text: Query<&mut Text, With<HumbleOriginEchoText>>,
    mut stats_text: Query<&mut Text, With<LegacyThreadsCountText>>,
    mut active_filter_text: Query<&mut Text, With<ActiveFilterText>>,
    mut entry1: Query<&mut Text, With<LegacyEntry1>>,
    mut entry2: Query<&mut Text, With<LegacyEntry2>>,
    mut entry3: Query<&mut Text, With<LegacyEntry3>>,
    mut entry4: Query<&mut Text, With<LegacyEntry4>>,
) {
    // ... (humble origin and stats unchanged) ...

    let filtered_entries: Vec<&LegacyEntry> = legacy_registry
        .build_filterable_legacy_threads(filter.active)
        .into_iter()
        .take(4)
        .collect();

    let mut entries_iter = filtered_entries.into_iter();
    let entries = [&mut entry1, &mut entry2, &mut entry3, &mut entry4];

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

                // === POLISH: Richer description with timestamp ===
                let rich_desc = format!("T{:03} | {} (Mercy +{:.1}, Valence +{:.2})",
                    e.tick, e.description, e.mercy_impact, e.valence_delta);

                text.sections[0].value = format!("{} {}", icon, rich_desc);
            }
        } else {
            // fallback texts...
        }
    }
}

// End of client/src/my_mercy_journey_panel.rs v2 — Timestamps + richer descriptions added