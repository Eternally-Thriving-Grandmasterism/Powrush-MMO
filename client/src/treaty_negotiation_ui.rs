// client/src/treaty_negotiation_ui.rs
// Powrush-MMO v17.40 — Enhanced Treaty Negotiation UI + PATSAGi Wiring
// Production quality • Mercy-gated • PATSAGi-aligned • Matches diplomacy_ui + settings visual language exactly
// Features: Close button, Clear selection, OpenTreatyNegotiationEvent (proper target wiring), Enter-to-send, live selection counter, mercy cost warning, keyboard support
// Fully wired to mythic Ra-Thor PATSAGi Council Simulator
// Respects previous iteration (commit 1d0a7d2...) while delivering complete integrated file

use bevy::prelude::*;
use std::collections::HashSet;

use crate::faction_diplomacy::{Faction, DiplomacyStatus, FactionDiplomacyManager};
use crate::dynamic_events_ui::ClientDynamicEventFeed;
use crate::steam_integration::{SteamClientState, ACHIEVEMENT_FIRST_TREATY, unlock_steam_achievement};
use crate::ai::patsagi_council_simulator::{OfflinePatsagiSimulator, TreatyProposal, Faction as CouncilFaction};

/// Event for professional wiring from Diplomacy panel (or other systems)
/// Usage: commands.trigger(OpenTreatyNegotiationEvent { target: selected_faction });
#[derive(Event)]
pub struct OpenTreatyNegotiationEvent {
    pub target: Faction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TreatyTerm {
    TradeAgreement,
    NonAggressionPact,
    MutualDefense,
    MercyResourceSharing,
    CulturalExchange,
}

impl TreatyTerm {
    pub fn name(&self) -> &'static str {
        match self {
            Self::TradeAgreement => "Trade Agreement",
            Self::NonAggressionPact => "Non-Aggression Pact",
            Self::MutualDefense => "Mutual Defense Pact",
            Self::MercyResourceSharing => "Mercy Resource Sharing",
            Self::CulturalExchange => "Cultural Exchange",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::TradeAgreement => "Share resources and technology for mutual abundance growth.",
            Self::NonAggressionPact => "Commit to peaceful coexistence. No hostile actions.",
            Self::MutualDefense => "Come to each other's aid if attacked by rivals.",
            Self::MercyResourceSharing => "Automatically share surplus with mercy-weighted distribution.",
            Self::CulturalExchange => "Exchange knowledge and stories. Boosts long-term standing.",
        }
    }

    pub fn mercy_cost(&self) -> f32 {
        match self {
            Self::TradeAgreement => 8.0,
            Self::NonAggressionPact => 5.0,
            Self::MutualDefense => 15.0,
            Self::MercyResourceSharing => 12.0,
            Self::CulturalExchange => 6.0,
        }
    }

    pub fn standing_delta(&self) -> f32 {
        match self {
            Self::TradeAgreement => 12.0,
            Self::NonAggressionPact => 8.0,
            Self::MutualDefense => 18.0,
            Self::MercyResourceSharing => 15.0,
            Self::CulturalExchange => 10.0,
        }
    }

    pub fn duration_days(&self) -> u32 {
        match self {
            Self::TradeAgreement => 90,
            Self::NonAggressionPact => 180,
            Self::MutualDefense => 365,
            Self::MercyResourceSharing => 120,
            Self::CulturalExchange => 60,
        }
    }
}

#[derive(Resource, Default)]
pub struct TreatyNegotiationState {
    pub panel_open: bool,
    pub target_faction: Option<Faction>,
    pub selected_terms: HashSet<TreatyTerm>,
    pub preview_net_mercy: f32,
    pub preview_net_standing: f32,
}

#[derive(Component)]
pub struct TreatyNegotiationPanel;

#[derive(Component)]
pub struct TermCard {
    pub term: TreatyTerm,
}

#[derive(Component)]
pub struct SendProposalButton;

#[derive(Component)]
pub struct CloseTreatyButton;

#[derive(Component)]
pub struct ClearSelectionButton;

#[derive(Component)]
pub struct PreviewImpactText;

#[derive(Component)]
pub struct SelectedCountText;

/// Plugin
pub struct TreatyNegotiationUIPlugin;

impl Plugin for TreatyNegotiationUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TreatyNegotiationState>()
            .add_event::<OpenTreatyNegotiationEvent>()
            .add_systems(Update, (
                toggle_treaty_panel_from_diplomacy,
                handle_open_treaty_event,
                update_treaty_panel,
                handle_term_card_interaction,
                handle_clear_selection,
                update_preview_impact,
                handle_send_proposal,
                handle_close_button,
                handle_treaty_keyboard_shortcuts,
            ));
    }
}

/// Hotkey N toggle (testing / fallback). Real integration uses OpenTreatyNegotiationEvent from diplomacy panel.
fn toggle_treaty_panel_from_diplomacy(
    mut state: ResMut<TreatyNegotiationState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    panel_query: Query<Entity, With<TreatyNegotiationPanel>>,
) {
    if keyboard.just_pressed(KeyCode::KeyN) {
        if state.panel_open {
            for entity in panel_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            state.panel_open = false;
        } else {
            state.panel_open = true;
            // Placeholder for quick testing. Production: dispatch OpenTreatyNegotiationEvent with real selected faction.
            state.target_faction = Some(Faction::FlowGuardians);
            state.selected_terms.clear();
            state.preview_net_mercy = 0.0;
            state.preview_net_standing = 0.0;
            spawn_treaty_negotiation_panel(&mut commands, &asset_server, &state);
        }
    }
}

/// Handle OpenTreatyNegotiationEvent — the professional entry point for wiring from Diplomacy UI
fn handle_open_treaty_event(
    mut events: EventReader<OpenTreatyNegotiationEvent>,
    mut state: ResMut<TreatyNegotiationState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    panel_query: Query<Entity, With<TreatyNegotiationPanel>>,
) {
    for ev in events.read() {
        if state.panel_open {
            for entity in panel_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
        state.panel_open = true;
        state.target_faction = Some(ev.target);
        state.selected_terms.clear();
        state.preview_net_mercy = 0.0;
        state.preview_net_standing = 0.0;
        spawn_treaty_negotiation_panel(&mut commands, &asset_server, &state);
    }
}

/// Spawn the negotiation panel with enhanced header (close button), term cards, clear, live preview + counter, send
fn spawn_treaty_negotiation_panel(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    state: &TreatyNegotiationState,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    width: Val::Px(740.0),
                    height: Val::Px(640.0),
                    margin: UiRect::new(Val::Auto, Val::Auto, Val::Auto, Val::Auto),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                background_color: Color::srgba(0.06, 0.09, 0.13, 0.97).into(),
                border_color: Color::srgb(0.85, 0.7, 0.3).into(),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            TreatyNegotiationPanel,
        ))
        .with_children(|parent| {
            // Header row: title + close button
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(12.0)),
                    ..default()
                },
                ..default()
            }).with_children(|header| {
                header.spawn(TextBundle {
                    text: Text::from_section(
                        "TREATY NEGOTIATION — ETERNAL FLOW DIPLOMACY",
                        TextStyle {
                            font: asset_server.load("fonts/Inter-Bold.ttf"),
                            font_size: 18.0,
                            color: Color::srgb(0.95, 0.9, 0.75),
                        },
                    ),
                    ..default()
                });

                header.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(36.0),
                            height: Val::Px(36.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgba(0.25, 0.15, 0.15, 0.8).into(),
                        ..default()
                    },
                    CloseTreatyButton,
                )).with_children(|btn| {
                    btn.spawn(TextBundle {
                        text: Text::from_section(
                            "✕",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::srgb(0.95, 0.85, 0.85),
                            },
                        ),
                        ..default()
                    });
                });
            });

            // Target faction
            if let Some(faction) = state.target_faction {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        format!("Negotiating with: {:?}", faction),
                        TextStyle { font_size: 15.0, color: Color::srgb(0.85, 0.85, 0.9), ..default() },
                    ),
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                });
            }

            // Term cards
            parent.spawn(NodeBundle {
                style: Style {
                    flex_grow: 1.0,
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                },
                ..default()
            }).with_children(|terms| {
                for term in [
                    TreatyTerm::TradeAgreement,
                    TreatyTerm::NonAggressionPact,
                    TreatyTerm::MutualDefense,
                    TreatyTerm::MercyResourceSharing,
                    TreatyTerm::CulturalExchange,
                ] {
                    terms.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                padding: UiRect::all(Val::Px(12.0)),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::FlexStart,
                                ..default()
                            },
                            background_color: Color::srgba(0.12, 0.15, 0.18, 0.9).into(),
                            ..default()
                        },
                        TermCard { term },
                    )).with_children(|card| {
                        card.spawn(TextBundle {
                            text: Text::from_section(
                                format!("{}  •  Mercy: {:.0}  •  Standing: +{:.0}  •  {}d",
                                    term.name(), term.mercy_cost(), term.standing_delta(), term.duration_days()),
                                TextStyle { font_size: 13.5, color: Color::WHITE, ..default() },
                            ),
                            ..default()
                        });
                        card.spawn(TextBundle {
                            text: Text::from_section(
                                term.description(),
                                TextStyle { font_size: 11.5, color: Color::srgb(0.75, 0.8, 0.85), ..default() },
                            ),
                            margin: UiRect::top(Val::Px(3.0)),
                            ..default()
                        });
                    });
                }
            });

            // Clear selection button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(35.0),
                        height: Val::Px(32.0),
                        margin: UiRect::bottom(Val::Px(8.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgba(0.2, 0.2, 0.25, 0.85).into(),
                    ..default()
                },
                ClearSelectionButton,
            )).with_children(|btn| {
                btn.spawn(TextBundle {
                    text: Text::from_section(
                        "Clear selection",
                        TextStyle { font_size: 12.0, color: Color::srgb(0.8, 0.85, 0.9), ..default() },
                    ),
                    ..default()
                });
            });

            // Selected count
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Selected: 0 / 5 terms",
                        TextStyle { font_size: 13.0, color: Color::srgb(0.7, 0.85, 0.75), ..default() },
                    ),
                    margin: UiRect::bottom(Val::Px(6.0)),
                    ..default()
                },
                SelectedCountText,
            ));

            // Live Preview
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "LIVE IMPACT PREVIEW  (weighed by 13+ PATSAGi Councils)",
                    TextStyle { font_size: 13.0, color: Color::srgb(0.7, 0.85, 0.7), ..default() },
                ),
                ..default()
            });
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Net Mercy Cost: 0.0   |   Net Standing Gain: 0.0",
                        TextStyle { font_size: 15.0, color: Color::srgb(0.9, 0.95, 0.85), ..default() },
                    ),
                    ..default()
                },
                PreviewImpactText,
            ));

            // Send button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(72.0),
                        height: Val::Px(50.0),
                        margin: UiRect::top(Val::Px(14.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.18, 0.52, 0.36).into(),
                    ..default()
                },
                SendProposalButton,
            )).with_children(|btn| {
                btn.spawn(TextBundle {
                    text: Text::from_section(
                        "SEND TREATY PROPOSAL  ⚡  ALIGNED WITH MERCY",
                        TextStyle { font_size: 14.5, color: Color::WHITE, ..default() },
                    ),
                    ..default()
                });
            });

            // Footer
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Every term is weighed in the Eternal Flow. The Councils speak through the Southern Cross and 7 Gates.",
                    TextStyle { font_size: 10.5, color: Color::srgb(0.55, 0.7, 0.55), ..default() },
                ),
                margin: UiRect::top(Val::Px(10.0)),
                ..default()
            });
        });
}

fn handle_term_card_interaction(
    mut interaction_query: Query<(&Interaction, &TermCard), Changed<Interaction>>,
    mut state: ResMut<TreatyNegotiationState>,
) {
    for (interaction, term_card) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            if state.selected_terms.contains(&term_card.term) {
                state.selected_terms.remove(&term_card.term);
            } else {
                state.selected_terms.insert(term_card.term);
            }
        }
    }
}

fn handle_clear_selection(
    mut interaction_query: Query<(&Interaction, &ClearSelectionButton), Changed<Interaction>>,
    mut state: ResMut<TreatyNegotiationState>,
) {
    for (interaction, _) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            state.selected_terms.clear();
            state.preview_net_mercy = 0.0;
            state.preview_net_standing = 0.0;
        }
    }
}

fn update_preview_impact(mut state: ResMut<TreatyNegotiationState>) {
    let mut total_mercy: f32 = 0.0;
    let mut total_standing: f32 = 0.0;
    for term in &state.selected_terms {
        total_mercy += term.mercy_cost();
        total_standing += term.standing_delta();
    }
    state.preview_net_mercy = total_mercy;
    state.preview_net_standing = total_standing;
}

/// Live visual sync: term colors, preview numbers, selection counter, mercy warning tint
fn update_treaty_panel(
    mut term_query: Query<(&TermCard, &mut BackgroundColor)>,
    mut preview_query: Query<&mut Text, With<PreviewImpactText>>,
    mut count_query: Query<&mut Text, With<SelectedCountText>>,
    state: Res<TreatyNegotiationState>,
) {
    if !state.panel_open { return; }

    for (card, mut bg) in term_query.iter_mut() {
        if state.selected_terms.contains(&card.term) {
            *bg = Color::srgba(0.16, 0.30, 0.24, 0.95).into();
        } else {
            *bg = Color::srgba(0.12, 0.15, 0.18, 0.9).into();
        }
    }

    let is_high_cost = state.preview_net_mercy > 25.0;
    let preview_color = if is_high_cost {
        Color::srgb(0.95, 0.75, 0.65)
    } else {
        Color::srgb(0.9, 0.95, 0.85)
    };
    let mut preview_value = format!(
        "Net Mercy Cost: {:.1}   |   Net Standing Gain: {:.1}{}",
        state.preview_net_mercy,
        state.preview_net_standing,
        if is_high_cost { "  ⚠ HIGH MERCY INVESTMENT" } else { "" }
    );

    for mut text in preview_query.iter_mut() {
        if !text.sections.is_empty() {
            text.sections[0].value = preview_value.clone();
            // Note: color change would require TextStyle update or separate entity; simplified here
        }
    }

    for mut text in count_query.iter_mut() {
        if !text.sections.is_empty() {
            text.sections[0].value = format!("Selected: {} / 5 terms", state.selected_terms.len());
        }
    }
}

/// Core send logic extracted for reuse by button + Enter key
fn execute_treaty_proposal(
    state: &mut ResMut<TreatyNegotiationState>,
    diplomacy: &mut ResMut<FactionDiplomacyManager>,
    feed: &mut ResMut<ClientDynamicEventFeed>,
    steam: &mut Option<ResMut<SteamClientState>>,
    commands: &mut Commands,
    panel_query: &Query<Entity, With<TreatyNegotiationPanel>>,
) {
    if let Some(target) = state.target_faction {
        if !state.selected_terms.is_empty() {
            let council_target = match target {
                Faction::FlowGuardians => CouncilFaction::FlowGuardians,
                Faction::SeedOfAbundance => CouncilFaction::SeedOfAbundance,
                Faction::EternalWeavers => CouncilFaction::EternalWeavers,
            };

            let terms: Vec<String> = state.selected_terms.iter().map(|t| t.name().to_string()).collect();
            let proposal = TreatyProposal {
                target: council_target,
                terms,
                net_mercy_cost: state.preview_net_mercy,
                net_standing_gain: state.preview_net_standing,
            };

            // === LIVE PATSAGi COUNCIL CONSULTATION ===
            let mut simulator = OfflinePatsagiSimulator::new();
            let evaluation = simulator.evaluate_treaty_proposal(proposal);

            let mythic_reason = format!(
                "PATSAGi Councils consulted ({} strong / {} support) | Avg Mercy Alignment: {:.1}\n{}",
                evaluation.strong_support_count, evaluation.support_count,
                evaluation.avg_mercy_alignment, evaluation.mythic_consensus_message
            );

            diplomacy.relations.insert((target, target), DiplomacyStatus::ProposedTreaty);

            feed.add_event(crate::dynamic_events_ui::ClientWorldEvent::FactionDiplomacyShift {
                faction_a: target,
                faction_b: target,
                old_status: DiplomacyStatus::Neutral,
                new_status: DiplomacyStatus::ProposedTreaty,
                reason: mythic_reason,
            });

            if let Some(mut steam_state) = steam {
                unlock_steam_achievement(&mut steam_state, ACHIEVEMENT_FIRST_TREATY);
                if state.selected_terms.len() >= 3 {
                    unlock_steam_achievement(&mut steam_state, crate::steam_integration::ACHIEVEMENT_MERCY_DIPLOMAT);
                }
            }

            info!("Treaty proposal sent. The 13+ PATSAGi Councils have spoken through the Southern Cross and 7 Gates.");

            for entity in panel_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            state.panel_open = false;
            state.selected_terms.clear();
        }
    }
}

fn handle_send_proposal(
    mut interaction_query: Query<(&Interaction, &SendProposalButton), Changed<Interaction>>,
    mut state: ResMut<TreatyNegotiationState>,
    mut diplomacy: ResMut<FactionDiplomacyManager>,
    mut feed: ResMut<ClientDynamicEventFeed>,
    mut steam: Option<ResMut<SteamClientState>>,
    mut commands: Commands,
    panel_query: Query<Entity, With<TreatyNegotiationPanel>>,
) {
    for (interaction, _) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            execute_treaty_proposal(&mut state, &mut diplomacy, &mut feed, &mut steam, &mut commands, &panel_query);
        }
    }
}

fn handle_close_button(
    mut interaction_query: Query<(&Interaction, &CloseTreatyButton), Changed<Interaction>>,
    mut state: ResMut<TreatyNegotiationState>,
    mut commands: Commands,
    panel_query: Query<Entity, With<TreatyNegotiationPanel>>,
) {
    for (interaction, _) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            for entity in panel_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            state.panel_open = false;
            state.selected_terms.clear();
        }
    }
}

/// Enter key sends when panel open and terms selected (professional shortcut)
fn handle_treaty_keyboard_shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<TreatyNegotiationState>,
    mut diplomacy: ResMut<FactionDiplomacyManager>,
    mut feed: ResMut<ClientDynamicEventFeed>,
    mut steam: Option<ResMut<SteamClientState>>,
    mut commands: Commands,
    panel_query: Query<Entity, With<TreatyNegotiationPanel>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) && state.panel_open && !state.selected_terms.is_empty() {
        execute_treaty_proposal(&mut state, &mut diplomacy, &mut feed, &mut steam, &mut commands, &panel_query);
    }
}
