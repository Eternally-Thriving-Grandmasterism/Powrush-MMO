/*!
 * Faction Reputation Visual UI
 *
 * v1.2 | Refactored to use shared faction components from crate::faction.
 * No more duplicate definitions.
 *
 * AG-SML v1.0 | TOLC 8
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::faction::{FactionMembership, FactionStanding};

/// Resource holding the local player's faction reputation data.
#[derive(Resource, Default, Clone)]
pub struct FactionReputationUIState {
    pub factions: Vec<FactionReputationEntry>,
}

#[derive(Clone, Debug)]
pub struct FactionReputationEntry {
    pub faction_id: u64,
    pub faction_name: String,
    pub standing: f32,
    pub description: String,
}

pub struct FactionReputationUIPlugin;

impl Plugin for FactionReputationUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .init_resource::<FactionReputationUIState>()
            .add_systems(Update, (
                sync_replicated_faction_standing,
                faction_reputation_window,
            ));
    }
}

/// Syncs replicated FactionStanding + FactionMembership into UI state.
fn sync_replicated_faction_standing(
    mut ui_state: ResMut<FactionReputationUIState>,
    faction_query: Query<(&FactionMembership, &FactionStanding)>,
) {
    let mut new_entries = Vec::new();

    for (membership, standing) in faction_query.iter() {
        let name = match membership.faction_id {
            1 => "The Radiant Accord".to_string(),
            2 => "The Silent Veil".to_string(),
            _ => format!("Faction {}", membership.faction_id),
        };

        let desc = match membership.faction_id {
            1 => "Builders of abundance and mercy".to_string(),
            2 => "Guardians of hidden knowledge".to_string(),
            _ => "Aligned faction".to_string(),
        };

        new_entries.push(FactionReputationEntry {
            faction_id: membership.faction_id,
            faction_name: name,
            standing: standing.standing,
            description: desc,
        });
    }

    if !new_entries.is_empty() {
        ui_state.factions = new_entries;
    }
}

fn faction_reputation_window(
    mut contexts: EguiContexts,
    ui_state: Res<FactionReputationUIState>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("⚡ Faction Reputation — Eternal Standing")
        .default_pos([420.0, 80.0])
        .default_size([320.0, 280.0])
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Your Standing in the Eternal Flow");
            ui.separator();

            if ui_state.factions.is_empty() {
                ui.label("You have not yet aligned with any factions.");
                ui.label("Participate in ToFaction distributions to begin building reputation.");
                ui.add_space(8.0);
            } else {
                for entry in &ui_state.factions {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.strong(&entry.faction_name);
                            ui.label(format!("(ID: {})", entry.faction_id));
                        });

                        ui.label(&entry.description);

                        let progress = (entry.standing / 5.0).clamp(0.0, 1.0);
                        ui.add(egui::ProgressBar::new(progress)
                            .text(format!("Standing: {:.1} / 5.0", entry.standing)));

                        let color = if entry.standing >= 3.5 {
                            egui::Color32::from_rgb(80, 200, 120)
                        } else if entry.standing >= 2.0 {
                            egui::Color32::from_rgb(200, 180, 80)
                        } else {
                            egui::Color32::from_rgb(180, 140, 80)
                        };

                        ui.colored_label(color, if entry.standing >= 3.5 { "Highly Respected" } else if entry.standing >= 2.0 { "Respected" } else { "Neutral" });
                    });
                    ui.add_space(6.0);
                }
            }

            ui.separator();
            ui.small("Standing increases when you participate in faction distributions.");
            ui.small("Higher standing = greater rewards from ProportionalToStanding.");
        });
}
